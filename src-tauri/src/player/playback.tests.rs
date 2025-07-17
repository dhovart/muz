use std::sync::mpsc::{self, Sender};
use std::thread;
use std::time::Duration;

use super::*;
use crate::player::track::Track;
use anyhow::Result;

pub struct TestPlaybackDriver;

impl TestPlaybackDriver {
    #[allow(clippy::new_ret_no_self)]
    fn new() -> impl PlaybackDriver {
        Self
    }
}

impl PlaybackDriver for TestPlaybackDriver {
    fn send_command(&mut self, command: AudioCommand) -> Result<()> {
        match command {
            AudioCommand::Play(_, event_sender) => {
                // Simulate progress events to trigger history updates
                thread::spawn(move || {
                    thread::sleep(Duration::from_millis(50));
                    event_sender.send(PlaybackEvent::Progress(5.0, 1000)).ok();
                });
            }
            _ => {}
        }
        Ok(())
    }

    fn get_command_sender(&self) -> Sender<AudioCommand> {
        mpsc::channel().0
    }
}

fn create_playback() -> Arc<Mutex<Playback>> {
    let playback_driver = TestPlaybackDriver::new();
    Playback::create(
        Box::new(playback_driver),
        |_, _| {},
        |_, _| {},
        |_| {},
        |_| {},
    )
}

#[test]
fn test_play_track() {
    let playback_arc = create_playback();
    let mut playback = playback_arc.lock().unwrap();
    let track = Track::new("/music/song.mp3");
    playback.enqueue(track.clone());
    let _ = playback.play();
    assert_eq!(playback.current_track(), Some(&track));
}

#[test]
fn test_stop() {
    let playback_arc = create_playback();
    let mut playback = playback_arc.lock().unwrap();
    let track = Track::new("/music/song.mp3");
    playback.enqueue(track.clone());
    let _ = playback.play();
    playback.stop();
    assert!(playback.current_track().is_none());
}

#[test]
fn test_queue_next_track() {
    let playback_arc = create_playback();
    let mut playback = playback_arc.lock().unwrap();
    let track1 = Track::new("/music/song1.mp3");
    let track2 = Track::new("/music/song2.mp3");
    playback.enqueue(track1.clone());
    playback.enqueue(track2.clone());
    let _ = playback.play();
    assert_eq!(playback.current_track(), Some(&track1));
    let _ = playback.next();
    assert_eq!(playback.current_track(), Some(&track2));
}

#[test]
fn test_pause_and_resume() {
    let playback_arc = create_playback();
    let mut playback = playback_arc.lock().unwrap();
    let track = Track::new("/music/song.mp3");
    playback.enqueue(track.clone());
    let _ = playback.play();
    playback.pause();
    match playback.state {
        PlaybackState::Paused => (),
        _ => panic!("playback should be paused"),
    }
    let result = playback.resume_play();
    assert!(result.is_ok());
    match playback.state {
        PlaybackState::Playing => (),
        _ => panic!("playback should be playing after resume"),
    }
}

#[test]
fn test_play_with_empty_queue() {
    let playback_arc = create_playback();
    let mut playback = playback_arc.lock().unwrap();
    playback.queue = Some(Queue::new());
    let error = playback.play().unwrap_err();
    assert!(error.to_string().contains("No track to play"));
}

#[test]
fn test_play_without_queue() {
    let playback_arc = create_playback();
    let mut playback = playback_arc.lock().unwrap();
    let error = playback.play().unwrap_err();
    assert_eq!(error.to_string(), "No track to play");
}

#[test]
fn test_previous_with_no_history() {
    let playback_arc = create_playback();
    let mut playback = playback_arc.lock().unwrap();
    let error = playback.previous().unwrap_err();
    assert_eq!(error.to_string(), "No track to play");
}
#[test]
fn test_play_appends_to_history() {
    let playback_arc = create_playback();

    let track = Track::new("/music/song.mp3");

    {
        let mut playback = playback_arc.lock().unwrap();
        playback.enqueue(track.clone());
        let _ = playback.play();
    }

    thread::sleep(Duration::from_millis(100));

    let playback = playback_arc.lock().unwrap();
    assert_eq!(playback.history.len(), 1);
    assert_eq!(playback.history[0], track.clone());
}

#[test]
fn test_play_multiple_tracks_appends_to_history() {
    let playback_arc = create_playback();

    let track1 = Track::new("/music/song1.mp3");
    let track2 = Track::new("/music/song2.mp3");

    {
        let mut playback = playback_arc.lock().unwrap();
        playback.enqueue(track1.clone());
        playback.enqueue(track2.clone());
        let _ = playback.play();
    }

    thread::sleep(Duration::from_millis(100));

    {
        let mut playback = playback_arc.lock().unwrap();
        let _ = playback.next();
    }

    thread::sleep(Duration::from_millis(100));

    let playback = playback_arc.lock().unwrap();
    assert_eq!(playback.history.len(), 2);
    assert_eq!(playback.history[0], track1.clone());
    assert_eq!(playback.history[1], track2.clone());
}
