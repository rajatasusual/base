class RecorderProcessor extends AudioWorkletProcessor {
  process(inputs) {
    const input = inputs[0]?.[0];
    if (input?.length) {
      this.port.postMessage(new Float32Array(input));
    }
    return true; // keep alive
  }
}

registerProcessor("recorder-processor", RecorderProcessor);