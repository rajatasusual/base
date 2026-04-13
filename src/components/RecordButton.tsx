import { JSX } from "preact/jsx-runtime";
import { Status } from "../types";

type Props = { status: Status; onClick: () => void };

const icons: Record<Status, JSX.Element> = {
  idle: (
    <svg viewBox="0 0 24 24" fill="currentColor">
      <rect x="9" y="2" width="6" height="13" rx="3" />
      <path d="M5 10a7 7 0 0 0 14 0M12 19v3M8 22h8" stroke="currentColor" stroke-width="2" fill="none" stroke-linecap="round" />
    </svg>
  ),
  recording: (
    <svg viewBox="0 0 24 24" fill="currentColor">
      <rect x="5" y="5" width="14" height="14" rx="2" />
    </svg>
  ),
  transcribing: (
    <svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
      <path d="M12 2v4M12 18v4M4.93 4.93l2.83 2.83M16.24 16.24l2.83 2.83M2 12h4M18 12h4M4.93 19.07l2.83-2.83M16.24 7.76l2.83-2.83" />
    </svg>
  ),
};

const hints: Record<Status, string> = {
  idle:         "Click to record",
  recording:    "Recording — click to stop",
  transcribing: "Transcribing…",
};

export function RecordButton({ status, onClick }: Props) {
  return (
    <>
      <button
        class={`rec-btn ${status}`}
        onClick={onClick}
        disabled={status === "transcribing"}
      >
        <span class="rec-btn__ring" />
        <span class="rec-btn__core">{icons[status]}</span>
      </button>
      <p class="hint">{hints[status]}</p>
    </>
  );
}