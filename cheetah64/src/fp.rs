use std::fmt::{Display, Formatter};
use std::hash::Hasher;
use std::convert::TryFrom;
use std::ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Neg, Sub, SubAssign};
use std::fmt::{self, Debug};
extern crate core;
use core::{ByteReader, ByteWriter, Deserializable, DeserializationError, Serializable, AsBytes, Randomizable};
use bitvec::macros::internal::core::slice;
use crypto_bigint::{U64, U256};
use crypto_bigint::generic_array::GenericArray;
use rand::Rng;
use traits::traits::{Field, PrimeField, Extensible};
use std::hash::Hash;
use crypto_bigint::subtle::{ConstantTimeEq, Choice,ConditionallySelectable,CtOption};
#[allow(unused_imports)]
use rand::RngCore;
#[cfg(feature = "serialize")]
use serde::de::Visitor;
#[cfg(feature = "serialize")]
use serde::{self, Deserialize, Deserializer, Serialize, Serializer};
extern crate utilities;
use utilities::{shl64_by_u32_with_carry,sub64_with_carry};

use crate::fp3::{BETA, Fp3, self, };
use crate::fp6::Fp6;

// Field modulus = 2^64 - 2^32 + 1
pub const M: Fp = Fp(0xffffffff00000001);
//0xffffffff00000001=18446744069414584321

pub const MODULUS: u64 = 18446744069414584321;
// Multiplicative generator g of order p-1
// g = 7
pub const GENERATOR: Fp = Fp(7);

// Epsilon = 2^32 - 1;
const E: u64 = 0xffffffff;

// Two-adicity of the field: (p-1) % 2^32 = 0
pub(crate) const TWO_ADICITY: u32 = 32;

// 2^32 root of unity = 1753635133440165772,generator of mult. subgrp of order 2^32
pub(crate)const TWO_ADIC_ROOT_OF_UNITY: Fp = Fp(1753635133440165772);

// Size of field elements of this elliptic curve.
pub type FieldSize = <U64 as crypto_bigint::ArrayEncoding>::ByteSize;

/// Byte representation of a base field element of a given curve.
pub type FieldBytes = GenericArray<u8, FieldSize>;

//To be used in AsBytes
pub(crate) const ELEMENT_BYTES: usize = std::mem::size_of::<u64>();

#[derive(Clone,Copy,Debug,Default,Eq)]
pub struct Fp(pub u64);

impl Fp{  
    //returns zero, the additive identity.
    pub const fn zero() -> Self {
        Self(0)
    }

    // Returns one, the multiplicative identity.
    pub const fn one()->Self{
       Self(1)
    }

    //Generates a new field element(within the range).
    pub const fn new(value: u64) -> Self {
        Self(value % M.0)// what is .0 here
    }

    //Computes 2^(num_times) power of the field element. 
    pub fn square_assign_multi(&mut self,num_times: usize)->Self {
        for _ in 0..num_times {
            *self = self.square();
        }
        *self
    }
    
    //Makes an element canonical by reducing modulo prime p
    pub const fn make_canonical(&self) -> Self {
        Self(self.0 % M.0)
    }
    //
    pub fn mul_by_u32(&self, rhs: u32) -> Self {
        let r0 = (self.0 as u128) * (rhs as u128);
    
        Self(reduce_u96(r0))
    }
    //
    pub fn to_repr(&self) -> FieldBytes {
        self.to_bytes().into()
    }
    //
    pub fn elements_as_bytes(elements: &[Self]) -> &[u8] {
        unsafe {
            slice::from_raw_parts(
                elements.as_ptr() as *const u8,
                elements.len() * Self::ELEMENT_BYTES,
            )
        }
    }
    

    /// Returns whether or not this element is strictly lexicographically
    /// larger than its negation.
    pub fn lexicographically_largest(&self) -> Choice {
    // This can be determined by checking to see if the element is
    // larger than (p - 1) // 2. If we subtract by ((p - 1) // 2) + 1
    // and there is no underflow, then the element must be larger than
    // (p - 1) // 2.

    // First, because self may not be canonical, we need to make_canonical it
    let tmp = self.make_canonical();

    // (p-1) // 2 + 1 = 0x7fffffff80000001
    let (_, borrow) = sub64_with_carry(tmp.0, 0x7fffffff80000001, 0);

    // If the element was smaller, the subtraction will underflow
    // producing a borrow value of 0xffff...ffff, otherwise it will
    // be zero. We create a Choice representing true if there was
    // overflow (and so this element is not lexicographically larger
    // than its negation) and then negate it.

    !Choice::from((borrow as u8) & 1)
    }

     /// Attempts to convert a little-endian byte representation of
    /// a scalar into a `Fp` element, failing if the input is not canonical.
    pub fn from_bytes(bytes: &[u8; 8]) -> CtOption<Self> {
        let mut tmp = Fp(u64::from_le_bytes(*bytes));

        // Try to subtract the modulus M
        let (_, borrow) = sub64_with_carry(tmp.0, M.0, 0);

        // If the element is smaller than M then the
        // subtraction will underflow, producing a borrow value
        // of 0xffff...ffff. Otherwise, it'll be zero.
        let is_some = (borrow as u8) & 1;

        // Convert to canonical form
        tmp = tmp.make_canonical();

        CtOption::new(tmp, Choice::from(is_some))
    }

    // Converts an `Fp` element into a byte representation in
    /// little-endian byte order.
    fn to_bytes(&self) -> [u8; 8] {
        // Turn into canonical form by removing modulus
        // if self is greater.
        let tmp = self.make_canonical();    
        tmp.0.to_le_bytes()
    }

    /// Exponentiates `self` by `power`, where `power` is a
    /// little-endian order integer exponent.
    ///
    /// **This operation is variable time with respect
    /// to the exponent.** If the exponent is fixed,
    /// this operation is effectively constant time.
    pub fn exp_vartime(&self, power: u64) -> Self {
        let mut res = Self::one();
        for i in (0..64).rev() {
            res = res.square();
    
            if ((power >> i) & 1) == 1 {
                res.mul_assign(*self);
            }
        }
        res
    }
}

impl Field for Fp{
    //Checks whether `self` is zero or not.
    fn is_zero(self)->bool{
        self==Fp::zero()
    }

    //Checks whether `self` is one or not.
    fn is_one(self) -> bool {
        self==Fp::one()
    }

    // Computes the double of a field element.
    fn double(self)->Self{
        let (d0,is_overflow)=shl64_by_u32_with_carry(self.0,1,0);
        let (d0,is_overflow)=d0.overflowing_add(E*(is_overflow as u64));
        Fp(d0+E*(is_overflow as u64))
    }
         
    const ZERO:Self=Self::zero();

    const ONE:Self=Self::one();

    // Computes the square of a field element.
    fn square(self) -> Self {
        mul(&self,&self)
    }

    // Computes the multiplicative inverse of this element,
    // failing if the element is zero.
    fn invert(self) -> CtOption<Self> {
        invert(&self)
    }
    
    // Computes the triple of a field element.
    fn triple(self) -> Self {
        let t = self.0 as u128;
        let t = t + (t << 1);

        Self(reduce_u96(t))
    }

    /// Computes the square root of this element, if it exists.
    fn sqrt(self) -> CtOption<Self> {
    // Tonelli-Shank's algorithm for q mod 16 = 1
    // See https://eprint.iacr.org/2020/1497.pdf, page 3 for a
    // constant time specification of the algorithm.

    // Compute the progenitor y of self
    // y = self^((t - 1) // 2)
    //   = self^0x7fffffff
    let y = self.power_by([0x7fffffff]);
    
    let mut s = self.mul(y);
    let mut t = s * y;
    
    let mut z = TWO_ADIC_ROOT_OF_UNITY;
    
    for k in (2..=TWO_ADICITY).rev() {
        let mut b = t;

        Self::square_assign_multi(&mut b, (k - 2) as usize);
    
        let new_s = s * z;
        s = Fp::conditional_select(&new_s, &s, b.ct_eq(&Fp::one()));
        z = z.square();
        let new_t = t * z;
        t = Fp::conditional_select(&new_t, &t, b.ct_eq(&Fp::one()));
    }

    CtOption::new(s, (s.square()).ct_eq(&self))
    }

    /// Generates a random canonical element
    fn random() -> Self {
    Self::new(rand::thread_rng().gen_range(0..(M.0)-1))
    }

    /// Exponentiates `self` by `pow`, where `pow` is a
    // little-endian order integer exponent.
    fn power_by<S: AsRef<[u64]>>(self, exp:S) -> Self {
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

    fn to_words(&self) -> Vec<u64> {
        vec![self.0]
    }
    type BaseField= Fp;

    const ELEMENT_BYTES: usize = ELEMENT_BYTES;

    fn  from_uint_reduced(_w: Fp) -> Self {
     unimplemented!()
    }

    fn from_words( a: &Vec<u64>) -> Self {
        Fp(a[0])
    }

    fn get_windows(&self, _exp: usize)->Vec<usize> {
     unimplemented!()
    }

    fn cube(self) -> Self {
        self * (self.square())
    }

    const IS_CANONICAL: bool = true;


}


impl PrimeField for Fp{
    type Repr= FieldBytes;

    //Chceks whether field element is odd or not.
    fn is_odd(self) -> Choice{
        let a=(self.0)%2;
        Fp(a).ct_eq(&Fp::ONE)
    }

    //Chceks whether field element is even or not.
    fn is_even(self) -> Choice{
        !self.is_odd()
    }

    // Outputs a `Fp` element of multiplicative order equals to 2^n
    fn get_root_of_unity(n: u32) -> Self {
        //given an elemnt of order 2^32=
        //TWO_ADIC_ROOT_OF_UNITY: Fp = Fp(1753635133440165772);
        assert!(n==0 || n<=32, "2^{n}th root does not exist");
        TWO_ADIC_ROOT_OF_UNITY.power_by([1<<(32-n)])
    }

    //Modulus of the Field
    const MODULUS: &'static str= "0xffffffff00000001";
   
    // No. of bits used to represent the field element
    const NUM_BITS: u32= 64;

    //Generator of multiplicative group of the field
    const GENERATOR: Fp= Fp(7);

    //Two addicity of the prime field
    const TWO_ADDICITY: u32= 32;

    //Two adic root of the field
    //const TWO_ADIC_ROOT: Self= Fp(1753635133440165772);
    const TWO_ADIC_ROOT: &'static str= "1753635133440165772";

}

impl Extensible<3> for Fp{

    /// Returns the product of `a` and `b` in the field extension of degrre 3 defined by this extension as an array.
    fn mul(a: [Self; 3], b: [Self; 3]) -> [Self; 3] {
        let t00 = (&a[0]).mul(b[0]).0 as u128;
        let t01 = (&a[1]).mul(b[1]).0 as u128;
        let t02 = (&a[2]).mul(b[2]).0 as u128;

        let s012 = (&a[1]).add(a[2]);
        let tmp = (&b[1]).add(b[2]);
        let s012 = (&s012).mul(tmp).0 as u128;

        let s001 = (&a[0]).add(a[1]);
        let tmp = (&b[0]).add(b[1]);
        let s001 = (&s001).mul(tmp).0 as u128;

        let s002 = (&a[0]).add(a[2]);
        let tmp = (&b[0]).add(b[2]);
        let s002 = (&s002).mul(tmp).0 as u128;
        //0x7fffffff800000008=147573952555316674568
        let d00 = t01 + t02;
        let d00 = s012 + 0x7fffffff800000008 - d00;
        let d00 = d00 * BETA;
        let d00 = d00 + t00;

        let d01 = t02 * BETA;
        let tmp = t00 + t01;
        let d01 = d01 + 0x7fffffff800000008 - tmp;
        let d01 = d01 + s001;

        let d02 = t00 + t02;
        let d02 = t01 + 0x7fffffff800000008 - d02;
        let d02 = d02 + s002;

        // Compute the final coordinates, reduced by the modulus
        let a0 = Fp(reduce_u96(d00));
        let a1 = Fp(reduce_u96(d01));
        let a2 = Fp(reduce_u96(d02));

        [a0, a1, a2 ]
    }

    ///Returns the product of 3 degree extension element with base field element as an array.
    fn mul_base(a: [Self; 3], b: Self) -> [Self; 3] {
        [a[0]*b, a[1]*b, a[2]*b]
    }

    //Returns the square of field element of 3 degree extension as an array.
    fn square(a: [Self; 3]) -> [Self; 3] {
        let t00 = (&a[0]).square().0 as u128;
        let t01 = (&a[1]).square().0 as u128;
        let t02 = (&a[2]).square().0 as u128;

        let s012 = (&a[1]).add(a[2]);
        let s012 = (&s012).square().0 as u128;

        let s001 = (&a[0]).add(a[1]);
        let s001 = (&s001).square().0 as u128;

        let s002 = (&a[0]).add(a[2]);
        let s002 = (&s002).square().0 as u128;

        let d00 = t01 + t02;
        let d00 = s012 + 0x7fffffff800000008 - d00;
        let d00 = d00 * BETA;
        let d00 = d00 + t00;

        let d01 = t02 * BETA;
        let tmp = t00 + t01;
        let d01 = d01 + 0x7fffffff800000008 - tmp;
        let d01 = d01 + s001;

        let d02 = t00 + t02;
        let d02 = t01 + 0x7fffffff800000008 - d02;
        let d02 = d02 + s002;

        // Compute the final coordinates, reduced by the modulus
        let a0 = Fp(reduce_u96(d00));
        let a1 = Fp(reduce_u96(d01));
        let a2 = Fp(reduce_u96(d02));

        [a0, a1, a2 ]
    }

    ///Returns the square root of of 3 degrre extension field as an array wrapped in CtOption.
    fn sqrt(a: [Self; 3]) -> CtOption<[Self; 3]> {
        // Tonelli-Shank's algorithm for q mod 16 = 1
        // See https://eprint.iacr.org/2020/1497.pdf, page 3 for a
        // constant time specification of the algorithm.

        // Compute the progenitor y of self
        // y = self^((t - 1) // 2)
        //   = self^0x7ffffffe80000002fffffffc80000002fffffffe
        let x= Fp3{a0: a[0], a1: a[1], a2: a[2]};
        let y = x.power_by(&[0x80000002fffffffe, 0x80000002fffffffc, 0x000000007ffffffe]);

        let mut s = x.mul(y);
        let mut t = s.mul(y);

        let mut z = fp3::TWO_ADIC_ROOT_OF_UNITY_FP3;

        for k in (2..=TWO_ADICITY).rev() {
            let mut b = t;

            for _ in 0..k - 2 {
                b = b.square();
            }

            let new_s = s.mul(z);
            s = Fp3::conditional_select(&new_s, &s, b.ct_eq(&Fp3::one()));
            z = z.square();
            let new_t = t.mul(z);
            t = Fp3::conditional_select(&new_t, &t, b.ct_eq(&Fp3::one()));
        }

        CtOption::new([s.a0, s.a1, s.a2], (s.square()).ct_eq(&x))
    }

    ///Returns the inverse of a 3 degree extension field as an array wrapped in CtOption.
    ///unimplemented.
    fn invert(_a: [Self; 3])->CtOption<[Self; 3]> {
        unimplemented!()
    }
}

impl Extensible<6> for Fp{
    fn mul(a: [Self; 6], b: [Self; 6]) -> [Self; 6] {

        let t00 = (a[0]).mul(b[0]).0 as u128;
        let t01 = (a[2]).mul(b[2]).0 as u128;
        let t02 = (a[4]).mul(b[4]).0 as u128;

        let s012 = (a[2]).add(a[4]);
        let tmp = (b[2]).add(b[4]);
        let s012 = (&s012).mul(tmp).0 as u128;

        let s001 = (a[0]).add(a[2]);
        let tmp = (b[0]).add(b[2]);
        let s001 = (&s001).mul(tmp).0 as u128;

        let s002 = (a[0]).add(a[4]);
        let tmp = (b[0]).add(b[4]);
        let s002 = (&s002).mul(tmp).0 as u128;

        // Precomputations for a_1 * b_1
        let t10 = (a[1]).mul(b[1]).0 as u128;
        let t11 = (a[3]).mul(b[3]).0 as u128;
        let t12 = (a[5]).mul(b[5]).0 as u128;

        let s112 = (a[3]).add(a[5]);
        let tmp = (b[3]).add(b[5]);
        let s112 = (&s112).mul(tmp).0 as u128;

        let s101 = (a[1]).add(a[3]);
        let tmp = (b[1]).add(b[3]);
        let s101 = (&s101).mul(tmp).0 as u128;

        let s102 = (a[1]).add(a[5]);
        let tmp = (b[1]).add(b[5]);
        let s102 = (&s102).mul(tmp).0 as u128;

        // Precomputations for a_0 * b_1
        let t20 = (a[0]).mul(b[1]).0 as u128;
        let t21 = (a[2]).mul(b[3]).0 as u128;
        let t22 = (a[4]).mul(b[5]).0 as u128;

        let s212 = (a[2]).add(a[4]);
        let tmp = (b[3]).add(b[5]);
        let s212 = (&s212).mul(tmp).0 as u128;

        let s201 = (a[0]).add(a[2]);
        let tmp = (b[1]).add(b[3]);
        let s201 = (&s201).mul(tmp).0 as u128;

        let s202 = (a[0]).add(a[4]);
        let tmp = (b[1]).add(b[5]);
        let s202 = (&s202).mul(tmp).0 as u128;

        // Precomputations for a_1 * b_0
        let t30 = (a[1]).mul(b[0]).0 as u128;
        let t31 = (a[3]).mul(b[2]).0 as u128;
        let t32 = (a[5]).mul(b[4]).0 as u128;

        let s312 = (a[3]).add(a[5]);
        let tmp = (b[2]).add(b[4]);
        let s312 = (&s312).mul(tmp).0 as u128;

        let s301 = (a[1]).add(a[3]);
        let tmp = (b[0]).add(b[2]);
        let s301 = (&s301).mul(tmp).0 as u128;

        let s302 = (a[1]).add(a[5]);
        let tmp = (b[0]).add(b[4]);
        let s302 = (&s302).mul(tmp).0 as u128;

        // c_0 = a_0*b_0 + a_1*b_1.γ;
        // c_1 = a_0*b_1 + a_1*b_0;

        // Compute a_0 * b_0
        let d00 = t01 + t02;
        let d00 = s012 + 0x7fffffff800000008 - d00;
        let d00 = d00 * BETA;
        let d00 = d00 + t00;

        let d01 = t02 * BETA;
        let tmp = t00 + t01;
        let d01 = d01 + 0x7fffffff800000008 - tmp;
        let d01 = d01 + s001;

        let d02 = t00 + t02;
        let d02 = t01 + 0x7fffffff800000008 - d02;
        let d02 = d02 + s002;

        // Compute a_1 * b_1
        let d10 = t11 + t12;
        let d10 = s112 + 0x7fffffff800000008 - d10;
        let d10 = d10 * BETA;
        let d10 = d10 + t10;

        let d11 = t12 * BETA;
        let tmp = t10 + t11;
        let d11 = d11 + 0x7fffffff800000008 - tmp;
        let d11 = d11 + s101;

        let d12 = t10 + t12;
        let d12 = t11 + 0x7fffffff800000008 - d12;
        let d12 = d12 + s102;

        // Compute a_0 * b_1
        let d20 = t21 + t22;
        let d20 = s212 + 0x7fffffff800000008 - d20;
        let d20 = d20 * BETA;
        let d20 = d20 + t20;

        let d21 = t22 * BETA;
        let tmp = t20 + t21;
        let d21 = d21 + 0x7fffffff800000008 - tmp;
        let d21 = d21 + s201;

        let d22 = t20 + t22;
        let d22 = t21 + 0x7fffffff800000008 - d22;
        let d22 = d22 + s202;

        // Compute a_1 * b_0
        let d30 = t31 + t32;
        let d30 = s312 + 0x7fffffff800000008 - d30;
        let d30 = d30 * BETA;
        let d30 = d30 + t30;

        let d31 = t32 * BETA;
        let tmp = t30 + t31;
        let d31 = d31 + 0x7fffffff800000008 - tmp;
        let d31 = d31 + s301;

        let d32 = t30 + t32;
        let d32 = t31 + 0x7fffffff800000008 - d32;
        let d32 = d32 + s302;

        // Compute the final coordinates, reduced by the modulus
        let c0 = Fp(reduce_u96(d00 + d12 * BETA));
        let c2 = Fp(reduce_u96(d01 + d10));
        let c4 = Fp(reduce_u96(d02 + d11));
        let c1 = Fp(reduce_u96(d20 + d30));
        let c3 = Fp(reduce_u96(d21 + d31));
        let c5 = Fp(reduce_u96(d22 + d32));

    [c0, c1, c2, c3, c4, c5]
        // let x= Fp6{c0: a[0],c1: a[1], c2: a[2], c3: a[3],c4: a[4], c5: a[5]};
        // let mut y= Fp6{c0: b[0],c1: a[1], c2: b[2], c3: b[3],c4: b[4], c5: b[5]};
        // y*=x;
        // [y.c0, y.c1, y.c2, y.c3, y.c4, y.c5]
    }

    fn mul_base(a: [Self; 6], b: Self) -> [Self; 6] {
        [a[0]*b, a[1]*b, a[2]*b, a[3]*b, a[4]*b, a[5]*b]
    }
    /// Returns the square of an Fp6 element
    fn square(a: [Self; 6]) -> [Self; 6] {
        // All helper values computed below are seen as u128 after modular reduction.
        // This allows for a faster computation of the result coordinates,
        // by computing all operations in /ZZ, and then finally converting the
        // values back to Fp through a modular reduction.
        //
        // The reduction uses `reduce_u96()` as all final values are less than 96 bits.

        let aa = (&a[0]).square().0 as u128;
        let ab = (a[0]).mul(a[1]).0 as u128;
        let ac = (a[0]).mul(a[2]).0 as u128;
        let ad = (a[0]).mul(a[3]).0 as u128;
        let ae = (a[0]).mul(a[4]).0 as u128;
        let af = (a[0]).mul(a[5]).0 as u128;

        let bb = (a[1]).square().0 as u128;
        let bc = (a[1]).mul(a[2]).0 as u128;
        let bd = (a[1]).mul(a[3]).0 as u128;
        let be = (a[1]).mul(a[4]).0 as u128;
        let bf = (a[1]).mul(a[5]).0 as u128;

        let cc = (&a[2]).square().0 as u128;
        let cd = (a[2]).mul(a[3]).0 as u128;
        let ce = (a[2]).mul(a[4]).0 as u128;
        let cf = (a[2]).mul(a[5]).0 as u128;

        let dd = (a[3]).square().0 as u128;
        let de = (a[3]).mul(a[4]).0 as u128;
        let df = (a[3]).mul(a[5]).0 as u128;

        let ee = (&a[4]).square().0 as u128;
        let ef = (a[4]).mul(a[5]).0 as u128;

        let ff = (a[5]).square().0 as u128;

        let c0 = bf + ce;
        let c0 = c0 << 1;
        let c0 = c0 + dd;
        let c0 = c0 * BETA;
        let c0 = c0 + aa;
        let c0 = Fp(reduce_u96(c0));

        let c1 = cf + de;
        let c1 = c1 * BETA;
        let c1 = c1 + ab;
        let c1 = c1 << 1;
        let c1 = Fp(reduce_u96(c1));

        let c2 = df << 1;
        let c2 = c2 + ee;
        let c2: u128 = c2 * BETA;
        let t2 = ac << 1;
        let c2 = c2 + t2;
        let c2 = c2 + bb;
        let c2 = Fp(reduce_u96(c2));

        let c3: u128 = ef * BETA;
        let c3 = c3 + ad;
        let c3 = c3 + bc;
        let c3 = c3 << 1;
        let c3 = Fp(reduce_u96(c3));

        let t4 = ff * BETA;
        let c4 = ae + bd;
        let c4 = c4 << 1;
        let c4 = c4 + cc;
        let c4 = c4 + t4;
        let c4 = Fp(reduce_u96(c4));

        let c5 = af + be;
        let c5 = c5 + cd;
        let c5 = c5 << 1;
        let c5 = Fp(reduce_u96(c5));

        
            [c0,c1,c2,c3,c4,c5]
        
    }
    fn sqrt(a: [Self; 6]) -> CtOption<[Self; 6]> {
        // Algorithm 10 of https://eprint.iacr.org/2012/685.pdf,
        // adapted for constant-time specifications.

        // c = u + 4 (multiplicative generator of the group)
        // d = -1
        // e = 1/dc
        // f = (dc)^2
        const E: Fp3<Fp> = Fp3 {
            a0: Fp(17474221990273267221),
            a1: Fp(11073203528037158631),
            a2: Fp(1587630551273719300),
        };
        const F: Fp3<Fp> = Fp3 {
            a0: Fp(16),
            a1: Fp(18446744069414584320),
            a2: Fp(0),
        };
        let z= Fp6{c0: a[0],c1: a[1], c2: a[2], c3: a[3], c4: a[4], c5: a[5]};
        // Compute the progenitor y of self
        // y = self^((p^3 - 1) // 4)
        //   = self^0x3fffffff400000017ffffffe400000017fffffff40000000
        let b = z.mul(frobenius(&z));
        let b = b.mul(frobenius_double(&z));
        let b = b.power_by([0x3fffffffc0000000]);
        let b2 = b.square();
        let b_pow_p3 = frobenius_triple(&b);

        let check = b_pow_p3 * b;
        let s = b2 * z;
        let sf = mul_by_low_fp3(&s,&F);

        let sqrt_s = Fp3::from(&s).sqrt().unwrap_or(Fp3::ZERO);
        let sqrt_sf = Fp3::from(&sf).sqrt().unwrap_or(Fp3::ZERO);

        let x0 = Fp3::conditional_select(&sqrt_sf, &sqrt_s, check.ct_eq(&Fp6::ONE));
      
        let x = mul_by_low_fp3(&b_pow_p3,&x0);

        let x = Fp6::conditional_select(&(mul_by_high_fp3(&x, &E)), &x, check.ct_eq(&Fp6::ONE));
        
        CtOption::new([x.c0, x.c1, x.c2, x.c3, x.c4, x.c5], (x.square()).ct_eq(&z))
    }

    fn is_supported() -> bool {
        true
    }

    fn invert(x: [Self; 6]) -> CtOption<[Self; 6]> {

        let a = Fp6{c0: x[0], c1: x[1], c2: x[2], c3: x[3], c4: x[4], c5: x[5]};
        let t0 = frobenius(&a);
        let t1 = frobenius_triple(&a);
        let t2 = (t0)*(t1);
        let t3 = frobenius(&t2);
        let t4 = frobenius_double(&t2);
        let t5 = (t4)*(t3);
        let t5 = (t5)*(t0);
        let inv = ((t5)*(a)).c0;

    let xy=inv.invert().map(|t| t5.mul_by_fp(&t));
    let x= xy.unwrap();
    CtOption::new([x.c0, x.c1, x.c2, x.c3, x.c4, x.c5], xy.is_some())
    }
}

///=========Trait bounds==========
impl From<[u64; 6]> for Fp{
    fn from(value: [u64; 6]) -> Self {
        Fp(value[0])
    }
}

impl From<U256> for Fp{
    fn from(value: U256) -> Self {
        Fp(value.to_words()[0])
    }
}




//Creates a field element using u128 value after modular reduction.
impl From<u128> for Fp{
    fn from(value: u128) -> Self {
        Self::new(reduce_u128(value)%M.0)
    }
}

//Creates a field element using u64 value after modular reduction.
impl From<u64> for Fp{
    fn from(value: u64) -> Self {
        Self::new(value%M.0)
    }
}

//Creates a field element using u32 value.
impl From<u32> for Fp{
    fn from(value: u32)->Self{
        Self(value as u64)
    }
}

//Creates a field element using u32 value.
impl From<u16> for Fp{
    fn from(value: u16)->Self{
        Self(value.into())
    }
}

//Creates a field element using u32 value.
impl From<u8> for Fp{
    fn from(value: u8)->Self{
        Self(value.into())
    }
}

impl Display for Fp {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}
    
impl Hash for Fp {
    fn hash<H: Hasher>(&self, hasher: &mut H) {
        self.0.hash(hasher);
    }
}
    
impl ConstantTimeEq for Fp {
    fn ct_eq(&self, other: &Self) -> Choice {
        self.make_canonical().0.ct_eq(&other.make_canonical().0)
    }
    fn ct_ne(&self, other: &Self) -> Choice {
        !self.ct_eq(other)
    }
}
    
impl PartialEq for Fp {
    fn eq(&self, other: &Self) -> bool {
        bool::from(self.ct_eq(other))
    }
}
    
impl PartialOrd for Fp{
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}
impl Ord for Fp {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.0.cmp(&other.0)
    }
}

    
impl ConditionallySelectable for Fp {
    fn conditional_select(a: &Self, b: &Self, choice: Choice) -> Self {
        Self(u64::conditional_select(&a.0, &b.0, choice))
    }
}

impl Add for Fp{
    type Output=Fp;
    fn add(self,rhs: Self) -> Self::Output {
        add(&self,&rhs)
    }
}    

impl Sub for Fp{
    type Output = Fp;
    fn sub(self, rhs: Self) -> Self::Output {
        sub(&self,&rhs)
    }
}

impl Mul for Fp{
    type Output=Fp;
    fn mul(self, rhs: Fp) -> Self::Output {
        mul(&self,&rhs)
    }
}

impl Div for Fp{
    type Output=Fp;
    fn div(self, rhs: Fp) -> Self::Output {
        div(&self,&rhs)
    }
}

impl AddAssign<Fp> for Fp{
    fn add_assign(&mut self, other: Self) {
        *self=add(self, &other);
    }
}

impl SubAssign for Fp{
    fn sub_assign(&mut self, other: Self) {
        *self=sub(self,&other);
    }
}

impl MulAssign<Fp> for Fp{
    fn mul_assign(&mut self, rhs: Self) {
        *self=mul(self, &rhs)  //doubt
    }
}

impl DivAssign for Fp{
    fn div_assign(&mut self, rhs: Self) {
        *self=div(self, &rhs);
    }
}

impl AsBytes for Fp{
    fn as_bytes(&self) -> &[u8] {
       let ptr: *const Fp= self;
       unsafe{slice::from_raw_parts(ptr as *const u8, ELEMENT_BYTES)}
    }
}

impl Neg for Fp{
    type Output=Fp;
    fn neg(self) -> Self::Output {
        neg(&self)
    }
}

impl<'a> TryFrom<&'a [u8]> for Fp {
    type Error = String;

    /// Converts a slice of bytes into a field element; returns error if the value encoded in bytes
    /// is not a valid field element. The bytes are assumed to be in little-endian byte order.
    fn try_from(bytes: &[u8]) -> Result<Self, Self::Error> {
        let value:u64 = bytes
            .try_into()
            .map(u64::from_le_bytes)
            .map_err(|error| format!("{error}"))?;
        if value >= M.0 {
            return Err(format!(
                "cannot convert bytes into a field element: \
                value {value} is greater or equal to the field modulus"
            ));
        }
        Ok(Fp(value))
    }
}

impl Randomizable for Fp {
    const VALUE_SIZE: usize = ELEMENT_BYTES;
    fn from_random_bytes(bytes: &[u8]) -> Option<Self> {
        Self::try_from(bytes).ok()
    }
}

impl Serializable for Fp {
    fn write_into<W: ByteWriter>(&self, target: &mut W) {
        target.write_u8_slice(&self.0.to_le_bytes());
    }
}

impl Deserializable for Fp {
    fn read_from<R: ByteReader>(source: &mut R) -> Result<Self, DeserializationError> {
        let value = source.read_u64()?;
        if value >= M.0 {
            return Err(DeserializationError::InvalidValue(format!(
                "invalid field element: value {value} is greater than or equal to the field modulus"
            )));
        }
        Ok(Self::new(value))
    }
}

// Reduces a 128-bit value by M such that the output fits in a u64.
pub const fn reduce_u128(x: u128) -> u64 {
    // See https://github.com/mir-protocol/plonky2/blob/main/plonky2.pdf
    // for a more complete description of the reduction.

    // Decompose x = a + b.2^32 + c.2^64 + d.2^96 with a,b,c and d u32 values
    let ab = x as u64;
    let cd = (x >> 64) as u64;
    let c = (cd as u32) as u64;
    let d = cd >> 32;

    // r0 = ab - d
    let (r0, is_overflow) = ab.overflowing_sub(d);
    // d > ab may happen, hence handling potential overflow
    let r0 = r0.wrapping_sub(E * (is_overflow as u64));

    // r1 = c * 2^32 - c
    // this cannot underflow
    let r1 = (c << 32) - c;

    // result = r0 + r1
    let (result, is_overflow) = r0.overflowing_add(r1);
    // handle potential overflow
    result.wrapping_add(E * (is_overflow as u64))
}

/// Reduces a 96-bit value (stored as u128) by M such that the output fits in a u64.
/// This is similar to reduce_u128() but is aimed to be used when we are guaranteed that
/// the value to be reduced is fitting in 96 bits.
pub const fn reduce_u96(x: u128) -> u64 {
    // Decompose x = r0 + c.2^64 with r0 a u64 value and c a u32 value
    let c = ((x >> 64) as u32) as u64;

    let r0 = x as u64;

    // r1 = c * (E)
    // this cannot underflow
    let r1 = (c << 32) - c;

    // result = r0 + r1
    let (result, is_overflow) = r0.overflowing_add(r1);
    // handle potential overflow
    result.wrapping_add(E * (is_overflow as u64))
}

/// Computes the summation of two field elements handling overflow.
/// Used to give definition of trait bound Sum for Fp.
fn add(a: &Fp,b: &Fp)->Fp{
    let (d0,is_overflow)=a.0.overflowing_add(b.0);
    let (d0, is_overflow) = d0.overflowing_add(E * (is_overflow as u64));

    Fp(d0+E*(is_overflow as u64))
}

/// Subtracts b from a i.e, a-b, handles underflow.
/// Used to give definition of trait bound Sub for Fp.
fn sub(a: &Fp,b: &Fp) -> Fp{
    let (d0, is_overflow) = a.0.overflowing_sub(b.0);      
    let (d0, is_overflow) = d0.overflowing_sub(E * (is_overflow as u64));

    Fp(d0 - E * (is_overflow as u64))
}

/// Computes the multiplication of two field elements handling overflow.
/// Used to give definition of trait bound Mul for Fp
pub fn mul(a: &Fp, b: &Fp) -> Fp {
    let r0 = (a.0 as u128) * (b.0 as u128);

    Fp(reduce_u128(r0))
}

/// Computes the multiplicative inverse of field element,
// panics if the element is zero. Used to define Common field trait invert for Fp
fn invert(a: &Fp) -> CtOption<Fp> {
    // found using https://github.com/kwantam/addchain for M - 2
    assert!(a.0!=0,"invalid input"); //handles the inverse of 0
    let mut t2 = a.square(); //        1: 2
    let mut t3 = t2 * *a; //            2: 3
    let mut t0 = t3.square(); //          3: 6
    t0 = t0.square(); //                  4: 12
    t2 *= t0; //                          5: 14
    t3 *= t0; //                          6: 15
    t0 = t3.square(); //                  7: 30
    Fp::square_assign_multi(&mut t0, 3); //  10: 240
    t2 *= t0; //                         11: 254
    t3 *= t0; //                         12: 255
    t0 = t3.square(); //                 13: 510
    Fp::square_assign_multi(&mut t0, 7); //  20: 65280
    t2 *= t0; //                         21: 65534
    t0 *= t3; //                         22: 65535
    Fp::square_assign_multi(&mut t0, 16); // 38: 4294901760
    t2 *= t0; //                         39: 4294967294
    t0 = t2.square(); //                 40: 8589934588
    Fp::square_assign_multi(&mut t0, 31); // 71: 18446744065119617024
    t0 *= t2; //                         72: 18446744069414584318
    t0 *= *a; //                       73: 18446744069414584319 = M - 2

   CtOption::new(t0, !a.ct_eq(&Fp::zero()))
}

/// Computes a/b or a*(b^{-1}) in Fp,
//Used to give definition of trait bound Div for Fp
fn div(a:&Fp,b:&Fp)->Fp{
    let rhs:Fp=b.invert().unwrap();
    a.mul(rhs)
}

// Computes -a for given a in Fp,
//Used to give definition of trait bound Neg for Fp
fn neg(a: &Fp) -> Fp {
    (Fp::zero()).sub(*a)
}

/// Computes the multiplication of an Fp6 element with an Fp3 element
/// seen as the lower half of an element in Fp3[Y]/(Y^2 − γ).
pub fn mul_by_low_fp3(a: &Fp6<Fp>, other: &Fp3<Fp>) -> Fp6<Fp> {
    // Precomputations for a_0 * b_0
    let t00 = (a.c0).mul(other.a0).0 as u128;
    let t01 = (a.c2).mul(other.a1).0 as u128;
    let t02 = (a.c4).mul(other.a2).0 as u128;

    let s012 = (a.c2).add(a.c4);
    let tmp = (&other.a1).add(other.a2);
    let s012 = (&s012).mul(tmp).0 as u128;

    let s001 = (a.c0).add(a.c2);
    let tmp = (&other.a0).add(other.a1);
    let s001 = (&s001).mul(tmp).0 as u128;

    let s002 = (a.c0).add(a.c4);
    let tmp = (&other.a0).add(other.a2);
    let s002 = (&s002).mul(tmp).0 as u128;

    // Precomputations for a_1 * b_0
    let t30 = (a.c1).mul(other.a0).0 as u128;
    let t31 = (a.c3).mul(other.a1).0 as u128;
    let t32 = (a.c5).mul(other.a2).0 as u128;

    let s312 = (a.c3).add(a.c5);
    let tmp = (&other.a1).add(other.a2);
    let s312 = (&s312).mul(tmp).0 as u128;

    let s301 = (a.c1).add(a.c3);
    let tmp = (&other.a0).add(other.a1);
    let s301 = (&s301).mul(tmp).0 as u128;

    let s302 = (a.c1).add(a.c5);
    let tmp = (&other.a0).add(other.a2);
    let s302 = (&s302).mul(tmp).0 as u128;

    // c_0 = a_0*b_0 + a_1*b_1.γ;
    // c_1 = a_0*b_1 + a_1*b_0;

    // Compute a_0 * b_0
    let d00 = t01 + t02;
    // We need to make sure this doesn't underflow, hence adding a multiple of the modulus.
    // Running the tests in debug mode complains about a potential underflow when adding 2.p
    // hence adding a larger value (8.p).
    let d00 = s012 + 0x7fffffff800000008 - d00;
    let d00 = d00 * BETA;
    let d00 = d00 + t00;

    let d01 = t02 * BETA;
    let tmp = t00 + t01;
    let d01 = d01 + 0x7fffffff800000008 - tmp;
    let d01 = d01 + s001;

    let d02 = t00 + t02;
    let d02 = t01 + 0x7fffffff800000008 - d02;
    let d02 = d02 + s002;

    // Compute a_1 * b_0
    let d30 = t31 + t32;
    let d30 = s312 + 0x7fffffff800000008 - d30;
    let d30 = d30 * BETA;
    let d30 = d30 + t30;

    let d31 = t32 * BETA;
    let tmp = t30 + t31;
    let d31 = d31 + 0x7fffffff800000008 - tmp;
    let d31 = d31 + s301;

    let d32 = t30 + t32;
    let d32 = t31 + 0x7fffffff800000008 - d32;
    let d32 = d32 + s302;

    // Compute the final coordinates, reduced by the modulus
    // 0x7fffffff800000008 * BETA = 0x37ffffffc800000038
    let c0 = Fp(reduce_u96(d00 + 0x37ffffffc800000038));
    let c2 = Fp(reduce_u96(d01 + 0x37ffffffc800000038));
    let c4 = Fp(reduce_u96(d02 + 0x7fffffff800000008));
    let c1 = Fp(reduce_u96(d30 + 0x37ffffffc800000038));
    let c3 = Fp(reduce_u96(d31 + 0x7fffffff800000008));
    let c5 = Fp(reduce_u96(d32 + 0x7fffffff800000008));

    Fp6 {
        c0,
        c1,
        c2,
        c3,
        c4,
        c5,
    }
}

/// Computes the multiplication of an Fp6 element with an Fp3 element
/// seen as the higher half of an element in Fp3[Y]/(Y^2 − γ).
pub fn mul_by_high_fp3(a: &Fp6<Fp>, other: &Fp3<Fp>) -> Fp6<Fp> {
    // Precomputations for a_1 * b_1
    let t10 = (a.c1).mul(other.a0).0 as u128;
    let t11 = (a.c3).mul(other.a1).0 as u128;
    let t12 = (a.c5).mul(other.a2).0 as u128;

    let s112 = (a.c3).add(a.c5);
    let tmp = (other.a1).add(other.a2);
    let s112 = (&s112).mul(tmp).0 as u128;

    let s101 = (a.c1).add(a.c3);
    let tmp = (other.a0).add(other.a1);
    let s101 = (&s101).mul(tmp).0 as u128;

    let s102 = (a.c1).add(a.c5);
    let tmp = (other.a0).add(other.a2);
    let s102 = (&s102).mul(tmp).0 as u128;

    // Precomputations for a_0 * b_1
    let t20 = (a.c0).mul(other.a0).0 as u128;
    let t21 = (a.c2).mul(other.a1).0 as u128;
    let t22 = (a.c4).mul(other.a2).0 as u128;

    let s212 = (a.c2).add(a.c4);
    let tmp = (other.a1).add(other.a2);
    let s212 = (&s212).mul(tmp).0 as u128;

    let s201 = (a.c0).add(a.c2);
    let tmp = (other.a0).add(other.a1);
    let s201 = (&s201).mul(tmp).0 as u128;

    let s202 = (a.c0).add(a.c4);
    let tmp = (other.a0).add(other.a2);
    let s202 = (&s202).mul(tmp).0 as u128;

    // c_0 = a_0*b_0 + a_1*b_1.γ;
    // c_1 = a_0*b_1 + a_1*b_0;

    // Compute a_1 * b_1
    let d10 = t11 + t12;
    let d10 = s112 + 0x7fffffff800000008 - d10;
    let d10 = d10 * BETA;
    let d10 = d10 + t10;

    let d11 = t12 * BETA;
    let tmp = t10 + t11;
    let d11 = d11 + 0x7fffffff800000008 - tmp;
    let d11 = d11 + s101;

    let d12 = t10 + t12;
    let d12 = t11 + 0x7fffffff800000008 - d12;
    let d12 = d12 + s102;

    // Compute a_0 * b_1
    let d20 = t21 + t22;
    let d20 = s212 + 0x7fffffff800000008 - d20;
    let d20 = d20 * BETA;
    let d20 = d20 + t20;

    let d21 = t22 * BETA;
    let tmp = t20 + t21;
    let d21 = d21 + 0x7fffffff800000008 - tmp;
    let d21 = d21 + s201;

    let d22 = t20 + t22;
    let d22 = t21 + 0x7fffffff800000008 - d22;
    let d22 = d22 + s202;

    // Compute the final coordinates, reduced by the modulus
    // 0x7fffffff800000008 * BETA = 0x37ffffffc800000038
    let c0 = Fp(reduce_u96((d12 + 0x7fffffff800000008) * BETA));
    let c2 = Fp(reduce_u96(d10 + 0x7fffffff800000008));
    let c4 = Fp(reduce_u96(d11 + 0x7fffffff800000008));
    let c1 = Fp(reduce_u96(d20 + 0x37ffffffc800000038));
    let c3 = Fp(reduce_u96(d21 + 0x7fffffff800000008));
    let c5 = Fp(reduce_u96(d22 + 0x7fffffff800000008));

    Fp6 {
        c0,
        c1,
        c2,
        c3,
        c4,
        c5,
    }
}
/// Computes the multiplication of an Fp6 element with a u32 element.
pub fn mul_by_u32(a: &Fp6<Fp>, other: u32) -> Fp6<Fp> {
    let c0 = (a.c0).mul_by_u32(other);
    let c1 = (a.c1).mul_by_u32(other);
    let c2 = (a.c2).mul_by_u32(other);
    let c3 = (a.c3).mul_by_u32(other);
    let c4 = (a.c4).mul_by_u32(other);
    let c5 = (a.c5).mul_by_u32(other);

    Fp6 {c0,c1,c2,c3,c4,c5}
}

/// Computes the Frobenius endomorphism
pub fn frobenius(a: &Fp6<Fp>) -> Fp6<Fp> {
    Fp6 {
        c0: a.c0,
        c1: (&Fp(18446744065119617026)).mul(a.c1),
        c2: (&Fp(18446744065119617025)).mul(a.c2),
        c3: (a.c3).neg(),
        c4: (a.c4).mul_by_u32(4294967295),
        c5: (&Fp(4294967296)).mul(a.c5),
    }
}
/// Computes the Frobenius endomorphism twice
pub fn frobenius_double(a: &Fp6<Fp>) -> Fp6<Fp> {
    Fp6 {
        c0: a.c0,
        c1: (&Fp(18446744065119617025)).mul(a.c1),
        c2: (a.c2).mul_by_u32(4294967295),
        c3: a.c3,
        c4: (&Fp(18446744065119617025)).mul(a.c4),
        c5: (a.c5).mul_by_u32(4294967295),
    }
}
/// Computes the Frobenius endomorphism thrice
pub fn frobenius_triple(a: &Fp6<Fp>) -> Fp6<Fp> {
    Fp6 {
        c0: a.c0,
        c1: (a.c1).neg(),
        c2: a.c2,
        c3: (a.c3).neg(),
        c4: a.c4,
        c5: (a.c5).neg(),
    }
}
