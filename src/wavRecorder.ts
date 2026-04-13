function encodeWAV(samples: Float32Array, sampleRate: number): Blob {
  const buf = new ArrayBuffer(44 + samples.length * 2);
  const view = new DataView(buf);
  const str = (off: number, s: string) =>
    [...s].forEach((c, i) => view.setUint8(off + i, c.charCodeAt(0)));

  str(0, "RIFF"); view.setUint32(4,  36 + samples.length * 2, true);
  str(8, "WAVE"); str(12, "fmt ");
  view.setUint32(16, 16,            true); // chunk size
  view.setUint16(20, 1,             true); // PCM
  view.setUint16(22, 1,             true); // mono
  view.setUint32(24, sampleRate,    true);
  view.setUint32(28, sampleRate * 2,true); // byte rate
  view.setUint16(32, 2,             true); // block align
  view.setUint16(34, 16,            true); // 16-bit
  str(36, "data"); view.setUint32(40, samples.length * 2, true);

  let off = 44;
  for (let i = 0; i < samples.length; i++, off += 2) {
    const s = Math.max(-1, Math.min(1, samples[i]));
    view.setInt16(off, s < 0 ? s * 0x8000 : s * 0x7fff, true);
  }
  return new Blob([buf], { type: "audio/wav" });
}

export function useWavRecorder() {
  let ctx: AudioContext;
  let src: MediaStreamAudioSourceNode;
  let worklet: AudioWorkletNode;
  const chunks: Float32Array[] = [];

  const start = async () => {
    chunks.length = 0; // ← reset before each new recording

    const stream = await navigator.mediaDevices.getUserMedia({ audio: true });

    ctx = new AudioContext();
    await ctx.audioWorklet.addModule("/recorder-processor.js");

    src = ctx.createMediaStreamSource(stream);
    worklet = new AudioWorkletNode(ctx, "recorder-processor");

    worklet.port.onmessage = (e: MessageEvent<Float32Array>) => {
      chunks.push(e.data);
    };

    src.connect(worklet);
    worklet.connect(ctx.destination);
  };

  const stop = async (): Promise<{blob: Blob }> => {
    worklet.disconnect();
    src.disconnect();
    src.mediaStream.getTracks().forEach((t) => t.stop());

    const sampleRate = ctx.sampleRate;
    await ctx.close();

    const total = chunks.reduce((n, c) => n + c.length, 0);
    const merged = new Float32Array(total);
    chunks.reduce((off, c) => { merged.set(c, off); return off + c.length; }, 0);

    const blob = encodeWAV(merged, sampleRate);
    return { blob };
  };

  return { start, stop };
}