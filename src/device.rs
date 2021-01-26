use stm32l4xx_hal::pac::{Interrupt, I2C2};
use stm32l4xx_hal::gpio::{PA5, Output, PushPull, PC13, Input, PullUp, OpenDrain, AF4, Alternate, PB10, PB11, PD15, PullDown, Floating};
use crate::button::{Button, ButtonEvent};
use drogue_device::{
    prelude::*,
    synchronization::Mutex,
    driver::{
        sensor::hts221::{
            Hts221,
        },
        led::SimpleLED,
    },
};
use stm32l4xx_hal::i2c::I2c;
use drogue_device::driver::ActiveHigh;
use drogue_device::driver::sensor::hts221::Sensor;

type Ld1Actor = SimpleLED<PA5<Output<PushPull>>, ActiveHigh>;
type ButtonInterrupt = Button<PC13<Input<PullUp>>>;

type I2cScl = PB10<Alternate<AF4, Output<OpenDrain>>>;
type I2cSda = PB11<Alternate<AF4, Output<OpenDrain>>>;
type I2cPeriph = I2c<I2C2, (I2cScl, I2cSda)>;
type I2cActor = Mutex<I2cPeriph>;

type Hts221Package = Hts221<PD15<Input<PullDown>>, I2cPeriph>;
type Hts221Sensor = Sensor<I2cPeriph>;

pub struct MyDevice {
    pub ld1: ActorContext<Ld1Actor>,
    pub button: InterruptContext<ButtonInterrupt>,
    pub i2c: ActorContext<I2cActor>,
    pub hts221: Hts221Package,
}

impl Device for MyDevice {
    fn start(&'static mut self, supervisor: &mut Supervisor) {
        let ld1_addr = self.ld1.mount(supervisor);
        let i2c_addr = self.i2c.mount(supervisor);
        let hts221_addr = self.hts221.mount(supervisor);

        hts221_addr.bind(&i2c_addr);

        let button_addr = self.button.start(supervisor);
    }
}

pub struct ButtonToLed {
    ld1: Address<Ld1Actor>,
    hts221: Address<Hts221Sensor>,
}

impl ButtonToLed {
    pub fn new(ld1: Address<Ld1Actor>, hts221: Address<Hts221Sensor>) -> Self {
        Self {
            ld1,
            hts221,
        }
    }
}

/*
impl Sink<ButtonEvent> for ButtonToLed {
    fn notify(&self, message: ButtonEvent) {
        match message {
            ButtonEvent::Pressed => {
                self.ld1.turn_on();
                self.hts221.trigger_read_temperature();
            }
            ButtonEvent::Released => {
                self.ld1.turn_off();
            }
        }
    }
}

 */

