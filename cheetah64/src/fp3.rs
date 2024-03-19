extern crate core;
use crate::{fp6::Fp6, fp::Fp};

use std::{fmt::{self,Display, Formatter},slice};
use std::ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Neg, Sub, SubAssign};
use crypto_bigint::U256;
use subtle::{Choice, ConditionallySelectable, ConstantTimeEq, CtOption};
use traits::traits::{Field, Extensible, PrimeField};
use core::{ByteReader, ByteWriter, Deserializable, DeserializationError, Serializable, AsBytes, Randomizable, SliceReader};

pub const BETA: u128 = crate::fp::GENERATOR.0 as u128;

const ELEMENT_BYTES: usize = std::mem::size_of::<u64>()*3;

pub(crate) const TWO_ADIC_ROOT_OF_UNITY_FP3: Fp3<Fp> = Fp3 {
    a0: Fp(2800184025912956819),
    a1: Fp::zero(),
    a2: Fp::zero(),
};

//Elements in Fp3 are like a_0+(a_1)x+(a_2)x^2.
#[derive(Clone,Copy,Debug,Default,PartialEq,Eq)]
pub struct Fp3<B: Extensible<3>> {
    pub a0: B,
    pub a1: B,
    pub a2: B
}

impl <B: Extensible<3>> Fp3<B>{
    //Generates a new Fp3 element(within the range).
    pub fn new(value: [B; 3])->Self{
        Self{a0: B::from(value[0]),a1: B::from(value[1]),a2: B::from(value[2])}
    }

    //returns zero, the additive identity.
    pub const fn zero() -> Self {
        Self{
            a0: B::ZERO,
            a1: B::ZERO,
            a2: B::ZERO,
        }
    }
    //pub const VALUE:u64 =2800184025912956819; 

    pub const ELEMENT_BYTES: usize = std::mem::size_of::<u64>()*3;
    
    // Returns one, the multiplicative identity.
    pub const fn one() -> Self {
        Self{
            a0: B::ONE,
            a1: B::ZERO,
            a2: B::ZERO,
        }
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

impl <B: Extensible<3>> Field for Fp3<B>{
    //Geneartes a random element.
    fn random() -> Self {
        Fp3{a0: B::random(),a1:B::random() ,a2: B::random()}
    }

    // Computes the square of a Fp3 element.
    fn square(self) -> Fp3<B> {
        let z= <B as Extensible<3>>::square([self.a0, self.a1, self.a2]);
        Fp3{a0: z[0], a1: z[1], a2: z[2]}
    }

    ///Unsupproted
    // Computes the multiplicative inverse of Fp3 element,
    // panics if the element is zero. Used to define Common field trait  for Fp
    fn invert(self)->CtOption<Self> {
        unimplemented!()
    }

    /// Computes the square root of this element, if it exists.
    fn sqrt(self) -> CtOption<Self> {
        let z= <B as Extensible<3>>::sqrt([self.a0, self.a1, self.a2]);
        if bool::from(z.is_some()){
        CtOption::new(Fp3::new(z.unwrap()), z.is_some())
        }
        else {
            CtOption::new(Fp3::new([B::ZERO;3]), z.is_some())
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

    // Computes the double of a Fp3 element.
    fn double(self)->Self {
        self+(self)
    }

    // Computes the triple of a Fp3 element.
    fn triple(self)->Self {
        self+self+self
    }

    // Checks whether `self` is zero or not.
    fn is_zero(self)->bool {
        self==Fp3::zero()
    }

    fn is_one(self) -> bool {
        self==Fp3::one()
    }
    
    fn to_curve_bytes(&self) -> &[u8] {
        self.as_bytes()
    }

    fn to_words(&self) -> Vec<u64> {
        unimplemented!()
    }
    const ZERO:Self= Self::zero();

    const ONE:Self = Self::one();

    type BaseField= Fp3<B>;

    // element bytes
    const ELEMENT_BYTES: usize = ELEMENT_BYTES;

    fn cube(self) -> Self{
        self*(self.square())
    }

    const IS_CANONICAL: bool = true;

    fn  from_uint_reduced(_w: Fp3<B>) -> Self {
     unimplemented!()
    }

    fn from_words( _a: &Vec<u64>) -> Self {
     unimplemented!()
    }

    fn get_windows(&self, _exp: usize)->Vec<usize> {
     unimplemented!()
    }

 
}

impl <B: Extensible<3>> PrimeField for Fp3<B>{
    type Repr= [u8; 24];

    fn is_odd(self)->Choice {
        self.a0.is_odd()
    }

    const MODULUS: &'static str= B::MODULUS;

    const NUM_BITS: u32= B::NUM_BITS*3;

    const GENERATOR: Self= unimplemented!();

    const TWO_ADDICITY: u32= B::TWO_ADDICITY;

    const TWO_ADIC_ROOT: &'static str= unimplemented!();
   

    fn get_root_of_unity(n: u32) -> Self {
       assert!(n==0 || n<=32, "2^{n}th root does not exist");
       Fp3::<B>::from(TWO_ADIC_ROOT_OF_UNITY_FP3.a0.0).power_by([1<<(32-n)])
    }

    fn is_even(self)->Choice{
        !self.is_odd()
    }


}

impl <B: Extensible<3>> From<[u64; 6]> for Fp3<B>{
    fn from(value: [u64; 6]) -> Self {
        Self {
            a0: B::from(value),
            a1: B::ZERO,
            a2: B::ZERO
        }
    }
}

impl <B: Extensible<3>> From<U256> for Fp3<B>{
    fn from(value: U256) -> Self {
        Self {
            a0: B::from(value),
            a1: B::ZERO,
            a2: B::ZERO
        }
    }
}





//Creates a Fp3 element using u128 value after modular reduction.
impl <B: Extensible<3>> From<u128> for Fp3<B>{
    fn from(value: u128) -> Self {
        Self{a0: B::from(value),a2: B::ZERO,a1: B::ZERO}
    }
}

//Creates a Fp3 element using u64 value after modular reduction.
impl <B: Extensible<3>> From<u64> for Fp3<B>{
    fn from(value: u64) -> Self {
        Self{a0: B::from(value),a2: B::ZERO,a1: B::ZERO}
    }
}

//Creates a Fp3 element using u32 value after modular reduction.
impl <B: Extensible<3>> From<u32> for Fp3<B>{
    fn from(value: u32) -> Self {
        Self{a0: B::from(value),a2: B::ZERO,a1: B::ZERO}
    }
}

//Creates a Fp3 element using u16 value after modular reduction.
impl <B: Extensible<3>> From<u16> for Fp3<B>{
    fn from(value: u16) -> Self {
        Self{a0: B::from(value),a2: B::ZERO,a1: B::ZERO}
    }
}

//Creates a Fp3 element using u8 value after modular reduction.
impl <B: Extensible<3>> From<u8> for Fp3<B>{
    fn from(value: u8) -> Self {
        Self{a0: B::from(value),a2: B::ZERO,a1: B::ZERO}
    }
}

// When looking at the sextic extension as a towered one,
// i.e. Fp6 = Fp3[Y]/(Y^2 − γ) with γ = δ = 7, we use the
// lowest coefficient of the quadratic extension for
// conversion from Fp6 to Fp3 elements.
impl <B: Extensible<3>+Extensible<6>> From<&Fp6<B>> for Fp3<B> {
    fn from(f: &Fp6<B>) -> Self {
        Self {
            a0: f.c0,
            a1: f.c2,
            a2: f.c4,
        }
    }
}

//Creates a Fp3 element from Fp element.
impl <B: Extensible<3>> From<&B> for Fp3<B>{
    fn from(value: &B) -> Self {
        Fp3{a0: *value,a2: B::ZERO,a1: B::ZERO}
    }
}

///======Trait bounds=======

impl <B: Extensible<3>> Display for Fp3<B> {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl <B: Extensible<3>+ConstantTimeEq> ConstantTimeEq for Fp3<B> {
    fn ct_eq(&self, other: &Self) -> Choice {
        self.a0.ct_eq(&other.a0) & self.a1.ct_eq(&other.a1) & self.a2.ct_eq(&other.a2)
    }
}

impl <B: Extensible<3>+ConditionallySelectable> ConditionallySelectable for Fp3<B> {
    fn conditional_select(a: &Self, b: &Self, choice: Choice) -> Self {
        Self {
            a0: B::conditional_select(&a.a0, &b.a0, choice),
            a1: B::conditional_select(&a.a1, &b.a1, choice),
            a2: B::conditional_select(&a.a2, &b.a2, choice),
        }
    }
}

impl <B: Extensible<3>> Add for Fp3<B>{
    type Output=Self;
    fn add(self,rhs: Self) -> Self {
        add(&self,&rhs)
    }
}  

impl <B: Extensible<3>> Sub for Fp3<B>{
    type Output = Self;
    fn sub(self, rhs: Self) -> Self::Output {
        sub(&self,&rhs)
    }
}

impl <B: Extensible<3>> Mul for Fp3<B>{
    type Output = Self;
    fn mul(self, rhs: Self) -> Self::Output {
        let result= <B as Extensible<3>>::mul([self.a0, self.a1, self.a2], [rhs.a0, rhs.a1, rhs.a2]);
        Self{a0: result[0],a1: result[1],a2:  result[2]}
    }
}

impl <B: Extensible<3>> AddAssign<Fp3<B>> for Fp3<B>{
    fn add_assign(&mut self, other: Self){
        *self=add(self, &other);
    }
}

impl <B: Extensible<3>> SubAssign<Fp3<B>> for Fp3<B>{
    fn sub_assign(&mut self, other: Self){
        *self=sub(self,&other);
    }
}

impl <B: Extensible<3>> MulAssign<Fp3<B>> for Fp3<B>{
    fn mul_assign(&mut self, rhs: Self){
        *self= *self*rhs
    }
}

//Unsupported
impl <B: Extensible<3>> Div for Fp3<B>{
    type Output=Fp3<B>;
    fn div(self, _rhs: Fp3<B>) -> Self::Output {
       unimplemented!()
    }
}

//Unsupported
impl <B: Extensible<3>> DivAssign for Fp3<B>{
    fn div_assign(&mut self, _rhs: Self){
        unimplemented!()
    }
}

impl <B: Extensible<3>> Neg for Fp3<B>{
    type Output=Fp3<B>;
    fn neg(self) -> Self::Output {
        neg(&self)
    }
}



impl <B: Extensible<3>> PartialOrd for Fp3<B> {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}
impl<B: Extensible<3>> Ord for Fp3<B> {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.cmp(&other)
    }
}


impl<'a,B: Extensible<3>> TryFrom<&'a [u8]> for Fp3<B> {
    type Error = DeserializationError;

    /// Converts a slice of bytes into a Fp3 element; returns error if the value encoded in bytes
    /// is not a valid Fp3 element. The bytes are assumed to be in little-endian byte order.
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

impl <B: Extensible<3>> AsBytes for Fp3<B>{
    fn as_bytes(&self) -> &[u8] {
        let ptr: *const Fp3<B>= self;
       unsafe{slice::from_raw_parts(ptr as *const u8, ELEMENT_BYTES)}
    } 
}

impl <B: Extensible<3>> Randomizable for Fp3<B> {
    const VALUE_SIZE: usize = ELEMENT_BYTES;
    fn from_random_bytes(bytes: &[u8]) -> Option<Self> {
        Self::try_from(bytes).ok()
    }
}

impl <B: Extensible<3>> Serializable for Fp3<B> {
    fn write_into<W: ByteWriter>(&self, target: &mut W) {
        target.write_u8_slice(&self.a0.to_bytes());
        target.write_u8_slice(&self.a1.to_bytes());
        target.write_u8_slice(&self.a2.to_bytes());
    }
}

impl <B: Extensible<3>> Deserializable for Fp3<B> {
    fn read_from<R: ByteReader>(source: &mut R) -> Result<Self, DeserializationError> {
        let value0 = B::read_from(source)?;
        let value1 = B::read_from(source)?;
        let value2 = B::read_from(source)?;
        Ok(Fp3{a0: value0,a1: value1,a2: value2})
    }
}

//Global functions: add, sub, mul, square, div, neg

//Returns the summation of two Fp3 elements handling overflow.
//Used to give definition of trait bound Sum for Fp3.
fn add<B: Extensible<3>>(u: &Fp3<B>,v: &Fp3<B>)->Fp3<B>{
    let a0=u.a0+v.a0;
    let a1=u.a1+v.a1;
    let a2=u.a2+v.a2;

    Fp3{a0,a1,a2}
}

// Returns the Subtraction of b from a i.e, a-b, handles underflow.
//Used to give definition of trait bound Sub for Fp3.
fn sub<B: Extensible<3>>(u: &Fp3<B>,v: &Fp3<B>)->Fp3<B>{
    let a0=u.a0-v.a0;
    let a1=u.a1-v.a1;
    let a2=u.a2-v.a2;

    Fp3{a0,a1,a2}
}

// Computes -a for given a in Fp3,
//Used to give definition of trait bound Neg for Fp3
fn neg<B: Extensible<3>>(a: &Fp3<B>) -> Fp3<B> {
    (Fp3::zero()).sub(*a)
}



