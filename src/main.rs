use std::net::{IpAddr, Ipv4Addr, Ipv6Addr};
use std::io;


fn main() {
    let message : &str = "Enter url eg. https://google.com";
    println!("{}",message);
    url_to_addr("https://google.com",message);

}

fn url_to_addr (url : &str, message : &str) -> Ipv4Addr {

let addr = Ipv4Addr::UNSPECIFIED;

if url.starts_with("https://") == false {
    println!("Error {} ", message);
}

else  if url.contains(".") == false {
    println!("Error {} ", message);
}

else {
    println!("Good boy");
}
return addr;
}
