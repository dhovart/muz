use anyhow::{anyhow, Result};
use awedio::{
    sounds::{
        self,
        wrappers::{AdjustableVolume, CompletionNotifier, Controller, Pausable, Stoppable},
    },
    Sound,
};
use std::thread;
use std::{
    sync::mpsc::{self, Sender},
    time::Duration,
};

use crate::player::{playback::PlaybackEvent, sound::ProgressUpdate, track::Track};

pub trait PlaybackDriver: Send {
    fn send_command(&mut self, command: AudioCommand) -> Result<()>;
    fn get_command_sender(&self) -> Sender<AudioCommand>;
}

pub enum AudioCommand {
    Play(Track, Sender<PlaybackEvent>),
    Pause,
    Resume,
    Stop,
    Clear,
    SetVolume(f32),
    Seek(Duration),
    Exit,
}

pub struct DefaultPlaybackDriver {
    command_sender: Sender<AudioCommand>,
}

type SoundController = Controller<
    ProgressUpdate<CompletionNotifier<AdjustableVolume<Stoppable<Pausable<Box<dyn Sound>>>>>>,
>;

impl DefaultPlaybackDriver {
    #[allow(clippy::new_ret_no_self)]
    pub fn new(volume: f32) -> impl PlaybackDriver {
        let (command_sender, command_receiver) = mpsc::channel();

        // NOTE: CpalBackend is not Send, using a dedicated thread as a workaround
        thread::spawn(move || {
            let (mut manager, backend) = awedio::start().expect("Failed to start audio manager");
            let mut controller: Option<SoundController> = None;
            let mut volume = volume.clamp(0.0, 1.0);

            while let Ok(cmd) = command_receiver.recv() {
                match cmd {
                    AudioCommand::Play(track, playback_sender) => {
                        match sounds::open_file(&track.path) {
                            Ok(sound) => {
                                let progress_sender = playback_sender.clone();

                                let sound = sound
                                    .pausable()
                                    .stoppable()
                                    .with_adjustable_volume_of(volume);
                                let (sound, notifier) = sound.with_completion_notifier();

                                let sound = ProgressUpdate::new(
                                    sound,
                                    track.total_frames,
                                    Box::new(move |progress| {
                                        progress_sender
                                            .send(PlaybackEvent::Progress(progress as u64))
                                            .unwrap();
                                    }),
                                );
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
                    AudioCommand::Stop => {
                        if let Some(ctrl) = controller.as_mut() {
                            ctrl.set_stopped();
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
                    AudioCommand::SetVolume(vol) => {
                        volume = vol.clamp(0.0, 1.0);
                        if let Some(ctrl) = controller.as_mut() {
                            ctrl.set_volume(volume);
                        }
                    }
                    AudioCommand::Seek(duration) => {}
                    AudioCommand::Clear => {
                        controller = None;
                        manager.clear();
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
