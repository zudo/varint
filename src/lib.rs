use std::num::ParseIntError;
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
pub fn to_string<const A: usize>(u: u128) -> String {
    let mut string = format!("{}{}", "0".repeat(A), u);
    string.insert(string.len() - A, '.');
    string = string
        .trim_start_matches('0')
        .trim_end_matches('0')
        .trim_end_matches('.')
        .to_string();
    if string.starts_with('.') {
        let mut s = "0".to_string();
        s.push_str(&string);
        string = s;
    }
    if string.is_empty() {
        string.push('0');
    }
    string
}
pub fn from_str<const A: usize>(s: &str) -> Result<u128, ParseIntError> {
    let (mut string, diff) = match s.split_once('.') {
        Some((a, b)) => {
            let mut string = a.to_string();
            string.push_str(b);
            (string, A - b.len())
        }
        None => (s.to_string(), A),
    };
    string.push_str(&"0".repeat(diff));
    string.parse()
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
    #[test]
    fn test_to_string() {
        assert_eq!("10.01", to_string::<18>(10_010_000_000_000_000_000));
        assert_eq!("1", to_string::<18>(1_000_000_000_000_000_000));
        assert_eq!("10", to_string::<18>(10_000_000_000_000_000_000));
        assert_eq!("0.1", to_string::<18>(100_000_000_000_000_000));
        assert_eq!("0", to_string::<18>(0));
    }
    #[test]
    fn test_from_string() {
        assert_eq!(
            10_010_000_000_000_000_000,
            from_str::<18>("010.010").unwrap()
        );
        assert_eq!(1_000_000_000_000_000_000, from_str::<18>("1").unwrap());
        assert_eq!(10_000_000_000_000_000_000, from_str::<18>("10").unwrap());
        assert_eq!(10_000_000_000_000_000_000, from_str::<18>("10.").unwrap());
        assert_eq!(10_000_000_000_000_000_000, from_str::<18>("10.0").unwrap());
        assert_eq!(100_000_000_000_000_000, from_str::<18>(".1").unwrap());
        assert_eq!(0, from_str::<18>("0").unwrap());
    }
}
