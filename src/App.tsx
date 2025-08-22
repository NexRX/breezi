import { createResource, Suspense, type Component } from "solid-js";
import logo from "./logo.svg";
import { http, build_client } from "@qubit-rs/client";
import type { QubitServer } from "../bindings";

const rpcUrl = import.meta.env.DEV
  ? "http://localhost:8080/rpc"
  : window.location.origin + "/rpc";
const api = build_client<QubitServer>(http(rpcUrl));

const App: Component = () => {
  const [message] = createResource(api.hello_world.query);

  return (
    <div class="text-center">
      <header class=" bg-[#282c34] min-h-screen flex flex-col items-center justify-center text-[calc(10px+2vmin)] text-white">
        <img
          src={logo}
          class="animate-[logo-spin_20s_linear_infinite] h-[40vmin] pointer-events-none mb-[50px]"
          alt="logo"
        />
        <p>
          Edit{" "}
          <code class="bg-black/15 rounded-2xl px-3 py-1">src/App.tsx</code> and
          save to reload.
        </p>
        <a
          class="text-[#b318f0]"
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
