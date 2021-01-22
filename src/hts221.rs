use drogue_device::prelude::*;
use embedded_hal::blocking::i2c::{Read, Write, WriteRead};
use drogue_device::mutex::Mutex;
use core::fmt::Debug;

const CTRL_REG1: u8 = 0x20;

const ADDR: u8 = 0x5F;
const WRITE: u8 = 0xBE;
const READ: u8 = 0xBF;

const WHO_AM_I: u8 = 0x0F;

const T_OUT_L: u8 = 0x2A;
const T_OUT_H: u8 = 0x2B;
// auto-increment variant
const T_OUT: u8 = 0xAA;

const T0_OUT_L: u8 = 0x3C;
const T0_OUT_H: u8 = 0x3D;
// auto-increment variant
const T0_OUT: u8 = 0xBC;

const T0_DEGC_X8: u8 = 0x32;

const T1_OUT_L: u8 = 0x3E;
const T1_OUT_H: u8 = 0x3F;
// auto-increment variant
const T1_OUT: u8 = 0xBE;

const T1_DEGC_X8: u8 = 0x33;

const T1_T0_MSB: u8 = 0x35;

pub struct Hts221<I: WriteRead + Read + Write>
    where <I as WriteRead>::Error: Debug,
          <I as Write>::Error: Debug
{
    i2c: Option<Address<Mutex<I>>>
}

impl<I: WriteRead + Read + Write> Hts221<I>
    where <I as WriteRead>::Error: Debug,
          <I as Write>::Error: Debug
{
    pub fn new() -> Self {
        Self {
            i2c: None
        }
    }

    fn read_t_out(i2c: &mut I) -> i16 {
        let mut buf = [0; 2];
        i2c.write_read(ADDR, &[T_OUT], &mut buf);
        i16::from_le_bytes(buf)
    }

    fn read_t0_out(i2c: &mut I) -> i16 {
        let mut buf = [0; 2];
        i2c.write_read(ADDR, &[T0_OUT], &mut buf);
        i16::from_le_bytes(buf)
    }

    fn read_t0_degc_x8(i2c: &mut I) -> u8 {
        let mut buf = [0; 1];
        i2c.write_read(ADDR, &[T0_DEGC_X8], &mut buf);
        buf[0]
    }

    fn read_t1_out(i2c: &mut I) -> i16 {
        let mut buf = [0; 2];
        i2c.write_read(ADDR, &[T1_OUT], &mut buf);
        i16::from_le_bytes(buf)
    }

    fn read_t1_degc_x8(i2c: &mut I) -> u8 {
        let mut buf = [0; 1];
        i2c.write_read(ADDR, &[T1_DEGC_X8], &mut buf);
        buf[0]
    }

    fn read_t1_t0_msb(i2c: &mut I) -> (u8, u8) {
        let mut buf = [0; 1];
        i2c.write_read(ADDR, &[T1_T0_MSB], &mut buf);

        let t0_msb = (buf[0] & 0b00000011) >> 2;
        let t1_msb = (buf[0] & 0b00001100) >> 2;

        (t1_msb, t0_msb)
    }

    fn calibrated_temperature_degc(i2c: &mut I) -> f32 {
        let t_out = Self::read_t_out(i2c);

        let t0_out = Self::read_t0_out(i2c);
        let t1_out = Self::read_t1_out(i2c);

        let t0_degc = Self::read_t0_degc_x8(i2c);
        let t1_degc = Self::read_t1_degc_x8(i2c);

        let (t1_msb, t0_msb) = Self::read_t1_t0_msb(i2c);

        let t0_degc = i16::from_le_bytes([t0_degc, t0_msb]) as f32 / 8.0;
        let t1_degc = i16::from_le_bytes([t1_degc, t1_msb]) as f32 / 8.0;

        let slope = (t1_degc - t0_degc ) / (t1_out - t0_out ) as f32;

        let t_degc = t0_degc as f32 + (slope * (t_out - t0_out) as f32);

        t_degc
    }

    fn c_to_f(c: f32) -> f32 {
        c * (9.0/5.0) + 32.0
    }

}

impl<I: WriteRead + Read + Write> Actor for Hts221<I>
    where <I as WriteRead>::Error: Debug,
          <I as Write>::Error: Debug
{}

pub struct SetI2c<I: WriteRead + Read + Write>(pub Address<Mutex<I>>);

impl<I: WriteRead + Read + Write> NotificationHandler<SetI2c<I>> for Hts221<I>
    where <I as WriteRead>::Error: Debug,
          <I as Write>::Error: Debug
{
    fn on_notification(&'static mut self, message: SetI2c<I>) -> Completion {
        self.i2c.replace(message.0);
        Completion::immediate()
    }
}

pub struct TakeReading;

impl<I: WriteRead + Read + Write> NotificationHandler<TakeReading> for Hts221<I>
    where <I as WriteRead>::Error: Debug,
          <I as Write>::Error: Debug
{
    fn on_notification(&'static mut self, message: TakeReading) -> Completion {
        Completion::defer(async move {
            if let Some(ref i2c) = self.i2c {
                let temp_degc = Self::calibrated_temperature_degc(&mut i2c.lock().await as &mut I);
                log::info!(" ** The temperature is {} °F", Self::c_to_f( temp_degc) );
            }
        })
    }
}

