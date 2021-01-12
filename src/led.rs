use drogue_device::component::{Component, ComponentContext, spawn};
use stm32l4xx_hal::hal::digital::v2::OutputPin;

#[derive(Debug)]
pub enum LEDCommand {
    On,
    Off,
}

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

impl<PIN: OutputPin + 'static> Component for LED<PIN> {
    type InboundMessage = LEDCommand;
    type OutboundMessage = ();

    fn start(&'static mut self, ctx: &'static ComponentContext<Self>) {
        spawn("led", async move {
            loop {
                let message = ctx.receive().await;
                match message {
                    LEDCommand::On => {
                        self.turn_on();
                    }
                    LEDCommand::Off => {
                        self.turn_off();
                    }
                }
            }
        })
        .unwrap();
    }
}
