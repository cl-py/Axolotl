use libbpf_rs::{UserRingBuffer};
use plain::Plain;

#[repr(C)]
pub struct configs{
    pub message: [u8;11]
}

unsafe impl Plain for configs{}

pub fn pass_arguments(user_ring: &mut UserRingBuffer) {
    let mut buffersample = user_ring.reserve(11).unwrap(); //reserve in bytes how much space

    //conversion of the UserRingBufferSample into configs struct
    let bytes = buffersample.as_mut();
    let my_struct = plain::from_mut_bytes::<configs>(bytes).unwrap();
    //setting the values
    my_struct.message = *b"Hello User!";

    //submit and then ignore the result
    let _ = user_ring.submit(buffersample);
}