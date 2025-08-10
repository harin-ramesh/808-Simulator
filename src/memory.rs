use std::fs::File;
use std::io::{self, Read};

const MEMORY_SIZE: usize = 1024 * 1024;
const MEMORY_ACCESS_MASK: u32 = 0xfffff;

pub struct Memory {
    pub bytes: [u8; MEMORY_SIZE],
}

impl Memory {
    pub fn new() -> Self {
        Self {
            bytes: [0; MEMORY_SIZE],
        }
    }

    pub fn read(&self, absolute_address: u32) -> u8 {
        assert!((absolute_address as usize) < self.bytes.len());
        self.bytes[absolute_address as usize]
    }

    pub fn load_from_file(&mut self, filename: &str, at_offset: u32) -> io::Result<u32> {
        if (at_offset as usize) >= self.bytes.len() {
            return Ok(0);
        }

        let mut file = File::open(filename)?;
        let mut buffer = Vec::new();
        let bytes_read = file.read_to_end(&mut buffer)?;
        
        let available_space = self.bytes.len() - (at_offset as usize);
        let bytes_to_copy = bytes_read.min(available_space);
        
        self.bytes[at_offset as usize..at_offset as usize + bytes_to_copy]
            .copy_from_slice(&buffer[..bytes_to_copy]);
            
        Ok(bytes_to_copy as u32)
    }
}

#[derive(Debug, Clone, Copy)]
pub struct SegmentedAccess {
    pub segment_base: u16,
    pub segment_offset: u16,
}

impl Default for SegmentedAccess {
    fn default() -> Self {
        Self {
            segment_base: 0,
            segment_offset: 0,
        }
    }
}

impl SegmentedAccess {
    pub fn get_absolute_address(&self, additional_offset: u16) -> u32 {
        let result = (((self.segment_base as u32) << 4) + 
                     (self.segment_offset + additional_offset) as u32) & MEMORY_ACCESS_MASK;
        result
    }
}

