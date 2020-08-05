/// X-code?
#[derive(Clone)]
pub struct Code {}

/// Random number generator.
#[derive(Default)]
struct Rng(u32);

impl Rng {
    pub fn new(seed: u32) -> Rng {
        Rng(seed)
    }

    pub fn peek(&self) -> u32 {
        self.0
    }

    pub fn next(&mut self) -> u32 {
        let old_seed = self.0;
        self.0 = (1103515245 * old_seed + 12345) & 0xFFFFFFFF;
        (self.0 ^ (old_seed << 16) ^ (old_seed >> 16)) & 0xFFFFFFFF
    }

    pub fn set_seed(&mut self, seed: u32) {
        self.0 = seed;
    }
}

impl Code {
    /// Creates a code from the given index.
    pub fn new(index: u32) -> Code {
        // create a program

        // generate a code
        for stage in (1..=5).rev() {
            
        }

        panic!("failed to generate a code: all stages failed")
    }

    pub fn execute(&mut self, code: u32) -> u32 {
        todo!()
    }
}
