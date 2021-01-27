#![no_std]
#![no_main]

mod button;
//mod led;
mod device;
//mod hts221;

use cortex_m_rt::{entry, exception};
use stm32l4xx_hal::{prelude::*, rcc::RccExt, stm32::Peripherals};

use log::LevelFilter;
use panic_rtt_target as _;
use rtt_logger::RTTLogger;
use rtt_target::rtt_init_print;

use stm32l4xx_hal::gpio::{Edge, Input, Output, PullUp, PushPull, PA5, PC13};

use button::Button;
use device::MyDevice;

use stm32l4xx_hal::pac::Interrupt::{EXTI15_10, TIM15};
use drogue_device::{
    prelude::*,
    synchronization::Mutex,
    driver::{
        sensor::hts221::Hts221,
        led::SimpleLED,
    },
};
use stm32l4xx_hal::pac::{I2C2, RCC};
use stm32l4xx_hal::i2c::I2c;
use stm32l4xx_hal::time::Hertz;
use cortex_m::peripheral::NVIC;
use drogue_device::driver::timer::{
    Timer,
    stm32l4xx::Timer as McuTimer,
};
use drogue_device::driver::led::Blinker;
use drogue_device::hal::Active;
use drogue_device::domain::time::duration::Milliseconds;

static LOGGER: RTTLogger = RTTLogger::new(LevelFilter::Debug);

#[entry]
fn main() -> ! {
    rtt_init_print!(BlockIfFull);
    log::set_logger(&LOGGER).unwrap();
    log::set_max_level(log::LevelFilter::Debug);
    log::info!("Init");

    let mut device = Peripherals::take().unwrap();

    log::info!("initializing");
    let mut flash = device.FLASH.constrain();
    let mut rcc = device.RCC.constrain();
    let mut pwr = device.PWR.constrain(&mut rcc.apb1r1);
    let clocks = rcc
        .cfgr
        .sysclk(80.mhz())
        .pclk1(80.mhz())
        .pclk2(80.mhz())
        .freeze(&mut flash.acr, &mut pwr);

    let mut gpioa = device.GPIOA.split(&mut rcc.ahb2);
    let mut gpiob = device.GPIOB.split(&mut rcc.ahb2);
    let mut gpioc = device.GPIOC.split(&mut rcc.ahb2);
    let mut gpiod = device.GPIOD.split(&mut rcc.ahb2);

    // == LEDs ==

    let ld1 = gpioa
        .pa5
        .into_push_pull_output(&mut gpioa.moder, &mut gpioa.otyper);

    let ld1 = SimpleLED::new(ld1, Active::High);

    let ld2 = gpiob
        .pb14
        .into_push_pull_output(&mut gpiob.moder, &mut gpiob.otyper);

    let ld2 = SimpleLED::new(ld2, Active::High);

    // == Blinker ==

    let blinker1 = Blinker::new(Milliseconds(500u32));
    let blinker2 = Blinker::new(Milliseconds(1000u32));

    // == Button ==

    let mut button = gpioc
        .pc13
        .into_pull_up_input(&mut gpioc.moder, &mut gpioc.pupdr);

    button.make_interrupt_source(&mut device.SYSCFG, &mut rcc.apb2);
    button.enable_interrupt(&mut device.EXTI);
    button.trigger_on_edge(&mut device.EXTI, Edge::RISING_FALLING);

    let button = Button::new(button);

    // == i2c

    let scl = gpiob
        .pb10
        .into_open_drain_output(&mut gpiob.moder, &mut gpiob.otyper)
        .into_af4(&mut gpiob.moder, &mut gpiob.afrh);

    let sda = gpiob.pb11
        .into_open_drain_output(&mut gpiob.moder, &mut gpiob.otyper)
        .into_af4(&mut gpiob.moder, &mut gpiob.afrh);

    let i2c = I2c::i2c2(device.I2C2, (scl, sda), Hertz(100_000u32), clocks, &mut rcc.apb1r1);

    let i2c = Mutex::new(i2c);

    // == HTS221 ==

    let mut ready = gpiod.pd15.into_pull_down_input(&mut gpiod.moder, &mut gpiod.pupdr);
    //let mut ready = gpiod.pd15;
    ready.enable_interrupt(&mut device.EXTI);
    ready.make_interrupt_source(&mut device.SYSCFG, &mut rcc.apb2);
    ready.trigger_on_edge(&mut device.EXTI, Edge::RISING);

    let hts221 = Hts221::new(ready, EXTI15_10);

    // == Timer ==

    let mcu_timer = McuTimer::tim15( device.TIM15, clocks, &mut rcc.apb2);
    let timer = Timer::new(mcu_timer );



    // == Device ==

    let device = MyDevice {
        ld1: ActorContext::new(ld1).with_name("ld1"),
        ld2: ActorContext::new(ld2).with_name("ld2"),
        blinker1: ActorContext::new(blinker1) .with_name("blinker1"),
        blinker2: ActorContext::new(blinker2) .with_name("blinker2"),

        i2c: ActorContext::new(i2c).with_name("i2c"),
        //hts221: ActorContext::new(hts221).with_name("hts221" ),
        hts221,
        button: InterruptContext::new(button, EXTI15_10).with_name("button"),
        timer: InterruptContext::new( timer, TIM15 ).with_name( "timer"),
    };

    device!( MyDevice = device; 1024 );
}

