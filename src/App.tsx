import { useRef, useState, useEffect } from "preact/hooks";

import { invoke } from "@tauri-apps/api/core";
import { TitleBar } from "./components/TItleBar";
import { useWavRecorder } from "./wavRecorder";
import { Waveform } from "./components/Waveform";
import { RecordButton } from "./components/RecordButton";
import { Transcript } from "./components/Transcript";
import { Answer } from "./components/Answer";
import { ErrorBox } from "./components/ErrorBox";
import { Status } from "./types";

import "./app.css";

export default function App() {
  const [status, setStatus] = useState<Status>("idle");
  const [transcript, setTranscript] = useState<string | null>(null);
  const [answer, setAnswer] = useState<string | null>(null);
  const [error, setError] = useState<string | null>(null);
  const [seconds, setSeconds] = useState(0);
  const timerRef = useRef<number | null>(null);
  const rec = useRef(useWavRecorder());

  useEffect(() => {
    if (status === "recording") {
      setSeconds(0);
      timerRef.current = window.setInterval(() => setSeconds((s) => s + 1), 1000);
    } else {
      if (timerRef.current) clearInterval(timerRef.current);
    }
    return () => { if (timerRef.current) clearInterval(timerRef.current); };
  }, [status]);

  const fmt = (s: number) =>
    `${String(Math.floor(s / 60)).padStart(2, "0")}:${String(s % 60).padStart(2, "0")}`;

  const toggle = async () => {
    if (status === "idle") {
      setError(null);
      setTranscript(null);
      setAnswer(null);
      await rec.current.start();
      setStatus("recording");
    } else if (status === "recording") {
      const { blob } = await rec.current.stop();
      setStatus("transcribing");
      try {
        const bytes = Array.from(new Uint8Array(await blob.arrayBuffer()));
        const text = await invoke<string>("transcribe", { wav: bytes });
        setTranscript(text);
        setStatus("answering");
        const ans = await invoke<string>("answer", { prompt: text });
        setAnswer(ans);
      } catch (e) {
        setError(e as string);
      } finally {
        setStatus("idle");
      }
    }
  };

  return (
    <div class="shell">
      <TitleBar />
      <main class="content">

        <div class="record-area">
          <RecordButton status={status} onClick={toggle} />
          <div class={`timer ${status === "recording" ? "visible" : "hidden"}`}>
            {fmt(seconds)}
          </div>
          <Waveform active={status === "recording"} />
        </div>

        <div class="output-area">
          {transcript && <Transcript text={transcript} />}
          {answer && <Answer text={answer} />}
          {error && <ErrorBox message={error} onDismiss={() => setError(null)} />}
        </div>

      </main>
    </div>
  );
}