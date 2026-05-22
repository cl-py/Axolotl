// Implementation for logging system

//these imports create a "compiler shortcut"

// use std::library::function
use std::mem::MaybeUninit; //allows for safe memory allocation without initialization
use std::time::Duration; //Imports time types (seconds, etc)

//these are for command-line handling
use clap::{ArgAction, Parser};

//these are for trait resolution. It's implementing the interfaces without explicitly calling so 
//it is left as _
use libbpf_rs::{UserRingBuffer, UserRingBufferSample, libbpf_sys::user_ring_buffer, skel::{OpenSkel, Skel, SkelBuilder}};
use libbpf_rs::MapCore as _;
use libbpf_rs::MapFlags;
use libbpf_rs::TcHookBuilder;
use libbpf_rs::TC_CUSTOM;
use libbpf_rs::TC_EGRESS;
use libbpf_rs::TC_H_CLSACT;
use libbpf_rs::TC_H_MIN_INGRESS;
use libbpf_rs::TC_INGRESS;

//these are used for logging
// use tracing::subscriber::set_global_default as set_global_subscriber;
// use tracing_subscriber::filter::LevelFilter; //used for logging verosity
// use tracing_subscriber::fmt::format::FmtSpan; // used to declare when a logger prints
// use tracing_subscriber::fmt::time::SystemTime; // used to allow for actual system clock and timestamps
// use tracing_subscriber::FmtSubscriber; //ties all the above features together

//how to import the eBPF program
mod filtering{
    include!(concat!(env!("OUT_DIR"), "/filtering.skel.rs"));
}
mod event;

use filtering::*;

#[derive(Parser)]
struct Args{

}

// fn pass_arguments(user_ring: UserRingBuffer){
//     let mut example = user_ring.reserve( 12);
//     // Hello User!
//     match example{
//         Ok(mut sample)=>{
//             sample.as_mut().copy_from_slice(b"Hello User!");
//             sample.commit();
//         }
//         Err(e)=>{
//             println!("Failed");
//         }
//     }
// }

//this sets the return type of main
fn main() -> Result<(), libbpf_rs::Error>{

    let args = Args::parse();

    //Initialize BPF Skeleton -- (eBPFname)SkelBuilder
    let skeleton_builder = FilteringSkelBuilder::default();
    let mut open_object = MaybeUninit::uninit();
    let open_skeleton = skeleton_builder.open(&mut open_object).unwrap(); 
    let mut skeleton = open_skeleton.load().unwrap();

    //assigns attack to something so its not destructed after it executes..was causing a silent crash
    let mut __link_state = skeleton.attach();

    let mut tc_builder = TcHookBuilder::new(skel.progs.tc_ingress.as_fd());
    tc_builder.ifindex(if_nametoindex("wlo1").unwrap()).handle(1).priority(1);

    let mut ingress = tc_builder.hook(TC_INGRESS);
    ingress.create().context("Failed to create TC ingress hook layer")?;
    ingress.attach().context("Failed to attach eBPF program to ingress")?;
    //creates new user ring buffer
    // let mut user_ring = libbpf_rs::UserRingBuffer::new(&skeleton.maps.user_ring).unwrap();
    // pass_arguments(user_ring);

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
