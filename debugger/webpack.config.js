// Needed to run plugins
var path = require('path');
var webpack = require('webpack');

// Might want to consider adding jsx-loader to the loaders
// See: http://gaearon.github.io/react-hot-loader/getstarted/ example repo at the bottom

module.exports = {
    entry: [
      'webpack-dev-server/client?http://0.0.0.0:3000', // WebpackDevServer host and port
      'webpack/hot/only-dev-server',
      './entry.js' // Your appʼs entry point
    ],
    output: {
        path: path.join(__dirname, 'dist'),
        filename: "bundle.js"
    },
    resolve: {
      extensions: ['', '.js', '.jsx']
    },
    module: {
        loaders: [
          {
            test: /\.jsx?$/,
            exclude: /(node_modules|bower_components)/,
            loaders: ['react-hot', 'babel?stage=0']
          },
          {
            test: /\.scss$/,
            loader: "style!css!sass"
          }
        ]
    },
    plugins: [
      new webpack.HotModuleReplacementPlugin(),
      new webpack.NoErrorsPlugin()
    ]
};
