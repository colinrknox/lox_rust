use std::{
    alloc::{alloc, dealloc, realloc, Layout},
    process,
    ptr::{self, copy_nonoverlapping, null_mut},
};

use super::memory::{grow_capacity, reallocate};

pub trait Chunkable {
    fn init() -> Self;
    fn write(&mut self, byte: u8);
    fn free(&mut self);
}

pub enum OpCode {
    OpReturn,
}

pub struct Chunk {
    code: *mut u8,
    count: usize,
    capacity: usize,
}

impl Chunkable for Chunk {
    fn init() -> Chunk {
        Chunk {
            code: null_mut(),
            count: 0,
            capacity: 0,
        }
    }

    fn write(&mut self, byte: u8) {
        if self.is_at_capacity() {
            self.resize();
        }
        self.code[self.count] = byte;
        self.count += 1;
    }

    fn free(&mut self) {
        *self = Chunk::init();
    }

    fn is_at_capacity(&self) -> bool {
        self.capacity < self.count + 1
    }
}

impl Chunk {
    fn resize(&mut self) {
        let old_capacity = self.capacity;
        self.capacity = grow_capacity(self.capacity);
        self.code = reallocate(self.code, old_capacity, self.capacity);
    }
}

impl Drop for Chunk {
    fn drop(&mut self) {
        if !self.code.is_null() {
            let layout = Layout::array::<u8>(self.capacity).unwrap();
            self.reallocate(code, self.capacity, 0);
        }
    }
}
