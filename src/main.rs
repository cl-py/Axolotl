// Implementation for logging system

//these imports create a "compiler shortcut"

// use std::library::function
use std::mem::MaybeUninit; //allows for safe memory allocation without initialization
use std::time::Duration; //Imports time types (seconds, etc)
use std::os::unix::io::AsFd as  _;

//these are for command-line handling
use clap::{ArgAction, Parser, Subcommand};

//these are for trait resolution. It's implementing the interfaces without explicitly calling so 
//it is left as _
use libbpf_rs::{skel::{OpenSkel, Skel, SkelBuilder}};
// use libbpf_rs::MapCore as _;
// use libbpf_rs::MapFlags;
// use libbpf_rs::TcHookBuilder;
// use libbpf_rs::TC_INGRESS;

//allows for conversion of wol1 to the system's index
use nix::net::if_::if_nametoindex;

//how to import the eBPF program
mod logging{
    include!(concat!(env!("OUT_DIR"), "/logging.skel.rs"));
}
mod event;
mod userbuf;

use logging::*;

#[derive(Parser)]
struct Cli{
    #[command(subcommand)]
    cmd: TopLevelCommand,
}

#[derive(Subcommand)]
enum TopLevelCommand{
    Ipfilter{
        #[command(subcommand)]
        cmd: IpFilterCommand
    },
    // NOTE: More programs would go below here, once we add them.
    
}

// NOTE: This version of ipfilter takes one optional ip if passed with add command.
// Using a String type, it can likely not take more than one ip at a time as of right now.
#[derive(Subcommand)]
enum IpFilterCommand{
    Add{
        ip: String
    },

    Del{
        ip: String
    },

    Ls,
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

    //Parsing command-line arguments
    let args = Cli::parse();

    match args.cmd {
        TopLevelCommand::Ipfilter {cmd} => {
            match cmd {
                IpFilterCommand::Add {ip} => {
                    println!("add Command not implemented.");
                }
                IpFilterCommand::Del {ip} => {
                    println!("del Command not implemented.");
                }
                IpFilterCommand::Ls => {
                    println!("ls Command not implemented.");
                }
            }
        }
    }
    
    //Initialize BPF Skeleton -- (eBPFname)SkelBuilder
    let skeleton_builder = LoggingSkelBuilder::default();
    let mut open_object = MaybeUninit::uninit();
    let open_skeleton = skeleton_builder.open(&mut open_object).unwrap(); 
    let mut skeleton = open_skeleton.load().unwrap();
    //assigns attack to something so its not destructed after it executes..was causing a silent crash
    let mut __link_state = skeleton.attach();


/*
    //builds the hook for the traffic control layer
    let mut tc_builder = TcHookBuilder::new(skeleton.progs.tc_ingress.as_fd());
    tc_builder.ifindex(if_nametoindex("wlo1").unwrap() as i32).handle(1).priority(1); //configurations for hook
    let mut ingress = tc_builder.hook(TC_INGRESS); // assigns which direction to attach to
    ingress.create()?; //creates new interface for the hook
    ingress.attach()?; // attaches the hook

*/
    //creates new user ring buffer
    let mut user_ring = libbpf_rs::UserRingBuffer::new(&skeleton.maps.user_ring).unwrap();
    userbuf::pass_arguments(&mut user_ring);

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
