# Development Roadmap

## Architecture

### Architecture Improvements
- [x] **Async/Await Migration** - Convert synchronous operations to async using tokio
- [x] **Proper Error Handling** - Replace `eprintln!` with structured logging framework (tracing/log)
- [ ] **Core Library Extraction** - Create `muz-core` crate with platform-agnostic music player logic
- [ ] **Configuration Management** - Centralized, type-safe configuration system

### Multi-Platform Architecture
- [ ] **Core Library (`muz-core`)** - Platform-agnostic music player engine
- [ ] **GUI Application (`muz-gui`)** - Tauri-based desktop/mobile app
- [ ] **Terminal Application (`muz-tui`)** - CLI music player (low priority). ASCII spectrum visualizer?

## Core Music Player Features

### Queue Management
- [x] **Queue Reordering** - Drag & drop to reorder tracks in queue
- [ ] **Shuffle Mode** - Randomize track playback order
- [ ] **Repeat Modes** - None, One Track, All Tracks
- [ ] **Queue Persistence** - Save and restore queue state on app restart
- [ ] **Clear Queue** - Remove all tracks from queue
- [ ] **Remove from Queue** - Remove individual tracks from queue

### Library Management
- [ ] **Async Library Scanning** - Non-blocking library scan with progress reporting
- [ ] **Incremental Scanning** - Only scan changed/new files
- [ ] **Multiple Library Support** - Support multiple music library locations
- [ ] **Library Statistics** - Track count, total duration, storage size
- [ ] **Auto-Rescan** - Watch filesystem for changes and auto-update library

### Playback Features
- [ ] **Gapless Playback** - Seamless transition between tracks
- [ ] **Crossfade** - Smooth fade between tracks

## User Interface & Experience

### Visual Improvements
- [ ] **Modern UI Styling** - Clean, modern interface design
- [ ] **Dark/Light Themes** - Toggle between theme modes
- [ ] **Album Art Display** - Show album covers in player and library
- [ ] **Progress Visualization** - Better progress bar with hover seek
- [ ] **Spectrum Visualizer** - Real-time audio spectrum display

### Desktop Navigation & Controls
- [ ] **Keyboard Shortcuts** - Space (play/pause), arrows (next/prev/seek), etc.
- [ ] **Global Hotkeys** - System-wide media key support
- [ ] **Mini Player Mode** - Compact player window
- [ ] **System Tray Integration** - Minimize to system tray
- [ ] **Now Playing Notifications** - Show track changes in system notifications

### Mobile UI & Experience
- [ ] **Touch-Optimized Interface** - Large touch targets, swipe gestures
- [ ] **Mobile Navigation** - Bottom tab bar, pull-to-refresh, infinite scroll
- [ ] **Swipe Gestures** - Swipe to next/previous track, swipe to add to queue
- [ ] **Mobile Player Controls** - Large play/pause button, easy seek controls
- [ ] **Lock Screen Integration** - Show controls on iOS/Android lock screen
- [ ] **Background Playback** - Continue playing when app is minimized
- [ ] **Mobile-Specific Layouts** - Responsive design for phone/tablet screens
- [ ] **Pull-to-Refresh** - Refresh library and playlists with pull gesture
- [ ] **Mobile Search** - Touch-friendly search with voice input support
- [ ] **Haptic Feedback** - Tactile feedback for button presses and gestures

### Library Organization
- [ ] **Search Functionality** - Search tracks, albums, artists
- [ ] **Advanced Filtering** - Filter by genre, year, rating, etc.
- [ ] **Sorting Options** - Multiple sort criteria for library views
- [ ] **Grid/List Views** - Toggle between different library view modes
- [ ] **Recently Played** - Track and display recently played songs

## Advanced Features

### Playlists
- [ ] **Playlist Creation** - Create and manage custom playlists
- [ ] **Playlist Import/Export** - Support M3U, PLS formats
- [ ] **Smart Playlists** - Auto-generated playlists based on criteria
- [ ] **Playlist Folders** - Organize playlists in folders

### Metadata & Organization
- [ ] **Tag Editing** - Edit track metadata (title, artist, album, etc.)
- [ ] **Automatic Tagging** - Online metadata lookup and correction
- [ ] **Custom Fields** - User-defined metadata fields
- [ ] **Rating System** - 5-star rating system for tracks
- [ ] **Play Count Tracking** - Track how many times songs are played

### Audio Format Support
- [ ] **Streaming Support** - Internet radio streams
- [ ] **Podcast Support** - Basic podcast playback features

## Technical Improvements

### Project Structure & Crates
- [ ] **Workspace Organization** - Restructure as Cargo workspace with multiple crates
  ```
  muz/
  ├── muz-core/          # Core music player library
  ├── muz-gui/           # Tauri GUI application  
  ├── muz-tui/           # Terminal UI application
  ```

### Development & Testing
- [ ] **Comprehensive Testing** - Unit tests, integration tests, UI tests
- [ ] **CI/CD Pipeline** - Automated testing and build pipeline
- [ ] **Error Reporting** - Crash reporting and analytics
- [ ] **Documentation** - API documentation and user guides