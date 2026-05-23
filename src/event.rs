// use blazesym::symbolize; //takes raw emory addresses and converts them to names
// use nix::sys::sysinfo; // Rust bindings to Unix system Calls
use std::mem; // for reading memory from the ringbuffer
// use std::time::{SystemTime, UNIX_EPOCH}; // time stamps
// use std::str;

#[repr(C)]
pub struct PacketEvent{
    pub src_ip: u32,
    pub dest_ip: u32,
    pub timestamp: u64,
    pub protocol: String
}

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

        //this is checking that the recieved data is the same as the packetevent struct should be
        // if data.len() != mem::size_of::<PacketEvent>() {
        //     eprintln!(
        //         "Invalid size {} != {}",
        //         data.len(),
        //         mem::size_of::<PacketEvent>()
        //     );
        //     // return 1;
        // }

        // //the actual conversion
        // let event = unsafe { &*(data.as_ptr() as *const PacketEvent) };
    }
}