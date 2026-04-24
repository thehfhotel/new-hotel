import type { Config } from 'tailwindcss'

/**
 * Design system: SAP Fiori Compact + oxidized blood-red brand.
 *
 * - 13px base font, 28px row height, dense spacing.
 * - Single light mode only — `darkMode: []` disables Tailwind's dark variant.
 * - Borders are 2px (visible but never pill). `rounded-full` is preserved for
 *   actual circular elements (avatars, status dots).
 */
const config: Config = {
  content: [
    './pages/**/*.{js,ts,jsx,tsx,mdx}',
    './components/**/*.{js,ts,jsx,tsx,mdx}',
    './app/**/*.{js,ts,jsx,tsx,mdx}',
  ],
  // Disable dark mode — single light theme only.
  darkMode: [],
  theme: {
    extend: {
      fontFamily: {
        sans: ['var(--font-sarabun)', 'Sarabun', 'Helvetica Neue', 'Arial', 'sans-serif'],
      },
      fontSize: {
        // Compact scale — 13px base is the SAP Fiori Compact convention.
        'xs':   ['11px', { lineHeight: '14px' }],
        'sm':   ['12px', { lineHeight: '16px' }],
        'base': ['13px', { lineHeight: '18px' }],
        'lg':   ['15px', { lineHeight: '20px' }],
        'xl':   ['17px', { lineHeight: '22px' }],
        '2xl':  ['20px', { lineHeight: '26px' }],
      },
      colors: {
        // Oxidized blood red — sober, NOT bright crimson red-600.
        brand: {
          50:  '#FBEAEA',
          100: '#F5C9C9',
          300: '#C76060',
          500: '#8B0000',
          600: '#7A0000',
          700: '#6B1212',
          800: '#4F0E0E',
        },
        // SAP shell neutrals
        shell:        '#F5F6F7',
        panel:        '#FFFFFF',
        headerBar:    '#EFF1F2',
        rowAlt:       '#FAFAFB',
        border:       '#D5D9DD',
        borderStrong: '#89919A',
        text:         '#32363A',
        textMuted:    '#6A6D70',
        // Semantic
        success: '#107E3E',
        warning: '#E9730C',
        error:   '#BB0000',
        info:    '#0A6ED1',
      },
      borderRadius: {
        // Globally squash radii to 2px. `full` is preserved for circles.
        DEFAULT: '2px',
        none:    '0',
        sm:      '2px',
        md:      '2px',
        lg:      '2px',
        xl:      '2px',
        '2xl':   '2px',
        '3xl':   '2px',
        full:    '9999px',
      },
    },
  },
  plugins: [],
}
export default config
