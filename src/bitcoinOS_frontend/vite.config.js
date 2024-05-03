import { fileURLToPath, URL } from 'url';
import react from '@vitejs/plugin-react';
import { defineConfig } from 'vite';
import environment from 'vite-plugin-environment';
import dotenv from 'dotenv';

dotenv.config({ path: '../../.env' });

const processEnvCanisterIds = Object.fromEntries(
  Object.entries(process.env)
    .filter(([key]) => key.startsWith("CANISTER_ID"))
    .map(([key, value]) => [`process.env.${key}`, JSON.stringify(value)])
);
// console.log(process.env)
console.log(process.env.CANISTER_ID_INTERNET_IDENTITY)
const internetIdentityUrl =
  process.env.DFX_NETWORK === "local"
    ? `http://localhost:4943/?canisterId=${process.env.CANISTER_ID_INTERNET_IDENTITY}`
    : `https://identity.ic0.app`;

export default defineConfig({
  build: {
    emptyOutDir: true,
  },
  optimizeDeps: {
    esbuildOptions: {
      define: {
        global: "globalThis",
      },
    },
  },
  server: {
    proxy: {
      "/api": {
        target: "http://127.0.0.1:4943",
        changeOrigin: true,
      },
    },
  },
  plugins: [
    react(),
    environment("all", { prefix: "CANISTER_" }),
    environment("all", { prefix: "DFX_" }),
  ],
  resolve: {
    alias: [
      {
        find: "declarations",
        replacement: fileURLToPath(
          new URL("../declarations", import.meta.url)
        ),
      },
    ],
  },
  define: {
    // Define the canister ids for the frontend to use. Currently, dfx generated
    // code relies on variables being defined as process.env.CANISTER_ID_*
    ...processEnvCanisterIds,
    "process.env.NODE_ENV": JSON.stringify(process.env.NODE_ENV),
    "process.env.DFX_NETWORK": JSON.stringify(process.env.DFX_NETWORK),
    "process.env.II_URL": JSON.stringify(internetIdentityUrl),
    global: "globalThis",
  },
});
