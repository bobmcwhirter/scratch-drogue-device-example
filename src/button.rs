use cortex_m::interrupt::Nr;
use drogue_device::interrupt::{Interrupt, InterruptContext};
use stm32l4xx_hal::gpio::ExtiPin;
use stm32l4xx_hal::hal::digital::v2::InputPin;

#[derive(Debug)]
pub enum ButtonState {
    Pressed,
    Released,
}

pub struct Button<IRQ, PIN> {
    irq: IRQ,
    pin: PIN,
}

impl<IRQ: Nr, PIN: InputPin + ExtiPin> Button<IRQ, PIN> {
    pub fn new(irq: IRQ, pin: PIN) -> Self {
        Self { irq, pin }
    }
}

impl<IRQ: Nr, PIN: InputPin + ExtiPin> Interrupt for Button<IRQ, PIN> {
    type OutboundMessage = ButtonState;

    fn irq(&self) -> u8 {
        self.irq.nr()
    }

    fn on_interrupt(&mut self, context: &InterruptContext<Self>) {
        if self.pin.check_interrupt() {
            if self.pin.is_high().unwrap_or(false) {
                context.send(ButtonState::Released)
            } else {
                context.send(ButtonState::Pressed)
            }
            self.pin.clear_interrupt_pending_bit();
        }
    }
}
