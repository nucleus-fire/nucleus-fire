/** @type {import('tailwindcss').Config} */
module.exports = {
  content: ["./src/**/*.{html,js,ncl,rs}"],
  theme: {
    extend: {
      colors: {
        brand: {
          dark: '#0f172a',
          primary: '#3b82f6',
          accent: '#8b5cf6',
        }
      },
      fontFamily: {
        sans: ['Inter', 'sans-serif'],
      }
    },
  },
  plugins: [],
}
