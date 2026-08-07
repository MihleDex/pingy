use core::time;
use std::io::{self, Error, ErrorKind};
use std::net::{SocketAddr, ToSocketAddrs};
use std::env;
use raw_socket::{Domain, Protocol, RawSocket, Type};
use chrono::{DateTime, Local};

fn main() {
    // Takes in a url in the form of domain.zone eg. google.com
    let args: Vec<String> = env::args().collect();
    let  identifier:[u8;2] = [0x00,0x01];
    let mut sequence_n: [u8;2] = [0x00,0x01];
    let payload: Vec<u8> = build_payload(Local::now());
    let packet: IcmpPacket = build_packet(8, 0, p_checksum, [0x00,0x01], [0x00,0x01], payload);
    let input_url = &args[1];
    

   let addr = match make_dns_request(input_url) {
        Ok(addr) => addr,
        Err(error) => {
            eprintln!("Error resolving DNS: {}", error);
            return; // Stop execution if resolution fails
        },
    };

}
    

    


//Function to build the packet
fn build_packet (p_type:u8,p_code:u8,p_checksum:u16,p_identifier:[u8;2],p_sequence_n:[u8;2],payload: Vec<u8>) -> IcmpPacket{
    IcmpPacket { p_type, p_code, p_checksum, p_identifier, p_sequence_n, payload }
}

// Function to make the ping request
fn _make_echo_req(packet:IcmpPacket,target:SocketAddr) -> Result<SocketAddr,Error> {
     //Initialise raw socket
    let socket = RawSocket::new(Domain::ipv4(), Type::raw(), Some(Protocol::icmpv4())).expect("Failed creating socket");
    let mut buffer = [0u8; 1024];
    let sent_bytes = socket.send_to(packet, target)?;
    let (bytes_received, src_addr) = socket.recv_from(&mut buffer)?;
   
}

//Function to Calculate Checksum
fn calc_checksum(){
    todo!();
}

//Function to build payload

fn build_payload(current_time:DateTime<Local>) -> Vec<u8> { 
    let timestamp: i64 = current_time.timestamp();
    let payload_bytes: Vec<u8> = timestamp.to_be_bytes().to_vec();
    println!("Timestamp {:?}",timestamp);
    println!("Timestamp binary: {:#010b}",timestamp);
    return payload_bytes;
}

//The ICMP Packet structure
struct IcmpPacket {
    p_type: u8,
    p_code: u8,
    p_checksum: u16,
    p_identifier: [u8;2],
    p_sequence_n:[u8;2],
    payload: Vec<u8>,
}

//Fnction to resolve hostname to IP
fn make_dns_request(url_str: &str) -> io::Result<SocketAddr> {
    let url_str: String = format!("{}:80", url_str);
    //stores result from the to_socket_addrs function
    let mut q_result= url_str.to_socket_addrs()?;

    let addr: SocketAddr = q_result
    .next()
    .ok_or_else(|| Error::new(ErrorKind::NotFound, "No addresses resolved"))?;
    return Ok(addr);
}
