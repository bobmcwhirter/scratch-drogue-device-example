use stm32l4xx_hal::pac::{Interrupt, I2C2};
use stm32l4xx_hal::gpio::{PA5, Output, PushPull, PC13, Input, PullUp, OpenDrain, AF4, Alternate, PB10, PB11, PD15, PullDown, Floating, PB14};
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
use drogue_device::driver::sensor::hts221::Sensor;
use drogue_device::driver::timer::Timer;

type Ld1Actor = SimpleLED<MyDevice, PA5<Output<PushPull>>>;
type Ld2Actor = SimpleLED<MyDevice, PB14<Output<PushPull>>>;
type ButtonInterrupt = Button<PC13<Input<PullUp>>>;

type I2cScl = PB10<Alternate<AF4, Output<OpenDrain>>>;
type I2cSda = PB11<Alternate<AF4, Output<OpenDrain>>>;
type I2cPeriph = I2c<I2C2, (I2cScl, I2cSda)>;
type I2cActor = Mutex<MyDevice, I2cPeriph>;

use drogue_device::driver::timer::stm32l4xx::Timer as McuTimer;
use stm32l4xx_hal::pac::TIM15;
use drogue_device::driver::led::Blinker;

type Blinker1Actor = Blinker<MyDevice, PA5<Output<PushPull>>, TIM15, McuTimer<TIM15>>;
type Blinker2Actor = Blinker<MyDevice, PB14<Output<PushPull>>, TIM15, McuTimer<TIM15>>;

type TimerActor = Timer<MyDevice, TIM15, McuTimer<TIM15>>;

type Hts221Package = Hts221<MyDevice, PD15<Input<PullDown>>, I2cPeriph>;
type Hts221Sensor = Sensor<MyDevice, I2cPeriph>;

pub struct MyDevice {
    pub ld1: ActorContext<MyDevice, Ld1Actor>,
    pub ld2: ActorContext<MyDevice, Ld2Actor>,
    pub blinker1: ActorContext<MyDevice, Blinker1Actor>,
    pub blinker2: ActorContext<MyDevice, Blinker2Actor>,
    pub button: InterruptContext<MyDevice, ButtonInterrupt>,
    pub i2c: ActorContext<MyDevice, I2cActor>,
    pub hts221: Hts221Package,
    pub timer: InterruptContext<MyDevice, Timer<MyDevice, TIM15, McuTimer<TIM15>>>,
}

impl Device for MyDevice {
    fn mount(&'static mut self, supervisor: &mut Supervisor) {
        let ld1_addr = self.ld1.mount(self, supervisor);
        let ld2_addr = self.ld2.mount(self, supervisor);

        let blinker1_addr = self.blinker1.mount(self, supervisor);
        let blinker2_addr = self.blinker2.mount(self, supervisor);

        let i2c_addr = self.i2c.mount(self, supervisor);
        let hts221_addr = self.hts221.mount(self, supervisor);
        let timer_addr = self.timer.mount(self, supervisor);

        blinker1_addr.bind(&timer_addr);
        blinker1_addr.bind(&ld1_addr);

        blinker2_addr.bind(&timer_addr);
        blinker2_addr.bind(&ld2_addr);


        hts221_addr.bind(&i2c_addr);

        let button_addr = self.button.mount(self, supervisor);
    }
}

pub struct ButtonToLed {
    ld1: Address<MyDevice, Ld1Actor>,
    hts221: Address<MyDevice, Hts221Sensor>,
}

impl ButtonToLed {
    pub fn new(ld1: Address<MyDevice, Ld1Actor>, hts221: Address<MyDevice, Hts221Sensor>) -> Self {
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

