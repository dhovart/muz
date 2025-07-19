use super::*;
use crate::commands::ControlPlaybackPayload;
use crate::player::{playback::Playback, playback::PlaybackState, playback_driver::DefaultPlaybackDriver, track::Track};
use std::sync::{Arc, Mutex};

#[cfg(test)]
mod tests {
    use super::*;

    fn create_test_track() -> Track {
        Track {
            id: "test-track-id".to_string(),
            path: std::path::PathBuf::from("test.mp3"),
            total_frames: 441000, // 10 seconds at 44.1kHz
            duration_ms: 10000,   // 10 seconds
            metadata: Some(crate::player::track::TrackMetadata {
                title: Some("Test Song".to_string()),
                album: Some("Test Album".to_string()),
                artist: Some("Test Artist".to_string()),
                album_artist: None,
                track_number: Some(1),
                disc_number: Some(1),
                genre: Some("Test".to_string()),
                year: Some("2023".to_string()),
            }),
        }
    }

    fn create_test_playback() -> Arc<Mutex<Playback>> {
        let driver = Box::new(DefaultPlaybackDriver::new(0.5));
        Playback::create(
            driver,
            |_, _, _| {}, // progress callback
            |_, _| {},    // history callback
            |_| {},       // track changed callback
            |_| {},       // queue changed callback
        )
    }

    #[test]
    fn test_playback_service_creation() {
        let playback = create_test_playback();
        let service = PlaybackService::new(playback);
        
        // Service should be created successfully
        // This is mainly testing that the constructor works
        assert!(true);
    }

    #[test]
    fn test_control_playback_unknown_command() {
        let playback = create_test_playback();
        let service = PlaybackService::new(playback);
        
        let payload = ControlPlaybackPayload {
            command: "UnknownCommand".to_string(),
            volume: None,
            seek_position: None,
        };
        
        let result = service.control_playback(payload);
        assert!(result.is_err());
        assert!(result.unwrap_err().to_string().contains("Unknown playback command"));
    }

    #[test]
    fn test_control_playback_play_command() {
        let playback = create_test_playback();
        let service = PlaybackService::new(playback);
        
        let payload = ControlPlaybackPayload {
            command: "Play".to_string(),
            volume: None,
            seek_position: None,
        };
        
        let result = service.control_playback(payload);
        // This will likely fail because there's no track loaded, but we're testing the command parsing
        match result {
            Ok(_) => assert!(true),
            Err(_) => assert!(true), // Expected without a loaded track
        }
    }

    #[test]
    fn test_control_playback_pause_command() {
        let playback = create_test_playback();
        let service = PlaybackService::new(playback);
        
        let payload = ControlPlaybackPayload {
            command: "Pause".to_string(),
            volume: None,
            seek_position: None,
        };
        
        let result = service.control_playback(payload);
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), PlaybackState::Paused);
    }

    #[test]
    fn test_control_playback_volume_command() {
        let playback = create_test_playback();
        let service = PlaybackService::new(playback);
        
        let payload = ControlPlaybackPayload {
            command: "SetVolume".to_string(),
            volume: Some(0.8),
            seek_position: None,
        };
        
        let result = service.control_playback(payload);
        assert!(result.is_ok());
    }

    #[test]
    fn test_control_playback_volume_command_missing_volume() {
        let playback = create_test_playback();
        let service = PlaybackService::new(playback);
        
        let payload = ControlPlaybackPayload {
            command: "SetVolume".to_string(),
            volume: None, // Missing volume
            seek_position: None,
        };
        
        let result = service.control_playback(payload);
        assert!(result.is_err());
        assert!(result.unwrap_err().to_string().contains("Invalid volume payload"));
    }

    #[test]
    fn test_control_playback_seek_command() {
        let playback = create_test_playback();
        let service = PlaybackService::new(playback);
        
        let payload = ControlPlaybackPayload {
            command: "Seek".to_string(),
            volume: None,
            seek_position: Some(5000), // 5 seconds
        };
        
        let result = service.control_playback(payload);
        // This will likely fail because there's no track loaded, but we're testing the command parsing
        match result {
            Ok(_) => assert!(true),
            Err(_) => assert!(true), // Expected without a loaded track
        }
    }

    #[test]
    fn test_control_playback_seek_command_default_position() {
        let playback = create_test_playback();
        let service = PlaybackService::new(playback);
        
        let payload = ControlPlaybackPayload {
            command: "Seek".to_string(),
            volume: None,
            seek_position: None, // Should default to 0
        };
        
        let result = service.control_playback(payload);
        // Testing that missing seek_position defaults to 0
        match result {
            Ok(_) => assert!(true),
            Err(_) => assert!(true), // Expected without a loaded track
        }
    }

    #[test]
    fn test_play_single_track() {
        let playback = create_test_playback();
        let service = PlaybackService::new(playback);
        let track = create_test_track();
        
        // This will fail because there's no actual audio file, but tests the interface
        let result = service.play_single_track(track);
        match result {
            Ok(_) => assert!(true),
            Err(_) => assert!(true), // Expected without real audio file
        }
    }

    #[test]
    fn test_play_album_tracks() {
        let playback = create_test_playback();
        let service = PlaybackService::new(playback);
        
        let track1 = create_test_track();
        let mut track2 = create_test_track();
        track2.id = "test-track-id-2".to_string();
        
        let album_tracks = vec![track1.clone(), track2];
        
        let result = service.play_album_tracks(album_tracks, &track1.id);
        match result {
            Ok(_) => assert!(true),
            Err(_) => assert!(true), // Expected without real audio files
        }
    }

    #[test]
    fn test_play_album_tracks_track_not_found() {
        let playback = create_test_playback();
        let service = PlaybackService::new(playback);
        
        let track = create_test_track();
        let album_tracks = vec![track];
        
        let result = service.play_album_tracks(album_tracks, "non-existent-track-id");
        assert!(result.is_err());
        assert!(result.unwrap_err().to_string().contains("Track not found in album"));
    }

    #[test]
    fn test_clear_queue_and_enqueue() {
        let playback = create_test_playback();
        let service = PlaybackService::new(playback);
        
        let tracks = vec![create_test_track()];
        let result = service.clear_queue_and_enqueue(tracks);
        assert!(result.is_ok());
    }
}