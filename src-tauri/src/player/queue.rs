use crate::player::track::Track;

pub struct Queue {
    tracks: Vec<Track>,
}

impl Queue {
    pub fn new() -> Self {
        Self { tracks: Vec::new() }
    }

    pub fn get(&self, index: usize) -> Option<&Track> {
        self.tracks.get(index)
    }

    pub fn len(&self) -> usize {
        self.tracks.len()
    }

    pub fn iter(&self) -> impl Iterator<Item = &Track> {
        self.tracks.iter()
    }

    pub fn is_empty(&self) -> bool {
        self.tracks.is_empty()
    }

    pub fn prepend(&mut self, track: Track) {
        self.tracks.insert(0, track);
    }

    pub fn enqueue(&mut self, track: Track) {
        self.tracks.push(track);
    }

    pub fn dequeue(&mut self) -> Option<Track> {
        if self.tracks.is_empty() {
            None
        } else {
            Some(self.tracks.remove(0))
        }
    }

    pub fn remove(&mut self, track: &Track) {
        self.tracks.retain(|t: &Track| t != track);
    }

    pub fn clear(&mut self) {
        self.tracks.clear();
    }

    pub fn move_item(&mut self, from: usize, to: usize) {
        if from < self.tracks.len() && to < self.tracks.len() {
            let track = self.tracks.remove(from);
            self.tracks.insert(to, track);
        }
    }

    pub fn tracks(&self) -> Vec<Track> {
        self.tracks.clone()
    }
}

#[cfg(test)]
#[path = "./queue.tests.rs"]
mod tests;
