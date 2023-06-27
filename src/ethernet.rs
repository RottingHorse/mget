use smoltcp::wire;

pub struct MacAddress([u8; 6]);

impl MacAddress {
    pub fn new() -> MacAddress {
        unimplemented!()
    }
}

impl Into<wire::EthernetAddress> for MacAddress {
    fn into(self) -> wire::EthernetAddress {
        unimplemented!()
    }
}