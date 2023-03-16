#![forbid(unsafe_code)]
#![no_std]

use core::marker::PhantomData;
use core::{f64, i32, result::Result};

trait Readable<E> {
    fn read(&mut self) -> Result<i32, E>;
}

#[derive(Debug, PartialEq)]
struct Scales<T: Readable<E>, E> {
    tare: i32,
    calibration: f32,
    device: T,
    ph: PhantomData<E>,
}

#[allow(dead_code)]
impl<T, E> Scales<T, E>
where
    T: Readable<E>,
{
    fn new(tare: i32, calibration: f32, device: T) -> Self {
        Self {
            tare,
            calibration,
            device,
            ph: PhantomData,
        }
    }

    fn tare(&mut self, tare: i32) {
        self.tare = tare;
    }

    fn read(&mut self) -> Result<i32, E> {
        let value = self.device.read()?;
        Ok((f64::from(value - self.tare) * f64::from(self.calibration)) as i32)
    }
}
