use super::*;
use crate::player::track::Track;

#[test]
fn test_enqueue_dequeue() {
    let mut queue = Queue::new();
    let track = Track::new("/music/song.mp3");
    queue.enqueue(track.clone());
    assert_eq!(queue.len(), 1);
    let dequeued = queue.dequeue();
    assert_eq!(dequeued, Some(track));
    assert_eq!(queue.len(), 0);
}

#[test]
fn test_prepend() {
    let mut queue = Queue::new();
    let track1 = Track::new("/music/track1.mp3");
    let track2 = Track::new("/music/track2.mp3");
    queue.enqueue(track1.clone());
    queue.prepend(track2.clone());
    assert_eq!(queue.len(), 2);
    assert_eq!(queue.get(0), Some(&track2));
    assert_eq!(queue.get(1), Some(&track1));
}

#[test]
fn test_prepend_duplicate_track() {
    let mut queue = Queue::new();
    let track1 = Track::new("/music/track1.mp3");
    let track2 = Track::new("/music/track2.mp3");
    let track3 = Track::new("/music/track3.mp3");
    
    queue.enqueue(track1.clone());
    queue.enqueue(track2.clone());
    queue.enqueue(track3.clone());
    
    // Prepend track2 which already exists at index 1
    queue.prepend(track2.clone());
    
    // Should move track2 to front without changing queue length
    assert_eq!(queue.len(), 3);
    assert_eq!(queue.get(0), Some(&track2));
    assert_eq!(queue.get(1), Some(&track1));
    assert_eq!(queue.get(2), Some(&track3));
}

#[test]
fn test_prepend_to_empty_queue() {
    let mut queue = Queue::new();
    let track = Track::new("/music/track.mp3");
    
    queue.prepend(track.clone());
    
    assert_eq!(queue.len(), 1);
    assert_eq!(queue.get(0), Some(&track));
}

#[test]
fn test_prepend_duplicate_first_track() {
    let mut queue = Queue::new();
    let track1 = Track::new("/music/track1.mp3");
    let track2 = Track::new("/music/track2.mp3");
    
    queue.enqueue(track1.clone());
    queue.enqueue(track2.clone());
    
    // Prepend track1 which is already at index 0
    queue.prepend(track1.clone());
    
    // Should not change the queue since track1 is already at the front
    assert_eq!(queue.len(), 2);
    assert_eq!(queue.get(0), Some(&track1));
    assert_eq!(queue.get(1), Some(&track2));
}

#[test]
fn test_prepend_duplicate_last_track() {
    let mut queue = Queue::new();
    let track1 = Track::new("/music/track1.mp3");
    let track2 = Track::new("/music/track2.mp3");
    let track3 = Track::new("/music/track3.mp3");
    
    queue.enqueue(track1.clone());
    queue.enqueue(track2.clone());
    queue.enqueue(track3.clone());
    
    // Prepend track3 which is at the last position
    queue.prepend(track3.clone());
    
    // Should move track3 to front
    assert_eq!(queue.len(), 3);
    assert_eq!(queue.get(0), Some(&track3));
    assert_eq!(queue.get(1), Some(&track1));
    assert_eq!(queue.get(2), Some(&track2));
}

#[test]
fn test_remove() {
    let mut queue = Queue::new();
    let track1 = Track::new("/music/track1.mp3");
    let track2 = Track::new("/music/track2.mp3");
    queue.enqueue(track1.clone());
    queue.enqueue(track2.clone());
    queue.remove(&track1);
    assert_eq!(queue.len(), 1);
    assert_eq!(queue.get(0), Some(&track2));
}

#[test]
fn test_clear() {
    let mut queue = Queue::new();
    let track1 = Track::new("/music/track1.mp3");
    let track2 = Track::new("/music/track2.mp3");
    queue.enqueue(track1);
    queue.enqueue(track2);
    queue.clear();
    assert_eq!(queue.len(), 0);
    assert!(queue.is_empty());
}

#[test]
fn test_move_item() {
    let mut queue = Queue::new();
    let track1 = Track::new("/music/track1.mp3");
    let track2 = Track::new("/music/track2.mp3");
    let track3 = Track::new("/music/track3.mp3");
    queue.enqueue(track1.clone());
    queue.enqueue(track2.clone());
    queue.enqueue(track3.clone());
    queue.move_item(0, 2);
    assert_eq!(queue.get(0), Some(&track2));
    assert_eq!(queue.get(1), Some(&track3));
    assert_eq!(queue.get(2), Some(&track1));
}

#[test]
fn test_is_empty() {
    let mut queue = Queue::new();
    assert!(queue.is_empty());
    let track = Track::new("/music/track.mp3");
    queue.enqueue(track);
    assert!(!queue.is_empty());
}
