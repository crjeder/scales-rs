#![forbid(unsafe_code)]
#![no_std]

use core::marker::PhantomData;
//use core::{f64, i32, result::Result};

trait Readable<E> {
    fn read(&mut self) -> Result<i32, E>;
}

#[derive(Debug, PartialEq)]
/// Struct to hold the important values: tare, calibration factor and the device to 
/// read the data from.
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
    /// Create a new scales instance for a specific device and set the values for 
    /// tare and calibration
    fn new(tare: i32, calibration: f32, device: T) -> Self {
        Self {
            tare,
            calibration,
            device,
            ph: PhantomData,
        }
    }

    /// Change the tare value to an new one.
    fn tare(&mut self, tare: i32) {
        self.tare = tare;
    }

    /// Get a current reading from the scales device 
    /// (calibrated and tare'ed)
    fn read(&mut self) -> Result<i32, E> {
        let value = self.device.read()?;
        Ok((f64::from(value - self.tare) * f64::from(self.calibration)) as i32)
    }
}
