use drogue_device::prelude::*;
use embedded_hal::blocking::i2c::{Read, Write, WriteRead};
use drogue_device::mutex::Mutex;
use core::fmt::Debug;

const CTRL_REG1: u8 = 0x20;

const ADDR: u8 = 0x5F;
const WRITE: u8 = 0xBE;
const READ: u8 = 0xBF;

const WHO_AM_I: u8 = 0x0F;
const TEMP_OUT_L: u8 = 0x2A;
const TEMP_OUT_H: u8 = 0x2B;

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
        log::info!("wired up i2c");
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
            log::info!("taking reading --- dude");
            let mut i2c = self.i2c.as_mut().unwrap().lock().await;


            let buf = [CTRL_REG1, 1 << 7];
            let result = i2c.write(ADDR, &buf);
            match result {
                Ok(_) => {
                    log::info!("### GOOD write CTRL_REG1");
                }
                Err(e) => {
                    log::info!("### BAD write CTRL_REG1 {:?}", e);
                }
            }

            let buf = [TEMP_OUT_L];
            let mut temp = [0; 1];
            let result = i2c.write_read(ADDR, &buf, &mut temp);
            match result {
                Ok(_) => {
                    log::info!("### GOOD write_read");
                }
                Err(e) => {
                    log::info!("### BAD write_read {:?}", e);
                }
            }

            log::info!("***************** temp L {}", temp[0]);

            let buf = [TEMP_OUT_H];
            let mut temp = [0; 1];
            let result = i2c.write_read(ADDR, &buf, &mut temp);
            match result {
                Ok(_) => {
                    log::info!("### GOOD write_read");
                }
                Err(e) => {
                    log::info!("### BAD write_read {:?}", e);
                }
            }

            log::info!("***************** temp H {}", temp[0]);

        })
    }
}

