import type { Config } from "tailwindcss";
import { addDynamicIconSelectors } from "@iconify/tailwind";

export default {
  content: ["./app/index.html", "./app/**/*.{js,ts,jsx,tsx}"],
  theme: {
    extend: {
      fontFamily: {
        sans: ["Inter Variable", "sans-serif"],
      }
    },
  },
  plugins: [addDynamicIconSelectors()],
} satisfies Config;
