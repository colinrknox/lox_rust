use std::{
    alloc::{realloc, Layout},
    process,
    ptr::null_mut,
};

pub fn grow_capacity(capacity: usize) -> usize {
    if capacity < 8 {
        8
    } else {
        capacity * 2
    }
}
pub fn reallocate(code: *mut u8, old_size: usize, new_size: usize) -> *mut u8 {
    let layout = Layout::array::<u8>(old_size).unwrap();
    if new_size == 0 {
        unsafe { std::alloc::dealloc(code, layout) }
        return null_mut();
    }

    let ptr;
    unsafe {
        ptr = realloc(code, layout, new_size);
    }
    if ptr == null_mut() {
        println!("Failed to realloc space");
        process::exit(1);
    }
    return ptr;
}
