module.exports = {
  content: ["./src/views/*.ncl", "./src/**/*.rs"],
  theme: {
    extend: {
      colors: {
        background: '#0f172a', // slate-900
        surface: '#1e293b',    // slate-800
        primary: '#3b82f6',    // blue-500
        secondary: '#64748b',  // slate-500
        success: '#22c55e',    // green-500
        warning: '#eab308',    // yellow-500
        danger: '#ef4444',     // red-500
      },
      fontFamily: {
        sans: ['Inter', 'sans-serif'],
      },
      animation: {
        'pulse-slow': 'pulse 3s cubic-bezier(0.4, 0, 0.6, 1) infinite',
      }
    },
  },
  plugins: [],
}
