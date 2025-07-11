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
    fn send_command(&mut self, _command: AudioCommand) -> Result<()> {
        Ok(())
    }

    fn get_command_sender(&self) -> Sender<AudioCommand> {
        mpsc::channel().0
    }
}

#[test]
fn test_play_track() {
    let test_playback_driver = TestPlaybackDriver::new();
    let mut playback: Playback = Playback::new(Box::new(test_playback_driver));
    let track = Track::new("/music/song.mp3");
    let _ = playback.play(track.clone());
    assert_eq!(playback.current_track(), Some(&track));
}

#[test]
fn test_stop() {
    let test_playback_driver = TestPlaybackDriver::new();
    let mut playback: Playback = Playback::new(Box::new(test_playback_driver));
    let track = Track::new("/music/song.mp3");
    let _ = playback.play(track);
    playback.stop();
    assert!(playback.current_track().is_none());
}

#[test]
fn test_queue_next_track() {
    let test_playback_driver = TestPlaybackDriver::new();
    let mut playback: Playback = Playback::new(Box::new(test_playback_driver));
    let track1 = Track::new("/music/song1.mp3");
    let track2 = Track::new("/music/song2.mp3");
    let mut queue = Queue::new();
    queue.enqueue(track1.clone());
    queue.enqueue(track2.clone());
    playback.queue = Some(queue);
    let _ = playback.next();
    assert_eq!(playback.current_track(), Some(&track2));
}

#[test]
fn test_pause_and_resume() {
    let test_playback_driver = TestPlaybackDriver::new();
    let mut playback: Playback = Playback::new(Box::new(test_playback_driver));
    let track = Track::new("/music/song.mp3");
    let _ = playback.play(track.clone());
    playback.pause();
    match playback.state {
        PlaybackState::Paused(_) => (),
        _ => panic!("playback should be paused"),
    }
    let result = playback.resume_play();
    assert!(result.is_ok());
    match playback.state {
        PlaybackState::Playing(_) => (),
        _ => panic!("playback should be playing after resume"),
    }
}

#[test]
fn test_next_with_empty_queue() {
    let test_playback_driver = TestPlaybackDriver::new();
    let mut playback: Playback = Playback::new(Box::new(test_playback_driver));
    playback.queue = Some(Queue::new());
    let error = playback.next().unwrap_err();
    assert!(error.to_string().contains("Queue is empty"));
}

#[test]
fn test_next_without_queue() {
    let test_playback_driver = TestPlaybackDriver::new();
    let mut playback: Playback = Playback::new(Box::new(test_playback_driver));
    let error = playback.next().unwrap_err();
    assert_eq!(error.to_string(), "No queue available");
}

#[test]
fn test_previous_with_no_history() {
    let test_playback_driver = TestPlaybackDriver::new();
    let mut playback: Playback = Playback::new(Box::new(test_playback_driver));
    let error = playback.previous().unwrap_err();
    assert_eq!(error.to_string(), "No previous track available");
}

#[test]
fn test_play_appends_to_history() {
    let test_playback_driver = TestPlaybackDriver::new();
    let mut playback: Playback = Playback::new(Box::new(test_playback_driver));
    let track = Track::new("/music/song.mp3");
    let _ = playback.play(track.clone());
    assert_eq!(playback.history.len(), 1);
    assert_eq!(playback.history[0], track);
}

#[test]
fn test_play_multiple_tracks_appends_to_history() {
    let test_playback_driver = TestPlaybackDriver::new();
    let mut playback: Playback = Playback::new(Box::new(test_playback_driver));
    let track1 = Track::new("/music/song1.mp3");
    let track2 = Track::new("/music/song2.mp3");
    let _ = playback.play(track1.clone());
    let _ = playback.play(track2.clone());
    assert_eq!(playback.history.len(), 2);
    assert_eq!(playback.history[0], track1);
    assert_eq!(playback.history[1], track2);
}
