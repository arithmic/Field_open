use core::{
    AsBytes, ByteReader, ByteWriter, Deserializable, DeserializationError, Randomizable,
    Serializable,
};
use std::{
    fmt::{Display, Formatter},
    ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Neg, Sub, SubAssign},
    slice,
};

use crypto_bigint::{
    generic_array::GenericArray,
    subtle::{Choice, ConditionallySelectable, ConstantTimeEq, ConstantTimeLess, CtOption},
    ArrayEncoding, Encoding, Integer, Limb, Random, Zero, U256,
};
use rand::rngs::OsRng;
use traits::traits::{Field, PrimeField};

// Size of field elements of this elliptic curve.
pub type FieldSize = <U256 as crypto_bigint::ArrayEncoding>::ByteSize;

/// Byte representation of a base/scalar field element of a given curve.
pub type FieldBytes = GenericArray<u8, FieldSize>;

// Number of bytes needed to represent field element
pub const ELEMENT_BYTES: usize = std::mem::size_of::<U256>();

//prime order of the curve, q=52435875175126190479447740508185965837690552500527637822603658699938581184513
pub const SCALAR_MODULUS: U256 =
    U256::from_be_hex("73eda753299d7d483339d80809a1d80553bda402fffe5bfeffffffff00000001");

//52435875175126190479447740508185965837690552500527637822603658699938581184512
pub const SCALAR_MODULUS_MINUS_ONE: U256 =
    U256::from_be_hex("73eda753299d7d483339d80809a1d80553bda402fffe5bfeffffffff00000000");

// SCALAR_MODULUS/2
pub const FRAC_SCALAR_MODULUS_2: Scalar = Scalar(SCALAR_MODULUS.shr_vartime(1));

pub const TWO_ADDICITY: u32 = 32;
pub const TWO_ADIC_ROOT: U256 =
    U256::from_be_hex("16A2A19EDFE81F20D09B681922C813B4B63683508C2280B93829971F439F0D2B");

//MU=floor(2^512/q)=255699135089535202043525422716183576215815630510683217819334674386498370757523
//2355094EDFEDE377C38B5DCB707E08ED365043EB4BE4BAD7142737A020C0D6393
pub const MU: [u64; 5] = [
    0x4273_7A02_0C0D_6393,
    0x6504_3EB4_BE4B_AD71,
    0x38B5_DCB7_07E0_8ED3,
    0x3550_94ED_FEDE_377C,
    0x0000_0000_0000_0002,
];

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, PartialOrd, Ord)]
// creating the scalar struct
pub struct Scalar(pub U256);
impl Scalar {
    // constants zero and one
    pub const ZERO: Scalar = Scalar(U256::ZERO);
    pub const ONE: Scalar = Scalar(U256::ONE);
    // Returns self  mod n
    pub fn new(other: U256) -> Self {
        Scalar(CtOption::new(other, other.ct_lt(&SCALAR_MODULUS)).unwrap())
    }
    // Shift right by one bit
    pub fn shr1(&mut self) {
        self.0 >>= 1;
    }
    // returns the scalar field element from its byte representation.
    pub fn from_repr(bytes: &FieldBytes) -> Scalar {
        let inner = U256::from_be_byte_array(*bytes);
        <Scalar as Field>::from_uint_reduced(Scalar(inner))
    }
    // returns the remainder obtained when self is divided by d.
    pub fn rem(self, d: Self) -> Self {
        let mut a = self;
        while a >= d {
            a -= d;
        }
        a
    }
    // gives the bytes representation of given Scalar field element
    pub fn to_bytes(a: &Scalar) -> FieldBytes {
        a.0.to_be_byte_array()
    }
    // elements as bytes
    pub fn elements_as_bytes(elements: &[Self]) -> &[u8] {
        unsafe {
            slice::from_raw_parts(
                elements.as_ptr() as *const u8,
                elements.len() * Self::ELEMENT_BYTES,
            )
        }
    }
}
impl Field for Scalar {
    // checks whether the given scakar field element is zero or  not.
    fn is_zero(self) -> bool {
        self == Self::ZERO
    }
    // checks ehether the given scalar field element is one or not
    fn is_one(self) -> bool {
        self == Self::ONE
    }
    // returns the twice of self
    fn double(self) -> Self {
        add(&self, &self)
    }
    // return the thrice of self
    fn triple(self) -> Self {
        add(&self.double(), &self)
    }
    // returns the square of the self
    fn square(self) -> Self {
        let product = self.0.square_wide();
        let limbs = barrett_reduce(product.0, product.1);
        let words: [u64; 4] = limbs.to_words();
        let out: Scalar = Scalar(U256::from_words(words));
        out
    }
    // returns the multiplicative inverse of the self
    fn invert(self) -> CtOption<Scalar> {
        invert(&self)
    }
    // Additive and Multiplicative identity of the scalar field i.e zero and one
    const ZERO: Self = Self::ZERO;

    const ONE: Self = Self::ONE;
    const ELEMENT_BYTES: usize = ELEMENT_BYTES;
    // returns t he random scalar field element
    fn random() -> Self {
        let r = U256::random(&mut OsRng);
        if r < SCALAR_MODULUS {
            return Scalar(r);
        } else {
            return Scalar(r >> 2);
        }
    }
    fn sqrt(self) -> CtOption<Self> {
        // (q - 1)/2
        let w = self.power_by(&[
            0x7fff_2dff_7fff_ffff,
            0x04d0_ec02_a9de_d201,
            0x94ce_bea4_199c_ec04,
            0x0000_0000_39f6_d3a9,
        ]);
        // v is the number of leading zeros bit in the bit representation of q-1
        let mut v = 32 as u32;
        let mut x = self * w;
        let mut b = x * w;
        let multiplicative_generator = Scalar(U256::from_u32(7));
        let q_minus_1 =
            U256::from_be_hex("0000000073eda753299d7d483339d80809a1d80553bda402fffe5bfeffffffff");
        let mut z = multiplicative_generator.power_by(&q_minus_1.to_words());
        for max_v in (1..=32).rev() {
            let mut k = 1;
            let mut tmp = b.square();
            let mut j_less_than_v = Choice::from(1);
            for j in 2..max_v {
                let tmp_is_one = tmp.ct_eq(&Self::ONE);
                let squared = Self::conditional_select(&tmp, &z, tmp_is_one).square();
                tmp = Self::conditional_select(&squared, &tmp, tmp_is_one);
                let new_z = Self::conditional_select(&z, &squared, tmp_is_one);
                j_less_than_v &= !j.ct_eq(&v);
                k = u32::conditional_select(&j, &k, tmp_is_one);
                z = Self::conditional_select(&z, &new_z, j_less_than_v);
            }
            let result = x * z;
            x = Self::conditional_select(&result, &x, b.ct_eq(&Self::ONE));
            z = z.square();
            b = b * z;
            v = k;
        }
        CtOption::new(x, x.square().ct_eq(&self))
    }
    // Exponentiates the self by pow
    fn power_by<S: AsRef<[u64]>>(self, pow: S) -> Self {
        let mut exp = [0, 0, 0, 0];
        for (iter, value) in pow.as_ref().iter().enumerate() {
            exp[iter] = *value
        }
        let mut r = self;
        let mut a = Self::ONE;
        if exp[1] == 0 && exp[2] == 0 && exp[3] == 0 {
            let e = exp[0];
            if e < (1_u64 << 32) {
                for i in 0..32 {
                    if ((e >> i) & 1) == 1 {
                        a = a * r;
                    }
                    r = r.square();
                }
            } else {
                for i in 0..64 {
                    if ((e >> i) & 1) == 1 {
                        a = a * r;
                    }
                    r = r.square();
                }
            }
        } else if exp[2] == 0 && exp[3] == 0 {
            for e in [exp[0], exp[1]] {
                for i in 0..64 {
                    if ((e >> i) & 1) == 1 {
                        a = a * r;
                    }
                    r = r.square();
                }
            }
        } else if exp[3] == 0 {
            for e in [exp[0], exp[1], exp[2]] {
                for i in 0..64 {
                    if ((e >> i) & 1) == 1 {
                        a = a * r;
                    }
                    r = r.square();
                }
            }
        } else {
            for e in exp.iter().rev() {
                for i in (0..64).rev() {
                    a = a.square();
                    if ((*e >> i) & 1) == 1 {
                        a.mul_assign(self)
                    }
                }
            }
        }
        a
    }
    type BaseField = Scalar;

    fn cube(self) -> Self {
        self * self * self
    }
    fn to_curve_bytes(&self) -> &[u8] {
        self.as_bytes()
    }

    fn to_words(&self) -> Vec<u64> {
        self.0.to_words().into()
    }

    fn from_words(a: &Vec<u64>) -> Self {
        let k = [a[0], a[1], a[2], a[3]];
        let value = U256::from_words(k);
        Scalar(value)
    }

    fn from_uint_reduced(w: Self) -> Self {
        let (r, underflow) = w.0.sbb(&SCALAR_MODULUS, Limb::ZERO);
        let underflow = Choice::from((underflow.0 >> (Limb::BITS - 1)) as u8);
        Self(U256::conditional_select(&w.0, &r, !underflow))
    }

    fn get_windows(&self, window_bits: usize) -> Vec<usize> {
        let int = &self.0;

        let window_marker = 1usize << window_bits;

        let n_windows = (256 / window_bits) + 1;

        let mut windows = vec![0usize; n_windows];

        for i in 0..(n_windows) {
            windows[i] = ((int >> (i * window_bits)).to_words()[0] as usize) % (window_marker);
        }
        windows
    }

    const IS_CANONICAL: bool = true;
}
impl PrimeField for Scalar {
    type Repr = FieldBytes;

    fn is_odd(self) -> Choice {
        self.0.is_odd()
    }

    fn get_root_of_unity(k: u32) -> Self {
        //if n == 0 ; 2^0 root does not exist
        // if n is more than 32 no root exist
        assert!(k == 0 || k <= 32, "2^{:?} th root does not exist", k);
        if k == 32 {
            println!("{:?}", Scalar(TWO_ADIC_ROOT))
        };
        Scalar(TWO_ADIC_ROOT).power_by((U256::ONE << ((32 - k) as usize)).to_words())
    }

    const MODULUS: &'static str =
        "73eda753299d7d483339d80809a1d80553bda402fffe5bfeffffffff00000001";

    const NUM_BITS: u32 = (ELEMENT_BYTES * 8) as u32;

    const GENERATOR: Self = Scalar(U256::from_u32(7));

    const TWO_ADDICITY: u32 = TWO_ADDICITY;

    fn is_even(self) -> Choice {
        !self.is_odd()
    }
    //two_adic_root=10238227357739495823651030575849232062558860180284477541189508159991286009131
    const TWO_ADIC_ROOT: &'static str =
        "16A2A19EDFE81F20D09B681922C813B4B63683508C2280B93829971F439F0D2B";
}
impl Display for Scalar {
    fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}
// Basic arithmetic operators and assignment operators on Scalar like + , - , * , /

impl Add for Scalar {
    type Output = Scalar;
    fn add(self, other: Self) -> Self {
        add(&self, &other)
    }
}
impl Sub for Scalar {
    type Output = Scalar;
    fn sub(self, other: Scalar) -> Self::Output {
        sub(&self, &other)
    }
}
impl Mul for Scalar {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self {
        mul(&self, &rhs)
    }
}
impl Div for Scalar {
    type Output = Self;
    fn div(self, rhs: Self) -> Self::Output {
        mul(&self, &rhs.invert().unwrap())
    }
}
impl AddAssign for Scalar {
    fn add_assign(&mut self, other: Self) {
        *self = add(self, &other);
    }
}
impl SubAssign for Scalar {
    fn sub_assign(&mut self, other: Self) {
        *self = sub(self, &other);
    }
}
impl MulAssign for Scalar {
    fn mul_assign(&mut self, rhs: Self) {
        *self = mul(self, &rhs)
    }
}
impl DivAssign for Scalar {
    fn div_assign(&mut self, rhs: Self) {
        let rhsinv = (&rhs).invert().unwrap();
        *self = mul(self, &rhsinv)
    }
}
impl Neg for Scalar {
    type Output = Scalar;
    fn neg(self) -> Scalar {
        Scalar::ZERO - self
    }
}
impl From<u128> for Scalar {
    fn from(num: u128) -> Self {
        Scalar(num.into())
    }
}
// conversion of u128,u64,u32,u16,u8 to Scalar
impl From<u64> for Scalar {
    fn from(num: u64) -> Self {
        Scalar(num.into())
    }
}
impl From<u32> for Scalar {
    fn from(num: u32) -> Self {
        Scalar(num.into())
    }
}
impl From<u16> for Scalar {
    fn from(num: u16) -> Self {
        Scalar(U256::from_u16(num))
    }
}
impl From<u8> for Scalar {
    fn from(num: u8) -> Self {
        Scalar(U256::from_u8(num))
    }
}
impl From<[u64; 6]> for Scalar {
    fn from(value: [u64; 6]) -> Self {
        let value = U256::from_words([value[0], value[1], value[2], value[3]]);
        Self::new(value)
    }
}

impl From<U256> for Scalar {
    fn from(value: U256) -> Self {
        Self(value)
    }
}

impl<'a> TryFrom<&'a [u8]> for Scalar {
    type Error = String;

    /// Converts a slice of bytes into a field element; returns error if the value encoded in bytes
    /// is not a valid field element. The bytes are assumed to be in little-endian byte order.
    fn try_from(bytes: &[u8]) -> Result<Self, Self::Error> {
        let value = bytes
            .try_into()
            .map(Self::from_repr)
            .map_err(|error| format!("{error}"))?;
        if value >= Scalar(SCALAR_MODULUS) {
            return Err(format!(
                "cannot convert bytes into a field element: \
                value {value} is greater or equal to the field modulus"
            ));
        }
        Ok(value)
    }
}
impl AsBytes for Scalar {
    fn as_bytes(&self) -> &[u8] {
        let self_ptr: *const Scalar = self;
        unsafe { slice::from_raw_parts(self_ptr as *const u8, ELEMENT_BYTES) }
    }
}
impl Randomizable for Scalar {
    const VALUE_SIZE: usize = ELEMENT_BYTES;

    fn from_random_bytes(bytes: &[u8]) -> Option<Self> {
        Self::try_from(bytes).ok()
    }
}
// SERIALIZATION / DESERIALIZATION
// ------------------------------------------------------------------------------------------------
impl Serializable for Scalar {
    fn write_into<W: ByteWriter>(&self, target: &mut W) {
        target.write_u8_slice(&self.0.to_le_bytes());
    }
}
impl Deserializable for Scalar {
    fn read_from<R: ByteReader>(source: &mut R) -> Result<Self, DeserializationError> {
        let value = source.read_u8_array()?;
        let value = U256::from_le_byte_array(value.into());
        if value >= SCALAR_MODULUS {
            return Err(DeserializationError::InvalidValue(format!(
                "invalid field element: value {} is greater than or equal to the scalar modulus",
                value
            )));
        }
        Ok(Scalar(value))
    }
}
impl ConstantTimeEq for Scalar {
    fn ct_eq(&self, other: &Self) -> Choice {
        self.0.ct_eq(&other.0)
    }
}

pub trait Reduce<U256: Integer>: Sized {
    /// Perform a modular reduction, returning a field element.
    fn from_uint_reduced(n: U256) -> Self;
}
impl Reduce<U256> for Scalar {
    fn from_uint_reduced(w: U256) -> Self {
        let (r, underflow) = w.sbb(&SCALAR_MODULUS, Limb::ZERO);
        let underflow = Choice::from((underflow.0 >> (Limb::BITS - 1)) as u8);
        Self(U256::conditional_select(&w, &r, !underflow))
    }
}
impl ConditionallySelectable for Scalar {
    fn conditional_select(a: &Self, b: &Self, choice: Choice) -> Self {
        Self(U256::conditional_select(&a.0, &b.0, choice))
    }
}

// Returns a+b mod n
fn add(a: &Scalar, b: &Scalar) -> Scalar {
    Scalar(a.0.add_mod(&b.0, &SCALAR_MODULUS))
}
// Returns a-b mod n
pub const fn sub(a: &Scalar, b: &Scalar) -> Scalar {
    Scalar(a.0.sub_mod(&b.0, &SCALAR_MODULUS))
}
// Returns -a mod n
pub const fn neg(a: &Scalar) -> Scalar {
    Scalar(a.0.neg_mod(&SCALAR_MODULUS))
}
//Returns a * b mod n
pub fn mul(a: &Scalar, b: &Scalar) -> Scalar {
    if a == &Scalar::ONE || b == &Scalar::ONE {
        if a == &Scalar::ONE {
            return b.clone();
        } else {
            return a.clone();
        }
    }
    if a == &Scalar::ZERO || b == &Scalar::ZERO {
        return Scalar::ZERO;
    }
    let product = a.0.mul_wide(&b.0);
    let limbs = barrett_reduce(product.0, product.1);
    let words: [u64; 4] = limbs.to_words();
    let out: Scalar = Scalar(U256::from_words(words));
    out
}
// Returns the multiplicative inverse
#[allow(non_snake_case)]
pub fn invert(a: &Scalar) -> CtOption<Scalar> {
    let mut u = *a;
    let mut v = Scalar(SCALAR_MODULUS);
    let mut A = Scalar::ONE;
    let mut C = Scalar::ZERO;

    while !bool::from(u.0.is_zero()) {
        while bool::from(u.0.is_even()) {
            u.shr1();

            let was_odd: bool = A.0.is_odd().into();
            A.shr1();

            if was_odd {
                A += FRAC_SCALAR_MODULUS_2;
                A += Scalar::ONE;
            }
        }

        while bool::from(v.0.is_even()) {
            v.shr1();

            let was_odd: bool = C.0.is_odd().into();
            C.shr1();

            if was_odd {
                C += FRAC_SCALAR_MODULUS_2;
                C += Scalar::ONE;
            }
        }

        if u.0 >= v.0 {
            u -= v;
            A -= C;
        } else {
            v -= u;
            C -= A;
        }
    }

    CtOption::new(C, !a.0.is_zero())
}
#[cfg(feature = "serde")]
#[cfg_attr(docsrs, doc(cfg(feature = "serde")))]
impl Serialize for Scalar {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: ser::Serializer,
    {
        ScalarCore::from(self).serialize(serializer)
    }
}

#[cfg(feature = "serde")]
#[cfg_attr(docsrs, doc(cfg(feature = "serde")))]
impl<'de> Deserialize<'de> for Scalar {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: de::Deserializer<'de>,
    {
        Ok(ScalarCore::deserialize(deserializer)?.into())
    }
}

#[cfg(feature = "bits")]
#[cfg_attr(docsrs, doc(cfg(feature = "bits")))]
impl From<&Scalar> for ScalarBits {
    fn from(scalar: &Scalar) -> ScalarBits {
        scalar.0.to_words().into()
    }
}

#[inline]
#[allow(clippy::too_many_arguments)]
pub(super) const fn barrett_reduce(lo: U256, hi: U256) -> U256 {
    let lo = lo.as_words();
    let hi = hi.as_words();
    let a0 = lo[0];
    let a1 = lo[1];
    let a2 = lo[2];
    let a3 = lo[3];
    let a4 = hi[0];
    let a5 = hi[1];
    let a6 = hi[2];
    let a7 = hi[3];
    let q1: [u64; 5] = [a3, a4, a5, a6, a7];
    let q3 = q1_times_mu_shift_five(&q1);

    let r1: [u64; 5] = [a0, a1, a2, a3, a4];
    let r2: [u64; 5] = q3_times_n_keep_five(&q3);
    let r: [u64; 5] = sub_inner_five(r1, r2);

    // Result is in range (0, 3*n - 1),
    // and 90% of the time, no subtraction will be needed.
    let r = subtract_n_if_necessary(r[0], r[1], r[2], r[3], r[4]);
    let r = subtract_n_if_necessary(r[0], r[1], r[2], r[3], r[4]);
    U256::from_words([r[0], r[1], r[2], r[3]])
}

const fn q1_times_mu_shift_five(q1: &[u64; 5]) -> [u64; 5] {
    // Schoolbook multiplication.

    let (_w0, carry) = mac(0, q1[0], MU[0], 0);
    let (w1, carry) = mac(0, q1[0], MU[1], carry);
    let (w2, carry) = mac(0, q1[0], MU[2], carry);
    let (w3, carry) = mac(0, q1[0], MU[3], carry);
    let (w4, w5) = mac(0, q1[0], MU[4], carry);

    let (_w1, carry) = mac(w1, q1[1], MU[0], 0);
    let (w2, carry) = mac(w2, q1[1], MU[1], carry);
    let (w3, carry) = mac(w3, q1[1], MU[2], carry);
    let (w4, carry) = mac(w4, q1[1], MU[3], carry);
    let (w5, w6) = mac(w5, q1[1], MU[4], carry);

    let (_w2, carry) = mac(w2, q1[2], MU[0], 0);
    let (w3, carry) = mac(w3, q1[2], MU[1], carry);
    let (w4, carry) = mac(w4, q1[2], MU[2], carry);
    let (w5, carry) = mac(w5, q1[2], MU[3], carry);
    let (w6, w7) = mac(w6, q1[2], MU[4], carry);

    let (_w3, carry) = mac(w3, q1[3], MU[0], 0);
    let (w4, carry) = mac(w4, q1[3], MU[1], carry);
    let (w5, carry) = mac(w5, q1[3], MU[2], carry);
    let (w6, carry) = mac(w6, q1[3], MU[3], carry);
    let (w7, w8) = mac(w7, q1[3], MU[4], carry);

    let (_w4, carry) = mac(w4, q1[4], MU[0], 0);
    let (w5, carry) = mac(w5, q1[4], MU[1], carry);
    let (w6, carry) = mac(w6, q1[4], MU[2], carry);
    let (w7, carry) = mac(w7, q1[4], MU[3], carry);
    let (w8, w9) = mac(w8, q1[4], MU[4], carry);

    // let q2 = [_w0, _w1, _w2, _w3, _w4, w5, w6, w7, w8, w9];
    [w5, w6, w7, w8, w9]
}

const fn q3_times_n_keep_five(q3: &[u64; 5]) -> [u64; 5] {
    // Schoolbook multiplication.

    let scalar_modulus = SCALAR_MODULUS.as_words();

    let (w0, carry) = mac(0, q3[0], scalar_modulus[0], 0);
    let (w1, carry) = mac(0, q3[0], scalar_modulus[1], carry);
    let (w2, carry) = mac(0, q3[0], scalar_modulus[2], carry);
    let (w3, carry) = mac(0, q3[0], scalar_modulus[3], carry);
    let (w4, _) = mac(0, q3[0], 0, carry);

    let (w1, carry) = mac(w1, q3[1], scalar_modulus[0], 0);
    let (w2, carry) = mac(w2, q3[1], scalar_modulus[1], carry);
    let (w3, carry) = mac(w3, q3[1], scalar_modulus[2], carry);
    let (w4, _) = mac(w4, q3[1], scalar_modulus[3], carry);

    let (w2, carry) = mac(w2, q3[2], scalar_modulus[0], 0);
    let (w3, carry) = mac(w3, q3[2], scalar_modulus[1], carry);
    let (w4, _) = mac(w4, q3[2], scalar_modulus[2], carry);

    let (w3, carry) = mac(w3, q3[3], scalar_modulus[0], 0);
    let (w4, _) = mac(w4, q3[3], scalar_modulus[1], carry);

    let (w4, _) = mac(w4, q3[4], scalar_modulus[0], 0);

    [w0, w1, w2, w3, w4]
}

#[inline]
#[allow(clippy::too_many_arguments)]
const fn sub_inner_five(l: [u64; 5], r: [u64; 5]) -> [u64; 5] {
    let (w0, borrow) = sbb(l[0], r[0], 0);
    let (w1, borrow) = sbb(l[1], r[1], borrow);
    let (w2, borrow) = sbb(l[2], r[2], borrow);
    let (w3, borrow) = sbb(l[3], r[3], borrow);
    let (w4, _borrow) = sbb(l[4], r[4], borrow);

    // If underflow occurred on the final limb - don't care (= add b^{k+1}).
    [w0, w1, w2, w3, w4]
}

#[inline]
#[allow(clippy::too_many_arguments)]
const fn subtract_n_if_necessary(r0: u64, r1: u64, r2: u64, r3: u64, r4: u64) -> [u64; 5] {
    let scalar_modulus = SCALAR_MODULUS.as_words();

    let (w0, borrow) = sbb(r0, scalar_modulus[0], 0);
    let (w1, borrow) = sbb(r1, scalar_modulus[1], borrow);
    let (w2, borrow) = sbb(r2, scalar_modulus[2], borrow);
    let (w3, borrow) = sbb(r3, scalar_modulus[3], borrow);
    let (w4, borrow) = sbb(r4, 0, borrow);

    // If underflow occurred on the final limb, borrow = 0xfff...fff, otherwise
    // borrow = 0x000...000. Thus, we use it as a mask to conditionally add the
    // scalar_modulus.
    let (w0, carry) = adc(w0, scalar_modulus[0] & borrow, 0);
    let (w1, carry) = adc(w1, scalar_modulus[1] & borrow, carry);
    let (w2, carry) = adc(w2, scalar_modulus[2] & borrow, carry);
    let (w3, carry) = adc(w3, scalar_modulus[3] & borrow, carry);
    let (w4, _carry) = adc(w4, 0, carry);

    [w0, w1, w2, w3, w4]
}
/// Computes `a + b + carry`, returning the result along with the new carry. 64-bit version.
#[inline(always)]
pub const fn adc(a: u64, b: u64, carry: u64) -> (u64, u64) {
    let ret = (a as u128) + (b as u128) + (carry as u128);
    (ret as u64, (ret >> 64) as u64)
}

/// Computes `a - (b + borrow)`, returning the result along with the new borrow. 64-bit version.
#[inline(always)]
pub const fn sbb(a: u64, b: u64, borrow: u64) -> (u64, u64) {
    let ret = (a as u128).wrapping_sub((b as u128) + ((borrow >> 63) as u128));
    (ret as u64, (ret >> 64) as u64)
}

/// Computes `a + (b * c) + carry`, returning the result along with the new carry.
#[inline(always)]
pub const fn mac(a: u64, b: u64, c: u64, carry: u64) -> (u64, u64) {
    let ret = (a as u128) + ((b as u128) * (c as u128)) + (carry as u128);
    (ret as u64, (ret >> 64) as u64)
}
