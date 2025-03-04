# avr-tester &emsp; [![crates-badge]][crates-link] [![docs-badge]][docs-link]

[crates-badge]: https://img.shields.io/crates/v/avr-tester.svg
[crates-link]: https://crates.io/crates/avr-tester
[docs-badge]: https://img.shields.io/docsrs/avr-tester
[docs-link]: https://docs.rs/avr-tester

Functional testing framework for [AVR] binaries, powered by [simavr].

tl;dr get your microcontroller's firmware black-box-tested in seconds!

Status: alpha; work in progress.

[AVR]: https://en.wikipedia.org/wiki/AVR_microcontrollers
[simavr]: https://github.com/buserror/simavr

## Getting Started

Create a crate dedicated to your project's tests:

```shell
$ cargo new yourproject-tests --lib
```

... add `avr-tester` as its dependency:

```toml
# yourproject-tests/Cargo.toml

[dependencies]
avr-tester = "0.1"
```

... and, just like that, start writing tests:

```rust
// yourproject-tests/src/lib.rs

use avr_tester::AvrTester;

fn avr() -> AvrTester {
    AvrTester::atmega328p()
        .with_clock_of_16_mhz()
        .load("../../yourproject/target/atmega328p/release/yourproject.elf")
}

// Assuming `yourproject` implements an ROT-13 encoder:

#[test]
fn short_text() {
    let mut avr = avr();

    // Let's give our firmware a moment to initialize:
    avr.run_for_ms(1);

    // Now, let's send the string:
    avr.uart0().send("Hello, World!");

    // ... give the AVR a moment to retrieve it & send back, encoded:
    avr.run_for_ms(1);

    // ... and, finally, let's assert the outcome:
    assert_eq!("Uryyb, Jbeyq!", avr.uart0().recv::<String>());
}

#[test]
fn long_text() {
    let mut avr = avr();

    avr.run_for_ms(1);
    avr.uart0().send("Lorem ipsum dolor sit amet, consectetur adipiscing elit");
    avr.run_for_ms(1);

    assert_eq!(
        "Yberz vcfhz qbybe fvg nzrg, pbafrpgrghe nqvcvfpvat ryvg",
        avr.uart0().recv::<String>(),
    );
}
```

... having the tests ready (and [requirements](#requirements--supported-platforms)
met!), just run `cargo test` inside `yourproject-tests` :-)

Note that because AvrTester _simulates an actual AVR_, you don't have to modify
`yourproject` at all - it's free to use timers, GPIOs etc. and everything should
just work.

In fact, `yourproject` doesn't even have to be written in Rust - you can create
Rust-based tests for a firmware written in C, Zig or anything else!

## Usage

- [Analog pins](avr-tester/tests/tests/pins-analog.rs),
- [Digital pins](avr-tester/tests/tests/pins-digital.rs),
- [UARTs](avr-tester/tests/tests/uart.rs),
- [Shift registers](avr-tester/tests/tests/components-shift-register.rs).

## Requirements & supported platforms

See: [simavr-ffi](https://github.com/Patryk27/simavr-ffi).

## Roadmap

Following features seem to be supported by simavr, but haven't been yet exposed
in AvrTester:

- interrupts,
- EEPROM,
- SPI,
- I2C,
- watchdog,
- TWI,
- <https://lib.rs/crates/simavr-section>,
- USB.

(i.e. your firmware can use those features, but you won't be able to test them.)

## Caveats

- Triggering AVR's sleep mode will cause the Rust code to gracefully `panic!()`,
  because the only way to wake an AVR is to trigger an interrupt and those are
  not yet supported.

## Contributing

Pull requests are very much welcome!

### Tests

AvrTester's integration tests lay in `avr-tester/tests` - you can run them with:

```shell
$ cd avr-tester
$ cargo test
```

Note that for those tests to work, you might need some additional
dependencies:

#### ... on Nix

```shell
$ nix-shell
# and then `cargo test`
```

#### ... on Ubuntu

```shell
$ sudo apt install avr-libc gcc-avr
# and then `cargo test`
```

## License

Copyright (c) 2022, Patryk Wychowaniec <pwychowaniec@pm.me>.    
Licensed under the MIT license.
