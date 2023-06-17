use std::usize;
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct Varint<const A: usize>(pub [u8; A]);
impl<const A: usize> Varint<A> {
    pub fn from(u: u128) -> Varint<A> {
        let mut varint = Varint([0; A]);
        if u == 0 {
            return varint;
        }
        let bytes = u.to_be_bytes();
        let mut i = 0;
        for byte in bytes {
            if byte != 0 {
                break;
            }
            i += 1;
        }
        let size = 15 - i;
        for (j, v) in varint.0.iter_mut().enumerate().take(A) {
            let k = i + j;
            if k == 16 {
                break;
            }
            *v = bytes[k];
        }
        varint.0[A - 1] = (varint.0[A - 1] & 0xf0) | size as u8;
        varint
    }
    pub fn u128(self) -> u128 {
        let size = self.0[A - 1] as usize & 0x0f;
        let mut bytes = [0; 16];
        for (i, v) in self.0.iter().enumerate().take(A) {
            let j = 15 - size + i;
            if j == 16 {
                break;
            }
            if i == A - 1 {
                bytes[j] = v & 0xf0;
                break;
            }
            bytes[j] = *v;
        }
        u128::from_be_bytes(bytes)
    }
    pub fn floor(u: u128) -> u128 {
        Varint::<A>::from(u).u128()
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn from() {
        assert_eq!(Varint::from(0x00).0, [0x00]);
        assert_eq!(Varint::from(0x01).0, [0x00]);
        assert_eq!(Varint::from(0x10).0, [0x10]);
        assert_eq!(Varint::from(0x11).0, [0x10]);
        assert_eq!(Varint::from(0x10000000000000000000000000000000).0, [0x1f]);
        assert_eq!(Varint::from(0x00).0, [0x00, 0x00]);
        assert_eq!(Varint::from(0x01).0, [0x01, 0x00]);
        assert_eq!(Varint::from(0x10).0, [0x10, 0x00]);
        assert_eq!(Varint::from(0x11).0, [0x11, 0x00]);
        assert_eq!(
            Varint::from(0x10000000000000000000000000000000).0,
            [0x10, 0x0f]
        );
    }
    #[test]
    fn u128() {
        assert_eq!(Varint([0x00]).u128(), 0x00);
        assert_eq!(Varint([0x01]).u128(), 0x00);
        assert_eq!(Varint([0x10]).u128(), 0x10);
        assert_eq!(Varint([0x11]).u128(), 0x1000);
        assert_eq!(Varint([0x1f]).u128(), 0x10000000000000000000000000000000);
        assert_eq!(Varint([0x00, 0x00]).u128(), 0x00);
        assert_eq!(Varint([0x01, 0x00]).u128(), 0x01);
        assert_eq!(Varint([0x10, 0x00]).u128(), 0x10);
        assert_eq!(Varint([0x11, 0x00]).u128(), 0x11);
        assert_eq!(
            Varint([0x10, 0x0f]).u128(),
            0x10000000000000000000000000000000
        );
    }
}
