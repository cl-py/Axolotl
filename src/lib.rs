/**
 * @file lib.rs
 * 
 * @brief Utilities for Axolotl userspace
 * 
 * @details This function is responsible for holding functions that userspace may
 *          reference throughout its execution.
 */

use std::net::{Ipv4Addr, Ipv6Addr};

/**==========================================================================================*
 * 									    Axolotl Code	   								     *
 *===========================================================================================*/

/**
 * @brief   Get the formatted IP address
 * 
 * @param ip as a string to format
 * @return  Parsed Address in proper network byte order
 * 
 * @details  This functions takes an IP, then returns the network byte order
 *           of the IP for use in the PacketEvent struct.
 */
pub fn parseip(ip: &str)-> Result<u32, std::net::AddrParseError>{
    let addr: Ipv4Addr = ip.parse()?;
    Ok(u32::from(addr).to_be())
}
