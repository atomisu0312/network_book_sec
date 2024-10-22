use std::fmt;
use std::mem;
use std::ops::*;

#[derive(Default, Debug, Clone, Copy, PartialEq)]
pub struct aesGF {
    pub value: u8,
}

// ＋演算子をオーバーロードしているだけ
impl Add for aesGF {
    type Output = Self;
    fn add(self, rhs: Self) -> Self {
        Self {
            value: (self.value ^ rhs.value),
        }
    }
}

// ＊演算子のオーバーロードしているだけ
// 原始多項式はx^8 + x^4 + x^3 + x + 1
impl Mul for aesGF {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self::Output {
        let mut ret: u8 = 0;
        let mut n: u8 = rhs.value;
        let mut a: u8 = self.value;
        while n > 0 {
            if n & 1 == 1 {
                ret = ret ^ a;
            }
            // オーバーフローを考慮している。
            a = (a << 1) ^ (if a & 0x80 == 0x80 { 0x1b } else { 0 });
            //println!("{:>08b} {} {:>08b}",ret,n,a);

            n >>= 1;
        }
        Self { value: ret }
    }
}
#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_multiply() {
        assert_eq!(
            aesGF { value: 0x57 } * aesGF { value: 0x83 },
            aesGF { value: 0xc1 }
        );
    }
}

impl fmt::Display for aesGF {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:x}", self.value)
    }
}

impl aesGF {
    pub fn inv(self) -> Self {
        let mut ret = self;
        for i in 0..253 {
            ret = ret * self;
        }
        //println!("ret is {}",ret);
        ret
    }
}
