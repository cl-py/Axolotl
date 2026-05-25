use std::net::{Ipv4Addr, Ipv6Addr};

/// This function takes an IP, determines if the IP is IPv4, IPv6,
/// or neither. Then returns the network btye order of the IP for 
/// use in the PacketEvent struct.
pub fn parseip(ip: String){
    if ip.len() == 15 {
        // IPv4
        let tokens: Vec<&str> = ip.split('.').collect();
        println!("ipv4 detected");
    } else if ip.len() == 39 {
        // IPv6
    } else {
        println!("Invalid IP. Must be either IPv6 or IPv4.");
    }
}
