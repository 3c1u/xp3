/// X-code?
#[derive(Clone)]
pub struct Code {
    rng: Rng,
    seed: u32,
    parameter: u32,
    code_len: usize,
}

/// Random number generator.
#[derive(Default, Clone, Copy)]
struct Rng(u32);

impl Rng {
    pub fn new(seed: u32) -> Rng {
        Rng(seed)
    }

    /* pub fn peek(&self) -> u32 {
        self.0
    } */

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
            code_len: 0,
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
        self.code_len = 0;

        // push edi, push esi, push ebx, push ecx, push edx
        // mov edi, dword ptr ss:[esp+18] (esp+18 == parameter)
        self.append_code(9)?;

        let val = self.strategy_1(stage)?;

        // pop edx, pop ecx, pop ebx, pop esi, pop edi
        // retn
        self.append_code(6)?;

        Some(val)
    }

    fn append_code(&mut self, len: usize) -> Option<()> {
        self.code_len += len;

        // if the shellcode overflows, return None
        if self.code_len > 0x80 {
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
                self.append_code(5)?;

                // mov eax, dword ptr ds:[esi+((rand() & 0x3FF) * 4]
                self.append_code(2)?;
                let tmp = self.rng.next() & 0x3FF;
                self.append_code(4)?;

                Some(super::CTRL_BLOCK[tmp as usize])
            }
            1 => {
                // mov eax, rand()
                self.append_code(1)?;
                let tmp = self.rng.next();
                self.append_code(4)?;
                Some(tmp)
            }
            2 => {
                // mov eax, edi
                self.append_code(2)?;
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
                self.append_code(2)?;
                eax ^= 0xFFFFFFFF;
            }
            6 => {
                // dec eax
                self.append_code(1)?;
                eax = eax.wrapping_sub(1);
            }
            3 => {
                // neg eax
                self.append_code(2)?;
                eax = (-(eax as i32)) as u32;
            }
            7 => {
                // inc eax
                self.append_code(1)?;
                eax = eax.wrapping_add(1);
            }
            1 => {
                // mov esi, &settings.control_block
                // and eax, 3ff
                // mov eax, dword ptr ds:[esi+eax*4]
                self.append_code(13)?;

                eax = super::CTRL_BLOCK[(eax & 0x3FF) as usize];
            }
            0 => {
                // push ebx
                // mov ebx, eax
                // and ebx, aaaaaaaa
                // and eax, 55555555
                // shr ebx, 1
                // shl eax, 1
                // or eax, ebx
                // pop ebx
                self.append_code(21)?;

                let mut ebx = eax;
                ebx &= 0xAAAAAAAA;
                eax &= 0x55555555;
                ebx >>= 1;
                eax <<= 1;
                eax |= ebx;
            }
            2 => {
                // xor eax, rand()
                self.append_code(1)?;
                let tmp = self.rng.next();
                self.append_code(4)?;
                eax ^= tmp;
            }
            5 => {
                if (self.rng.next() & 1) != 0 {
                    // add eax, rand()
                    self.append_code(1)?;
                    let tmp = self.rng.next();
                    self.append_code(4)?;

                    eax = eax.wrapping_add(tmp);
                } else {
                    // sub eax, rand()
                    self.append_code(1)?;
                    let tmp = self.rng.next();
                    self.append_code(4)?;

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
        self.append_code(1)?;

        let mut eax = if self.rng.next() & 0x01 == 0 {
            self.strategy_0(stage - 1)?
        } else {
            self.strategy_1(stage - 1)?
        };

        // mov ebx, eax
        self.append_code(2)?;
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
                // mov ecx, ebx
                // and ecx, 0f
                // shr eax, cl
                // pop ecx
                self.append_code(9)?;

                let ecx = ebx & 0x0F;
                eax >>= ecx;
            }
            4 => {
                // push ecx
                // mov ecx, ebx
                // and ecx, 0f
                // shl eax, cl
                // pop ecx
                self.append_code(9)?;

                let ecx = ebx & 0x0F;
                eax <<= ecx;
            }
            0 => {
                // add eax, ebx
                self.append_code(2)?;
                eax = eax.wrapping_add(ebx);
            }
            2 => {
                // neg eax
                // add eax, ebx
                self.append_code(4)?;
                eax = ebx.wrapping_sub(eax);
            }
            3 => {
                // imul eax, ebx
                self.append_code(3)?;
                eax = eax.wrapping_mul(ebx);
            }
            5 => {
                // sub eax, ebx
                self.append_code(2)?;
                eax = eax.wrapping_sub(ebx);
            }
            _ => unreachable!("bad routine number"),
        }

        // pop ebx
        self.append_code(1)?;

        Some(eax)
    }
}
