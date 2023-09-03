/** @type {import('tailwindcss').Config}*/
const config = {
  content: [
    "./src/**/*.{html,js,svelte,ts}",
    "../node_modules/flowbite-svelte/**/*.{html,js,svelte,ts}",
  ],

  plugins: [require("flowbite/plugin")],

  darkMode: "class",

  theme: {
    fontFamily: {
      pixel: ["'Press Start 2P'"],
    },
    extend: {
      colors: {
        // flowbite-svelte
        fractalorange: "#E94B2A",
        primary: {
          50: "#fef3ee",
          100: "#fce4d8",
          200: "#f8c6b0",
          300: "#f39e7e",
          400: "#ed6c4a",
          500: "#e94b2a",
          600: "#da301c",
          700: "#b52119",
          800: "#901e1c",
          900: "#741c1a",
          950: "#3f0b0c",
        },
      },
    },
  },
};

module.exports = config;
