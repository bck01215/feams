/** @type {import('tailwindcss').Config} */
export default {
    content: ['./src/**/*.{html,js,svelte,ts}'],
    theme: {
      extend: {
        colors: {
            'teams-blue': {
                100: '#cce0f5',
                200: '#99c1eb',
                300: '#66a2e0',
                400: '#3382d6',
                500: '#0063cc',
                600: '#0052a3',
                700: '#004180',
                800: '#00305d',
                900: '#00203a',
              },
        },

      }
    },
    plugins: []
};