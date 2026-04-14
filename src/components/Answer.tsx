import { useEffect, useRef } from "preact/hooks";

type Props = { text: string };

export function Answer({ text }: Props) {
  const ref = useRef<HTMLDivElement>(null);

  useEffect(() => {
    ref.current?.scrollIntoView({ behavior: "smooth", block: "start" });
  }, [text]);

  return (
    <div class="answer" ref={ref}>
      <label>I said:</label>
      <p>{text}</p>
    </div>
  );
}