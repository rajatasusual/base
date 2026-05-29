# Base

A minimal desktop app for recording audio and transcribing it locally using [whisper.cpp](https://github.com/ggerganov/whisper.cpp). Then fetching the answer by running the inference to an SLM using [llama.cpp] (https://github.com/ggerganov/whisper.cpp). No internet connection required — everything runs on-device.

Built with [Tauri v2](https://tauri.app), [Preact](https://preactjs.com), and TypeScript.

---

## Features

- One-click audio recording via the Web Audio API
- Local transcription via `whisper-cli` — no data leaves your machine
- Local SLM inference via `llama-cli` to generate short succinct answers.

---

## Requirements (to build)

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
│   ├── wavRecorder.ts          # AudioWorklet-based WAV
│   ├── components/
│   │   ├── TitleBar.tsx
│   │   ├── Waveform.tsx
│   │   ├── RecordButton.tsx
│   │   ├── Transcript.tsx
│   │   └── ErrorBox.tsx
│   │   └── Answer.tsx
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
│   │       └── answer.rs       # Calls llama-completion and returns answer
│   └── tauri.conf.json
└── └── third-party/    # git sub-module to house the third party pre-built binaries and models
        └── whisper.cpp/
        │   ├── whisper-cli.exe     # Built whisper binary
        │   └── model/
        │       └── ggml-base.en.bin # Default whisper model
        └── llama.cpp/
            ├── llama-completion.exe     # Built llama binary
            └── model/
                └── gemma3-270m-it.gguf # Memory efficient SLM
```

---

## Setup

### 1. Install dependencies

```bash
npm install
```

### 2. Whisper

note: You can skip this step if you have pulled this repository along with its submodules (i.e. recursively)

#### 2.1 Build whisper.cpp (or download binaries)

```bash
git clone --recursive https://github.com/ggerganov/whisper.cpp third-party/whisper.cpp
cd third-party/whisper.cpp
cmake -B build
cmake --build build --config Release
```

The built binary should end up at `src-tauri/third-party/whisper.cpp/whisper-cli.exe` (Windows) or adjust the path in `commands/transcribe.rs` for your platform.

#### 2.2 Download a model

```bash
cd third-party/whisper.cpp
./model/download-ggml-model.sh base.en
```

This places `ggml-base.en.bin` in `third-party/whisper.cpp/model/`. Move or symlink it to match the path in `transcribe.rs`:

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


### 3. Llama
#### 3.1 Build llama.cpp (or download binaries)

note: You can skip this step if you have pulled this repository along with its submodules (i.e. recursively)

```bash
git clone --recursive https://github.com/ggerganov/llama.cpp third-party/llama.cpp
cd third-party/llama.cpp
cmake -B build
cmake --build build --config Release
```

The built binary should end up at `src-tauri/third-party/llama.cpp/llama-completion.exe` (Windows) or adjust the path in `commands/answer.rs` for your platform.

#### 3.2 Download a model

note: I prefer LFM2-1.2B for <1B SLM. You can choose any other. Update the versions.conf file with name and Download URL. You can run the command in src-tauri/third-party submodule to download the model.

```bash
./download-models
```

This places `LFM2-1.2B.gguf` in `third-party/llama.cpp/model/`. Move or symlink it to match the path in `transcribe.rs`:

```
third-party/llama.cpp/model/LFM2-1.2B.gguf
```

---

## Development

```bash
npm run tauri dev
```
This will spawn a development server with hot fixes.

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

4. **Inference** — The transcribed content is sent to Rust via a Tauri command (`invoke("answer", ...)`). Rust spawns `llama-completion` with `CREATE_NO_WINDOW`, reads stdout, and returns the cleaned answer string.

5. **Display** — The transcript and answer appear in the scrollable output area below the record button. Errors surface inline with a dismiss button.

---

## Configuration

The whisper and llama binaries and models paths are resolved relative to the project root in `src-tauri/src/commands/*`:
e.g. for transcription.
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