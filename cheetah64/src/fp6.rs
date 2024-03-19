//! This module implements arithmetic over the extension field Fp6,
//! defined with irreducible polynomial u^6 - 7.
extern crate core;
use std::{fmt::Display,
    convert::TryFrom,
    fmt::{self, Formatter},
    ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Neg, Sub, SubAssign},
    slice,
};
use crypto_bigint::U256;
#[cfg(feature = "serialize")]
use serde::de::Visitor;
#[cfg(feature = "serialize")]
use serde::{self, Deserialize, Deserializer, Serialize, Serializer};
use subtle::{Choice, ConditionallySelectable, ConstantTimeEq, CtOption};
use traits::traits::{Field, Extensible, PrimeField};
use core::{
    ByteReader, ByteWriter, Deserializable, DeserializationError, Serializable, SliceReader, AsBytes, Randomizable
};
pub const ELEMENT_BYTES: usize = (std::mem::size_of::<u64>())*6;
use crate::{fp::Fp, scalar::Scalar};
use crate::fp3::Fp3;

pub const BETA: u128 = crate::fp::GENERATOR.0 as u128;

/// It represents the field extension element
/// c5.u^5 + c4.u^4 + c3.u^3 + c2.u^2 + c1.u + c0
/// where u is a root of the polynomial defining
/// the sextic extension.
#[derive(Clone,Copy, Debug, PartialEq, Eq)]
pub struct Fp6<B: Extensible<6>>{
    pub c0: B,
    pub c1: B,
    pub c2: B,
    pub c3: B,
    pub c4: B,
    pub c5: B,
}

impl <B: Extensible<6>> Fp6<B> {
    /// Creates a new Fp6 element from a [u64; 6] value.
    /// The value is converted to canonical form by reducing
    /// each coordinate if necessary.
    pub fn new(value: [B; 6]) -> Self {
        Fp6 { c0: value[0], c1: value[1], c2: value[2], c3: value[3], c4: value[4], c5: value[5] }
    }

    /// The additive identity
    pub const fn zero() -> Self {
        Fp6 { c0: B::ZERO, c1: B::ZERO, c2: B::ZERO, c3: B::ZERO, c4: B::ZERO, c5: B::ZERO }
    }

    /// The multiplicative identity
    pub const fn one() -> Self {
        Fp6 { c0: B::ONE, c1: B::ZERO, c2: B::ZERO, c3: B::ZERO, c4: B::ZERO, c5: B::ZERO }
    }

    pub const ELEMENT_BYTES: usize = (std::mem::size_of::<u64>())*6;

    /// Makes the element canonical by reducing each coordinate by the modulus if needed
    pub fn make_canonical(&self) -> Self {
        Self {
            c0: B::from(self.c0),
            c1: B::from(self.c1),
            c2: B::from(self.c2),
            c3: B::from(self.c3),
            c4: B::from(self.c4),
            c5: B::from(self.c5),
        }
    }

    /// Returns whether or not this element is strictly lexicographically
    /// larger than its negation.
    pub fn lexicographically_largest( element: Fp6<Fp>) -> Choice {
        // If this element's c5 coefficient is lexicographically largest
        // then it is lexicographically largest.
        // In the event the c5 coefficient is zero and the c4 coefficient is
        // lexicographically largest, then this element is lexicographically
        // largest.
        // Otherwise, in the event both the c5 and c4 coefficients are zero
        // and the c3 coefficient is lexicographically largest, then this
        // element is lexicographically largest.
        // Otherwise, in the event both the c5, c4 and c3 coefficients are
        // zero and the c2 coefficient is lexicographically largest, then
        // this element is lexicographically largest.
        // Otherwise, in the event both the c5, c4, c3 and c2 coefficients
        // are zero and the c1 coefficient is lexicographically largest,
        // then this element is lexicographically largest.
        // Otherwise, in the event both the c5, c4, c3, c2 and c1 coefficients
        // are zero and the c0 coefficient is lexicographically largest,
        // then this element is lexicographically largest.

        element.c5.lexicographically_largest()
            | (Choice::from(element.c5.is_zero() as u8) & element.c4.lexicographically_largest())
            | (Choice::from(element.c4.is_zero() as u8) & element.c3.lexicographically_largest())
            | (Choice::from(element.c3.is_zero() as u8) & element.c2.lexicographically_largest())
            | (Choice::from(element.c2.is_zero() as u8)& element.c1.lexicographically_largest())
            | (Choice::from(element.c1.is_zero() as u8) & element.c0.lexicographically_largest())
    }

    /// Computes the multiplication of an Fp6 element with an Fp element
    pub fn mul_by_fp(&self, other: &B) -> Fp6<B> {
        let c0 = (self.c0)*(*other);
        let c1 = (self.c1)*(*other);
        let c2 = (self.c2)*(*other);
        let c3 = (self.c3)*(*other);
        let c4 = (self.c4)*(*other);
        let c5 = (self.c5)*(*other);

        Self { c0, c1, c2, c3, c4, c5, }
    }

    // multiply extension Fp6 element to the base Fp6 element
    pub fn mul_base(a: Self, b: B) -> Self {
        let b = Self::from(&[ b, B::ZERO, B::ZERO, B::ZERO, B::ZERO, B::ZERO, ]);
        Fp6::mul(a, b)
    }
    
    pub fn to_repr(&self) -> [u8; 48] {
        self.to_bytes()
    }

    /// Converts an `Fp6` element into a byte representation in
    /// little-endian byte order.
    pub fn to_bytes(&self) -> [u8; 48] {
        let mut bytes = [0u8; 48];

        bytes[0..8].copy_from_slice(&self.c0.to_bytes());
        bytes[8..16].copy_from_slice(&self.c1.to_bytes());
        bytes[16..24].copy_from_slice(&self.c2.to_bytes());
        bytes[24..32].copy_from_slice(&self.c3.to_bytes());
        bytes[32..40].copy_from_slice(&self.c4.to_bytes());
        bytes[40..48].copy_from_slice(&self.c5.to_bytes());
        bytes
    }

    /// Attempts to convert a little-endian byte representation of
    /// a scalar into a `Fp6` element, failing if the input is not canonical.
    pub fn from_bytes(bytes: &[u8; 48]) -> CtOption<Fp6<Fp>> {
        let mut array = [0u8; 8];

        array.copy_from_slice(&bytes[0..8]);
        let c0 = Fp::from_bytes(&array);

        array.copy_from_slice(&bytes[8..16]);
        let c1 = Fp::from_bytes(&array);

        array.copy_from_slice(&bytes[16..24]);
        let c2 = Fp::from_bytes(&array);

        array.copy_from_slice(&bytes[24..32]);
        let c3 = Fp::from_bytes(&array);

        array.copy_from_slice(&bytes[32..40]);
        let c4 = Fp::from_bytes(&array);

        array.copy_from_slice(&bytes[40..48]);
        let c5 = Fp::from_bytes(&array);

        let is_some =
            c0.is_some() & c1.is_some() & c2.is_some() & c3.is_some() & c4.is_some() & c5.is_some();

        CtOption::new(
            Fp6 {
                c0: c0.unwrap_or(Fp::ZERO),
                c1: c1.unwrap_or(Fp::ZERO),
                c2: c2.unwrap_or(Fp::ZERO),
                c3: c3.unwrap_or(Fp::ZERO),
                c4: c4.unwrap_or(Fp::ZERO),
                c5: c5.unwrap_or(Fp::ZERO),
            },
            is_some,
        )
    }

    pub fn elements_as_bytes(elements: &[Self]) -> &[u8] {
        unsafe {
            slice::from_raw_parts(
                elements.as_ptr() as *const u8,
                elements.len() * Self::ELEMENT_BYTES,
            )
        }
    }

    /// Converts a slice of bytes into a Fp6 element; returns error if the value encoded in bytes
    /// is not a valid Fp6 element. The bytes are assumed to be in little-endian byte order.
    pub fn from_random_bytes(bytes: &[u8]) -> Option<Self> {
        Self::try_from(bytes).ok()
    }

}

impl <B: Extensible<6>> Field for Fp6<B>{
    // Checks whether `self` is zero or not.
    fn is_zero(self)->bool {
        self==Fp6::ZERO
    }


    fn is_one(self) -> bool {
        self==Fp6::one()
    }

    //Returns the double of an Fp6 element.
    fn double(self)->Self {
        self+self
    }

    //Returns the triple of an Fp6 element.
    fn invert(self)->CtOption<Self>{
        let z= <B as Extensible<6>>::invert([self.c0, self.c1, self.c2, self.c3, self.c4, self.c5]);
        CtOption::new(Fp6{c0: z.unwrap()[0],c1: z.unwrap()[1],c2: z.unwrap()[2], c3: z.unwrap()[3], c4: z.unwrap()[4], c5: z.unwrap()[5]}, z.is_some())
    }

    //Returns the double of an Fp6 element.
    fn triple(self)->Self {
        self+self+self
    }

    const ZERO:Self= Self::zero();

    const ONE:Self = Self::one();

    fn random()->Self{
        let x0=B::random();
        let x1=B::random();
        let x2=B::random();
        let x3=B::random();
        let x4=B::random();
        let x5=B::random();
        Self{c0: x0, c1: x1, c2: x2, c3: x3, c4: x4, c5: x5}
    }

    fn sqrt(self)->CtOption<Self> {
        let z= <B as Extensible<6>>::sqrt([self.c0, self.c1, self.c2, self.c3, self.c4, self.c5]);
        if bool::from(z.is_some()){
        CtOption::new(Fp6::new(z.unwrap()), z.is_some())
        }
        else {
            CtOption::new(Fp6::new([B::ZERO;6]), z.is_some())
        }
    }


    fn power_by<S:AsRef<[u64]>>(self,exp:S)->Self {
        let mut res = Self::ONE;
        for e in exp.as_ref().iter().rev() {
            for i in (0..64).rev() {
                res = res.square();
                if ((*e >> i) & 1) == 1 {
                    res = res * self;
                }
            }
        }
        res
    }
   
    fn to_curve_bytes(&self) -> &[u8] {
        self.as_bytes()
    }

    // used for monotegmey reduction in signatures
    fn to_words(&self) -> Vec<u64> {
     let mut  a = Vec::new();
        a.push(self.c0.to_words()[0]);
        a.push(self.c1.to_words()[0]);
        a.push(self.c2.to_words()[0]);
        a.push(self.c3.to_words()[0]);
        a.push(self.c4.to_words()[0]);
        a.push(self.c5.to_words()[0]);
       Scalar::montgomery_reduce(a[0], a[1], a[2], a[3], a[4], a[5], 0, 0).to_words()
    }
    fn square(self) -> Fp6<B> {
    let z= <B as Extensible<6>>::square([self.c0, self.c1, self.c2, self.c3, self.c4, self.c5]);
    Fp6{c0: z[0], c1: z[1], c2: z[2], c3: z[3], c4: z[4], c5: z[5]}

    }
    type BaseField= Fp6<B>;
    // element bytes
    const ELEMENT_BYTES: usize = ELEMENT_BYTES;

    fn cube(self) -> Self{
        self*(self.square())
    }

    const IS_CANONICAL: bool = true;

    fn  from_uint_reduced(_w: Fp6<B>) -> Self {
     unimplemented!()
    }

    fn from_words( a: &Vec<u64>) -> Self {
      Fp6 { c0: (a[0]).into(), c1: (a[1]).into(), c2: (a[2]).into(), c3: (a[3]).into(), c4: (a[4]).into(), c5: (a[5]).into() }
    }

    fn get_windows(&self, _exp: usize)->Vec<usize> {
     unimplemented!()
    }


}

impl <B: Extensible<6>> PrimeField for Fp6<B>{
    type Repr= [u8; 32];

    /// Checks whether this element is odd or not
    fn is_odd(self)->Choice {
        self.c0.is_odd()
    }

    fn get_root_of_unity(_n: u32)->Self {
        unimplemented!()
    }

    const MODULUS:&'static str= "0xffffffff00000001";

    const NUM_BITS:u32= 64;

    const GENERATOR:Self= unimplemented!();

    const TWO_ADDICITY: u32= 32;

    const TWO_ADIC_ROOT: &'static str= unimplemented!();

    fn is_even(self)->Choice{
        !self.is_odd()
    }

}

impl <B: Extensible<6>> From<[u64; 6]> for Fp6<B>{
    fn from(_value: [u64; 6]) -> Self {
        unimplemented!()
    }
}

//Creates a Fp6 element from Fp element.
impl <B: Extensible<6>> From<&B> for Fp6<B> {
    fn from(f: &B) -> Self {
        Fp6 { c0: *f, c1: B::ZERO, c2: B::ZERO, c3: B::ZERO, c4: B::ZERO, c5: B::ZERO }
    }
}

//Creates a Fp6 element from Fp3 element.
impl <B: Extensible<6>+Extensible<3>> From<Fp3<B>> for Fp6<B> {
    fn from(f: Fp3<B>) -> Self {
        Fp6 { c0: f.a0, c1: B::ZERO, c2: f.a1, c3: B::ZERO, c4: f.a2, c5: B::ZERO }
    }
}

//Creates a Fp6 element from an array of six Fp element.
impl <B: Extensible<6>> From<&[B; 6]> for Fp6<B> {
    fn from(f: &[B; 6]) -> Self {
        Fp6 { c0: f[0], c1: f[1], c2: f[2], c3: f[3], c4: f[4], c5: f[5] }
    }
}

//Creates an array of 6 Fp elements from Fp6 element.
impl <B: Extensible<6>> From<Fp6<B>> for [B; 6] {
    fn from(f: Fp6<B>) -> [B; 6] {
        [f.c0,f.c1,f.c2,f.c3,f.c4,f.c5]
    }
}

//Creates an array of 6 Fp elements from Fp6 element(using referencing).
impl <B: Extensible<6>> From<&Fp6<B>> for [B; 6] {
    fn from(f: &Fp6<B>) -> [B; 6] {
        [f.c0,f.c1,f.c2,f.c3,f.c4,f.c5]
    }
}

//Creates a Fp6 element using u128 value after modular reduction.
impl <B: Extensible<6>> From<u128> for Fp6<B>{
    fn from(value: u128) -> Self {
        Fp6{c0: B::from(value),c1: B::ZERO,c2: B::ZERO,c3: B::ZERO, c4: B::ZERO, c5: B::ZERO}
    }
}

//Creates a Fp6 element using u64 value after modular reduction.
impl <B: Extensible<6>> From<u64> for Fp6<B> {
    /// Converts a 64-bit value into a Fp6 element. If the value is greater than or equal to
    /// the field modulus, modular reduction is silently performed.
    fn from(value: u64) -> Self {
        Fp6{c0: B::from(value),c1: B::ZERO,c2: B::ZERO,c3: B::ZERO, c4: B::ZERO, c5: B::ZERO}
        }
}

//Creates a Fp6 element using u32 value after modular reduction.
impl <B: Extensible<6>> From<u32> for Fp6<B> {
    /// Converts a 32-bit value into a Fp6 element.
    fn from(value: u32) -> Self {
        Fp6{c0: B::from(value),c1: B::ZERO,c2: B::ZERO,c3: B::ZERO, c4: B::ZERO, c5: B::ZERO}
    }
}

//Creates a Fp6 element using u16 value after modular reduction.
impl <B: Extensible<6>> From<u16> for Fp6<B> {
    /// Converts a 16-bit value into a Fp6 element.
    fn from(value: u16) -> Self {
        Fp6{c0: B::from(value),c1: B::ZERO,c2: B::ZERO,c3: B::ZERO, c4: B::ZERO, c5: B::ZERO}
    }
}

//Creates a Fp6 element using u8 value after modular reduction.
impl <B: Extensible<6>> From<u8> for Fp6<B> {
    /// Converts an 8-bit value into a Fp6 element.
    fn from(value: u8) -> Self {
        Fp6{c0: B::from(value),c1: B::ZERO,c2: B::ZERO,c3: B::ZERO, c4: B::ZERO, c5: B::ZERO}
     }
}

//Creates a Fp6 element using u8 value after modular reduction.
impl <B: Extensible<6>> From<U256> for Fp6<B> {
    /// Converts an 8-bit value into a Fp6 element.
    fn from(_value: U256) -> Self {
       unimplemented!()
     }
}



///========Trait bounds========
/// 
impl <B: Extensible<6>> Display for Fp6<B> {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl <B: Extensible<6>> Default for Fp6<B> {
    fn default() -> Self {
        Self::ZERO
    }
}

impl <B: Extensible<6>> Add for Fp6<B>{
    type Output=Fp6<B>;
    fn add(self,rhs: Self) -> Self::Output {
        add(&self,&rhs)
    }
}    

impl <B: Extensible<6>> AddAssign<Fp6<B>> for Fp6<B>{
    fn add_assign(&mut self, other: Self){
        *self=add(self, &other);
    }
}

impl <B: Extensible<6>> Sub for Fp6<B>{
    type Output = Fp6<B>;
    fn sub(self, rhs: Self) -> Self::Output {
        sub(&self,&rhs)
    }
}

impl <B: Extensible<6>> SubAssign for Fp6<B>{
    fn sub_assign(&mut self, other: Self){
        *self=sub(self,&other);
    }
}

impl <B: Extensible<6>> Mul<Fp6<B>> for Fp6<B>{
    type Output = Self;
    fn mul(self, rhs: Self) -> Self::Output {
        let result= <B as Extensible<6>>::mul([self.c0, self.c1, self.c2, self.c3, self.c4, self.c5], [rhs.c0, rhs.c1, rhs.c2, rhs.c3, rhs.c4, rhs.c5]);
        Self{c0: result[0], c1: result[1], c2:  result[2], c3: result[3], c4: result[4], c5: result[5] }
    }
}

impl <B: Extensible<6>> MulAssign<Fp6<B>> for Fp6<B>{
    fn mul_assign(&mut self, rhs: Self){
        *self= *self*rhs 
    }
}

impl <B: Extensible<6>> Div for Fp6<B>{
    type Output=Fp6<B>;
    fn div(self, rhs: Fp6<B>) -> Self::Output {
        div(&self,&rhs)
    }
}
impl <B: Extensible<6>> DivAssign for Fp6<B>{
    fn div_assign(&mut self, rhs: Self){
     *self=div(self, &rhs); 
    }
}

impl <B: Extensible<6>> Neg for Fp6<B>{
    type Output=Fp6<B>;
    fn neg(self) -> Self::Output {
        neg(&self)
    }
}

impl <B: Extensible<6>+ConstantTimeEq> ConstantTimeEq for Fp6<B> {
    fn ct_eq(&self, other: &Self) -> Choice {
              self.c0.ct_eq(&other.c0)
            & self.c1.ct_eq(&other.c1)
            & self.c2.ct_eq(&other.c2)
            & self.c3.ct_eq(&other.c3)
            & self.c4.ct_eq(&other.c4)
            & self.c5.ct_eq(&other.c5)
    }
}


impl <B: Extensible<6>+ConditionallySelectable> ConditionallySelectable for Fp6<B> {
    fn conditional_select(a: &Self, b: &Self, choice: Choice) -> Self {
        Self {
            c0: B::conditional_select(&a.c0, &b.c0, choice),
            c1: B::conditional_select(&a.c1, &b.c1, choice),
            c2: B::conditional_select(&a.c2, &b.c2, choice),
            c3: B::conditional_select(&a.c3, &b.c3, choice),
            c4: B::conditional_select(&a.c4, &b.c4, choice),
            c5: B::conditional_select(&a.c5, &b.c5, choice),
        }
    }
}

impl <B: Extensible<6>> Serializable for Fp6<B> {
    fn write_into<W: ByteWriter>(&self, target: &mut W) {
        target.write_u8_slice(&self.c0.to_bytes());
        target.write_u8_slice(&self.c1.to_bytes());
        target.write_u8_slice(&self.c2.to_bytes());
        target.write_u8_slice(&self.c3.to_bytes());
        target.write_u8_slice(&self.c4.to_bytes());
        target.write_u8_slice(&self.c5.to_bytes());
    
    }
}

impl <B: Extensible<6>> Deserializable for Fp6<B> {
    fn read_from<R: ByteReader>(source: &mut R) -> Result<Self, DeserializationError> {
        let value0 = B::read_from(source)?;
        let value1 = B::read_from(source)?;
        let value2 = B::read_from(source)?;
        let value3 = B::read_from(source)?;
        let value4 = B::read_from(source)?;
        let value5 = B::read_from(source)?;
        Ok(Self {
            c0: value0,
            c1: value1,
            c2: value2,
            c3: value3,
            c4: value4,
            c5: value5,
        })
    }
}

impl<'a,B: Extensible<6>> TryFrom<&'a [u8]> for Fp6<B> {
    type Error = DeserializationError;

    /// Converts a slice of bytes into a Fp6 element; returns error if the value encoded in bytes
    /// is not a valid Fp6 element. The bytes are assumed to be in little-endian byte order.
    fn try_from(bytes: &[u8]) -> Result<Self, Self::Error> {
        if bytes.len() < Self::ELEMENT_BYTES {
            return Err(DeserializationError::InvalidValue(format!(
                "not enough bytes for a full Fp6 element; expected {} bytes, but was {} bytes",
                Self::ELEMENT_BYTES,
                bytes.len(),
            )));
        }
        if bytes.len() > Self::ELEMENT_BYTES {
            return Err(DeserializationError::InvalidValue(format!(
                "too many bytes for a Fp6 element; expected {} bytes, but was {} bytes",
                Self::ELEMENT_BYTES,
                bytes.len(),
            )));
        }
        let mut reader = SliceReader::new(bytes);
        Self::read_from(&mut reader)
    }
}

impl <B: Extensible<6>> PartialOrd for Fp6<B> {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl <B: Extensible<6>> Ord for Fp6<B> {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.to_bytes().cmp(&other.to_bytes())
    }
}

impl <B: Extensible<6>> AsBytes for Fp6<B>{
    fn as_bytes(&self) -> &[u8] {
        let ptr: *const Fp6<B>= self;
       unsafe{slice::from_raw_parts(ptr as *const u8, ELEMENT_BYTES)}
    } 
}

impl <B: Extensible<6>> Randomizable for Fp6<B> {
    const VALUE_SIZE: usize = ELEMENT_BYTES;
    fn from_random_bytes(bytes: &[u8]) -> Option<Self> {
        Self::try_from(bytes).ok()
    }
}

//Returns the summation of two Fp6 elements handling overflow.
//Used to give definition of trait bound Sum for Fp6.
fn add<B: Extensible<6>>(u: &Fp6<B>,v: &Fp6<B>)->Fp6<B>{
    let c0=u.c0+ v.c0;
    let c1=u.c1+v.c1;
    let c2=u.c2+v.c2;
    let c3=u.c3+v.c3;
    let c4=u.c4+v.c4;
    let c5=u.c5+v.c5;
    Fp6{c0,c1,c2,c3,c4,c5}
}

// Returns the Subtraction of b from a i.e, a-b, handles underflow.
//Used to give definition of trait bound Sub for Fp6.
fn sub<B: Extensible<6>> (u: &Fp6<B>,v: &Fp6<B>)->Fp6<B>{
    let c0=u.c0-v.c0;
    let c1=u.c1-v.c1;
    let c2=u.c2-v.c2;
    let c3=u.c3-v.c3;
    let c4=u.c4-v.c4;
    let c5=u.c5-v.c5;
    Fp6{c0,c1,c2,c3,c4,c5}
}


// Computes a/b or a*(b^{-1}) in Fp6,
//Used to give definition of trait bound Div for Fp6
fn div<B: Extensible<6>>(a:&Fp6<B>,b:&Fp6<B>)->Fp6<B>{
    let rhs:Fp6<B>=(*b).invert().unwrap();
    
    (*a).mul(rhs)
}

// Computes -a for given a in Fp6,
//Used to give definition of trait bound Neg for Fp6
fn neg<B: Extensible<6>>(a: &Fp6<B>) -> Fp6<B> {
    Fp6::ZERO-(*a)
}


//implement field trait for fp6
// ================================================================================================

#[cfg(feature = "serialize")]
impl Serialize for Fp6 {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        use serde::ser::SerializeTuple;
        let mut tup = serializer.serialize_tuple(48)?;
        for byte in self.to_bytes().iter() {
            tup.serialize_element(byte)?;
        }
        tup.end()
    }
}


#[cfg(feature = "serialize")]
impl<'de> Deserialize<'de> for Fp6 {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct Fp6Visitor;

        impl<'de> Visitor<'de> for Fp6Visitor {
            type Value = Fp6;

            fn expecting(&self, formatter: &mut Formatter) -> fmt::Result {
                formatter.write_str("a valid field element")
            }

            fn visit_seq<A>(self, mut seq: A) -> Result<Fp6, A::Error>
            where
                A: serde::de::SeqAccess<'de>,
            {
                let mut bytes = [0u8; 48];
                for (i, byte) in bytes.iter_mut().enumerate() {
                    *byte = seq
                        .next_element()?
                        .ok_or_else(|| serde::de::Error::invalid_length(i, &"expected 48 bytes"))?;
                }
                let elem = Fp6::from_bytes(&bytes);
                if bool::from(elem.is_none()) {
                    Err(serde::de::Error::custom("decompression failed"))
                } else {
                    Ok(elem.unwrap())
                }
            }
        }

        deserializer.deserialize_tuple(48, Fp6Visitor)
    }
}
