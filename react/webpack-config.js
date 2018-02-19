var webpack = require('webpack');
var path = require('path');

module.exports = {
    entry: [
        path.join(__dirname, 'src/index.js')
    ],
    module: {
        loaders: [{
                test: /\.(js|jsx)$/,
                    exclude: /node_modules/,
                    loader: 'babel-loader'
            },
            {
                test: /\.css$/,
                use: [ 'style-loader', 'css-loader' ]
            },
            {
                test: /\.(png|woff|woff2|eot|ttf|svg)(\?v=[0-9]\.[0-9]\.[0-9])?$/,
                loader: 'url-loader'
            }
        ]
    },
    output: {
        path: '/dist',
        filename: 'bundle.js'
    }
}