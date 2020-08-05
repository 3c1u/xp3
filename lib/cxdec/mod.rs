use lazy_static::*;

// NOTE: required for decoding.
// Copy tempBlock from xp3filter.tjs to table.rs as `TEMP_BLOCK: &[u8]` to make this work.
// Alternatively, you can use the control block in u32 format from the decryption scheme (and replace the CTRL_BLOCK below).
mod table;

/// x-code executor.
mod code;

lazy_static! {
    pub(super) static ref CTRL_BLOCK: Vec<u32> = generate_control_block();
}

/// Generates the control block from `tempBlock` (or `TEMP_BLOCK`).
fn generate_control_block() -> Vec<u32> {
    use table::TEMP_BLOCK;

    let total_len = TEMP_BLOCK.len() >> 2;
    let mut ctrl_block = Vec::with_capacity(total_len);

    ctrl_block.resize_with(total_len, || 0u32);

    for i in 0..total_len {
        let offset = i << 2;
        ctrl_block[i] = u32::from_le_bytes([
            TEMP_BLOCK[offset],
            TEMP_BLOCK[offset + 1],
            TEMP_BLOCK[offset + 2],
            TEMP_BLOCK[offset + 3],
        ]);
    }

    ctrl_block
}

use code::Code;

/// cxdec decoder scheme.
pub struct CxDec {
    code: Vec<Option<Code>>,
}

impl CxDec {
    pub fn new() -> CxDec {
        CxDec {
            code: vec![None; 0x80],
        }
    }

    /// Decrypts the buffer with a given key. In most cases, the key is the hash of the entry.
    pub fn decrypt(&mut self, mut buffer: &mut [u8], key: u32, mut offset: usize) {
        let boundary = ((key & 0x17c) + 0x77) as usize;

        if offset < boundary {
            let dec_len = buffer.len().min(boundary - offset);

            self.decrypt_inner(&mut buffer[..dec_len], key, offset);

            // skip decoded section
            offset += dec_len; // TODO: this is somehow used in decrypt_inner
            buffer = &mut buffer[dec_len..];
        }

        if buffer.len() != 0 {
            // rotate key and decode the rest
            self.decrypt_inner(&mut buffer, (key >> 16) ^ key, offset);
        }
    }

    fn decrypt_inner(&mut self, buffer: &mut [u8], key: u32, offset: usize) {
        // convert offset to u32
        let offset = offset as u32;

        // execute the code and obtain keys
        let (v0, v1) = self.execute(key);

        let key2 = v1 & 0xffff;
        let key3 = v0 & 0xff;

        let key1 = v1 >> 16;
        let key2 = if key1 == key2 { key2 + 1 } else { key2 };
        let key3 = if key3 == 0 { 1 } else { key3 } as u8;
        let key4 = ((v0 >> 16) & 0xff) as u8;
        let key5 = ((v0 >> 8) & 0xff) as u8;

        if key2 >= offset && key2 < offset + buffer.len() as u32 {
            buffer[(key2 - offset) as usize] ^= key4;
        }

        if key1 >= offset && key1 < offset + buffer.len() as u32 {
            buffer[(key2 - offset) as usize] ^= key5;
        }

        for v in buffer {
            *v ^= key3;
        }
    }

    fn execute(&mut self, key: u32) -> (u32, u32) {
        let index = key & 0x7f;
        let value = key >> 7;

        let code = if let Some(code) = &mut self.code[index as usize] {
            code
        } else {
            let code = Code::new(index);
            self.code[index as usize] = Some(code);
            self.code[index as usize].as_mut().unwrap()
        };

        (code.execute(value), code.execute(!value))
    }
}
