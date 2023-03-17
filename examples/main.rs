//use embedded_hal::blocking::spi::{Transfer, Write};

// embedded_hal implementation
use embedded_hal_mock::spi::{Mock as Spi, Transaction as SpiTransaction};

use hx711_spi::Hx711;
use nb::block;

// minimal example
fn main() -> Result<(), core::error::Error> {
    // Data the mocked up SPI bus shou
    let expectations = [
    SpiTransaction::send(0x09),
    SpiTransaction::read(0x0A),
    SpiTransaction::send(0xFE),
    SpiTransaction::read(0xFF),
    SpiTransaction::write(vec![1, 2]),
    SpiTransaction::transfer(vec![3, 4], vec![5, 6]),
];

    let mut spi = Spi::new(&expectations);
    let mut hx711 = Hx711::new(spi);

    hx711.reset()?;
    let v = block!(hx711.read())?;
    println!("value = {}", v);

    Ok(())
}
