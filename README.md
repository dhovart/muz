# Muz

Muz is a music player made in Rust with a [Rescript](https://rescript-lang.org/) frontend developed with [Tauri](https://tauri.app/). It is currently in a very pre-alpha stage.

## Getting Started

### Prerequisites
- [Rust](https://rustup.rs/)
- [Node.js](https://nodejs.org/)

### Development Setup

1. **Clone the repository**
```bash
git clone https://github.com/dhovart/muz.git
cd muz
```

2. **Install dependencies**

```
npm install --legacy-peer-deps
```

Note; There is a peer dependency conflict with MUI components. React 19 is installed but MUI packages expect React 17 or 18.

That's why you need `--legacy-peer-deps`, it won't break anything.

I'm using Mui 5 (we're currently at 7) as I wanted an existing design system to prototype faster with and the existing, typed rescript bindings are still using version 5.

3. **Run in development mode**

```bash
npm tauri dev
```

### Building for Production

```bash
# Build the application
npm tauri build

# The built application will be in src-tauri/target/release/bundle/
```

## Roadmap

See detailed [ROADMAP.md](ROADMAP.md) for planned features and improvements.

### Near-term Goals
- Queue reordering with drag & drop
- Async library scanning
- Improved error handling and logging

### Long-term Vision
- Multi-platform core library (`muz-core`)
- Terminal UI application (`muz-tui`)

##  Acknowledgments

- [Tauri](https://tauri.app/) for the great cross-platform framework
- [Rodio](https://github.com/RustAudio/rodio) for the audio engine
- [Symphonia](https://github.com/pdeljanov/Symphonia) for multiple codecs support
- [Rescript](https://rescript-lang.org/) for a very pleasant frontend development experience
- [rescript-mui](https://github.com/cca-io/rescript-mui) for the ReScript bindings to MUI components
- [rust](https://www.rust-lang.org/fr) for the awesome language

## Support

- **Issues**: [GitHub Issues](https://github.com/dhovart/muz/issues)
