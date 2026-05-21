// Implementation for logging system

//these imports create a "compiler shortcut"

// use std::library::function
use std::mem::MaybeUninit; //allows for safe memory allocation without initialization
use std::time::Duration; //Imports time types (seconds, etc)

//these are for command-line handling
use clap::{ArgAction, Parser};

//these are for trait resolution. It's implementing the interfaces without explicitly calling so 
//it is left as _
use libbpf_rs::skel::{OpenSkel, SkelBuilder, Skel};

//these are used for logging
// use tracing::subscriber::set_global_default as set_global_subscriber;
// use tracing_subscriber::filter::LevelFilter; //used for logging verosity
// use tracing_subscriber::fmt::format::FmtSpan; // used to declare when a logger prints
// use tracing_subscriber::fmt::time::SystemTime; // used to allow for actual system clock and timestamps
// use tracing_subscriber::FmtSubscriber; //ties all the above features together

//how to import the eBPF program
mod logging{
    include!(concat!(env!("OUT_DIR"), "/logging.skel.rs"));
}
mod event;

use logging::*;

#[derive(Parser)]
struct Args{

}

//this sets the return type of main
fn main() -> Result<(), libbpf_rs::Error>{

    let args = Args::parse();

    //Initialize BPF Skeleton -- (eBPFname)SkelBuilder
    let skeleton_builder = LoggingSkelBuilder::default();
    let mut open_object = MaybeUninit::uninit();
    let open_skeleton = skeleton_builder.open(&mut open_object).unwrap(); 
    let mut skeleton = open_skeleton.load().unwrap();

    //assigns attack to something so its not destructed after it executes..was causing a silent crash
    let mut __link_state = skeleton.attach();

    //Set up ring buffer with event handler
    let mut builder = libbpf_rs::RingBufferBuilder::new();

    //pulls the EventHandler function from event to create the callback
    let event_hander = event::EventHandler::new(); // creates the event handler
    builder.add(&skeleton.maps.events, move |data|{
        event_hander.handle(data);
        0
    }).unwrap(); //passes the handler into the ringbuffer as the callback

    let ringbuf = builder.build().unwrap(); // builds the buffer
    while ringbuf.poll(Duration::MAX).is_ok(){}; //continuously polls ring buffer

    Ok(())
}