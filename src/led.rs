//use drogue_device::component::{Component, ComponentContext, spawn};
use stm32l4xx_hal::hal::digital::v2::OutputPin;
use drogue_device::prelude::*;

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
        self.pin.set_high().ok().unwrap();
    }

    pub fn turn_off(&mut self) {
        self.pin.set_low().ok().unwrap();
    }
}

impl<PIN: OutputPin> Actor for LED<PIN> {

}

impl<PIN: OutputPin> NotificationHandler<On> for LED<PIN> {
    fn on_notification(&'static mut self, message: On) -> Completion {
        log::info!("LED turn on");
        self.turn_on();
        Completion::immediate()
    }
}

impl<PIN: OutputPin> NotificationHandler<Off> for LED<PIN> {
    fn on_notification(&'static mut self, message: Off) -> Completion {
        log::info!("LED turn off");
        Completion::defer( async move {
            self.turn_off();
        })
    }
}
