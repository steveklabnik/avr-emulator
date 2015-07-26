var path = require('path');
var webpack = require('webpack');

module.exports = {
    entry: "./entry.js",
    output: {
        path: path.join(__dirname, 'dist'),
        filename: "bundle.js"
    },
    module: {
        loaders: [
          {
            test: /\.jsx?$/,
            exclude: /(node_modules|bower_components)/,
            loaders: ['babel?stage=0']
          },
          {
            test: /\.scss$/,
            loader: 'style!css!sass'
          }
        ]
    },
};
