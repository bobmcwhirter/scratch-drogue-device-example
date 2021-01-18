use cortex_m::interrupt::Nr;
use stm32l4xx_hal::gpio::ExtiPin;
use stm32l4xx_hal::hal::digital::v2::InputPin;
use drogue_device::interrupt::Interrupt;
use drogue_device::sink::Sink;
use drogue_device::handler::TellHandler;

pub enum ButtonEvent {
    Pressed,
    Released,
}

pub struct Button<IRQ, PIN, SINK: Sink<ButtonEvent>> {
    irq: IRQ,
    pin: PIN,
    sink: Option<SINK>,
}

impl<IRQ: Nr, PIN: InputPin + ExtiPin, SINK: Sink<ButtonEvent>> Button<IRQ, PIN, SINK> {
    pub fn new(irq: IRQ, pin: PIN) -> Self {
        Self {
            irq,
            pin,
            sink: None,
        }
    }

    pub fn set_sink(&mut self, sink: SINK) {
        self.sink.replace(sink);
    }
}

pub struct ButtonStartArguments<SINK: Sink<ButtonEvent>> {
    pub sink: SINK,
}

impl<IRQ: Nr, PIN: InputPin + ExtiPin, SINK: Sink<ButtonEvent>> Interrupt for Button<IRQ, PIN, SINK> {
    type StartArguments = ButtonStartArguments<SINK>;

    fn irq(&self) -> u8 {
        self.irq.nr()
    }

    fn start(&mut self, args: Self::StartArguments) {
        self.sink.replace(args.sink);
    }

    fn on_interrupt(&mut self) {
        if self.pin.check_interrupt() {
            if let Some(ref sink) = &self.sink {
                if self.pin.is_high().unwrap_or(false) {
                    sink.tell(ButtonEvent::Pressed)
                } else {
                    sink.tell(ButtonEvent::Released)
                }
            }
            self.pin.clear_interrupt_pending_bit();
        }
    }
}
