# Atmel 8-bit AVR Emulator in React and Rust

## Summary

This is an Atmel 8-bit AVR Emulator written in **Rust** with a **React** frontend for rendering a debugger. The two communicate via a websocket server. 


* The React/Redux debugger is located in the `debugger` directory
* The Rust websocket server and emulator library are located in the `server` directory

## Live Demo 

#### [realscout.github.io/avr-emulator](http://realscout.github.io/avr-emulator)

## Why?

Inspired by [Starfighter's Summer 2015 announcement](http://sockpuppet.org/blog/2015/07/13/starfighter/), we thought that implementing our own [Atmel AVR 8-bit](http://www.atmel.com/images/atmel-0856-avr-instruction-set-manual.pdf) emulator would be a really fun problem to tackle. We also thought it'd be a great excuse to dive a little deeper into a few technologies we've had our eyes on. During our last biweekly hack day we dove in, and in the last week got a working version up.

## Reducers

We've fallen in love with [redux's](https://github.com/gaearon/redux) reducers, so we decided to take them to the server as well. From their docs, a reducer is:

```
(previousState, action) => newState
```

In the emulator, `previousState` and `newState` capture the emulator's memory: Registers, IO, and SRAM, etc. The action is the next instruction (ex: `inc r1`):

```
(previousEmulatorMemory, instruction) => newEmulatorMemory
```

Here is the complete diagram of how everything works together. (The reducers are in yellow)

<img src="https://s3.amazonaws.com/uploads.hipchat.com/65625/949611/xi6FKFfeeXLYsjI/Rust-React%20Emulator.png"/>

## Development

#### Requirements

* [Rust 1.1](http://www.rust-lang.org/install.html)
* [Node >=0.10.32](https://nodejs.org/download/)

Start node server to activate [react-hot-loader](https://github.com/gaearon/react-hot-loader) from within the `./debugger` directory

``` bash
npm start
```

Start Rust websocket server from within the `./server` directory

``` bash
cargo run # will build and run
```

Navigate to http://localhost:3000/index.html

## Testing

From within the `./server` directory

``` bash
cargo test
```

## Wish List

There is still a lot of ground to cover - here's what's top of mind:

* Opcode coverage - there is a base of [7 opcodes](https://github.com/RealScout/avr-emulator/tree/master/server/src/opcodes), but there are ~150 to go!
* Add SRAM memory display to [EmulatorApp](https://github.com/RealScout/avr-emulator/blob/master/debugger/containers/EmulatorApp.js).
* Render ATmega8515 in debugger with buttons and LEDS based on DDRA, DDRB and DDRC port configuration.
* Ability to load arbitrary assembly programs into the emulator.
* Add dissassembly to [assembler](https://github.com/RealScout/avr-emulator/blob/master/server/src/assembler.rs).
* Refactor instruction parsing (opcodes are currently handling operand parsing in `.perform` - tsk tsk).
* Get frontend testing up and running

## About RealScout

We [pair program](http://eatcodeplay.com/why-we-killed-off-code-reviews/), hold [hack days](https://twitter.com/chrisconley/status/618830194971774976) every other Friday and have recently improved **test coverage** from **45% to 80%**. The focus seems to be paying off: Our agents have closed **$440 million** in sales with RealScout over the last 6 months.

Blog: [eatcodeplay.com](http://eatcodeplay.com/)

Twitter: [@realscouteng](https://twitter.com/realscouteng)
