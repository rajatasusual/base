import { useState } from "preact/hooks";
import { invoke } from "@tauri-apps/api/core";
import "./App.css";

function App() {
  const [url, setURL] = useState("");

  async function openUrl() {
    await invoke("open_url", { input: url });
  }

  return (
    <main class="container">
      <h1>Welcome to the <span class="logo">Base</span></h1>

      <form
        class="row"
        onSubmit={(e) => {
          e.preventDefault();
          openUrl();
        }}
      >
        <input
          id="url"
          onInput={(e) => setURL(e.currentTarget.value)}
          placeholder="Enter a URL..."
        />
        <button type="submit">GO</button>
      </form>
    </main>
  );
}

export default App;
