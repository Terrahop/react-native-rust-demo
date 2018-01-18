extern crate env_logger;
extern crate mdns;
extern crate mdns_responder;
extern crate dns_parser;

use self::mdns::{Record, RecordKind};
use self::mdns_responder::Responder;
use std::net::IpAddr;

const SERVICE_NAME: &'static str = "_echo._tcp.local";

pub fn start_mdns() {
    env_logger::init().unwrap();

    let responder = Responder::new().unwrap();
    let _svc = responder.register(
        SERVICE_NAME.to_owned(),
        "Echo P2P".to_owned(),
        12345,
        &["path=/"]);

    loop {
        for response in mdns::discover::all(SERVICE_NAME).expect("Failed to discover") {
            let response = match response {
                Ok(response) => response,
                Err(_) => {
                    println!("Failed to parse response");
                    continue;
                }
            };

            let addr = response.records()
                .filter_map(self::to_ip_addr)
                .next();

            if let Some(addr) = addr {
                println!("Found device at {}", addr);
            } else {
                println!("Device not advertising address");
            }
        }

        ::std::thread::sleep(::std::time::Duration::from_secs(10));
    }
}

fn to_ip_addr(record: &Record) -> Option<IpAddr> {
    println!("Record {:?}", record);
    match record.kind {
        RecordKind::A(addr) => Some(addr.into()),
        RecordKind::AAAA(addr) => Some(addr.into()),
         _ => None,
    }
}
