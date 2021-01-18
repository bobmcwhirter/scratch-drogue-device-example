//use drogue_device::component::{Component, ComponentContext, spawn};
use stm32l4xx_hal::hal::digital::v2::OutputPin;
use drogue_device::actor::Actor;
use drogue_device::handler::{AskHandler, Response, TellHandler, Completion};
use stm32l4xx_hal::pac::Interrupt::COMP;
use stm32l4xx_hal::pac::i2c1::isr::TC_A::COMPLETE;

pub struct On;
pub struct Off;

pub struct LED<PIN: OutputPin> {
    pin: PIN,
}

impl<PIN: OutputPin> LED<PIN> {
    pub fn new(pin: PIN) -> Self {
        Self { pin }
    }

    pub fn turn_on(&mut self) {
        self.pin.set_low().ok().unwrap();
    }

    pub fn turn_off(&mut self) {
        self.pin.set_high().ok().unwrap();
    }
}

impl<PIN: OutputPin> Actor for LED<PIN> {

}

impl<PIN: OutputPin> TellHandler<On> for LED<PIN> {

    fn on_message(&'static mut self, message: On) -> Completion {
        self.turn_on();
        Completion::immediate()
    }
}

impl<PIN: OutputPin> TellHandler<Off> for LED<PIN> {

    fn on_message(&'static mut self, message: Off) -> Completion {
        Completion::defer( async move {
            self.turn_off();
        })
    }
}
