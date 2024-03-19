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
    subtle::{Choice, ConditionallySelectable, ConstantTimeLess, CtOption, ConstantTimeEq},
    ArrayEncoding, Encoding, Integer, Limb, Random, Zero, U256,
};
use rand::rngs::OsRng;
use traits::traits::{Field, PrimeField};

use crate::scalar::{sbb, adc, mac};

// Size of field elements of this elliptic curve.
pub type FieldSize = <U256 as crypto_bigint::ArrayEncoding>::ByteSize;

/// Byte representation of a base/JubScalar field element of a given curve.
pub type FieldBytes = GenericArray<u8, FieldSize>;

// Number of bytes needed to represent field element
pub const ELEMENT_BYTES: usize = std::mem::size_of::<U256>();

//JubScalar modulus = 6554484396890773809930967563523245729705921265872317281365359162392183254199
pub const JUB_SCALAR_MODULUS: U256 =
    U256::from_be_hex("0e7db4ea6533afa906673b0101343b00a6682093ccc81082d0970e5ed6f72cb7");

// JubScalar_MODULUS/2
pub const FRAC_JUB_SCALAR_MODULUS_2: JubScalar = JubScalar(JUB_SCALAR_MODULUS.shr_vartime(1));

pub const GENERATOR: u32 = 3;

pub const TWO_ADIC_ROOT: U256 = U256::from_be_hex("0e7db4ea6533afa906673b0101343b00a6682093ccc81082d0970e5ed6f72cb6");

//mu = floor({2^512}/p)= 2045593080716281616348203381729468609728209645786990242449482205581148743408809
//                  = 11_AA84_A76F_F6F1_BBE1_C5AE_E5B8_3F04_76A0_1B7C_7219_2B99_F029_7756_5EE9_B246_1CA9
pub const MU: [u64; 5] = [
    0x7756_5EE9_B246_1CA9,
    0x1B7C_7219_2B99_F029,
    0xC5AE_E5B8_3F04_76A0,
    0xAA84_A76F_F6F1_BBE1,
    0x0000_0000_0000_0011,
];

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, PartialOrd, Ord)]
// creating the JubScalar struct
pub struct JubScalar(pub U256);

impl JubScalar {
    // constants zero and one
    pub const ZERO: JubScalar = JubScalar(U256::ZERO);
    pub const ONE: JubScalar = JubScalar(U256::ONE);
    // Returns self  mod n
    pub fn new(other: U256) -> Self {
        JubScalar(CtOption::new(other, other.ct_lt(&JUB_SCALAR_MODULUS)).unwrap())
    }
    pub fn from_repr(bytes: &FieldBytes) -> JubScalar {
        let inner = U256::from_be_byte_array(*bytes);
        <JubScalar as Field>::from_uint_reduced(JubScalar(inner))
    }
    // Shift right by one bit
    pub fn shr1(&mut self) {
        self.0 >>= 1;
    }
    // gives the bytes representation of given JubScalar field element
    pub fn to_bytes(a: &JubScalar) -> FieldBytes {
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

//=====Field implementation =======
//implement Field for JubScalar
impl Field for JubScalar {
    // checks whether the given scakar field element is zero or  not.
    fn is_zero(self) -> bool {
        self == Self::ZERO
    }
    // checks ehether the given JubScalar field element is one or not
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
        let out: JubScalar = JubScalar(U256::from_words(words));
        out
    }
    // returns the multiplicative inverse of the self
    fn invert(self) -> CtOption<JubScalar> {
        invert(&self)
    }
    // Additive and Multiplicative identity of the JubScalar field i.e zero and one
    const ZERO: Self = Self::ZERO;

    const ONE: Self = Self::ONE;

    // returns the random JubScalar field element
    fn random()->Self {
        //return random JubScalar element
        let mut kk = JubScalar(U256::random(&mut OsRng));
        while kk >= JubScalar(JUB_SCALAR_MODULUS) {
            kk -= JubScalar(JUB_SCALAR_MODULUS);
        }
        kk
    }

    fn sqrt(self)->CtOption<Self> {
       // Tonelli-Shank's algorithm for q mod 4 = 3
        // See https://eprint.iacr.org/2012/685.pdf
        // M+1/4= 1638621099222693452482741890880811432426480316468079320341339790598045813550
        // Compute s^((M+1)/4)
        let s = self.power_by([
            0xB425C397B5BDCB2E,
            0x299A0824F3320420,
            0x4199CEC0404D0EC0,
            0x039F6D3A994CEBEA,
        ]);

        CtOption::new(s, (s.square()).ct_eq(&self))
    }
    // Exponentiates the self by pow
    fn power_by<S:AsRef<[u64]>>(self,pow:S)->Self {
        let mut res = Self::ONE;
        for e in pow.as_ref().iter().rev() {
            for i in (0..64).rev() {
                res = res.square();

                if ((*e >> i) & 1) == 1 {
                    res = res * self;
                }
            }
        }

        res
    }

    type BaseField=JubScalar;

    fn cube(self)->Self{
        self*self*self
    }
    fn to_curve_bytes(&self) -> &[u8] {
        self.as_bytes()
    }

    fn to_words(&self) -> Vec<u64> {
        self.0.to_words().into()
    }

    fn from_words( a: &Vec<u64>) -> Self {
        let k = [a[0],a[1],a[2],a[3]];
        let value = U256::from_words(k);
        JubScalar(value)
    }


    const ELEMENT_BYTES: usize  = ELEMENT_BYTES;

    fn  from_uint_reduced(w: Self) -> Self {
        let (r, underflow) = w.0.sbb(&JUB_SCALAR_MODULUS, Limb::ZERO);
        let underflow = Choice::from((underflow.0 >> (Limb::BITS - 1)) as u8);
        Self(U256::conditional_select(&w.0, &r, !underflow))
    }

    fn get_windows(&self, window_bits : usize)->Vec<usize> {

        let int = &self.0;

        let window_marker = 1usize<<window_bits;

        let n_windows = (256/window_bits) + 1;

        let mut windows = vec![0usize;n_windows];

        for i in 0..(n_windows){
            windows[i] = ((int>>(i*window_bits)).to_words()[0] as usize)%(window_marker);
        }
        windows
    
    }

    const IS_CANONICAL: bool = true;



}

impl PrimeField for JubScalar{
    type Repr=FieldBytes;

    fn is_odd(self)->Choice {
        self.0.is_odd()
    }

    fn get_root_of_unity(k:u32)->Self {
        assert!(k==1, "2^{k}th root does not exist");
        JubScalar(TWO_ADIC_ROOT)
    }

    const MODULUS:&'static str="0e7db4ea6533afa906673b0101343b00a6682093ccc81082d0970e5ed6f72cb7";

    const NUM_BITS:u32=(ELEMENT_BYTES*8) as u32;

    const GENERATOR:Self= JubScalar(U256::from_u32(3));

    const TWO_ADDICITY: u32= 1;

    fn is_even(self)->Choice{
        !self.is_odd()
    }

    const TWO_ADIC_ROOT: & 'static str = "0e7db4ea6533afa906673b0101343b00a6682093ccc81082d0970e5ed6f72cb6";


}

impl Display for JubScalar {
    fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}
impl Add for JubScalar {
    type Output = JubScalar;
    fn add(self, other: Self) -> Self {
        add(&self, &other)
    }
}
impl Sub for JubScalar {
    type Output = JubScalar;
    fn sub(self, other: JubScalar) -> Self::Output {
        sub(&self, &other)
    }
}
impl Mul for JubScalar {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self {
        mul(&self, &rhs)
    }
}
impl Div for JubScalar {
    type Output = Self;
    fn div(self, rhs: Self) -> Self::Output {
        mul(&self, &rhs.invert().unwrap())
    }
}
impl AddAssign for JubScalar {
    fn add_assign(&mut self, other: Self) {
        *self = add(self, &other);
    }
}
impl SubAssign for JubScalar {
    fn sub_assign(&mut self, other: Self) {
        *self = sub(self, &other);
    }
}
impl MulAssign for JubScalar {
    fn mul_assign(&mut self, rhs: Self) {
        *self = mul(self, &rhs)
    }
}
impl DivAssign for JubScalar {
    fn div_assign(&mut self, rhs: Self) {
        let rhsinv = (&rhs).invert().unwrap();
        *self = mul(self, &rhsinv)
    }
}
impl Neg for JubScalar {
    type Output = JubScalar;
    fn neg(self) -> JubScalar {
        JubScalar::ZERO - self
    }
}
impl From<u128> for JubScalar {
    fn from(num: u128) -> Self {
        JubScalar(num.into())
    }
}
// conversion of u128,u64,u32,u16,u8 to JubScalar
impl From<u64> for JubScalar {
    fn from(num: u64) -> Self {
        JubScalar(num.into())
    }
}
impl From<u32> for JubScalar {
    fn from(num: u32) -> Self {
        JubScalar(num.into())
    }
}
impl From<u16> for JubScalar {
    fn from(num: u16) -> Self {
        JubScalar(U256::from_u16(num))
    }
}
impl From<u8> for JubScalar {
    fn from(num: u8) -> Self {
        JubScalar(U256::from_u8(num))
    }
}
impl From<[u64; 6]> for JubScalar {
    fn from(value: [u64; 6]) -> Self {
        let value = U256::from_words([value[0],value[1],value[2],value[3]]);
        Self::new(value)
    }
}

impl From<U256> for JubScalar {
    fn from(value: U256) -> Self {
        let value = U256::from_words(value.into());
        Self::new(value)
    }
}

impl<'a> TryFrom<&'a [u8]> for JubScalar {
    type Error = String;

    /// Converts a slice of bytes into a field element; returns error if the value encoded in bytes
    /// is not a valid field element. The bytes are assumed to be in little-endian byte order.
    fn try_from(bytes: &[u8]) -> Result<Self, Self::Error> {
        let value = bytes
            .try_into()
            .map(Self::from_repr)
            .map_err(|error| format!("{error}"))?;
        if value >= JubScalar(JUB_SCALAR_MODULUS) {
            return Err(format!(
                "cannot convert bytes into a field element: \
                value {value} is greater or equal to the field modulus"
            ));
        }
        Ok(value)
    }
}
impl AsBytes for JubScalar {
    fn as_bytes(&self) -> &[u8] {
        let self_ptr: *const JubScalar = self;
        unsafe { slice::from_raw_parts(self_ptr as *const u8, ELEMENT_BYTES) }
    }
}
impl Randomizable for JubScalar {
    const VALUE_SIZE: usize = ELEMENT_BYTES;

    fn from_random_bytes(bytes: &[u8]) -> Option<Self> {
        Self::try_from(bytes).ok()
    }
}
// SERIALIZATION / DESERIALIZATION
// ------------------------------------------------------------------------------------------------
impl Serializable for JubScalar {
    fn write_into<W: ByteWriter>(&self, target: &mut W) {
        target.write_u8_slice(&self.0.to_le_bytes());
    }
}
impl Deserializable for JubScalar {
    fn read_from<R: ByteReader>(source: &mut R) -> Result<Self, DeserializationError> {
        let value = source.read_u8_array()?;
        let value = U256::from_be_byte_array(value.into());
        if value >= JUB_SCALAR_MODULUS {
            return Err(DeserializationError::InvalidValue(format!(
                "invalid field element: value {} is greater than or equal to the field modulus",
                value
            )));
        }
        Ok(JubScalar(value))
    }
}

// Returns a+b mod n
fn add(a: &JubScalar, b: &JubScalar) -> JubScalar {
    JubScalar(a.0.add_mod(&b.0, &JUB_SCALAR_MODULUS))
}
// Returns a-b mod n
pub const fn sub(a: &JubScalar, b: &JubScalar) -> JubScalar {
    JubScalar(a.0.sub_mod(&b.0, &JUB_SCALAR_MODULUS))
}
// Returns -a mod n
pub const fn neg(a: &JubScalar) -> JubScalar {
    JubScalar(a.0.neg_mod(&JUB_SCALAR_MODULUS))
}
//Returns a * b mod n
pub const fn mul(a: &JubScalar, b: &JubScalar) -> JubScalar {
    let product = a.0.mul_wide(&b.0);
    let limbs = barrett_reduce(product.0, product.1);
    let words: [u64; 4] = limbs.to_words();
    let out: JubScalar = JubScalar(U256::from_words(words));
    out
}
// Returns the multiplicative inverse
#[allow(non_snake_case)]
pub fn invert(a: &JubScalar) -> CtOption<JubScalar> {
    let mut u = *a;
    let mut v = JubScalar(JUB_SCALAR_MODULUS);
    let mut A = JubScalar::ONE;
    let mut C = JubScalar::ZERO;

    while !bool::from(u.0.is_zero()) {
        while bool::from(u.0.is_even()) {
            u.shr1();

            let was_odd: bool = A.0.is_odd().into();
            A.shr1();

            if was_odd {
                A += FRAC_JUB_SCALAR_MODULUS_2;
                A += JubScalar::ONE;
            }
        }

        while bool::from(v.0.is_even()) {
            v.shr1();

            let was_odd: bool = C.0.is_odd().into();
            C.shr1();

            if was_odd {
                C += FRAC_JUB_SCALAR_MODULUS_2;
                C += JubScalar::ONE;
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

pub trait Reduce<U256: Integer>: Sized {
    /// Perform a modular reduction, returning a field element.
    fn from_uint_reduced(n: U256) -> Self;
}

impl Reduce<U256> for JubScalar {
    fn from_uint_reduced(w: U256) -> Self {
        let (r, underflow) = w.sbb(&JUB_SCALAR_MODULUS, Limb::ZERO);
        let underflow = Choice::from((underflow.0 >> (Limb::BITS - 1)) as u8);
        Self(U256::conditional_select(&w, &r, !underflow))
    }
}
impl ConditionallySelectable for JubScalar{
    fn conditional_select(a: &Self, b: &Self, choice: Choice) -> Self {
        Self(U256::conditional_select(&a.0, &b.0, choice))
    }
}
impl ConstantTimeEq for JubScalar {
    fn ct_eq(&self, other: &Self) -> Choice {
        self.0.ct_eq(&other.0)
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
#[allow(unused)]
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
#[allow(unused)]
const fn q3_times_n_keep_five(q3: &[u64; 5]) -> [u64; 5] {
    // Schoolbook multiplication.
    let jub_scalar_modulus = JUB_SCALAR_MODULUS.as_words();
    let (w0, carry) = mac(0, q3[0], jub_scalar_modulus[0], 0);
    let (w1, carry) = mac(0, q3[0], jub_scalar_modulus[1], carry);
    let (w2, carry) = mac(0, q3[0], jub_scalar_modulus[2], carry);
    let (w3, carry) = mac(0, q3[0], jub_scalar_modulus[3], carry);
    let (w4, _) = mac(0, q3[0], 0, carry);
    let (w1, carry) = mac(w1, q3[1], jub_scalar_modulus[0], 0);
    let (w2, carry) = mac(w2, q3[1], jub_scalar_modulus[1], carry);
    let (w3, carry) = mac(w3, q3[1], jub_scalar_modulus[2], carry);
    let (w4, _) = mac(w4, q3[1], jub_scalar_modulus[3], carry);
    let (w2, carry) = mac(w2, q3[2], jub_scalar_modulus[0], 0);
    let (w3, carry) = mac(w3, q3[2], jub_scalar_modulus[1], carry);
    let (w4, _) = mac(w4, q3[2], jub_scalar_modulus[2], carry);
    let (w3, carry) = mac(w3, q3[3], jub_scalar_modulus[0], 0);
    let (w4, _) = mac(w4, q3[3], jub_scalar_modulus[1], carry);
    let (w4, _) = mac(w4, q3[4], jub_scalar_modulus[0], 0);
    [w0, w1, w2, w3, w4]
}
#[inline]
#[allow(unused)]
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
#[allow(unused)]
#[allow(clippy::too_many_arguments)]
const fn subtract_n_if_necessary(r0: u64, r1: u64, r2: u64, r3: u64, r4: u64) -> [u64; 5] {
    let jub_scalar_modulus = JUB_SCALAR_MODULUS.as_words();
    let (w0, borrow) = sbb(r0, jub_scalar_modulus[0], 0);
    let (w1, borrow) = sbb(r1, jub_scalar_modulus[1], borrow);
    let (w2, borrow) = sbb(r2, jub_scalar_modulus[2], borrow);
    let (w3, borrow) = sbb(r3, jub_scalar_modulus[3], borrow);
    let (w4, borrow) = sbb(r4, 0, borrow);
    // If underflow occurred on the final limb, borrow = 0xfff...fff, otherwise
    // borrow = 0x000...000. Thus, we use it as a mask to conditionally add the
    // JubScalar_modulus.
    let (w0, carry) = adc(w0, jub_scalar_modulus[0] & borrow, 0);
    let (w1, carry) = adc(w1, jub_scalar_modulus[1] & borrow, carry);
    let (w2, carry) = adc(w2, jub_scalar_modulus[2] & borrow, carry);
    let (w3, carry) = adc(w3, jub_scalar_modulus[3] & borrow, carry);
    let (w4, _carry) = adc(w4, 0, carry);
    [w0, w1, w2, w3, w4]
}