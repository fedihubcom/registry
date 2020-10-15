const path = require('path')

module.exports = {
  entry: './src/assets/index.js',
  output: {
    path: path.resolve(__dirname, 'public/assets'),
    publicPath: '/assets/',
    filename: 'site.js',
  },
}
