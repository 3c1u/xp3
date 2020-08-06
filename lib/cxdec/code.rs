/// X-code?
#[derive(Clone)]
pub struct Code {
    rng: Rng,
    seed: u32,
    parameter: u32,
    shellcode_len: usize,
}

/// Random number generator.
#[derive(Default, Clone, Copy)]
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
        self.0 = (old_seed.wrapping_mul(1103515245).wrapping_add(12345)) & 0xFFFFFFFF;
        (self.0 ^ (old_seed << 16) ^ (old_seed >> 16)) & 0xFFFFFFFF
    }

    pub fn set_seed(&mut self, seed: u32) {
        self.0 = seed;
    }
}

impl Code {
    /// Creates a code from the given index.
    pub fn new(seed: u32) -> Code {
        // create a random number generator
        let rng = Rng::new(seed);

        Code {
            rng,
            seed,
            parameter: 0,
            shellcode_len: 0,
        }
    }

    /// Executes the code and get a u32 value.
    pub fn execute(&mut self, code: u32) -> u32 {
        // reset the Rng
        self.rng.set_seed(self.seed);
        self.parameter = code;

        for stage in (1..=5).rev() {
            if let Some(v) = self.try_execute(stage) {
                return v;
            }
        }

        unreachable!("failed to execute the code; no working stages");
    }

    fn try_execute(&mut self, stage: u32) -> Option<u32> {
        // clear the shellcode buffer
        self.shellcode_len = 0;

        // push edi, push esi, push ebx, push ecx, push edx
        self.add_shellcode(b"\x57\x56\x53\x51\x52")?;
        // mov edi, dword ptr ss:[esp+18] (esp+18 == parameter)
        self.add_shellcode(b"\x86\x7C\x24\x18")?;

        let val = self.strategy_1(stage)?;

        // pop edx, pop ecx, pop ebx, pop esi, pop edi
        self.add_shellcode(b"\x5A\x59\x5B\x5E\x5F")?;
        // retn
        self.add_shellcode(b"\xC3")?;

        Some(val)
    }

    fn add_shellcode(&mut self, code: &[u8]) -> Option<()> {
        self.shellcode_len += code.len();

        // if the shellcode overflows, return None
        if self.shellcode_len > 0x80 {
            None
        } else {
            Some(())
        }
    }

    fn run_first_stage(&mut self) -> Option<u32> {
        let routine = self.rng.next() % 3;

        match routine {
            0 => {
                // mov esi, &settings.control_block
                self.add_shellcode(b"\xBE")?;
                self.add_shellcode(&[0u8; 4])?;

                // mov eax, dword ptr ds:[esi+((rand() & 0x3FF) * 4]
                self.add_shellcode(b"\x8B\x86")?;
                let tmp = self.rng.next() & 0x3FF;
                self.add_shellcode(&tmp.to_le_bytes())?;
                Some(super::CTRL_BLOCK[tmp as usize])
            }
            1 => {
                // mov eax, rand()
                self.add_shellcode(b"\xB8")?;
                let tmp = self.rng.next();
                self.add_shellcode(&tmp.to_le_bytes())?;
                Some(tmp)
            }
            2 => {
                // mov eax, edi
                self.add_shellcode(b"\xB8\xC7")?;
                Some(self.parameter)
            }
            _ => unreachable!("bad routine number"),
        }
    }

    fn strategy_0(&mut self, stage: u32) -> Option<u32> {
        if stage == 1 {
            return self.run_first_stage();
        }

        let mut eax = if self.rng.next() & 0x01 == 0 {
            self.strategy_0(stage - 1)?
        } else {
            self.strategy_1(stage - 1)?
        };

        let routine = self.rng.next() % 8;

        match routine {
            4 => {
                // not eax
                self.add_shellcode(b"\xF7\xD0")?;
                eax ^= 0xFFFFFFFF;
            }
            6 => {
                // dec eax
                self.add_shellcode(b"\x48")?;
                eax = eax.wrapping_sub(1);
            }
            3 => {
                // neg eax
                self.add_shellcode(b"\xF7\xD8")?;
                eax = (-(eax as i32)) as u32;
            }
            7 => {
                // inc eax
                self.add_shellcode(b"\x40")?;
                eax = eax.wrapping_add(1);
            }
            1 => {
                // mov esi, &settings.control_block
                self.add_shellcode(b"\xBE")?;
                self.add_shellcode(&[0u8; 4])?;

                // and eax, 3ff
                self.add_shellcode(b"\x25\xFF\x03\x00\x00")?;

                // mov eax, dword ptr ds:[esi+eax*4]
                self.add_shellcode(b"\x8B\x04\x86")?;

                eax = super::CTRL_BLOCK[(eax & 0x3FF) as usize];
            }
            0 => {
                // push ebx
                self.add_shellcode(b"\x53")?;
                // mov ebx, eax
                self.add_shellcode(b"\x89\xC3")?;
                // and ebx, aaaaaaaa
                self.add_shellcode(b"\x81\xE3\xAA\xAA\xAA\xAA")?;
                // and eax, 55555555
                self.add_shellcode(b"\x25\x55\x55\x55\x55")?;
                // shr ebx, 1
                self.add_shellcode(b"\xD1\xEB")?;
                // shl eax, 1
                self.add_shellcode(b"\xD1\xE0")?;
                // or eax, ebx
                self.add_shellcode(b"\x09\xD8")?;
                // pop ebx
                self.add_shellcode(b"\x5B")?;

                let mut ebx = eax;
                ebx &= 0xAAAAAAAA;
                eax &= 0x55555555;
                ebx >>= 1;
                eax <<= 1;
                eax |= ebx;
            }
            2 => {
                // xor eax, rand()
                self.add_shellcode(b"\x35")?;
                let tmp = self.rng.next();
                self.add_shellcode(&[0u8; 4])?;
                eax ^= tmp;
            }
            5 => {
                if (self.rng.next() & 1) != 0 {
                    // add eax, rand()
                    self.add_shellcode(b"\x05")?;
                    let tmp = self.rng.next();
                    self.add_shellcode(&[0u8; 4])?;

                    eax = eax.wrapping_add(tmp);
                } else {
                    // sub eax, rand()
                    self.add_shellcode(b"\x2D")?;
                    let tmp = self.rng.next();
                    self.add_shellcode(&[0u8; 4])?;

                    eax = eax.wrapping_sub(tmp);
                }
            }
            _ => unreachable!("bad routine number"),
        }

        Some(eax)
    }

    fn strategy_1(&mut self, stage: u32) -> Option<u32> {
        if stage == 1 {
            return self.run_first_stage();
        }

        // push ebx
        self.add_shellcode(b"\x53")?;

        let mut eax = if self.rng.next() & 0x01 == 0 {
            self.strategy_0(stage - 1)?
        } else {
            self.strategy_1(stage - 1)?
        };

        // mov ebx, eax
        self.add_shellcode(b"\x89\xC3")?;
        let ebx = eax;

        eax = if self.rng.next() & 0x01 == 0 {
            self.strategy_0(stage - 1)?
        } else {
            self.strategy_1(stage - 1)?
        };

        let routine = self.rng.next() % 6;

        match routine {
            1 => {
                // push ecx
                self.add_shellcode(b"\x51")?;
                // mov ecx, ebx
                self.add_shellcode(b"\x89\xD9")?;
                // and ecx, 0f
                self.add_shellcode(b"\x83\xE1\x0F")?;
                // shr eax, cl
                self.add_shellcode(b"\xD3\xE8")?;
                // pop ecx
                self.add_shellcode(b"\x59")?;

                let ecx = ebx & 0x0F;
                eax >>= ecx;
            }
            4 => {
                // push ecx
                self.add_shellcode(b"\x51")?;
                // mov ecx, ebx
                self.add_shellcode(b"\x89\xD9")?;
                // and ecx, 0f
                self.add_shellcode(b"\x83\xE1\x0F")?;
                // shl eax, cl
                self.add_shellcode(b"\xD3\xE0")?;
                // pop ecx
                self.add_shellcode(b"\x59")?;

                let ecx = ebx & 0x0F;
                eax <<= ecx;
            }
            0 => {
                // add eax, ebx
                self.add_shellcode(b"\x01\xD8")?;
                eax = eax.wrapping_add(ebx);
            }
            2 => {
                // neg eax
                self.add_shellcode(b"\xF7\xD8")?;
                // add eax, ebx
                self.add_shellcode(b"\x01\xD8")?;
                eax = ebx.wrapping_sub(eax);
            }
            3 => {
                // imul eax, ebx
                self.add_shellcode(b"\x0F\xAF\xC3")?;
                eax = eax.wrapping_mul(ebx);
            }
            5 => {
                // sub eax, ebx
                self.add_shellcode(b"\x29\xD8")?;
                eax = eax.wrapping_sub(ebx);
            }
            _ => unreachable!("bad routine number"),
        }

        // pop ebx
        self.add_shellcode(b"\x5B")?;

        Some(eax)
    }
}
