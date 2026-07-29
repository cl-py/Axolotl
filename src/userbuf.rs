use libbpf_rs::{UserRingBuffer};
use plain::Plain;

#[repr(C)]
pub struct configs{
    pub ip: u32,
}

unsafe impl Plain for configs{}

pub fn pass_arguments(user_ring: &mut UserRingBuffer, ip:u32, ) {
    let mut buffersample = user_ring.reserve(std::mem::size_of::<configs>()); //reserve in bytes how much space

    //conversion of the UserRingBufferSample into configs struct
    let bytes = buffersample.as_mut();
    let my_struct = plain::from_mut_bytes::<configs>(bytes).unwrap();
    //setting the values
    my_struct.ip = ip;

    //submit and then ignore the result
    let _ = user_ring.submit(buffersample);
}
