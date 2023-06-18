use serde::de::SeqAccess;
use serde::de::Visitor;
use serde::ser::SerializeTuple;
use serde::Deserialize;
use serde::Deserializer;
use serde::Serialize;
use serde::Serializer;
use std::cmp::Ordering;
use std::fmt;
use std::ops::Add;
use std::ops::AddAssign;
use std::ops::BitAnd;
use std::ops::BitAndAssign;
use std::ops::BitOr;
use std::ops::BitOrAssign;
use std::ops::BitXor;
use std::ops::BitXorAssign;
use std::ops::Div;
use std::ops::DivAssign;
use std::ops::Mul;
use std::ops::MulAssign;
use std::ops::Rem;
use std::ops::RemAssign;
use std::ops::Shl;
use std::ops::ShlAssign;
use std::ops::Shr;
use std::ops::ShrAssign;
use std::ops::Sub;
use std::ops::SubAssign;
use std::usize;
#[macro_export]
macro_rules! vint {
    ($value:expr) => {{
        Vint::new($value as u128)
    }};
    ($value:expr, $size:expr) => {{
        Vint::<$size>::new($value as u128)
    }};
}
#[macro_export]
macro_rules! floor {
    ($value:expr, $size:expr) => {{
        Vint::<$size>::floor($value as u128)
    }};
}
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct Vint<const A: usize>(pub [u8; A]);
impl<const A: usize> Vint<A> {
    pub fn new(u: u128) -> Vint<A> {
        let mut vint = Vint([0; A]);
        if u == 0 {
            return vint;
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
        for (j, v) in vint.0.iter_mut().enumerate().take(A) {
            let k = i + j;
            if k == 16 {
                break;
            }
            *v = bytes[k];
        }
        vint.0[A - 1] = (vint.0[A - 1] & 0xf0) | size as u8;
        vint
    }
    pub fn int(self) -> u128 {
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
        vint![u, A].int()
    }
}
impl<const A: usize> From<u128> for Vint<A> {
    fn from(value: u128) -> Self {
        vint![value]
    }
}
impl<const A: usize> From<Vint<A>> for u128 {
    fn from(value: Vint<A>) -> Self {
        value.int()
    }
}
impl<const A: usize> Add<Vint<A>> for u128 {
    type Output = Vint<A>;
    fn add(self, rhs: Vint<A>) -> Vint<A> {
        vint![self + rhs.int()]
    }
}
impl<const A: usize> Sub<Vint<A>> for u128 {
    type Output = Vint<A>;
    fn sub(self, rhs: Vint<A>) -> Vint<A> {
        vint![self - rhs.int()]
    }
}
impl<const A: usize> Mul<Vint<A>> for u128 {
    type Output = Vint<A>;
    fn mul(self, rhs: Vint<A>) -> Vint<A> {
        vint![self * rhs.int()]
    }
}
impl<const A: usize> Div<Vint<A>> for u128 {
    type Output = Vint<A>;
    fn div(self, rhs: Vint<A>) -> Vint<A> {
        vint![self / rhs.int()]
    }
}
impl<const A: usize> Rem<Vint<A>> for u128 {
    type Output = Vint<A>;
    fn rem(self, rhs: Vint<A>) -> Vint<A> {
        vint![self % rhs.int()]
    }
}
impl<const A: usize, T: Into<u128>> Add<T> for Vint<A> {
    type Output = Vint<A>;
    fn add(self, rhs: T) -> Vint<A> {
        vint![self.int() + rhs.into()]
    }
}
impl<const A: usize, T: Into<u128>> Sub<T> for Vint<A> {
    type Output = Vint<A>;
    fn sub(self, rhs: T) -> Vint<A> {
        vint![self.int() - rhs.into()]
    }
}
impl<const A: usize, T: Into<u128>> Mul<T> for Vint<A> {
    type Output = Vint<A>;
    fn mul(self, rhs: T) -> Vint<A> {
        vint![self.int() * rhs.into()]
    }
}
impl<const A: usize, T: Into<u128>> Div<T> for Vint<A> {
    type Output = Vint<A>;
    fn div(self, rhs: T) -> Vint<A> {
        vint![self.int() / rhs.into()]
    }
}
impl<const A: usize, T: Into<u128>> Rem<T> for Vint<A> {
    type Output = Vint<A>;
    fn rem(self, rhs: T) -> Vint<A> {
        vint![self.int() % rhs.into()]
    }
}
impl<const A: usize, T: Into<u128>> BitAnd<T> for Vint<A> {
    type Output = Vint<A>;
    fn bitand(self, rhs: T) -> Vint<A> {
        vint![self.int() & rhs.into()]
    }
}
impl<const A: usize, T: Into<u128>> BitOr<T> for Vint<A> {
    type Output = Vint<A>;
    fn bitor(self, rhs: T) -> Vint<A> {
        vint![self.int() | rhs.into()]
    }
}
impl<const A: usize, T: Into<u128>> BitXor<T> for Vint<A> {
    type Output = Vint<A>;
    fn bitxor(self, rhs: T) -> Vint<A> {
        vint![self.int() ^ rhs.into()]
    }
}
impl<const A: usize, T: Into<u128>> Shl<T> for Vint<A> {
    type Output = Vint<A>;
    fn shl(self, rhs: T) -> Vint<A> {
        vint![self.int() << rhs.into()]
    }
}
impl<const A: usize, T: Into<u128>> Shr<T> for Vint<A> {
    type Output = Vint<A>;
    fn shr(self, rhs: T) -> Vint<A> {
        vint![self.int() >> rhs.into()]
    }
}
impl<const A: usize, T: Into<u128>> AddAssign<T> for Vint<A> {
    fn add_assign(&mut self, rhs: T) {
        *self = vint![self.int() + rhs.into()];
    }
}
impl<const A: usize, T: Into<u128>> SubAssign<T> for Vint<A> {
    fn sub_assign(&mut self, rhs: T) {
        *self = vint![self.int() - rhs.into()];
    }
}
impl<const A: usize, T: Into<u128>> MulAssign<T> for Vint<A> {
    fn mul_assign(&mut self, rhs: T) {
        *self = vint![self.int() * rhs.into()];
    }
}
impl<const A: usize, T: Into<u128>> DivAssign<T> for Vint<A> {
    fn div_assign(&mut self, rhs: T) {
        *self = vint![self.int() / rhs.into()];
    }
}
impl<const A: usize, T: Into<u128>> RemAssign<T> for Vint<A> {
    fn rem_assign(&mut self, rhs: T) {
        *self = vint![self.int() % rhs.into()];
    }
}
impl<const A: usize, T: Into<u128>> BitAndAssign<T> for Vint<A> {
    fn bitand_assign(&mut self, rhs: T) {
        *self = vint![self.int() & rhs.into()];
    }
}
impl<const A: usize, T: Into<u128>> BitOrAssign<T> for Vint<A> {
    fn bitor_assign(&mut self, rhs: T) {
        *self = vint![self.int() | rhs.into()];
    }
}
impl<const A: usize, T: Into<u128>> BitXorAssign<T> for Vint<A> {
    fn bitxor_assign(&mut self, rhs: T) {
        *self = vint![self.int() ^ rhs.into()];
    }
}
impl<const A: usize, T: Into<u128>> ShlAssign<T> for Vint<A> {
    fn shl_assign(&mut self, rhs: T) {
        *self = vint![self.int() << rhs.into()];
    }
}
impl<const A: usize, T: Into<u128>> ShrAssign<T> for Vint<A> {
    fn shr_assign(&mut self, rhs: T) {
        *self = vint![self.int() >> rhs.into()];
    }
}
impl<const A: usize> PartialOrd for Vint<A> {
    fn partial_cmp(&self, rhs: &Vint<A>) -> Option<Ordering> {
        Some(self.cmp(rhs))
    }
}
impl<const A: usize> Ord for Vint<A> {
    fn cmp(&self, rhs: &Vint<A>) -> Ordering {
        self.int().cmp(&rhs.int())
    }
}
impl<const A: usize> fmt::Display for Vint<A> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.int())
    }
}
impl<const A: usize> Serialize for Vint<A> {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        let mut seq = serializer.serialize_tuple(A)?;
        for value in self.0.iter() {
            seq.serialize_element(value)?;
        }
        seq.end()
    }
}
impl<'de, const A: usize> Deserialize<'de> for Vint<A> {
    fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Vint<A>, D::Error> {
        struct VintVisitor<const A: usize>;
        impl<'de, const A: usize> Visitor<'de> for VintVisitor<A> {
            type Value = Vint<A>;
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
                Ok(Vint(array))
            }
        }
        deserializer.deserialize_tuple(A, VintVisitor)
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn vint() {
        assert_eq!(vint![0].0, [0]);
        assert_eq!(vint![0, 1].0, [0]);
        assert_eq!(vint![0], Vint::<1>::new(0));
        assert_eq!(vint![0, 1], Vint::new(0));
    }
    #[test]
    fn floor() {
        assert_eq!(floor![100, 1], 96);
        assert_eq!(floor![100, 1], Vint::<1>::floor(100));
    }
    #[test]
    fn new() {
        assert_eq!(vint![0x00].0, [0x00]);
        assert_eq!(vint![0x01].0, [0x00]);
        assert_eq!(vint![0x10].0, [0x10]);
        assert_eq!(vint![0x11].0, [0x10]);
        assert_eq!(vint![0x10000000000000000000000000000000].0, [0x1f]);
        assert_eq!(vint![0x00].0, [0x00, 0x00]);
        assert_eq!(vint![0x01].0, [0x01, 0x00]);
        assert_eq!(vint![0x10].0, [0x10, 0x00]);
        assert_eq!(vint![0x11].0, [0x11, 0x00]);
        assert_eq!(vint![0x10000000000000000000000000000000].0, [0x10, 0x0f]);
    }
    #[test]
    fn int() {
        assert_eq!(Vint([0x00]).int(), 0x00);
        assert_eq!(Vint([0x01]).int(), 0x00);
        assert_eq!(Vint([0x10]).int(), 0x10);
        assert_eq!(Vint([0x11]).int(), 0x1000);
        assert_eq!(Vint([0x1f]).int(), 0x10000000000000000000000000000000);
        assert_eq!(Vint([0x00, 0x00]).int(), 0x00);
        assert_eq!(Vint([0x01, 0x00]).int(), 0x01);
        assert_eq!(Vint([0x10, 0x00]).int(), 0x10);
        assert_eq!(Vint([0x11, 0x00]).int(), 0x11);
        assert_eq!(Vint([0x10, 0x0f]).int(), 0x10000000000000000000000000000000);
    }
    #[test]
    fn bincode_serialize() {
        assert_eq!(bincode::serialize(&Vint([1])).unwrap(), [1]);
        assert_eq!(bincode::serialize(&Vint([1, 1])).unwrap(), [1, 1]);
    }
    #[test]
    fn bincode_deserialize() {
        assert_eq!(
            bincode::deserialize::<Vint<1>>(&vec![1]).unwrap(),
            Vint([1])
        );
        assert_eq!(
            bincode::deserialize::<Vint<2>>(&vec![1, 1]).unwrap(),
            Vint([1, 1])
        );
    }
    #[test]
    fn serde_json_to_string() {
        assert_eq!(serde_json::to_string(&Vint([1])).unwrap(), "[1]");
        assert_eq!(serde_json::to_string(&Vint([1, 1])).unwrap(), "[1,1]");
    }
    #[test]
    fn serde_json_from_str() {
        assert_eq!(serde_json::from_str::<Vint<1>>("[1]").unwrap(), Vint([1]));
        assert_eq!(
            serde_json::from_str::<Vint<2>>("[1,1]").unwrap(),
            Vint([1, 1])
        );
    }
    #[test]
    fn add() {
        let a = 1;
        let b = vint![a, 2];
        assert_eq!(a + b, vint![2]);
        assert_eq!(b + a, vint![2]);
        assert_eq!(b + b, vint![2]);
    }
    #[test]
    fn sub() {
        let a = 2;
        let b = vint![a, 2];
        assert_eq!(a - b, vint![0]);
        assert_eq!(b - a, vint![0]);
        assert_eq!(b - b, vint![0]);
    }
    #[test]
    fn mul() {
        let a = 2;
        let b = vint![a, 2];
        assert_eq!(a * b, vint![4]);
        assert_eq!(b * a, vint![4]);
        assert_eq!(b * b, vint![4]);
    }
    #[test]
    fn div() {
        let a = 4;
        let b = vint![a, 2];
        assert_eq!(a / b, vint![1]);
        assert_eq!(b / a, vint![1]);
        assert_eq!(b / b, vint![1]);
    }
    #[test]
    fn rem() {
        let a: u128 = 4;
        let b = vint![a, 2];
        assert_eq!(a % b, Vint::from(0));
        assert_eq!(b % a, Vint::from(0));
        assert_eq!(b % b, Vint::from(0));
    }
    #[test]
    fn bit() {
        let a: u128 = 1;
        let b = vint![a, 2];
        assert_eq!(b | a, Vint::from(1));
        assert_eq!(b & a, Vint::from(1));
        assert_eq!(b ^ a, Vint::from(0));
    }
}
