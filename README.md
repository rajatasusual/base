# Base

A minimal desktop app for recording audio and transcribing it locally using [whisper.cpp](https://github.com/ggerganov/whisper.cpp). No internet connection required — everything runs on-device.

Built with [Tauri v2](https://tauri.app), [Preact](https://preactjs.com), and TypeScript.

---

## Features

- One-click audio recording via the Web Audio API
- Local transcription via `whisper-cli` — no data leaves your machine
- Custom frameless window with frosted-glass vibrancy (macOS/Windows)
- Single-instance enforcement — reopening focuses the existing window

---

## Requirements

| Tool | Version |
|------|---------|
| [Rust](https://rustup.rs) | stable |
| [Node.js](https://nodejs.org) | 18+ |
| [Tauri CLI](https://tauri.app/start/create-project/) | v2 |

---

## Project Structure

```bash
├── src/                        # Preact frontend
│   ├── App.tsx
│   ├── app.css
│   ├── types.ts
│   ├── wavRecorder.ts          # AudioWorklet-based WAV recorder
│   ├── components/
│   │   ├── TitleBar.tsx
│   │   ├── Waveform.tsx
│   │   ├── RecordButton.tsx
│   │   ├── Transcript.tsx
│   │   └── ErrorBox.tsx
│   └── ...
├── public/
│   └── recorder-processor.js  # AudioWorklet processor (served as static file)
├── src-tauri/
│   ├── src/
│   │   ├── lib.rs
│   │   ├── utility.rs
│   │   └── commands/
│   │       ├── mod.rs
│   │       ├── transcribe.rs   # Calls whisper-cli, returns transcript
│   │       └── browser.rs      # In-app URL navigation
│   └── tauri.conf.json
    └── third-party/
        └── whisper.cpp/
            ├── whisper-cli.exe     # Built whisper binary
            └── model/
                └── ggml-base.en.bin
```

---

## Setup

### 1. Install dependencies

```bash
npm install
```

### 2. Build whisper.cpp (or download binaries)

```bash
git clone https://github.com/ggerganov/whisper.cpp third-party/whisper.cpp
cd third-party/whisper.cpp
cmake -B build
cmake --build build --config Release
```

The built binary should end up at `src-tauri/third-party/whisper.cpp/whisper-cli.exe` (Windows) or adjust the path in `commands/transcribe.rs` for your platform.

### 3. Download a model

```bash
cd third-party/whisper.cpp
./models/download-ggml-model.sh base.en
```

This places `ggml-base.en.bin` in `third-party/whisper.cpp/models/`. Move or symlink it to match the path in `transcribe.rs`:

```
third-party/whisper.cpp/model/ggml-base.en.bin
```

Available models (tradeoff between speed and accuracy):

| Model | Size | Notes |
|-------|------|-------|
| `tiny.en` | 75 MB | Fastest |
| `base.en` | 142 MB | Recommended |
| `small.en` | 466 MB | More accurate |
| `medium.en` | 1.5 GB | High accuracy |

---

## Development

```bash
npm run tauri dev
```

## Build

```bash
npm run tauri build
```

The installer is output to `src-tauri/target/release/bundle/`.

---

## How It Works

1. **Recording** — The frontend uses the Web Audio API with an `AudioWorkletNode` (`recorder-processor.js`) to capture raw PCM from the microphone on a dedicated audio thread. Samples are streamed back to the main thread via `MessagePort`.

2. **Encoding** — On stop, all captured `Float32Array` chunks are merged and written into a WAV file in memory (44-byte header + 16-bit PCM samples).

3. **Transcription** — The WAV bytes are sent to Rust via a Tauri command (`invoke("transcribe", ...)`). Rust writes a temp file, spawns `whisper-cli` with `CREATE_NO_WINDOW`, reads stdout, and returns the cleaned transcript string.

4. **Display** — The transcript appears in the scrollable output area below the record button. Errors surface inline with a dismiss button.

---

## Configuration

The whisper binary and model paths are resolved relative to the project root in `src-tauri/src/commands/transcribe.rs`:

```rust
let whisper_bin = root.join("third-party/whisper.cpp/whisper-cli.exe");
let model = root.join("third-party/whisper.cpp/model/ggml-base.en.bin");
```

Adjust these paths to match your setup or platform.

---

## Platform Notes

| Platform | Notes |
|----------|-------|
| Windows | Requires Windows 11 |
| macOS | Requires `NSMicrophoneUsageDescription` in `Info.plist` |
| Linux | None |

---

## License

MIT