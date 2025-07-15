use anyhow::{anyhow, Result};
use awedio::{
    sounds::{
        self,
        wrappers::{AdjustableVolume, CompletionNotifier, Controller, Pausable},
    },
    Sound,
};
use std::sync::mpsc::{self, Sender};
use std::thread;

use crate::player::{playback::PlaybackEvent, sound::CompletionTracking, track::Track};

pub trait PlaybackDriver: Send {
    fn send_command(&mut self, command: AudioCommand) -> Result<()>;
    fn get_command_sender(&self) -> Sender<AudioCommand>;
}

pub enum AudioCommand {
    Play(Track, Sender<PlaybackEvent>),
    Pause,
    Resume,
    Stop,
    SetVolume(f32),
    Exit,
}

pub struct DefaultPlaybackDriver {
    command_sender: Sender<AudioCommand>,
}

type SoundController =
    Controller<CompletionTracking<CompletionNotifier<AdjustableVolume<Pausable<Box<dyn Sound>>>>>>;

impl DefaultPlaybackDriver {
    #[allow(clippy::new_ret_no_self)]
    pub fn new() -> impl PlaybackDriver {
        let (command_sender, command_receiver) = mpsc::channel();

        // NOTE: CpalBackend is not Send, using a dedicated thread as a workaround
        thread::spawn(move || {
            let (mut manager, backend) = awedio::start().expect("Failed to start audio manager");
            let mut controller: Option<SoundController> = None;

            while let Ok(cmd) = command_receiver.recv() {
                match cmd {
                    AudioCommand::Play(track, playback_sender) => {
                        match sounds::open_file(&track.path) {
                            Ok(sound) => {
                                let sound = sound.pausable().with_adjustable_volume();
                                let (sound, notifier) = sound.with_completion_notifier();
                                let sound = CompletionTracking::new(sound, track.total_frames);
                                let (sound, ctrl) = sound.controllable();

                                controller = Some(ctrl);

                                thread::spawn(move || {
                                    notifier.recv().unwrap();
                                    playback_sender.send(PlaybackEvent::TrackCompleted)
                                });

                                manager.play(Box::new(sound));
                            }
                            Err(err) => {
                                playback_sender
                                    .send(PlaybackEvent::FailedOpeningFile(err.into()))
                                    .unwrap();
                            }
                        }
                    }
                    AudioCommand::Pause => {
                        if let Some(ctrl) = controller.as_mut() {
                            ctrl.set_paused(true);
                        }
                    }
                    AudioCommand::Resume => {
                        if let Some(ctrl) = controller.as_mut() {
                            ctrl.set_paused(false);
                        }
                    }
                    AudioCommand::Stop => {
                        if let Some(ctrl) = controller.as_mut() {
                            ctrl.set_paused(true);
                        }
                        controller = None;
                    }
                    AudioCommand::SetVolume(vol) => {
                        if let Some(ctrl) = controller.as_mut() {
                            ctrl.set_volume(vol);
                        }
                    }
                    AudioCommand::Exit => break,
                }
            }
            drop(backend);
        });

        Self { command_sender }
    }
}

impl PlaybackDriver for DefaultPlaybackDriver {
    fn send_command(&mut self, command: AudioCommand) -> Result<()> {
        self.command_sender
            .send(command)
            .map_err(|e| anyhow!("Failed to send command: {}", e))
    }

    fn get_command_sender(&self) -> Sender<AudioCommand> {
        self.command_sender.clone()
    }
}

impl Drop for DefaultPlaybackDriver {
    fn drop(&mut self) {
        let _ = self.command_sender.send(AudioCommand::Exit);
    }
}
