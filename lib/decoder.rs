/// Decoder trait.
pub trait Decoder {
    /// Decrypts the buffer with a given key. In most cases, the key is the hash of the entry.
    fn decrypt(&mut self, buffer: &mut [u8], key: u32, offset: usize);
}
