import react from "@vitejs/plugin-react";
import { defineConfig } from "vite";

import type { UserConfig } from "vite";

const root = `${process.cwd()}/frontend`;

// https://vitejs.dev/config/
const config = defineConfig({
  plugins: [react()],
  build: {
    outDir: `${process.cwd()}/dist`,
    emptyOutDir: true,
  },
  resolve: {
    alias: {
      "~/": `${root}/`,
    },
  },
  root,
  envDir: process.cwd(),
  publicDir: `${process.cwd()}/public`,
}) satisfies UserConfig;

export default config;
