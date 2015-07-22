# Atmel 8-bit AVR Emulator in React and Rust

Original [HackPad doc](https://hackpad.com/Atmel-AVR-Emulator-oInhZ8NzxKG)

* The `debugger` directory holds the react/redux frontend app
* The `server` directory holds the rust websocket server

### Installing

#### Requirements

* [Rust 1.1](http://www.rust-lang.org/install.html)
* [Node >=0.10.32](https://nodejs.org/download/)

#### Building

``` bash
cd server
cargo build
cd debugger
npm install
debugger/node_modules/webpack/bin/webpack.js # TODO: This isn't building to ./bundle.js any more - maybe react-hot-loader is interfering?
```

### Running

``` bash
cd server
cargo run
```

``` bash
open debugger/index.html
```

### Development

``` bash
cd server
cargo run
```

``` bash
cd debugger
npm start
```

Navigate to http://localhost:3000/index.html

### Testing

``` bash
cd debugger
npm test
```

``` bash
cd server
cargo test
```
