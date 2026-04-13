import { useEffect, useRef } from "preact/hooks";

type Props = { text: string };

export function Transcript({ text }: Props) {
  const ref = useRef<HTMLDivElement>(null);

  useEffect(() => {
    ref.current?.scrollIntoView({ behavior: "smooth", block: "start" });
  }, [text]);

  return (
    <div class="transcript" ref={ref}>
      <label>I heard:</label>
      <p>{text}</p>
    </div>
  );
}