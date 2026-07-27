use std::net::{Ipv4Addr, Ipv6Addr};

/// This function takes an IP, then returns the network btye order of the IP for 
/// use in the PacketEvent struct.
pub fn parseip(ip: &str)-> Result<u32, std::net::AddrParseError>{
    let addr: Ipv4Addr = ip.parse()?;
    Ok(u32::from(addr).to_be())
}
