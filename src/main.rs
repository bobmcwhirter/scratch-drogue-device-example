#![no_std]
#![no_main]

mod button;
mod led;
mod device;
//mod spi;
//mod eswifi;

use cortex_m_rt::{entry, exception};
use stm32l4xx_hal::{prelude::*, rcc::RccExt, stm32::Peripherals};

use log::LevelFilter;
use panic_rtt_target as _;
use rtt_logger::RTTLogger;
use rtt_target::rtt_init_print;

use stm32l4xx_hal::gpio::{Edge, Input, Output, PullUp, PushPull, PA5, PC13};

use button::Button;
use led::LED;
use device::MyDevice;

use stm32l4xx_hal::pac::Interrupt::EXTI15_10;
use drogue_device::prelude::*;

static LOGGER: RTTLogger = RTTLogger::new(LevelFilter::Debug);

#[entry]
fn main() -> ! {
    rtt_init_print!();
    log::set_logger(&LOGGER).unwrap();
    log::set_max_level(log::LevelFilter::Debug);
    log::info!("Init");

    let mut device = Peripherals::take().unwrap();

    log::info!("initializing");
    let mut flash = device.FLASH.constrain();
    let mut rcc = device.RCC.constrain();
    let mut pwr = device.PWR.constrain(&mut rcc.apb1r1);
    let _clocks = rcc
        .cfgr
        .sysclk(80.mhz())
        .pclk1(80.mhz())
        .pclk2(80.mhz())
        .freeze(&mut flash.acr, &mut pwr);

    let mut gpioa = device.GPIOA.split(&mut rcc.ahb2);
    let ld1 = gpioa
        .pa5
        .into_push_pull_output(&mut gpioa.moder, &mut gpioa.otyper);

    let ld1 = LED::new(ld1);

    let mut gpiob = device.GPIOB.split(&mut rcc.ahb2);
    let ld2 = gpiob
        .pb14
        .into_push_pull_output(&mut gpiob.moder, &mut gpiob.otyper);

    let mut gpioc = device.GPIOC.split(&mut rcc.ahb2);
    let mut button = gpioc
        .pc13
        .into_pull_up_input(&mut gpioc.moder, &mut gpioc.pupdr);
    button.make_interrupt_source(&mut device.SYSCFG, &mut rcc.apb2);
    button.enable_interrupt(&mut device.EXTI);
    button.trigger_on_edge(&mut device.EXTI, Edge::RISING_FALLING);

    let button = Button::new( button);

    let device = MyDevice {
        ld1: ActorContext::new(ld1),
        button: InterruptContext::new(button, EXTI15_10),
    };

    device!( MyDevice = device; 1024 );
}

