use dns_message_parser::rr::{A, RR};
use dns_message_parser::{Dns, DomainName, Flags, Opcode, RCode};
use pnet::datalink::{self};
use std::convert::TryFrom;
use std::net::Ipv4Addr;

pub static SERVER_NAME: &'static str = "_drive._tcp.local.";

pub fn get_ipaddr() -> Ipv4Addr {
    let all_interfaces = datalink::interfaces();
    let default_interface = all_interfaces
        .iter()
        .find(|e| e.is_up() && !e.is_loopback() && !e.ips.is_empty())
        .unwrap();
    let ipv4_interface = default_interface
        .ips
        .first()
        .expect("No interface available");
    let ipv4 = ipv4_interface.ip();

    ipv4.to_string().parse().unwrap()
}

pub fn response_packet() -> Vec<u8> {
    // TODO: Dinamic id
    let id = 56092;
    let flags = Flags {
        qr: true,
        opcode: Opcode::Query,
        aa: true,
        tc: false,
        rd: true,
        ra: true,
        ad: false,
        cd: false,
        rcode: RCode::NoError,
    };

    let answers = {
        let domain_name = DomainName::try_from(SERVER_NAME).unwrap();
        let ttl = 3600;
        let ipv4_addr = get_ipaddr();
        let a = A {
            domain_name,
            ttl,
            ipv4_addr,
        };
        vec![RR::A(a)]
    };

    let dns = Dns {
        id,
        flags,
        questions: Vec::new(),
        answers,
        authorities: Vec::new(),
        additionals: Vec::new(),
    };

    let bytes = dns.encode().unwrap();
    let as_arr: Vec<u8> = bytes.to_vec();

    as_arr
}
