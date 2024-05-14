use std::ptr::null_mut;

use super::memory::{grow_capacity, reallocate};

pub trait Chunkable {
    fn init() -> Self;
    fn write(&mut self, byte: u8);
    fn free(&mut self);
}

pub enum OpCode {
    OpReturn,
    Unknown,
}

impl std::convert::From<u8> for OpCode {
    fn from(value: u8) -> Self {
        match value {
            0 => OpCode::OpReturn,
            _ => OpCode::Unknown,
        }
    }
}

impl std::convert::Into<u8> for OpCode {
    fn into(self) -> u8 {
        match self {
            OpCode::OpReturn => 0,
            OpCode::Unknown => 100,
        }
    }
}

pub struct Chunk {
    pub code: *mut u8,
    pub count: usize,
    pub capacity: usize,
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
        unsafe {
            *self.code.add(self.count) = byte;
        }
        self.count += 1;
    }

    fn free(&mut self) {
        *self = Chunk::init();
    }
}

impl Chunk {
    fn is_at_capacity(&self) -> bool {
        self.capacity < self.count + 1
    }

    fn resize(&mut self) {
        let old_capacity = self.capacity;
        self.capacity = grow_capacity(self.capacity);
        self.code = reallocate(self.code, old_capacity, self.capacity);
    }
}

impl Drop for Chunk {
    fn drop(&mut self) {
        if !self.code.is_null() {
            reallocate(self.code, self.capacity, 0);
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_chunk_init() {
        let chunk = Chunk::init();
        assert_eq!(chunk.code, null_mut());
    }

    #[test]
    fn test_chunk_write_value() {
        let chunk: Chunk = init_chunk();
        unsafe {
            assert_eq!(*chunk.code.add(0), 5);
        }
    }

    #[test]
    fn test_chunk_write_count() {
        let chunk = init_chunk();
        assert_eq!(chunk.count, 1);
    }

    #[test]
    fn test_chunk_write_capacity() {
        let chunk = init_chunk();
        assert_eq!(chunk.capacity, 8);
    }

    #[test]
    fn test_chunk_free() {
        let mut chunk = init_chunk();
        chunk.free();
        assert_eq!(chunk.code, null_mut());
    }

    fn init_chunk() -> Chunk {
        let mut chunk = Chunk::init();
        chunk.write(5);
        return chunk;
    }
}
