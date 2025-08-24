import { type Component } from "solid-js";
import logo from "./logo.svg";
import { http, build_client } from "@qubit-rs/client";
import type { QubitServer } from "../bindings";
import { Button } from "./components/ui/button";
import { TextField, TextFieldInput } from "./components/ui/text-field";
import { match, P } from "ts-pattern";
import { schema as UserSchema } from "../bindings/user.schema";
import * as v from "valibot";
import { createForm, valiForm } from "@modular-forms/solid";
import { ModularFormField } from "./components/form/field";

const rpcUrl = import.meta.env.DEV
  ? "http://localhost:8080/rpc"
  : window.location.origin + "/rpc";
const api = build_client<QubitServer>(http(rpcUrl));

async function handleRegister() {
  const username = document.getElementById("username") as HTMLInputElement;
  const password = document.getElementById("password") as HTMLInputElement;
  const email = document.getElementById("email") as HTMLInputElement;

  const userRegistration = {
    username: username.value,
    password: password.value,
    email: email.value,
  };
  console.log(userRegistration);

  const result = await api.register.mutate(userRegistration);
  match(result)
    .with({ Ok: P.nonNullable }, ({ Ok: id }) =>
      alert(`Registration successful!. ID: ${id}`),
    )
    .with({ Err: P.nonNullable }, ({ Err }) =>
      alert(`Registration failed! \nError:${JSON.stringify(Err)}`),
    )
    .exhaustive();
}

const App: Component = () => {
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
        <Register />
      </header>
    </div>
  );
};

const Register: Component = () => {
  const RegisterSchema = v.pick(UserSchema, ["username", "email", "password"]);
  type RegisterForm = v.InferInput<typeof RegisterSchema>;

  const [registerForm, { Form, Field }] = createForm<RegisterForm>({
    validate: valiForm(RegisterSchema),
  });

  return (
    <Form class="flex flex-col gap-5 p-5">
      <ModularFormField Field={Field} name="username" />
      <ModularFormField Field={Field} name="email" />
      <ModularFormField Field={Field} name="password" />
      <Button type="submit">Register</Button>
    </Form>
  );
};

export default App;
