//! # Scenario
//!
//! We're given an AVR that implements a ROT13 encoder on SPI.
//!
//! # Firmware
//!
//! See: [../../../avr-tester-tests/spi/src/main.rs].

use crate::prelude::*;

#[test]
fn test() {
    let mut avr = avr("spi");

    avr.run_for_ms(1);

    // let c = avr.spi().recv_byte();
    // println!("{c:?}");

    for i in 0..10 {
        avr.spi().send_byte('L' as u8);
        let c = avr.spi().recv_byte();
        println!("{c:?}");
    }
    let c = avr.spi().recv_byte();
    assert_eq!(c, Some('Y' as u8));


    // let mut assert = |given: &str, expected: &str| {
    //     avr.spi().send(given);
    //     avr.run_for_ms(50);

    //     assert_eq!(expected, avr.uart0().recv::<String>());
    // };

    // assert("Hello, World!", "Uryyb, Jbeyq!");

    // assert(
    //     "Lorem ipsum dolor sit amet, consectetur adipiscing elit. Praesent \
    //       non maximus purus. Fusce a neque condimentum, finibus dui et, tempor",
    //     "Yberz vcfhz qbybe fvg nzrg, pbafrpgrghe nqvcvfpvat ryvg. Cenrfrag \
    //       aba znkvzhf chehf. Shfpr n ardhr pbaqvzraghz, svavohf qhv rg, grzcbe",
    // );
}
