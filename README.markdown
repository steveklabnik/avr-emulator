# Atmel 8-bit AVR Emulator in React and Rust

Original [HackPad doc](https://hackpad.com/Atmel-AVR-Emulator-oInhZ8NzxKG)

* The `debugger` directory holds the react/redux frontend app
* The `server` directory holds the rust websocket server

### Installing

#### Requirements

* [Rust 1.1](http://www.rust-lang.org/install.html)
* [Node >=0.10.32](https://nodejs.org/download/)

```
cd server
cargo build
cd debugger
npm install
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
cd debugger
npm start
```

#### Testing
