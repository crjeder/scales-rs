use core::{u32, f32};

#[derive(Debug, PartialEq, Deserialize, Serialize)]
struct Scales <T> {
    tare: u32,
    calibration: f32,
    device: T impl read() -> u32;
}

impl Scales {
    fn new(tare: u32, calibration: f32) -> Self { Self { tare, calibration } }

    fn tare(&self, tare: u32) {
        self.tare = tare;
    }

    fn read
}
