
use stm32l4xx_hal::pac::Interrupt;
use crate::led::{LED, On, Off};
use stm32l4xx_hal::gpio::{PA5, Output, PushPull, PC13, Input, PullUp};
use crate::button::{Button, ButtonEvent, ButtonStartArguments};
use drogue_device::actor::ActorContext;
use drogue_device::interrupt::InterruptContext;
use drogue_device::device::Device;
use drogue_device::address::Address;
use drogue_device::sink::Sink;
use drogue_device::supervisor::Supervisor;

type Ld1Actor = LED<PA5<Output<PushPull>>>;
type ButtonInterrupt<SINK> = Button<Interrupt, PC13<Input<PullUp>>, SINK>;

pub struct MyDevice {
    pub ld1: ActorContext<Ld1Actor>,
    pub button: InterruptContext<ButtonInterrupt<ButtonToLed>>,
}

impl Device for MyDevice {
    fn start(&'static mut self, supervisor: &mut Supervisor) {
        let ld1_addr = self.ld1.start(supervisor);
        self.button.start(ButtonStartArguments {
            sink: ButtonToLed::new(ld1_addr),
        }, supervisor);
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
    fn tell(&self, message: ButtonEvent) {
        match message {
            ButtonEvent::Pressed => {
                self.sink.tell( On )
            }
            ButtonEvent::Released => {
                self.sink.tell( Off )
            }
        }
    }
}

