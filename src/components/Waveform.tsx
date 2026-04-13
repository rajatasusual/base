type Props = { active: boolean };

export function Waveform({ active }: Props) {
  return (
    <div class={`waveform ${active ? "active" : "hidden"}`}>
      {Array.from({ length: 40 }).map((_, i) => (
        <span 
          key={i} 
          class={`bar ${active ? "recording" : ""}`}
          style={{ 
            animationDelay: `${(i * 25) % 800}ms`,
            height: active ? `${Math.random() * 100}%` : undefined
          }} 
        />
      ))}
    </div>
  );
}