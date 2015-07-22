# Atmel 8-bit AVR Emulator in React and Rust

## Summary

Original [HackPad doc](https://hackpad.com/Atmel-AVR-Emulator-oInhZ8NzxKG)

* The `debugger` directory holds the react/redux frontend app
* The `server` directory holds the rust websocket server

## Installing

#### Requirements

* [Rust 1.1](http://www.rust-lang.org/install.html)
* [Node >=0.10.32](https://nodejs.org/download/)

#### Building the Rust Server

From within the `./server` directory:

``` bash
cargo build
```

#### Building `./demo.js`

From within the `./debugger` directory:

``` bash
npm install
./node_modules/webpack/bin/webpack.js # TODO: This isn't building to ./bundle.js any more - maybe react-hot-loader is interfering?
```

## Development

Start node server to activate [react-hot-loader](https://github.com/gaearon/react-hot-loader)
``` bash
cd debugger
npm start
```

Start Rust websocket server
``` bash
cd server
cargo run
```

Navigate to http://localhost:3000/index.html

## Testing

``` bash
cd debugger
npm test
```

``` bash
cd server
cargo test
```
