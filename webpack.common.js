const path = require('path');

module.exports = {
  entry: {
    app: './client/main.bs.js',
  },
  plugins: [],
  resolve: {
    extensions: ['.js'],
    alias: {
      '~': path.join(__dirname, 'client'),
    },
  },
  output: {
    publicPath: '/',
  },
};

