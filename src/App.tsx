import { createResource, Suspense, type Component } from "solid-js";
import logo from "./logo.svg";
import styles from "./App.module.css";
import { http, build_client } from "@qubit-rs/client";
import type { QubitServer } from "../bindings";

const rpcUrl = import.meta.env.DEV
  ? "http://localhost:8080/rpc"
  : window.location.origin + "/rpc";
const api = build_client<QubitServer>(http(rpcUrl));

const App: Component = () => {
  const [message] = createResource(api.hello_world.query);

  return (
    <div class={styles.App}>
      <header class={styles.header}>
        <img src={logo} class={styles.logo} alt="logo" />
        <p>
          Edit <code class="bg-black/15 rounded-2xl px-3 py-1">src/App.tsx</code> and save to reload.
        </p>
        <a
          class={styles.link}
          href="https://github.com/solidjs/solid"
          target="_blank"
          rel="noopener noreferrer"
        >
          Learn Solid
        </a>
        <Suspense fallback={"Loading..."}>
          <p>
            Backend says '
            <code class="text-3xl font-bold underline">{message()}</code>' from
            RPC ✨
          </p>
        </Suspense>
      </header>
    </div>
  );
};

export default App;
