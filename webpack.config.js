const path = require('path')
const MiniCssExtractPlugin = require('mini-css-extract-plugin')

module.exports = {
  entry: [
    './src/assets/index.js',
    './src/assets/index.css',
  ],
  output: {
    path: path.resolve(__dirname, 'public/assets'),
    publicPath: '/assets/',
    filename: 'site.js',
  },
  plugins: [
    new MiniCssExtractPlugin({
      filename: 'site.css',
    }),
  ],
  module: {
    rules: [
      {
        test: /\.css$/,
        use: [
          MiniCssExtractPlugin.loader,
          'css-loader',
        ],
      },
    ],
  },
}
