use serde::de::SeqAccess;
use serde::de::Visitor;
use serde::ser::SerializeTuple;
use serde::Deserialize;
use serde::Deserializer;
use serde::Serialize;
use serde::Serializer;
use std::fmt;
use std::ops::Add;
use std::ops::Div;
use std::ops::Mul;
use std::ops::Rem;
use std::ops::Sub;
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
impl<const A: usize> Serialize for Varint<A> {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        let mut seq = serializer.serialize_tuple(A)?;
        for value in self.0.iter() {
            seq.serialize_element(value)?;
        }
        seq.end()
    }
}
impl<'de, const A: usize> Deserialize<'de> for Varint<A> {
    fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Varint<A>, D::Error> {
        struct VarintVisitor<const A: usize>;
        impl<'de, const A: usize> Visitor<'de> for VarintVisitor<A> {
            type Value = Varint<A>;
            fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                formatter.write_fmt(format_args!("a byte array of length {}", A))
            }
            fn visit_seq<S: SeqAccess<'de>>(self, mut seq: S) -> Result<Self::Value, S::Error> {
                let mut array = [0; A];
                for i in 0..A {
                    array[i] = seq
                        .next_element::<u8>()?
                        .ok_or_else(|| serde::de::Error::invalid_length(i, &"more elements"))?;
                }
                Ok(Varint(array))
            }
        }
        deserializer.deserialize_tuple(A, VarintVisitor)
    }
}
impl<const A: usize> Add for Varint<A> {
    type Output = Varint<A>;
    fn add(self, other: Varint<A>) -> Varint<A> {
        Varint::from(self.u128() + other.u128())
    }
}
impl<const A: usize> Sub for Varint<A> {
    type Output = Varint<A>;
    fn sub(self, other: Varint<A>) -> Varint<A> {
        Varint::from(self.u128() - other.u128())
    }
}
impl<const A: usize> Mul for Varint<A> {
    type Output = Varint<A>;
    fn mul(self, other: Varint<A>) -> Varint<A> {
        Varint::from(self.u128() * other.u128())
    }
}
impl<const A: usize> Div for Varint<A> {
    type Output = Varint<A>;
    fn div(self, other: Varint<A>) -> Varint<A> {
        Varint::from(self.u128() / other.u128())
    }
}
impl<const A: usize> Rem for Varint<A> {
    type Output = Varint<A>;
    fn rem(self, other: Varint<A>) -> Varint<A> {
        Varint::from(self.u128() % other.u128())
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
    #[test]
    fn bincode_serialize() {
        assert_eq!(bincode::serialize(&Varint([1])).unwrap(), [1]);
        assert_eq!(bincode::serialize(&Varint([1, 1])).unwrap(), [1, 1]);
    }
    #[test]
    fn bincode_deserialize() {
        assert_eq!(
            bincode::deserialize::<Varint<1>>(&vec![1]).unwrap(),
            Varint([1])
        );
        assert_eq!(
            bincode::deserialize::<Varint<2>>(&vec![1, 1]).unwrap(),
            Varint([1, 1])
        );
    }
    #[test]
    fn serde_json_to_string() {
        assert_eq!(serde_json::to_string(&Varint([1])).unwrap(), "[1]");
        assert_eq!(serde_json::to_string(&Varint([1, 1])).unwrap(), "[1,1]");
    }
    #[test]
    fn serde_json_from_str() {
        assert_eq!(
            serde_json::from_str::<Varint<1>>("[1]").unwrap(),
            Varint([1])
        );
        assert_eq!(
            serde_json::from_str::<Varint<2>>("[1,1]").unwrap(),
            Varint([1, 1])
        );
    }
    #[test]
    fn add() {
        let a = Varint::<2>::from(1);
        assert_eq!(a + a, Varint::from(2));
    }
    #[test]
    fn sub() {
        let a = Varint::<2>::from(2);
        assert_eq!(a - a, Varint::from(0));
    }
    #[test]
    fn mul() {
        let a = Varint::<2>::from(2);
        assert_eq!(a * a, Varint::from(4));
    }
    #[test]
    fn div() {
        let a = Varint::<2>::from(4);
        assert_eq!(a / a, Varint::from(1));
    }
    #[test]
    fn rem() {
        let a = Varint::<2>::from(4);
        assert_eq!(a % a, Varint::from(0));
    }
}
