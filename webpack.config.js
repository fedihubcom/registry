const path = require('path')

module.exports = {
  entry: './src/index.js',
  output: {
    path: path.resolve(__dirname, 'public/assets'),
    publicPath: '/assets/',
    filename: 'site.js',
  },
}
