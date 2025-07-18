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
        |_, _, _| {},
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
        let _ = playback.next();
    }

    let playback = playback_arc.lock().unwrap();
    assert_eq!(playback.history.len(), 1);
    assert_eq!(playback.history[0], track.clone());
}

#[test]
fn test_play_multiple_tracks_appends_to_history() {
    let playback_arc = create_playback();

    let track1 = Track::new("/music/song1.mp3");
    let track2 = Track::new("/music/song2.mp3");

    let mut playback = playback_arc.lock().unwrap();
    playback.enqueue(track1.clone());
    playback.enqueue(track2.clone());
    let _ = playback.play();
    let _ = playback.next();
    let _ = playback.next();

    assert_eq!(playback.history.len(), 2);
    assert_eq!(playback.history[0], track1.clone());
    assert_eq!(playback.history[1], track2.clone());
}

#[test]
fn test_previous_prepends_current_track_to_queue() {
    let playback_arc = create_playback();

    let track1 = Track::new("/music/song1.mp3");
    let track2 = Track::new("/music/song2.mp3");

    let initial_queue_length = {
        let mut playback = playback_arc.lock().unwrap();
        playback.enqueue(track1.clone());
        playback.enqueue(track2.clone());
        let _ = playback.play();
        let _ = playback.next();
        playback.get_queue().len()
    };

    {
        let mut playback = playback_arc.lock().unwrap();
        let _ = playback.previous();
    }

    let playback = playback_arc.lock().unwrap();
    let final_queue_length = playback.get_queue().len();

    assert_eq!(final_queue_length, initial_queue_length + 1);

    let queue_tracks = playback.get_queue();
    assert_eq!(queue_tracks.first(), Some(&track2));

    assert_eq!(playback.current_track(), Some(&track1));
}

#[test]
fn test_previous_with_empty_queue_prepends_current_track() {
    let playback_arc = create_playback();

    let track1 = Track::new("/music/song1.mp3");
    let track2 = Track::new("/music/song2.mp3");

    {
        let mut playback = playback_arc.lock().unwrap();
        playback.enqueue(track1.clone());
        playback.enqueue(track2.clone());
        let _ = playback.play();
        let _ = playback.next();
        playback.clear_queue();
        let _ = playback.previous();
    }

    let playback = playback_arc.lock().unwrap();
    let queue_tracks = playback.get_queue();

    assert_eq!(queue_tracks.len(), 1);
    assert_eq!(queue_tracks[0], track2);

    assert_eq!(playback.current_track(), Some(&track1));
}

#[test]
fn test_previous_with_no_current_track_does_not_prepend_to_queue() {
    let playback_arc = create_playback();

    let track1 = Track::new("/music/song1.mp3");

    let mut playback = playback_arc.lock().unwrap();
    playback.enqueue(track1.clone());
    let _ = playback.play();

    playback.stop();
    let _ = playback.previous();

    let queue_tracks = playback.get_queue();

    assert_eq!(queue_tracks.len(), 0);
}

#[test]
fn test_multiple_previous_calls_prepend_correctly() {
    let playback_arc = create_playback();

    let track1 = Track::new("/music/song1.mp3");
    let track2 = Track::new("/music/song2.mp3");
    let track3 = Track::new("/music/song3.mp3");

    let initial_queue_length = {
        let mut playback = playback_arc.lock().unwrap();
        playback.enqueue(track1.clone());
        playback.enqueue(track2.clone());
        playback.enqueue(track3.clone());
        let _ = playback.play();
        let _ = playback.next();
        let _ = playback.next();
        playback.get_queue().len()
    };

    {
        let mut playback = playback_arc.lock().unwrap();
        let _ = playback.previous();
        let _ = playback.previous();
    }

    let playback = playback_arc.lock().unwrap();
    let final_queue_length = playback.get_queue().len();
    let queue_tracks = playback.get_queue();

    assert_eq!(final_queue_length, initial_queue_length + 2);

    assert_eq!(queue_tracks[0], track2);
    assert_eq!(queue_tracks[1], track3);

    assert_eq!(playback.current_track(), Some(&track1));
}

#[test]
fn test_select_track_from_queue_success() {
    let playback_arc = create_playback();
    let mut playback = playback_arc.lock().unwrap();
    
    let track1 = Track::new("/music/song1.mp3");
    let track2 = Track::new("/music/song2.mp3");
    let track3 = Track::new("/music/song3.mp3");
    
    playback.enqueue(track1.clone());
    playback.enqueue(track2.clone());
    playback.enqueue(track3.clone());
    
    let _ = playback.play();
    assert_eq!(playback.current_track(), Some(&track1));
    
    let result = playback.select_track_from_queue(&track3.id);
    assert!(result.is_ok());
    assert_eq!(result.unwrap(), PlaybackState::Playing);
    
    // Should now be playing track3
    assert_eq!(playback.current_track(), Some(&track3));
    
    // track1 should be in history
    assert_eq!(playback.history.len(), 1);
    assert_eq!(playback.history[0], track1);
    
    // track3 should be removed from queue and track2 should be first
    let queue = playback.get_queue();
    assert_eq!(queue.len(), 1);
    assert_eq!(queue[0], track2);
}

#[test]
fn test_select_track_from_queue_with_no_current_track() {
    let playback_arc = create_playback();
    let mut playback = playback_arc.lock().unwrap();
    
    let track1 = Track::new("/music/song1.mp3");
    let track2 = Track::new("/music/song2.mp3");
    
    playback.enqueue(track1.clone());
    playback.enqueue(track2.clone());
    
    let result = playback.select_track_from_queue(&track2.id);
    assert!(result.is_ok());
    assert_eq!(result.unwrap(), PlaybackState::Playing);
    
    // Should now be playing track2
    assert_eq!(playback.current_track(), Some(&track2));
    
    // History should be empty since no track was playing
    assert_eq!(playback.history.len(), 0);
    
    // track1 should be the only track left in queue
    let queue = playback.get_queue();
    assert_eq!(queue.len(), 1);
    assert_eq!(queue[0], track1);
}

#[test]
fn test_select_track_from_queue_moves_to_front() {
    let playback_arc = create_playback();
    let mut playback = playback_arc.lock().unwrap();
    
    let track1 = Track::new("/music/song1.mp3");
    let track2 = Track::new("/music/song2.mp3");
    let track3 = Track::new("/music/song3.mp3");
    let track4 = Track::new("/music/song4.mp3");
    
    playback.enqueue(track1.clone());
    playback.enqueue(track2.clone());
    playback.enqueue(track3.clone());
    playback.enqueue(track4.clone());
    
    let result = playback.select_track_from_queue(&track3.id);
    assert!(result.is_ok());
    
    // Should now be playing track3
    assert_eq!(playback.current_track(), Some(&track3));
    
    // Queue should have track3 removed and other tracks in original order
    let queue = playback.get_queue();
    assert_eq!(queue.len(), 3);
    assert_eq!(queue[0], track1);
    assert_eq!(queue[1], track2);
    assert_eq!(queue[2], track4);
}

#[test]
fn test_select_track_from_queue_nonexistent_track() {
    let playback_arc = create_playback();
    let mut playback = playback_arc.lock().unwrap();
    
    let track1 = Track::new("/music/song1.mp3");
    playback.enqueue(track1.clone());
    
    let result = playback.select_track_from_queue("nonexistent-id");
    assert!(result.is_err());
    assert_eq!(result.unwrap_err().to_string(), "Track not found in queue");
}

#[test]
fn test_select_track_from_queue_empty_queue() {
    let playback_arc = create_playback();
    let mut playback = playback_arc.lock().unwrap();
    
    let result = playback.select_track_from_queue("any-id");
    assert!(result.is_err());
    assert_eq!(result.unwrap_err().to_string(), "Track not found in queue");
}

#[test]
fn test_select_track_from_queue_no_queue() {
    let playback_arc = create_playback();
    let mut playback = playback_arc.lock().unwrap();
    
    // Don't create any queue
    let result = playback.select_track_from_queue("any-id");
    assert!(result.is_err());
    assert_eq!(result.unwrap_err().to_string(), "Track not found in queue");
}

#[test]
fn test_select_track_from_queue_first_track() {
    let playback_arc = create_playback();
    let mut playback = playback_arc.lock().unwrap();
    
    let track1 = Track::new("/music/song1.mp3");
    let track2 = Track::new("/music/song2.mp3");
    let track3 = Track::new("/music/song3.mp3");
    
    playback.enqueue(track1.clone());
    playback.enqueue(track2.clone());
    playback.enqueue(track3.clone());
    
    let _ = playback.play();
    assert_eq!(playback.current_track(), Some(&track1));
    
    // Select the first track in queue (track2)
    let result = playback.select_track_from_queue(&track2.id);
    assert!(result.is_ok());
    
    // Should now be playing track2
    assert_eq!(playback.current_track(), Some(&track2));
    
    // track1 should be in history
    assert_eq!(playback.history.len(), 1);
    assert_eq!(playback.history[0], track1);
    
    // Queue should have track3 only
    let queue = playback.get_queue();
    assert_eq!(queue.len(), 1);
    assert_eq!(queue[0], track3);
}

#[test]
fn test_select_track_from_queue_with_paused_state() {
    let playback_arc = create_playback();
    let mut playback = playback_arc.lock().unwrap();
    
    let track1 = Track::new("/music/song1.mp3");
    let track2 = Track::new("/music/song2.mp3");
    
    playback.enqueue(track1.clone());
    playback.enqueue(track2.clone());
    
    let _ = playback.play();
    let _ = playback.pause();
    assert_eq!(playback.state, PlaybackState::Paused);
    
    let result = playback.select_track_from_queue(&track2.id);
    assert!(result.is_ok());
    assert_eq!(result.unwrap(), PlaybackState::Playing);
    
    // Should now be playing track2
    assert_eq!(playback.current_track(), Some(&track2));
    assert_eq!(playback.state, PlaybackState::Playing);
}
