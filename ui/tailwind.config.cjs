/** @type {import('tailwindcss').Config}*/
const config = {
  content: [
    "./src/**/*.{html,js,svelte,ts}",
    "../node_modules/flowbite-svelte/**/*.{html,js,svelte,ts}",
  ],

  plugins: [require("flowbite/plugin")],

  darkMode: "class",

  theme: {
    extend: {
      colors: {
        // flowbite-svelte
        primary: {
          50: "#edefff",
          100: "#dfe0ff",
          200: "#c5c6ff",
          300: "#a2a1ff",
          400: "#897cfd",
          500: "#785df7",
          600: "#6a3fec",
          700: "#5c32d0",
          800: "#4b2ba8",
          900: "#3a277a",
          950: "#26194d",
        },
      },
    },
  },
};

module.exports = config;
