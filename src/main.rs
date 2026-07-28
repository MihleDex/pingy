use std::net::{ ToSocketAddrs,SocketAddr};
use std::io::{self, Error};
use std::vec::IntoIter;


fn main() {
    // Takes in a url in the form of domain.zone eg. google.com
     match make_dns_request("google.com") {
        Ok(urls) => {
            for addr in urls {
                println!("{}", addr);
            }
        }
        Err(e) => eprintln!("Error resolving DNS: {}", e),
    }
}


fn make_dns_request (url_str : &str) -> io::Result<IntoIter<SocketAddr>>
{
    let url_str: String = format!("{}:80",url_str);
    //stores result from the to_socket_addrs function
    let q_result: Result<IntoIter<SocketAddr>,Error>= url_str.to_socket_addrs();

    let q_result:IntoIter<SocketAddr> = match q_result {
        Ok(iter) => iter,
        Err(error) => return Err(error),
    };

    let addrs =q_result;
    return Ok(addrs);
}