use cortex_m::interrupt::Nr;
use stm32l4xx_hal::gpio::ExtiPin;
use stm32l4xx_hal::hal::digital::v2::InputPin;
use drogue_device::prelude::*;

pub enum ButtonEvent {
    Pressed,
    Released,
}

pub struct Button<PIN, SINK: Sink<ButtonEvent>> {
    pin: PIN,
    sink: Option<SINK>,
}


impl<PIN: InputPin + ExtiPin, SINK: Sink<ButtonEvent>> Actor for Button<PIN, SINK> {

}

pub struct SetSink<SINK: Sink<ButtonEvent>>(pub SINK);



impl<PIN: InputPin + ExtiPin, SINK: Sink<ButtonEvent>> Button<PIN, SINK> {
    pub fn new(pin: PIN) -> Self {
        Self {
            pin,
            sink: None,
        }
    }

    pub fn set_sink(&mut self, sink: SINK) {
        self.sink.replace(sink);
    }
}

impl<PIN: InputPin + ExtiPin, SINK: Sink<ButtonEvent>> NotificationHandler<SetSink<SINK>> for Button<PIN, SINK> {
    fn on_notification(&'static mut self, message: SetSink<SINK>) -> Completion {
        self.set_sink(message.0);
        Completion::immediate()
    }
}

impl<PIN: InputPin + ExtiPin, SINK: Sink<ButtonEvent>> Interrupt for Button<PIN, SINK> {
    fn on_interrupt(&mut self) {
        if self.pin.check_interrupt() {
            if let Some(ref sink) = &self.sink {
                if self.pin.is_high().unwrap_or(false) {
                    sink.notify(ButtonEvent::Released)
                } else {
                    sink.notify(ButtonEvent::Pressed)
                }
            }
            self.pin.clear_interrupt_pending_bit();
        }
    }
}
