use crate::clox::chunk::OpCode;

use super::chunk::Chunk;

pub fn disassemble_chunk(chunk: &Chunk, name: &str) {
    println!("== {} ==", name);

    let mut offset = 0;
    while offset < chunk.count {
        offset = disassemble_instruction(chunk, offset);
    }
}

pub fn disassemble_instruction(chunk: &Chunk, offset: usize) -> usize {
    println!("{:04}", offset);

    unsafe {
        let instruction = chunk.code.add(offset) as u8;
        match OpCode::try_from(instruction).unwrap() {
            OpCode::OpReturn => simple_instruction("OP_RETURN", offset),
            OpCode::Unknown => {
                println!("Unknown opcode {}", instruction);
                offset + 1
            }
        }
    }
}

fn simple_instruction(name: &str, offset: usize) -> usize {
    println!("{}", name);
    offset + 1
}

#[cfg(test)]
mod test {
    use crate::clox::chunk::{Chunk, Chunkable, OpCode};

    use super::disassemble_chunk;

    #[test]
    fn test_disassemble_chunk() {
        let mut chunk = Chunk::init();
        chunk.write(OpCode::OpReturn.into());
        disassemble_chunk(&chunk, "test chunk");
        chunk.free();
    }
}
