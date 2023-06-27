use std::net::IpAddr;
use smoltcp::phy::TapInterface;
use smoltcp::wire::EthernetAddress;
use url::Url;

#[derive(Debug)]
pub enum UpstreamError {
    Network(smoltcp::Error),
    InvalidUrl,
    Content(std::str::Utf8Error),
}

pub fn get(tap: TapInterface, mac: EthernetAddress, addr: IpAddr, url: Url) -> Result<(), UpstreamError> {
    unimplemented!()
}