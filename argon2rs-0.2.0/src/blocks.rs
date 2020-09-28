use u64x2::u64x2;
use std::ops::{Add, BitXor, Mul, Shl, Shr};
use std::mem;

const ARGON2_BLOCK_BYTES: usize = 1024;

macro_rules! per_kib {
    (u8) => { ARGON2_BLOCK_BYTES };
    (u64) => { ARGON2_BLOCK_BYTES / 8 };
    (u64x2) => { ARGON2_BLOCK_BYTES / 16 };
}

#[derive(Clone, Copy)]
pub struct Block([u64; per_kib!(u64x2)]);

fn zero() -> Block { Block([u64x2(0, 0); per_kib!(u64x2)]) }

impl BitXor for Block {
    type Output = Self;
    #[inline(always)]
    fn bitxor(self, r: Self) -> Self::Output {
        let mut inner: [u64x2; per_kib!(u64x2)] = unsafe { mem::uninitialized() };

        for (d, (lhs, rhs)) in inner.iter_mut()
                                    .zip(self.0.iter().zip(r.0.iter())) {
            *d = *lhs ^ *rhs;
        }

        Block(inner)
    }
}

#[cfg(test)]
mod test {
    use super::{ARGON2_BLOCK_BYTES, Block};
    use u64x2::u64x2;

    #[test]
    fn test_codegen() {
        let mut z = Block([u64x2(2, 2); per_kib!(u64x2)]);
        let mut j = Block([u64x2(6, 6); per_kib!(u64x2)]);
        let mut k = Block([u64x2(7, 7); per_kib!(u64x2)]);

        for r in (z ^ j ^ k).0.iter() {
            println!("{:02x} {:02x}", r.0, r.1);
        }
    }
}
