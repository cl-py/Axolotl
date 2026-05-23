// use blazesym::symbolize; //takes raw emory addresses and converts them to names
// use nix::sys::sysinfo; // Rust bindings to Unix system Calls
// use std::mem; // for reading memory from the ringbuffer
// use std::time::{SystemTime, UNIX_EPOCH}; // time stamps
use std::str;
pub struct EventHandler{

}

impl EventHandler{
    //returns itself
    pub fn new() -> Self{
        Self{}
    }
    //returns nothing
    // callback function for kernel ringbuffer
    pub fn handle(&self, data: &[u8]){
        println!("Data: {:?}", data);
    }
}