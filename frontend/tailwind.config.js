/** @type {import('tailwindcss').Config} */
export default {
  content: ['./index.html', './src/**/*.{js,ts,jsx,tsx}'],
  darkMode: 'class',
  theme: {
    extend: {
      colors: {
        primary: {
          DEFAULT: '#5C6AC4',
          50: '#F0F1F9',
          100: '#E1E4F3',
          200: '#C3C9E7',
          300: '#A5AEDB',
          400: '#8793CF',
          500: '#5C6AC4',
          600: '#4A559D',
          700: '#374076',
          800: '#252B4E',
          900: '#121627',
        },
        secondary: {
          DEFAULT: '#FFB347',
          50: '#FFF9F0',
          100: '#FFF3E1',
          200: '#FFE7C3',
          300: '#FFDBA5',
          400: '#FFCF87',
          500: '#FFB347',
          600: '#FF9F1A',
          700: '#EB8C00',
          800: '#B86D00',
          900: '#854E00',
        },
        background: {
          light: '#FFFFFF',
          lighter: '#F5F6FA',
          dark: '#1A1A1A',
          darker: '#0F0F0F',
        },
        text: {
          primary: '#1A1A1A',
          secondary: '#737B8C',
          light: '#E4E6EB',
          dark: '#FFFFFF',
        },
        border: {
          light: '#E4E6EB',
          dark: '#2D2D2D',
        },
      },
      fontFamily: {
        sans: ['Inter', 'system-ui', 'sans-serif'],
        display: ['Poppins', 'system-ui', 'sans-serif'],
        heading: ['Montserrat', 'system-ui', 'sans-serif'],
        mono: ['Source Code Pro', 'monospace'],
      },
      boxShadow: {
        card: '0 4px 12px rgba(0, 0, 0, 0.06)',
        'card-dark': '0 4px 12px rgba(0, 0, 0, 0.3)',
      },
      borderRadius: {
        card: '1rem',
      },
    },
  },
  plugins: [],
};
