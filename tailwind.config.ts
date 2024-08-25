import type { Config } from "tailwindcss";

const config = {
  content: ["./frontend/**/*.tsx"],
  theme: {
    extend: {},
  },
  plugins: [],
} satisfies Config;

export default config;
