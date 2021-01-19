use drogue_device::actor::Actor;
use drogue_device::handler::{NotificationHandler, Completion};
use crate::spi::{SpiBus, LeaseRequest, LeaseRelinquish};
use drogue_device::address::Address;
use embedded_hal::blocking::spi::Transfer;
use core::marker::PhantomData;
use drogue_device::mutex::{Mutex, Lock};

pub struct EsWifi<Spi: Transfer<u8>> {
    spi: Address<Mutex<Spi>>,
}

impl<Spi: Transfer<u8>> Actor for EsWifi<Spi> {

}

struct Initialize;

impl<Spi: Transfer<u8>> NotificationHandler<Initialize> for EsWifi<Spi> {
    fn on_notification(&'static mut self, message: Initialize) -> Completion {
        Completion::defer( async move {
            let mut spi = self.spi.lock().await;
            let mut buf = [0; 12];
            spi.transfer( &mut buf );
        })
    }
}