import { defineConfig } from "vite";
import solid from "vite-plugin-solid";
import tailwindcss from "@tailwindcss/vite";
import path from "path";
import { spawn } from "child_process";
import * as fs from "fs";

export default defineConfig({
  plugins: [solid(), tailwindcss(), valibotBindings()],
  server: {
    port: 3000,
  },
  build: {
    target: "esnext",
  },
  resolve: {
    alias: {
      "~": path.resolve(__dirname, "./src"),
    },
  },
});

function valibotBindings() {
  return {
    name: "watch-and-run",
    configureServer() {
      watchSchema("user");
    },
  };
}
function watchSchema(schema: string) {
  const fileToWatch = path.resolve(__dirname, `bindings/${schema}.schema.json`);

  // Run the command once on startup
  runSchemaCommand(schema);

  // Then watch for changes
  fs.watchFile(fileToWatch, { interval: 100 }, () => {
    console.log(`File changed: ${fileToWatch}`);
    runSchemaCommand(schema);
  });
}

function runSchemaCommand(schema: string) {
  const command = "pnpx";
  const args = [
    "json-schema-to-valibot",
    "-i",
    `bindings/${schema}.schema.json`,
    "-o",
    `bindings/${schema}.schema.ts`,
  ];

  const process = spawn(command, args, { stdio: "inherit" });

  process.on("close", (code) => {
    if (code === 0) {
      console.log("Command executed successfully.");
    } else {
      console.error(`Command failed with exit code ${code}.`);
    }
  });
}
