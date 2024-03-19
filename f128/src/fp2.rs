
use core::{
    AsBytes, ByteReader, ByteWriter, Deserializable, DeserializationError, Randomizable,
    Serializable, SliceReader,
};
use crypto_bigint::{
    subtle::CtOption, U256};

use std::{
    fmt::Display,
    ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Neg, Sub, SubAssign},
    slice, u128,
};

use traits::traits::{Field, Extensible, ExtensionOf};

use crate::fp::Fp;

//Maximum size of quadratic extension element in terms of bytes, it is equal to the space held by 2 128-bit integers
const ELEMENT_BYTES: usize = std::mem::size_of::<u128>() * 2;
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]


pub struct Fp2<B: Extensible<2>> {
    pub c0: B,
    pub c1: B,
}

impl<B:Extensible<2>> Fp2<B> {
    const fn new(x: B, y:B)->Fp2<B>{
        Fp2{c0:x, c1:y}
    }
    pub fn elements_as_bytes(elements: &[Self]) -> &[u8] {
        unsafe {
            slice::from_raw_parts(
                elements.as_ptr() as *const u8,
                elements.len() * Self::ELEMENT_BYTES,
            )
        }
    }
}
//Field implementation block for quadratic extension.
impl<B:Extensible<2>> Field for Fp2<B> {
    const ONE: Self = Fp2::<B>::new(B::ONE, B::ZERO);
    const ZERO: Self = Fp2::<B>::new(B::ZERO, B::ZERO);

    fn is_one(self) -> bool {
        self == Self::ONE
    }

    fn is_zero(self) -> bool {
        self == Self::ZERO
    }

    fn invert(self) -> CtOption<Fp2<B>> {
        let inverse = <B as Extensible<2>>::invert([self.c0, self.c1]);
        CtOption::new(Fp2::<B>::from(inverse.unwrap()), inverse.is_some())
    }
    
    fn square(self) -> Self {
        Fp2::<B>::from(<B as Extensible<2>>::square([self.c0, self.c1]))
    }

    fn random()->Self {
        Fp2::<B>::new(B::random(), B::random())
    }

    fn sqrt(self) -> crypto_bigint::subtle::CtOption<Self> {
        unimplemented!()
    }
    
    type BaseField = B;

    fn power_by<S:AsRef<[u64]>>(self,_pow:S)->Self {
        unimplemented!()
    }
    fn to_curve_bytes(&self) -> &[u8] {
        self.as_bytes()
    }
    fn to_words(&self) -> Vec<u64> {
        unimplemented!()
    }

    const ELEMENT_BYTES: usize = ELEMENT_BYTES;

    fn  from_uint_reduced(_w: Fp2<B>) -> Self {
     unimplemented!()
    }

    fn from_words( _a: &Vec<u64>) -> Self {
     unimplemented!()
    }

    fn get_windows(&self, _exp: usize)->Vec<usize> {
     unimplemented!()
    }

  
}

/// A field is always an extension of itself.
impl<B:Extensible<2>> ExtensionOf<B> for Fp2<B> where Fp2<B>: From<B> {
    fn mul_base(self, other: B) -> Self {
        B::mul_base([self.c0, self.c1], other).into()
    }
}
//Quadratic extension for 128-bit field with respect to irreducible polynomial x^2 - x - 1. Basis considered is {1, w}.

//Trait implementation for addition trait to use "+" operation with field elements.
impl<B: Extensible<2>> Add for Fp2<B> {
    type Output = Fp2<B>;
    fn add(self, rhs: Self) -> Self::Output {
        Fp2::from(add([self.c0, self.c1], [rhs.c0, rhs.c1]))
    }
}

//Trait implementation for add-assign trait to use "+=" operation with field elements.
impl<B: Extensible<2>> AddAssign for Fp2<B> {
    fn add_assign(&mut self, rhs: Self) {
        *self = *self + rhs
    }
}

impl<B: Extensible<2>> Neg for Fp2<B> {
    type Output = Fp2<B>;
    fn neg(self) -> Self::Output {
        let x = -self.c0;
        let y = -self.c1;
        Fp2::<B>::new(x, y)
    }
}

//Trait implementation for subtraction trait to use "-" with field elements.
impl<B: Extensible<2>> Sub for Fp2<B> {
    type Output = Fp2<B>;
    fn sub(self, rhs: Self) -> Self::Output {
        Fp2::<B>::from(sub([self.c0, self.c1], [rhs.c0, rhs.c1]))
    }
}

//Trait implementation for sub-assign trait to use "-=" operation with field elements.
impl<B: Extensible<2>> SubAssign for Fp2<B> {
    fn sub_assign(&mut self, rhs: Self) {
        *self = *self - rhs
    }
}

//Trait implementation for multiplication trait to use "*" operation with field elements.
impl<B: Extensible<2>> Mul for Fp2<B> where Fp2<B>: From<[B;2]> {
    type Output = Fp2<B>;
    fn mul(self, rhs: Self) -> Self::Output {
        Fp2::<B>::from(<B as Extensible<2>>::mul([self.c0, self.c1], [rhs.c0, rhs.c1]))
    }
}

//Trait implementation for multiply-assign trait to use "*=" operation with field elements.
impl<B: Extensible<2>> MulAssign for Fp2<B> {
    fn mul_assign(&mut self, rhs: Self) {
        *self = *self * rhs
    }
}

//Trait implementation for division trait to use "/" operation with field elements.
impl<B: Extensible<2>> Div for Fp2<B> {
    type Output = Fp2<B>;
    fn div(self, rhs: Self) -> Self::Output {
        //In fields, a/b = a*(1/b), that is division of a by b is equivalent to
        //multiplying a into the inverse of b.
        self * rhs.invert().unwrap()
    }
}

//Trait implementation for divide-assign trait to use "/=" operation with field elements.
impl<B: Extensible<2>> DivAssign for Fp2<B> {
    fn div_assign(&mut self, rhs: Self) {
        *self = *self / rhs
    }
}

impl From<Fp> for Fp2<Fp>{
    fn from(value: Fp) -> Self {
        Fp2::<Fp>::new(Fp::from(value), Fp::ZERO)
    }
}
//Conversion from double of field elements
impl<B:From<U>+Extensible<2>,U:Copy> From<[U; 2]> for Fp2<B> {
    fn from(value: [U; 2]) -> Self {
        Fp2::new(B::from(value[0]),B::from(value[1]))
    }
}

//Embeds 128-bit integer into the quadratic extension in the usual way.
impl<B: Extensible<2>> From<u128> for Fp2<B> {
    fn from(value: u128) -> Self {
        Fp2::new(B::from(value), B::ZERO)
    }
}

//Embeds 64-bit integer into the quadratic extension in the usual way.
impl<B: Extensible<2>> From<u64> for Fp2<B> {
    fn from(value: u64) -> Self {
        Fp2::new(B::from(value), B::ZERO)
    }
}

//Embeds 32-bit integer into the quadratic extension in the usual way.
impl<B: Extensible<2>> From<u32> for Fp2<B> {
    fn from(value: u32) -> Self {
        Fp2::new(B::from(value), B::ZERO)
    }
}

//Embeds 16-bit integer into the quadratic extension in the usual way.
impl<B: Extensible<2>> From<u16> for Fp2<B> {
    fn from(value: u16) -> Self {
        Fp2::new(B::from(value), B::ZERO)
    }
}

//Embeds 8-bit integer into the quadratic extension in the usual way.
impl<B: Extensible<2>> From<u8> for Fp2<B> {
    fn from(value: u8) -> Self {
        Fp2::new(B::from(value), B::ZERO)
    }
}

impl<B:Extensible<2>> From<[u64;6]> for Fp2<B> {
    fn from(_value: [u64;6]) -> Self {
        unimplemented!()
    }
}

impl<B:Extensible<2>> From<U256> for Fp2<B> {
    fn from(_value: U256) -> Self {
        unimplemented!()
    }
}


impl<'a, B: Extensible<2>> TryFrom<&'a [u8]> for Fp2<B> {
    type Error = DeserializationError;

    /// Converts a slice of bytes into a field element; returns error if the value encoded in bytes
    /// is not a valid field element. The bytes are assumed to be in little-endian byte order.
    fn try_from(bytes: &[u8]) -> Result<Self, Self::Error> {
        let element_bytes:usize = ((B::NUM_BITS)>>3).try_into().unwrap();
        if bytes.len() < element_bytes {
            return Err(DeserializationError::InvalidValue(format!(
                "not enough bytes for a full field element; expected {} bytes, but was {} bytes",
                element_bytes,
                bytes.len(),
            )));
        }
        if bytes.len() > element_bytes {
            return Err(DeserializationError::InvalidValue(format!(
                "too many bytes for a field element; expected {} bytes, but was {} bytes",
                element_bytes,
                bytes.len(),
            )));
        }
        let mut reader = SliceReader::new(bytes);
        Self::read_from(&mut reader)
    }
}

impl<B: Extensible<2>> Display for Fp2<B> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({},{})", self.c0, self.c1)
    }
}

impl<B: Extensible<2>> Serializable for Fp2<B> {
    fn write_into<W: ByteWriter>(&self, target: &mut W) {
        self.c0.write_into(target);
        self.c1.write_into(target);
    }
}

impl<B: Extensible<2>> Deserializable for Fp2<B> {
    fn read_from<R: ByteReader>(source: &mut R) -> Result<Self, DeserializationError> {
        let value0 = B::read_from(source)?;
        let value1 = B::read_from(source)?;
        Ok(Self::new(value0, value1))
    }
}

//Conversion of raw pointer of quadratic extension element to a byte slice i.e. &[u8]
impl<B: Extensible<2>> AsBytes for Fp2<B> {
    fn as_bytes(&self) -> &[u8] {
        let self_ptr: *const Fp2<B> = self;
        unsafe { slice::from_raw_parts(self_ptr as *const u8, B::NUM_BITS as usize>>2)}
    }
}

impl<B: Extensible<2>> Randomizable for Fp2<B> {
    const VALUE_SIZE: usize = ELEMENT_BYTES;
    fn from_random_bytes(bytes: &[u8]) -> Option<Self> {
        Fp2::try_from(bytes).ok()
    }
}

//Addition in quadratic extension is simply pointwise addition. We view the quadratic extension as a double of base-field elements.
fn add<B:Extensible<2>>(x: [B; 2], rhs: [B; 2]) -> [B; 2] {
    let a = x[0] + rhs[0];
    let b = x[1] + rhs[1];
    [a, b]
}

//Subtraction in quadratic extension is simply pointwise subtraction. We view the quadratic extension as a double of base-field elements.
fn sub<B:Extensible<2>>(x: [B; 2], rhs: [B; 2]) -> [B; 2] {
    let a = x[0] - rhs[0];
    let b = x[1] - rhs[1];
    [a, b]
}






