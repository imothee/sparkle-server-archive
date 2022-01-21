module.exports = {
  content: [
      './client/**/*.res',
      './client/**/*.jsx',
      './templates/**/*.html',
  ],
  theme: {
    extend: {
      colors: {
        light: {
          shade: '#F6F5F4',
          accent: '#64B8E9',
        },
        brand: '#6B9CC2',
        dark: {
          shade: '#324871',
          accent: '#908D8E',
        },
      },
    },
  },
  plugins: [],
}
