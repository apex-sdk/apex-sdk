import type { Config } from 'tailwindcss'

const config: Config = {
  content: [
    "./app/**/*.{js,ts,jsx,tsx,mdx}",
    "./pages/**/*.{js,ts,jsx,tsx,mdx}",
    "./components/**/*.{js,ts,jsx,tsx,mdx}",
    "./src/**/*.{js,ts,jsx,tsx,mdx}",
  ],
  theme: {
    extend: {
      colors: {
        // Core Palette
        obsidian: {
          DEFAULT: '#0B0C10', // Deepest background
          light: '#1F2833',   // Component background
          lighter: '#2C3440', // Borders/Separators
        },
        hyperBlue: {
          DEFAULT: '#2176FF', // Primary Action / Active State
          dim: 'rgba(33, 118, 255, 0.1)', // Subtle background for active items
          hover: '#005BEA',
        },
        vividOrange: {
          DEFAULT: '#FF5A1F', // "Desire Line" / Call to Action / Pulse
          light: '#FF7F50',
        },
        slate: {
          gray: '#94a3b8', // Inactive text
          light: '#cbd5e1', // Secondary text
        },
        deepNavy: '#0F172A', // Headers (start of gradient)
      },
      fontFamily: {
        sans: ['Inter', 'system-ui', 'sans-serif'], // Clean, modern sans
        mono: ['JetBrains Mono', 'Fira Code', 'monospace'], // High-legibility code
      },
      transitionDuration: {
        DEFAULT: '200ms', // "Linear-style" mechanical feel
      },
      transitionTimingFunction: {
        DEFAULT: 'cubic-bezier(0.4, 0, 0.2, 1)', // Snappy, precise
      },
      animation: {
        'pulse-orange': 'pulse-border 2s cubic-bezier(0.4, 0, 0.6, 1) infinite',
      },
      keyframes: {
        'pulse-border': {
          '0%, 100%': { borderColor: '#FF5A1F', boxShadow: '0 0 0 0 rgba(255, 90, 31, 0.7)' },
          '50%': { borderColor: '#FF5A1F', boxShadow: '0 0 0 4px rgba(255, 90, 31, 0)' },
        },
      },
    },
  },
  plugins: [],
}

export default config
