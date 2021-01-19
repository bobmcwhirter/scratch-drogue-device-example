use embedded_hal::blocking::spi::Transfer;
use drogue_device::handler::{RequestHandler, Response, NotificationHandler, Completion};
use drogue_device::actor::Actor;
use drogue_device::address::Address;

pub trait SpiBus<SPI: Transfer<u8>>:
    where Self: Actor
    + RequestHandler<LeaseRequest, Response=Lease<SPI>>
    + NotificationHandler<LeaseRelinquish<SPI>>
{}

pub struct Spi<SPI: Transfer<u8>> {
    address: Option<Address<Self>>,
    spi: Rendezvous<SPI>,
}

impl<SPI: Transfer<u8>> Spi<SPI> {
    pub fn new(spi: SPI) -> Self {
        Self {
            address: None,
            spi: Rendezvous::new(spi),
        }
    }
}

impl<SPI: Transfer<u8>> Actor for Spi<SPI> {
    fn start(&mut self, address: Address<Self>) where Self: Sized {
        self.address.replace( address );
    }
}

impl<SPI: Transfer<u8> + 'static> SpiBus<SPI> for Spi<SPI> {}

pub struct Lease<SPI: Transfer<u8>> {
    pub spi: SPI,
}

pub struct LeaseRequest;

pub struct LeaseRelinquish<SPI: Transfer<u8>>(pub Lease<SPI>);

impl<SPI: Transfer<u8> + 'static> RequestHandler<LeaseRequest> for Spi<SPI> {
    type Response = Lease<SPI>;

    fn on_request(&'static mut self, message: LeaseRequest) -> Response<Self::Response> {
        Response::defer(async move {
            Lease {
                spi: self.spi.take().await
            }
        })
    }
}

impl<SPI: Transfer<u8>> NotificationHandler<LeaseRelinquish<SPI>> for Spi<SPI> {
    fn on_notification(&'static mut self, message: LeaseRelinquish<SPI>) -> Completion {
        self.spi.release(message.0.spi);
        Completion::immediate()
    }
}
