# Atmel 8-bit AVR Emulator in React and Rust

## Summary

This is an Atmel 8-bit AVR Emulator written in Rust with a React frontend for rendering a debugger. The two communicate via a websocket server. Check out the live demo at [realscout.github.io/avr-emulator](http://realscout.github.io/avr-emulator).

* The React/Redux debugger is located in the `debugger` directory
* The Rust websocket server and emulator library are located in the `server` directory

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

Start node server to activate [react-hot-loader](https://github.com/gaearon/react-hot-loader) from within the `./debugger` directory

``` bash
npm start
```

Start Rust websocket server from within the `./server` directory

``` bash
cargo run
```

Navigate to http://localhost:3000/index.html

## Testing

From within the `./debugger` directory

``` bash
npm test
```

From within the `./server` directory

``` bash
cargo test
```

## Misc

Original [HackPad doc](https://hackpad.com/Atmel-AVR-Emulator-oInhZ8NzxKG)
