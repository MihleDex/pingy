use std::io::{self, Error};
use std::net::{SocketAddr, ToSocketAddrs};
use std::vec::IntoIter;
use std::env;

fn main() {
    // Takes in a url in the form of domain.zone eg. google.com
    let args: Vec<String> = env::args().collect();

    let input_url = &args[1];
    match make_dns_request(input_url) {
        Ok(urls) => {
            for addr in urls {
                println!("{}", addr);
            }
        }
        Err(e) => eprintln!("Error resolving DNS: {}", e),
    }
}

struct IcmpPacket {
    p_type: u8,
    p_code: u8,
    p_checksum: u16,
    payload:String,
}

fn build_packet(p_type: u8, p_code:u8, p_checksum:u16,payload: String) -> IcmpPacket {
    IcmpPacket { p_type, p_code, p_checksum, payload }
}

fn make_dns_request(url_str: &str) -> io::Result<IntoIter<SocketAddr>> {
    let url_str: String = format!("{}:80", url_str);
    //stores result from the to_socket_addrs function
    let q_result: Result<IntoIter<SocketAddr>, Error> = url_str.to_socket_addrs();

    let q_result: IntoIter<SocketAddr> = match q_result {
        Ok(iter) => iter,
        Err(error) => return Err(error),
    };

    let addrs = q_result;
    return Ok(addrs);
}
