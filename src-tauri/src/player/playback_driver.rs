use anyhow::{anyhow, Result};
use awedio::{
    sounds::{
        self,
        wrappers::{AdjustableVolume, Controller, Pausable},
    },
    Sound,
};
use std::path::PathBuf;
use std::sync::mpsc::{self, Sender};
use std::thread;

pub trait PlaybackDriver: Send {
    fn send_command(&mut self, command: AudioCommand) -> Result<()>;
    fn get_command_sender(&self) -> Sender<AudioCommand>;
}

pub enum AudioCommand {
    LoadFile(PathBuf),
    Play(Box<dyn FnOnce() + Send>),
    Pause,
    Resume,
    Stop,
    SetVolume(f32),
    Exit,
}

pub struct DefaultPlaybackDriver {
    command_sender: Sender<AudioCommand>,
}

type SoundController = Controller<AdjustableVolume<Pausable<Box<dyn Sound>>>>;

impl DefaultPlaybackDriver {
    #[allow(clippy::new_ret_no_self)]
    pub fn new() -> impl PlaybackDriver {
        let (command_sender, command_receiver) = mpsc::channel();

        thread::spawn(move || {
            let (mut manager, backend) = awedio::start().expect("Failed to start audio manager");
            let mut controller: Option<SoundController> = None;
            let mut current_sound = None;

            while let Ok(cmd) = command_receiver.recv() {
                match cmd {
                    AudioCommand::LoadFile(path) => {
                        let (sound, ctrl) = sounds::open_file(&path)
                            .expect("Failed to open file")
                            .pausable()
                            .with_adjustable_volume()
                            .controllable();

                        controller = Some(ctrl);
                        current_sound = Some(sound);
                    }
                    AudioCommand::Play(completion_handler) => {
                        if let Some(sound) = current_sound.take() {
                            let (sound, receiver) = sound.with_completion_notifier();

                            println!("spawning thread for playback completion");
                            thread::spawn(move || {
                                println!("blocking for playback completion");
                                if let Err(e) = receiver.recv() {
                                    println!("Error receiving playback completion: {e}");
                                } else {
                                    println!("Playback completed for file");
                                }
                                completion_handler();
                            });

                            manager.play(Box::new(sound));
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
                        current_sound = None;
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
