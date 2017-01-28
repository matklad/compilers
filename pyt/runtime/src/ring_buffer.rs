pub const EMPTY: RingBuff = RingBuff { mem: [0; 128], r: 0, w: 0 };
const MASK: usize = 127;

pub struct RingBuff {
    mem: [u8; 128],
    r: usize,
    w: usize,
}

impl RingBuff {
    pub fn next(&mut self) -> Option<u8> {
        if self.r == self.w {
            None
        } else {
            let byte = self.mem[self.r & MASK];
            self.r += 1;
            Some(byte)
        }
    }

    pub fn buff(&mut self) -> &mut [u8] {
        let r = self.r & MASK;
        let w = self.w & MASK;
        if r <= w {
            &mut self.mem[w..]
        } else {
            &mut self.mem[w..r]
        }
    }

    pub fn advance(&mut self, delta: usize) {
        self.w += delta;
    }
}
