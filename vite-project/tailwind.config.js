import {heroui} from "@heroui/theme";

/** @type {import('tailwindcss').Config} */
export default {
    content: [
        "./index.html",
        "./src/**/*.{js,ts,jsx,tsx}",
        "./node_modules/@heroui/theme/dist/**/*.{js,ts,jsx,tsx}",
        "./node_modules/@heroui/theme/dist/components/(button|user|ripple|spinner|avatar).js"
    ],
    theme: {
        extend: {},
    },
    darkMode: "class",
    plugins: [heroui()],
}