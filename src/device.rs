
use stm32l4xx_hal::pac::Interrupt;
use crate::led::{LED, On, Off};
use stm32l4xx_hal::gpio::{PA5, Output, PushPull, PC13, Input, PullUp};
use crate::button::{Button, ButtonEvent, SetSink};
use drogue_device::prelude::*;

type Ld1Actor = LED<PA5<Output<PushPull>>>;
type ButtonInterrupt<SINK> = Button<PC13<Input<PullUp>>, SINK>;

pub struct MyDevice {
    pub ld1: ActorContext<Ld1Actor>,
    pub button: InterruptContext<ButtonInterrupt<ButtonToLed>>,
}

impl Device for MyDevice {
    fn start(&'static mut self, supervisor: &mut Supervisor) {
        let ld1_addr = self.ld1.start(supervisor);
        let button_addr = self.button.start(supervisor);
        button_addr.notify( SetSink(ButtonToLed::new(ld1_addr)));
    }
}

pub struct ButtonToLed {
    sink: Address<Ld1Actor>,
}

impl ButtonToLed {
    pub fn new(sink: Address<Ld1Actor>) -> Self {
        Self {
            sink
        }
    }
}

impl Sink<ButtonEvent> for ButtonToLed {
    fn notify(&self, message: ButtonEvent) {
        match message {
            ButtonEvent::Pressed => {
                log::info!("notify on");
                self.sink.notify( On )
            }
            ButtonEvent::Released => {
                log::info!("notify off");
                self.sink.notify( Off )
            }
        }
    }
}

