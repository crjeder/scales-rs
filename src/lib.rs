#![forbid(unsafe_code)]
#![no_std]

use core::marker::PhantomData;
use core::ops::{Sub, Div, Mul};

trait Readable<Int, E> 
    where
    Int: Sub<Int>, 
{
    fn read(&mut self) -> Result<Int, E>;
}

#[derive(Debug, PartialEq)]
/// Struct to hold the important values: tare, calibration factor and the device to 
/// read the data from.
struct Scales<T: Readable<Int, E>, E, Int: Sub<Int> + From<Fp> + core::convert::From<<Int as core::ops::Sub>::Output>, Fp: Div<Fp> + From<Int>> {
    tare: Int,
    calibration: Fp,
    device: T,
    ph: PhantomData<E>,
}

#[allow(dead_code)]
impl<T, E, Int, Fp> Scales<T, E, Int, Fp>
where
    T: Readable<Int, E>,
    Fp: Div<Fp> + From<Int> + Mul<Output = Fp> + Copy, 
    Int: Sub<Int> + From<Fp> + core::convert::From<<Int as Sub>::Output> + Copy,
{
    /// Create a new scales instance for a specific device and set the values for 
    /// tare and calibration
    fn new(tare: Int, calibration: Fp, device: T) -> Self {
        Self {
            tare,
            calibration,
            device,
            ph: PhantomData,
        }
    }

    /// Change the tare value to an new one.
    fn tare(&mut self, tare: Int) {
        self.tare = tare;
    }

    /// Get a current reading from the scales device 
    /// (calibrated and tare'ed)
    /// # Errors
    /// The read() errors of the underlaying device
    fn read(&mut self) -> Result<Int, E> {
        let value: Int = self.device.read()?;
        //Ok(Int::from(Fp::from(value - self.tare) * self.calibration))
        let tared: Int = (value - self.tare).into();
        let calibrated: Fp = Fp::from(tared) * self.calibration;
        Ok(Int::from(calibrated))
    }
}
