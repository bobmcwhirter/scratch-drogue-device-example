use cortex_m::interrupt::Nr;
use stm32l4xx_hal::gpio::ExtiPin;
use stm32l4xx_hal::hal::digital::v2::InputPin;
use drogue_device::prelude::*;
use crate::device::ButtonToLed;

#[derive(Copy, Clone)]
pub enum ButtonEvent {
    Pressed,
    Released,
}

pub struct Button<PIN> {
    pin: PIN,
}

impl<PIN: InputPin + ExtiPin> Actor for Button<PIN> {
    type Event = ButtonEvent;
}

impl<PIN: InputPin + ExtiPin> NotificationHandler<Lifecycle> for Button<PIN> {
    fn on_notification(&'static mut self, message: Lifecycle) -> Completion {
        Completion::immediate()
    }
}

impl<PIN: InputPin + ExtiPin> Button<PIN> {
    pub fn new(pin: PIN) -> Self {
        Self {
            pin,
        }
    }
}


impl<PIN: InputPin + ExtiPin> Interrupt for Button<PIN> {
    fn on_interrupt(&mut self) {
        if self.pin.check_interrupt() {
            log::info!("button pressed");
            self.pin.clear_interrupt_pending_bit();
        }
    }
}
