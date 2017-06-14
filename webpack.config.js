const path = require('path');

const build = `${__dirname}/target/client`;

module.exports = {
  devtool: 'source-map',
  entry: {
    'empholite': ['babel-polyfill', './src/client/index.jsx']
  },
  output: {
    path: build,
    filename: '[name].js',
    library: '[name]',
    libraryTarget: 'umd'
  },
  resolve: {
    extensions: ['.js', '.jsx'],
    modules: [
      path.resolve('./src/client'),
      "node_modules"
    ]
  },
  module: {
    loaders: [
      {
        test: /\.jsx?$/,
        loader: 'babel-loader',
        exclude: /node_modules/,
        query: {
          presets: ['react', 'latest'],
          plugins: ['transform-class-properties']
        }
      }
    ],
  }
};
