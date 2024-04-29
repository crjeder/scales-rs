#![forbid(unsafe_code)]
#![no_std]

use core::ops::{Sub, Div, Mul};

#[derive(Debug, PartialEq)]
/// Struct to hold the important values: tare, calibration factor and the device to 
/// read the data from.
struct ScalesData<Int: Sub<Int> + From<Fp> + core::convert::From<<Int as core::ops::Sub>::Output>, Fp: Div<Fp> + From<Int>> {
    tare: Int,
    offset: Int,
    calibration: Fp,
}

#[allow(dead_code)]
pub trait Scales {
    type Error: Debug;
    /// Change the tare value to an new one.
    fn tare(&mut self, tare: Int) {
        self.tare = tare;
    }

    /// Get a current reading from the scales device 
    /// (calibrated and tare'ed)
    /// # Errors
    /// The read() errors of the underlaying device
    fn read(&mut self, ScalesData data) -> Result<Int, Self::Error> {
        let value: Int = self.device.read()?;
        // reading = (raw / calibration) - offset - tare
        //Ok(Int::from(Fp::from(value - self.tare) * self.calibration))
        let tared: Int = (value - self.offset - self.tare).into();
        let calibrated: Fp = Fp::from(tared) * self.calibration;
        Ok(Int::from(calibrated))
    }
}
