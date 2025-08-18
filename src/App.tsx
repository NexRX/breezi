import { createSignal } from "solid-js";
import logo from "./assets/logo.svg";
import { invoke } from "@tauri-apps/api/core";
import "./App.css";
import { Button } from "./components/ui/button";

function App() {
  const [clicks, setClicks] = createSignal(0);

  return (
    <main class="bg-purple-800">
      <h1 class="text-gray-200">Welcome to Tauri + Solid</h1>
      <h2 class="text-gray-300">Clicks: {clicks()}</h2>
      <Button onClick={() => setClicks((v) => v + 1)}>UI Button</Button>
    </main>
  );
}

export default App;
