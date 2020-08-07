/// x-code executor.
mod code;

use crate::Decoder;

/// Generates the control block from `tempBlock` (or `TEMP_BLOCK`).
fn generate_control_block(block: &[u8]) -> Vec<u32> {
    let total_len = block.len() >> 2;
    let mut ctrl_block = Vec::with_capacity(total_len);

    ctrl_block.resize_with(total_len, || 0u32);

    for i in 0..total_len {
        let offset = i << 2;
        ctrl_block[i] = u32::from_le_bytes([
            block[offset],
            block[offset + 1],
            block[offset + 2],
            block[offset + 3],
        ]);
    }

    ctrl_block
}

use code::Code;

/// cxdec decoder scheme.
pub struct CxDec<'a> {
    code: Vec<Option<Code<'a>>>,
    scheme: &'a CxDecScheme,
}

pub struct CxDecScheme {
    pub shuffler0: Vec<i32>,
    pub shuffler1: Vec<i32>,
    pub control_block: Vec<u32>,
}

use std::path::Path;

impl CxDecScheme {
    pub fn open<P: AsRef<Path>>(path: P) -> std::io::Result<Self> {
        use miniserde::json;
        use miniserde::Deserialize;

        #[derive(Deserialize)]
        struct CxDecSchemeTemplate {
            #[serde(rename = "cxdec.shuffler0")]
            pub shuffler0: Vec<i32>,
            #[serde(rename = "cxdec.shuffler1")]
            pub shuffler1: Vec<i32>,
            #[serde(rename = "cxdec.blob")]
            pub blob: Vec<u8>,
        }

        let c: CxDecSchemeTemplate =
            json::from_str(&std::fs::read_to_string(path)?).expect("failed to parse json");

        Ok(Self {
            shuffler0: c.shuffler0,
            shuffler1: c.shuffler1,
            control_block: generate_control_block(&c.blob),
        })
    }
}

impl<'a> CxDec<'a> {
    pub fn new(scheme: &'a CxDecScheme) -> CxDec<'a> {
        CxDec {
            code: vec![None; 0x80],
            scheme,
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

        // perform decryption
        if key2 >= offset && (key2 - offset) < buffer.len() as u32 {
            buffer[(key2 - offset) as usize] ^= key4;
        }

        if key1 >= offset && (key1 - offset) < buffer.len() as u32 {
            buffer[(key1 - offset) as usize] ^= key5;
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
            let code = Code::new(index, self.scheme);
            self.code[index as usize] = Some(code);
            self.code[index as usize].as_mut().unwrap()
        };

        (code.execute(value), code.execute(!value))
    }
}

impl<'a> Decoder for CxDec<'a> {
    fn decrypt(&mut self, mut buffer: &mut [u8], key: u32, mut offset: usize) {
        let boundary = ((key & 0x17c) + 0x77) as usize;

        // decode the first chunk of the buffer
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
}
