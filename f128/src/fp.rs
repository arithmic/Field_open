
use core::{
    AsBytes, ByteReader, ByteWriter, Deserializable, DeserializationError, Randomizable,
    Serializable,
};
use std::{
    fmt::{Debug, Display, Formatter},
    ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Neg, Sub, SubAssign},
    slice, u128,
};

use crypto_bigint::{subtle::{CtOption, Choice}, U256, U128};
use rand::Rng;

use traits::traits::{Field, PrimeField, Extensible};



// CONSTANTS
// ================================================================================================

// Field modulus = 2^128 - 45 * 2^40 + 1
pub const M: u128 = 340282366920938463463374557953744961537;
const M_HEX: &'static str = "0xffffffffffffffffffffd30000000001";
// The maximum u128 integer, 2^128 - 1 is less than 2M, this implies for any 128 bit integer a>=M,
// a mod M = a-M.

// 2^40 root of unity
const G: u128 = 23953097886125630542083529559205016746;
const G_HEX: &'static str = "0x120532e7b364080a86b8723e1920f4aa";
// Number of bytes needed to represent field element
const ELEMENT_BYTES: usize = std::mem::size_of::<u128>();



// FIELD ELEMENT
// ================================================================================================
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct Fp(pub u128);

impl Fp {
    //Constructor method, takes 128 bit integer and does modulo reduction if value>=M.
    pub fn new(value: u128) -> Self {
        if value >= M {
            Fp(value - M)
        } else {
            Fp(value)
        }
    }

    pub fn exp(self, power: u128) -> Self {
        let mut r = Self::ONE;
        let mut b = self;
        let mut p = power;

        let int_zero = 0u128;
        let int_one = 1u128;

        if p == int_zero {
            return Self::ONE;
        } else if b == Self::ZERO {
            return Self::ZERO;
        }

        while p > int_zero {
            if p & int_one == int_one {
                r *= b;
            }
            p >>= int_one;
            b = b.square();
        }

        r
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

impl PrimeField for Fp {

    //Get root of unity function, computes 2^n-th root given input n. Only accepts inputs in
    //the range [1,40] and panics otherwise.
    fn get_root_of_unity(n: u32) -> Fp {
        assert!(n <= 40, "Integer too large, two-addicity is 40");
        assert!(n != 0, " Zero is invalid input");
        Fp(G).exp(1u128 << (40 - n) as u128)
    }

    
    //Associated constants of the 128 bit field.

    //Prime number that characterises the finite field.
    const MODULUS: &'static str = M_HEX;
    const GENERATOR: Self = Fp(3) ;
    //2^40th root of unity.
    const TWO_ADIC_ROOT: &'static str = G_HEX;
    const TWO_ADDICITY: u32 = 40;

    type Repr = [u8;ELEMENT_BYTES];
    fn is_even(self)->crypto_bigint::subtle::Choice{
        Choice::from(!self.is_odd())
    }
    fn is_odd(self)->crypto_bigint::subtle::Choice {
        Choice::from((self.0&1) as u8)
    }

    const NUM_BITS: u32 = (ELEMENT_BYTES as u32)<<3;

    
    
}

impl Field for Fp {
    const ONE: Self = Fp(1);
    const ZERO: Self = Fp(0);

    //Inverse of integer w.r.t. modular multiplication
    fn invert(self) -> CtOption<Self> {
                if self.is_zero(){
                    CtOption::new(Self::ZERO, Choice::from(0))
                }
                else {
                    CtOption::new(Fp(invert(self.0)), Choice::from(1))
                }
    }

    fn sqrt(self) -> CtOption<Fp> {
        //self^((t-1)/2)
        let mut w = self.power_by(U128::from_be_hex("00000000007fffffffffffffffffffe9").to_words());
        // v is the number of leading zeros bit in the bit representation of q-1
        let mut v = 40 as u32; //v
        let mut x = self * w; //x
        let mut b = x * w; //b
        let multiplicative_generator = Fp(3);
        //g^((q-1)/2^5)
        let mut z = multiplicative_generator.power_by(U128::from_be_hex("0000000000ffffffffffffffffffffd3").to_words()); // z
        while !b.is_one()
        {let mut k = 0;
            let mut b2k = b;
            while !b2k.is_one(){
                b2k = b2k.square();
                k+=1;
            }
            let j = v-k;
            w =z;
            for _ in 1..j{
                w = w.square();
            }

            z = w.square();
            b *= z;
            x *= w;
            v = k;
        }    
        CtOption::new(x, Choice::from((x.square().0 == self.0) as u8))
    }

    fn power_by<S: AsRef<[u64]>>(self, exp:S)->Self {
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
    //Indicator functions
    fn is_one(self) -> bool {
        self == Self::ONE
    }
    fn is_zero(self) -> bool {
        self == Self::ZERO
    }

    fn random()->Self{
        // Fp::from(rand_value::<u128>())
        Self::new(rand::thread_rng().gen_range(0..(M-1)))
    }

    type BaseField = Fp;
    fn to_curve_bytes(&self) -> &[u8] {
        self.as_bytes()
    }
    fn to_words(&self) -> Vec<u64> {
        unimplemented!()
    }
    const ELEMENT_BYTES: usize = ELEMENT_BYTES;

    fn  from_uint_reduced(_w: Fp) -> Self {
     unimplemented!()
    }

    fn from_words( _a: &Vec<u64>) -> Self {
     unimplemented!()
    }

    fn get_windows(&self, _exp: usize)->Vec<usize> {
     unimplemented!()
    }

  
}

impl Extensible<2> for Fp {

    //Multiplication is polynomial  multiplication of x_0 + x_1*w and y_0 + y_1*w with respect to the identity w^2 = 1+w.
    fn mul(a: [Self; 2], b: [Self; 2]) -> [Self; 2] {
            let prod = (a[0] + a[1])*(b[0]+ b[1]);
            let c =a[0] * b[0];
            [c + a[1]*b[1], prod - c]
            //( a_0.b_0 + a_1.b_1, a_1.b_0 + a_0.b_1 + a_1.b_1)
        }
    
    //Multiplication of extension element with base field element is simply scalar multiplication.
    fn mul_base(a: [Self; 2], b: Self) -> [Self; 2] {
        [a[0]*b, a[1]*b]
    }

    fn square(a: [Self; 2])->[Self; 2] {
        let b0 = a[0].square();
        let b1 = a[1].square();
        let c = (a[0]* a[1]).double();
        [b0 + b1, c + b1]
    }

    fn sqrt(_a: [Self; 2])->CtOption<[Self; 2]> {
        unimplemented!()
    }

//Inversion in Quadratic extension. For a quadratic field element (a,b) viewed as a double of base-field elements,
// (a,b)^(-1)= (-b/c, (a+b)/c) where c = (a^2 - b^2 + ab).

    fn invert(a: [Self; 2])->CtOption<[Self; 2]> {
        if a[0].is_zero() & a[1].is_zero() {
            return CtOption::new(a, Choice::from(0));
        }
        let numerator = [a[0] + a[1], Self::ZERO - a[1]];

        let norm = <Fp as Extensible<2>>::mul(a, numerator);
        debug_assert_eq!(norm[1], Self::ZERO, "norm must be in the base field");
        let denom_inv = norm[0].invert().unwrap();

        CtOption::new([numerator[0] * denom_inv, numerator[1] * denom_inv], Choice::from(1))
    }
}
//
/// Represents a base field element.
///
/// Internal values are stored in their canonical form in the range [0, M). The backing type is
/// `u128`.

impl Randomizable for Fp {
    const VALUE_SIZE: usize = ELEMENT_BYTES;

    fn from_random_bytes(bytes: &[u8]) -> Option<Self> {
        Self::try_from(bytes).ok()
    }
}

impl Display for Fp {
    fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

// TYPE CONVERSIONS
// ================================================================================================
//implement from trait.
impl From<[u8; 16]> for Fp {
    /// Converts the value encoded in an array of 16 bytes into a field element. The bytes
    /// are assumed to be in little-endian byte order. If the value is greater than or equal
    /// to the field modulus, modular reduction is silently performed.
    fn from(bytes: [u8; 16]) -> Self {
        let value: u128 = u128::from_le_bytes(bytes);
        Fp::from(value)
    }
}

//Makes field element from 128-bit integer, computing modular reduction if required.

impl From<u128> for Fp {
    fn from(value: u128) -> Self {
        Fp::new(value)
    }
}

//Modulus size is larger than 64 bits, hence converting from integers of smaller types
//is equivalent to representing them as 128 bit integers and wrapping them in the
//Fp struct.

impl From<u64> for Fp {
    fn from(value: u64) -> Self {
        Fp(value as u128)
    }
}
impl From<u32> for Fp {
    fn from(value: u32) -> Self {
        Fp(value as u128)
    }
}
impl From<u16> for Fp {
    fn from(value: u16) -> Self {
        Fp(value as u128)
    }
}
impl From<u8> for Fp {
    fn from(value: u8) -> Self {
        Fp(value as u128)
    }
}

impl From<[u64;6]> for Fp {
    fn from(_value: [u64;6]) -> Self {
        Fp::new((1<<64)*_value[1] as u128 | _value[0] as u128)
    }
}

impl From<U256> for Fp {
    fn from(_value: U256) -> Self {
        let value  =  _value.to_words();
        Fp::new((1<<64)*value[1] as u128 | value[0] as u128)

    }
}

impl<'a> TryFrom<&'a [u8]> for Fp {
    type Error = String;

    /// Converts a slice of bytes into a field element; returns error if the value encoded in bytes
    /// is not a valid field element. The bytes are assumed to be in little-endian byte order.
    fn try_from(bytes: &[u8]) -> Result<Self, Self::Error> {
        let value = bytes
            .try_into()
            .map(u128::from_le_bytes)
            .map_err(|error| format!("{error}"))?;
        if value >= M {
            return Err(format!(
                "cannot convert bytes into a field element: \
            value {value} is greater or equal to the field modulus"
            ));
        }
        Ok(Fp(value))
    }
}

impl AsBytes for Fp {
    fn as_bytes(&self) -> &[u8] {
        // TODO: take endianness into account
        let self_ptr: *const Fp = self;
        unsafe { slice::from_raw_parts(self_ptr as *const u8, ELEMENT_BYTES) }
    }
}

// SERIALIZATION / DESERIALIZATION
// ------------------------------------------------------------------------------------------------

//Writes the byte slice representation of field-element as an integer into a little-endian byte slice.
impl Serializable for Fp {
    fn write_into<W: ByteWriter>(&self, target: &mut W) {
        target.write_u8_slice(&self.0.to_le_bytes());
    }
}

//Convert little-endian bytestring into field element.
impl Deserializable for Fp {
    fn read_from<R: ByteReader>(source: &mut R) -> Result<Self, DeserializationError> {
        let value = source.read_u128()?;
        if value >= M {
            return Err(DeserializationError::InvalidValue(format!(
            "invalid field element: value {value} is greater than or equal to the field modulus"
        )));
        }
        Ok(Fp(value))
    }
}
// OVERLOADED OPERATORS
// ================================================================================================
//Trait implementation for addition trait to use "+" operation with field elements.
impl Add for Fp {
    type Output = Self;
    fn add(self, rhs: Self) -> Self {
        Fp(add(self.0, rhs.0))
    }
}

//Trait implementation for add-assign trait to use "+=" operation with field elements.
impl AddAssign for Fp {
    fn add_assign(&mut self, rhs: Self) {
        *self = *self + rhs
    }
}
impl Neg for Fp {
    type Output = Self;
    fn neg(self) -> Self::Output {
        Fp(neg(self.0))
    }
}

//Trait implementation for subtraction trait to use "-" with field elements.
impl Sub for Fp {
    type Output = Self;
    fn sub(self, rhs: Self) -> Self {
        Fp(sub(self.0, rhs.0))
    }
}

//Trait implementation for sub-assign trait to use "-=" operation with field elements.
impl SubAssign for Fp {
    fn sub_assign(&mut self, rhs: Self) {
        *self = *self - rhs
    }
}

//Trait implementation for multiplication trait to use "*" operation with field elements.
impl Mul for Fp {
    type Output = Self;
    fn mul(self, rhs: Self) -> Self::Output {
        Fp(mul(self.0, rhs.0))
    }
}

//Trait implementation for multiply-assign trait to use "*=" operation with field elements.
impl MulAssign for Fp {
    fn mul_assign(&mut self, rhs: Self) {
        *self = *self * rhs
    }
}

//Trait implementation for division trait to use "/" operation with field elements.
impl Div for Fp {
    type Output = Self;
    fn div(self, rhs: Self) -> Self::Output {
        //In fields, a/b = a*(1/b), that is division of a by b is equivalent to
        //multiplying a into the inverse of b.
        self * rhs.invert().unwrap()
    }
}

//Trait implementation for divide-assign trait to use "/=" operation with field elements.
impl DivAssign for Fp {
    fn div_assign(&mut self, rhs: Self) {
        *self = *self / rhs
    }
}

//Field Operations
/// Computes (a + b) % m; a and b are assumed to be valid field elements.
///

fn add(a: u128, b: u128) -> u128 {
    let z = M - b;
    if a >= z {
        a - z
    } else {
        a + b
    }
}

// Computes -a % m, a is assumed to be a valid field element.
fn neg(a: u128) -> u128 {
    if a == 0 {
        0
    }
    //Returns integer
    else {
        M - a
    }
}

/// Computes (a-b) % m; a and b are assumed to be valid field elements.
fn sub(a: u128, b: u128) -> u128 {
    //If x<y computes wrap-around from the modulus. i.e M - (y-x) where y-x would be the overflow.
    //Otherwise, computes difference as usual.
    if a < b {
        neg(b - a)
    } else {
        a - b
    }
}

/// Computes (a * b) % m; a and b are assumed to be valid field elements.
fn mul(a: u128, b: u128) -> u128 {
    let (x0, x1, x2) = mul_128x64(a, (b >> 64) as u64); // x = a * b_hi
    let (mut x0, mut x1, x2) = mul_reduce(x0, x1, x2); // x = x - (x >> 128) * m
    if x2 == 1 {
        // if there was an overflow beyond 128 bits, subtract
        // modulus from the result to make sure it fits into
        // 128 bits; this can potentially be removed in favor
        // of checking overflow later
        let (t0, t1) = sub_modulus(x0, x1); // x = x - m
        x0 = t0;
        x1 = t1;
    }

    let (y0, y1, y2) = mul_128x64(a, b as u64); // y = a * b_lo

    let (mut y1, carry) = add64_with_carry(y1, x0, 0); // y = y + (x << 64)
    let (mut y2, y3) = add64_with_carry(y2, x1, carry);
    if y3 == 1 {
        // if there was an overflow beyond 192 bits, subtract
        // modulus * 2^64 from the result to make sure it fits
        // into 192 bits; this can potentially replace the
        // previous overflow check (but needs to be proven)
        let (t0, t1) = sub_modulus(y1, y2); // y = y - (m << 64)
        y1 = t0;
        y2 = t1;
    }

    let (mut z0, mut z1, z2) = mul_reduce(y0, y1, y2); // z = y - (y >> 128) * m

    // make sure z is smaller than m
    if z2 == 1 || (z1 == (M >> 64) as u64 && z0 >= (M as u64)) {
        let (t0, t1) = sub_modulus(z0, z1); // z = z - m
        z0 = t0;
        z1 = t1;
    }

    ((z1 as u128) << 64) + (z0 as u128)
}

/// Computes y such that (x * y) % m = 1 except for when when x = 0; in such a case,
/// 0 is returned; x is assumed to be a valid field element.
fn invert(x: u128) -> u128 {
    if x == 0 {
        return 0;
    };

    // initialize v, a, u, and d variables
    let mut v = M;
    let (mut a0, mut a1, mut a2) = (0, 0, 0);
    let (mut u0, mut u1, mut u2) = if x & 1 == 1 {
        // u = x
        (x as u64, (x >> 64) as u64, 0)
    } else {
        // u = x + m
        add_192x192(x as u64, (x >> 64) as u64, 0, M as u64, (M >> 64) as u64, 0)
    };
    // d = m - 1
    let (mut d0, mut d1, mut d2) = ((M as u64) - 1, (M >> 64) as u64, 0);

    // compute the inverse
    while v != 1 {
        while u2 > 0 || ((u0 as u128) + ((u1 as u128) << 64)) > v {
            // u > v
            // u = u - v
            let (t0, t1, t2) = sub_192x192(u0, u1, u2, v as u64, (v >> 64) as u64, 0);
            u0 = t0;
            u1 = t1;
            u2 = t2;

            // d = d + a
            let (t0, t1, t2) = add_192x192(d0, d1, d2, a0, a1, a2);
            d0 = t0;
            d1 = t1;
            d2 = t2;

            while u0 & 1 == 0 {
                if d0 & 1 == 1 {
                    // d = d + m
                    let (t0, t1, t2) = add_192x192(d0, d1, d2, M as u64, (M >> 64) as u64, 0);
                    d0 = t0;
                    d1 = t1;
                    d2 = t2;
                }

                // u = u >> 1
                u0 = (u0 >> 1) | ((u1 & 1) << 63);
                u1 = (u1 >> 1) | ((u2 & 1) << 63);
                u2 >>= 1;

                // d = d >> 1
                d0 = (d0 >> 1) | ((d1 & 1) << 63);
                d1 = (d1 >> 1) | ((d2 & 1) << 63);
                d2 >>= 1;
            }
        }

        // v = v - u (u is less than v at this point)
        v -= (u0 as u128) + ((u1 as u128) << 64);

        // a = a + d
        let (t0, t1, t2) = add_192x192(a0, a1, a2, d0, d1, d2);
        a0 = t0;
        a1 = t1;
        a2 = t2;

        while v & 1 == 0 {
            if a0 & 1 == 1 {
                // a = a + m
                let (t0, t1, t2) = add_192x192(a0, a1, a2, M as u64, (M >> 64) as u64, 0);
                a0 = t0;
                a1 = t1;
                a2 = t2;
            }

            v >>= 1;

            // a = a >> 1
            a0 = (a0 >> 1) | ((a1 & 1) << 63);
            a1 = (a1 >> 1) | ((a2 & 1) << 63);
            a2 >>= 1;
        }
    }

    // a = a mod m
    let mut a = (a0 as u128) + ((a1 as u128) << 64);
    while a2 > 0 || a >= M {
        let (t0, t1, t2) = sub_192x192(a0, a1, a2, M as u64, (M >> 64) as u64, 0);
        a0 = t0;
        a1 = t1;
        a2 = t2;
        a = (a0 as u128) + ((a1 as u128) << 64);
    }

    a
}

// HELPER FUNCTIONS
// ================================================================================================

#[inline]
fn mul_128x64(a: u128, b: u64) -> (u64, u64, u64) {
    let z_lo = ((a as u64) as u128) * (b as u128);
    let z_hi = (a >> 64) * (b as u128);
    let z_hi = z_hi + (z_lo >> 64);
    (z_lo as u64, z_hi as u64, (z_hi >> 64) as u64)
}

#[inline]
fn mul_reduce(z0: u64, z1: u64, z2: u64) -> (u64, u64, u64) {
    let (q0, q1, q2) = mul_by_modulus(z2);
    let (z0, z1, z2) = sub_192x192(z0, z1, z2, q0, q1, q2);
    (z0, z1, z2)
}

#[inline]
fn mul_by_modulus(a: u64) -> (u64, u64, u64) {
    let a_lo = (a as u128).wrapping_mul(M);
    let a_hi = if a == 0 { 0 } else { a - 1 };
    (a_lo as u64, (a_lo >> 64) as u64, a_hi)
}

#[inline]
fn sub_modulus(a_lo: u64, a_hi: u64) -> (u64, u64) {
    let mut z = 0u128.wrapping_sub(M);
    z = z.wrapping_add(a_lo as u128);
    z = z.wrapping_add((a_hi as u128) << 64);
    (z as u64, (z >> 64) as u64)
}

#[inline]
fn sub_192x192(a0: u64, a1: u64, a2: u64, b0: u64, b1: u64, b2: u64) -> (u64, u64, u64) {
    let z0 = (a0 as u128).wrapping_sub(b0 as u128);
    let z1 = (a1 as u128).wrapping_sub((b1 as u128) + (z0 >> 127));
    let z2 = (a2 as u128).wrapping_sub((b2 as u128) + (z1 >> 127));
    (z0 as u64, z1 as u64, z2 as u64)
}

#[inline]
fn add_192x192(a0: u64, a1: u64, a2: u64, b0: u64, b1: u64, b2: u64) -> (u64, u64, u64) {
    let z0 = (a0 as u128) + (b0 as u128);
    let z1 = (a1 as u128) + (b1 as u128) + (z0 >> 64);
    let z2 = (a2 as u128) + (b2 as u128) + (z1 >> 64);
    (z0 as u64, z1 as u64, z2 as u64)
}

#[inline]
const fn add64_with_carry(a: u64, b: u64, carry: u64) -> (u64, u64) {
    let ret = (a as u128) + (b as u128) + (carry as u128);
    (ret as u64, (ret >> 64) as u64)
}
