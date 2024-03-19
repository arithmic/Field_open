use core::{
    AsBytes, ByteReader, ByteWriter, Deserializable, DeserializationError, Randomizable,
    Serializable,
};
use std::{
    fmt::{Display, Formatter},
    ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Neg, Shr, Sub, SubAssign},
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

/// Byte representation of a base/BabyjubScalar field element of a given curve.
pub type FieldBytes = GenericArray<u8, FieldSize>;

// Number of bytes needed to represent field element
pub const ELEMENT_BYTES: usize = std::mem::size_of::<U256>();

//prime order of the curve, q=2736030358979909402780800718157159386076813972158567259200215660948447373041
pub const BABYJUB_SCALAR_MODULUS: U256 =
    U256::from_be_hex("060C89CE5C263405370A08B6D0302B0BAB3EEDB83920EE0A677297DC392126F1");

pub const BABYJUB_SCALAR_MODULUS_MINUS_ONE: U256 =
    U256::from_be_hex("060C89CE5C263405370A08B6D0302B0BAB3EEDB83920EE0A677297DC392126F0");

// BabyjubScalar_MODULUS/2
pub const FRAC_BABYJUB_SCALAR_MODULUS_2: BabyjubScalar =
    BabyjubScalar(BABYJUB_SCALAR_MODULUS.shr_vartime(1));

pub const TWO_ADDICITY: u32 = 4;
pub const GENERATOR: u32 = 31;
pub const TWO_ADIC_ROOT: U256 =
    U256::from_be_hex("024E71BAEF132586B8F9DB808928A940B3FC84235D486F20DFB451B0464FEF87");
//MU=floor(2^512/q)=4900460218190528591282491810200361256524647397976035428852112415485855338135554
//0000_0000_0000_002A_523A_3131_1D02_53D5_83A5_2C34_0398_0A0D_1CE0_6FEF_CA2F_BEA9_B90B_6FF4_2CE1_0002
pub const MU: [u64; 5] = [
    0xB90B_6FF4_2CE1_0002,
    0x1CE0_6FEF_CA2F_BEA9,
    0x83A5_2C34_0398_0A0D,
    0x523A_3131_1D02_53D5,
    0x0000_0000_0000_002A,
];

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, PartialOrd, Ord)]
// creating the BabyjubScalar struct
pub struct BabyjubScalar(pub U256);

impl BabyjubScalar {
    // constants zero and one
    pub const ZERO: BabyjubScalar = BabyjubScalar(U256::ZERO);
    pub const ONE: BabyjubScalar = BabyjubScalar(U256::ONE);
    // Returns self  mod n
    pub fn new(other: U256) -> Self {
        BabyjubScalar(CtOption::new(other, other.ct_lt(&BABYJUB_SCALAR_MODULUS)).unwrap())
    }
    // Shift right by one bit
    pub fn shr1(&mut self) {
        self.0 >>= 1;
    }
    // returns the BabyjubScalar field element from its byte representation.
    pub fn from_repr(bytes: &FieldBytes) -> BabyjubScalar {
        let inner = U256::from_be_byte_array(*bytes);
        <BabyjubScalar as Field>::from_uint_reduced(BabyjubScalar(inner))
    }
    // returns the remainder obtained when self is divided by d.
    pub fn rem(self, d: Self) -> Self {
        let mut a = self;
        while a >= d {
            a -= d;
        }
        a
    }
    // gives the bytes representation of given BabyjubScalar field element
    pub fn to_bytes(a: &BabyjubScalar) -> FieldBytes {
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

impl Field for BabyjubScalar {
    // checks whether the given scakar field element is zero or  not.
    fn is_zero(self) -> bool {
        self == Self::ZERO
    }
    // checks ehether the given BabyjubScalar field element is one or not
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
        let out: BabyjubScalar = BabyjubScalar(U256::from_words(words));
        out
    }
    // returns the multiplicative inverse of the self
    fn invert(self) -> CtOption<BabyjubScalar> {
        invert(self)
    }
    // Additive and Multiplicative identity of the BabyjubScalar field i.e zero and one
    const ZERO: Self = Self::ZERO;

    const ONE: Self = Self::ONE;
    const ELEMENT_BYTES: usize = ELEMENT_BYTES;
    // returns the random BabyjubScalar field element
    fn random() -> Self {
        let r = U256::random(&mut OsRng);
        if r < BABYJUB_SCALAR_MODULUS {
            return BabyjubScalar(r);
        } else {
            return BabyjubScalar(r >> 6);
        }
    }
    // sqrt
    fn sqrt(self) -> CtOption<Self> {
        //self^((t-1)/2)
        let mut w = self.power_by(
            U256::from_be_hex("0030644e72e131a029b85045b68181585d59f76dc1c90770533b94bee1c90937")
                .to_words(),
        );
        // v is the number of leading zeros bit in the bit representation of q-1
        let mut v = 4 as u32; //v
        let mut x = self * w; //x
        let mut b = x * w; //b
        let multiplicative_generator = BabyjubScalar(U256::from_u32(GENERATOR));
        //g^((q-1)/2^5)
        let mut z = multiplicative_generator.power_by(
            U256::from_be_hex("0060c89ce5c263405370a08b6d0302b0bab3eedb83920ee0a677297dc392126f")
                .to_words(),
        ); // z
        while !b.is_one() {
            let mut k = 0;
            let mut b2k = b;
            while !b2k.is_one() {
                b2k = b2k.square();
                k += 1;
            }
            let j = v - k;
            w = z;
            for _ in 1..j {
                w = w.square();
            }

            z = w.square();
            b *= z;
            x *= w;
            v = k;
        }
        CtOption::new(x, square(x).ct_eq(&self))
    }

    //Exponentiates the self by pow
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
    type BaseField = BabyjubScalar;

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
        BabyjubScalar(value)
    }

    fn from_uint_reduced(w: Self) -> Self {
        let (r, underflow) = w.0.sbb(&BABYJUB_SCALAR_MODULUS, Limb::ZERO);
        let underflow = Choice::from((underflow.0 >> (Limb::BITS - 1)) as u8);
        Self(U256::conditional_select(&w.0, &r, !underflow))
    }
    // fn frobenius_map(&mut self, power: usize){
    //     unimplemented!()
    // }

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
impl PrimeField for BabyjubScalar {
    type Repr = FieldBytes;

    fn is_odd(self) -> Choice {
        self.0.is_odd()
    }

    fn get_root_of_unity(k: u32) -> Self {
        //if n == 0 ; 2^0 root does not exist
        // if n is more than 32 no root exist
        assert!(k == 0 || k <= 4, "2^{:?} th root does not exist", k);
        if k == 4 {
            BabyjubScalar(TWO_ADIC_ROOT)
        } else {
            //TWO_ADIC_ROOT: & 'static str = "005282DB87529CFA3F0464519C8B0FA5AD187148E11A61616070024F42F8EF94";
            BabyjubScalar(TWO_ADIC_ROOT).power_by((U256::ONE << ((4 - k) as usize)).to_words())
        }
    }

    const MODULUS: &'static str =
        "060C89CE5C263405370A08B6D0302B0BAB3EEDB83920EE0A677297DC392126F1";

    const NUM_BITS: u32 = (ELEMENT_BYTES * 8) as u32;

    const GENERATOR: Self = BabyjubScalar(U256::from_u32(31));

    const TWO_ADDICITY: u32 = TWO_ADDICITY;

    fn is_even(self) -> Choice {
        !self.is_odd()
    }
    //two_adic_root=1043224705284028335988439394520573142339627108237299665735065585269676699527
    const TWO_ADIC_ROOT: &'static str =
        "024E71BAEF132586B8F9DB808928A940B3FC84235D486F20DFB451B0464FEF87";
}

impl Display for BabyjubScalar {
    fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}
// // Basic arithmetic operators and assignment operators on BabyjubScalar like + , - , * , /

impl Add for BabyjubScalar {
    type Output = BabyjubScalar;
    fn add(self, other: Self) -> Self {
        add(&self, &other)
    }
}
impl Sub for BabyjubScalar {
    type Output = BabyjubScalar;
    fn sub(self, other: BabyjubScalar) -> Self::Output {
        sub(&self, &other)
    }
}
impl Mul for BabyjubScalar {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self {
        mul(&self, &rhs)
    }
}
impl Div for BabyjubScalar {
    type Output = Self;
    fn div(self, rhs: Self) -> Self::Output {
        mul(&self, &rhs.invert().unwrap())
    }
}
impl AddAssign for BabyjubScalar {
    fn add_assign(&mut self, other: Self) {
        *self = add(self, &other);
    }
}
impl SubAssign for BabyjubScalar {
    fn sub_assign(&mut self, other: Self) {
        *self = sub(self, &other);
    }
}
impl MulAssign for BabyjubScalar {
    fn mul_assign(&mut self, rhs: Self) {
        *self = mul(self, &rhs)
    }
}
impl DivAssign for BabyjubScalar {
    fn div_assign(&mut self, rhs: Self) {
        let rhsinv = (&rhs).invert().unwrap();
        *self = mul(self, &rhsinv)
    }
}
impl Neg for BabyjubScalar {
    type Output = BabyjubScalar;
    fn neg(self) -> BabyjubScalar {
        BabyjubScalar::ZERO - self
    }
}
impl From<u128> for BabyjubScalar {
    fn from(num: u128) -> Self {
        BabyjubScalar(num.into())
    }
}
// conversion of u128,u64,u32,u16,u8 to BabyjubScalar
impl From<u64> for BabyjubScalar {
    fn from(num: u64) -> Self {
        BabyjubScalar(num.into())
    }
}
impl From<u32> for BabyjubScalar {
    fn from(num: u32) -> Self {
        BabyjubScalar(num.into())
    }
}
impl From<u16> for BabyjubScalar {
    fn from(num: u16) -> Self {
        BabyjubScalar(U256::from_u16(num))
    }
}
impl From<u8> for BabyjubScalar {
    fn from(num: u8) -> Self {
        BabyjubScalar(U256::from_u8(num))
    }
}
impl From<[u64; 6]> for BabyjubScalar {
    fn from(value: [u64; 6]) -> Self {
        let value = U256::from_words([value[0], value[1], value[2], value[3]]);
        Self::new(value)
    }
}

impl From<U256> for BabyjubScalar {
    fn from(value: U256) -> Self {
        Self(value)
    }
}

impl<'a> TryFrom<&'a [u8]> for BabyjubScalar {
    type Error = String;

    /// Converts a slice of bytes into a field element; returns error if the value encoded in bytes
    /// is not a valid field element. The bytes are assumed to be in little-endian byte order.
    fn try_from(bytes: &[u8]) -> Result<Self, Self::Error> {
        let value = bytes
            .try_into()
            .map(Self::from_repr)
            .map_err(|error| format!("{error}"))?;
        if value >= BabyjubScalar(BABYJUB_SCALAR_MODULUS) {
            return Err(format!(
                "cannot convert bytes into a field element: \
                value {value} is greater or equal to the field modulus"
            ));
        }
        Ok(value)
    }
}
impl AsBytes for BabyjubScalar {
    fn as_bytes(&self) -> &[u8] {
        let self_ptr: *const BabyjubScalar = self;
        unsafe { slice::from_raw_parts(self_ptr as *const u8, ELEMENT_BYTES) }
    }
}
impl Randomizable for BabyjubScalar {
    const VALUE_SIZE: usize = ELEMENT_BYTES;

    fn from_random_bytes(bytes: &[u8]) -> Option<Self> {
        Self::try_from(bytes).ok()
    }
}
// // SERIALIZATION / DESERIALIZATION
// // ------------------------------------------------------------------------------------------------
impl Serializable for BabyjubScalar {
    fn write_into<W: ByteWriter>(&self, target: &mut W) {
        target.write_u8_slice(&self.0.to_le_bytes());
    }
}
impl Deserializable for BabyjubScalar {
    fn read_from<R: ByteReader>(source: &mut R) -> Result<Self, DeserializationError> {
        let value = source.read_u8_array()?;
        let value = U256::from_le_byte_array(value.into());
        if value >= BABYJUB_SCALAR_MODULUS {
            return Err(DeserializationError::InvalidValue(format!(
                "invalid field element: value {} is greater than or equal to the field modulus",
                value
            )));
        }
        Ok(BabyjubScalar(value))
    }
}
impl ConstantTimeEq for BabyjubScalar {
    fn ct_eq(&self, other: &Self) -> Choice {
        self.0.ct_eq(&other.0)
    }
}

pub trait Reduce<U256: Integer>: Sized {
    /// Perform a modular reduction, returning a field element.
    fn from_uint_reduced(n: U256) -> Self;
}
impl Reduce<U256> for BabyjubScalar {
    fn from_uint_reduced(w: U256) -> Self {
        let (r, underflow) = w.sbb(&BABYJUB_SCALAR_MODULUS, Limb::ZERO);
        let underflow = Choice::from((underflow.0 >> (Limb::BITS - 1)) as u8);
        Self(U256::conditional_select(&w, &r, !underflow))
    }
}
impl ConditionallySelectable for BabyjubScalar {
    fn conditional_select(a: &Self, b: &Self, choice: Choice) -> Self {
        Self(U256::conditional_select(&a.0, &b.0, choice))
    }
}

// Returns a+b mod n
fn add(a: &BabyjubScalar, b: &BabyjubScalar) -> BabyjubScalar {
    BabyjubScalar(a.0.add_mod(&b.0, &BABYJUB_SCALAR_MODULUS))
}
// Returns a-b mod n
pub const fn sub(a: &BabyjubScalar, b: &BabyjubScalar) -> BabyjubScalar {
    BabyjubScalar(a.0.sub_mod(&b.0, &BABYJUB_SCALAR_MODULUS))
}
// Returns -a mod n
pub const fn neg(a: &BabyjubScalar) -> BabyjubScalar {
    BabyjubScalar(a.0.neg_mod(&BABYJUB_SCALAR_MODULUS))
}
pub fn square(a: BabyjubScalar) -> BabyjubScalar {
    a.square()
}
//Returns a * b mod n
pub const fn mul(a: &BabyjubScalar, b: &BabyjubScalar) -> BabyjubScalar {
    let product = a.0.mul_wide(&b.0);
    let limbs = barrett_reduce(product.0, product.1);
    let words: [u64; 4] = limbs.to_words();
    let out: BabyjubScalar = BabyjubScalar(U256::from_words(words));
    out
}
// Returns the multiplicative inverse
#[allow(non_snake_case)]
pub fn invert(a: BabyjubScalar) -> CtOption<BabyjubScalar> {
    #[allow(non_snake_case)]
    let element = a;
    let mut u = element; //removed the dereference
    let mut v = BabyjubScalar(BABYJUB_SCALAR_MODULUS);
    let mut A = BabyjubScalar(U256::from_u8(1));
    let mut C = BabyjubScalar(U256::ZERO);
    while !bool::from(u.is_zero()) {
        while bool::from(u.0.is_even()) {
            (&mut u).shr1();
            let was_odd: bool = A.is_odd().into();
            (&mut A).shr1();
            if was_odd {
                A = BabyjubScalar(
                    A.0.add_mod(&FRAC_BABYJUB_SCALAR_MODULUS_2.0, &BABYJUB_SCALAR_MODULUS),
                );
                A = BabyjubScalar(A.0.add_mod(&U256::ONE, &BABYJUB_SCALAR_MODULUS));
            }
        }
        while bool::from(v.0.is_even()) {
            (&mut v).shr1();
            let was_odd: bool = C.is_odd().into();
            (&mut C).shr1();
            if was_odd {
                C = BabyjubScalar(
                    C.0.add_mod(&FRAC_BABYJUB_SCALAR_MODULUS_2.0, &BABYJUB_SCALAR_MODULUS),
                );
                C = BabyjubScalar(C.0.add_mod(&U256::ONE, &BABYJUB_SCALAR_MODULUS));
            }
        }
        if u >= v {
            u = BabyjubScalar(u.0.sub_mod(&v.0, &BABYJUB_SCALAR_MODULUS));
            A = BabyjubScalar(A.0.sub_mod(&C.0, &BABYJUB_SCALAR_MODULUS));
        } else {
            v = BabyjubScalar(v.0.sub_mod(&u.0, &BABYJUB_SCALAR_MODULUS));
            C = BabyjubScalar(C.0.sub_mod(&A.0, &BABYJUB_SCALAR_MODULUS));
        }
    }
    CtOption::new(BabyjubScalar(C.0), !element.0.is_zero())
}

#[cfg(feature = "serde")]
#[cfg_attr(docsrs, doc(cfg(feature = "serde")))]
impl Serialize for BabyjubScalar {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: ser::Serializer,
    {
        BabyjubScalarCore::from(self).serialize(serializer)
    }
}

#[cfg(feature = "serde")]
#[cfg_attr(docsrs, doc(cfg(feature = "serde")))]
impl<'de> Deserialize<'de> for BabyjubScalar {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: de::Deserializer<'de>,
    {
        Ok(BabyjubScalarCore::deserialize(deserializer)?.into())
    }
}

#[cfg(feature = "bits")]
#[cfg_attr(docsrs, doc(cfg(feature = "bits")))]
impl From<&BabyjubScalar> for BabyjubScalarBits {
    fn from(BabyjubScalar: &BabyjubScalar) -> BabyjubScalarBits {
        BabyjubScalar.0.to_words().into()
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

    let babyjub_scalar_modulus = BABYJUB_SCALAR_MODULUS.as_words();

    let (w0, carry) = mac(0, q3[0], babyjub_scalar_modulus[0], 0);
    let (w1, carry) = mac(0, q3[0], babyjub_scalar_modulus[1], carry);
    let (w2, carry) = mac(0, q3[0], babyjub_scalar_modulus[2], carry);
    let (w3, carry) = mac(0, q3[0], babyjub_scalar_modulus[3], carry);
    let (w4, _) = mac(0, q3[0], 0, carry);

    let (w1, carry) = mac(w1, q3[1], babyjub_scalar_modulus[0], 0);
    let (w2, carry) = mac(w2, q3[1], babyjub_scalar_modulus[1], carry);
    let (w3, carry) = mac(w3, q3[1], babyjub_scalar_modulus[2], carry);
    let (w4, _) = mac(w4, q3[1], babyjub_scalar_modulus[3], carry);

    let (w2, carry) = mac(w2, q3[2], babyjub_scalar_modulus[0], 0);
    let (w3, carry) = mac(w3, q3[2], babyjub_scalar_modulus[1], carry);
    let (w4, _) = mac(w4, q3[2], babyjub_scalar_modulus[2], carry);

    let (w3, carry) = mac(w3, q3[3], babyjub_scalar_modulus[0], 0);
    let (w4, _) = mac(w4, q3[3], babyjub_scalar_modulus[1], carry);

    let (w4, _) = mac(w4, q3[4], babyjub_scalar_modulus[0], 0);

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
    let babyjub_scalar_modulus = BABYJUB_SCALAR_MODULUS.as_words();

    let (w0, borrow) = sbb(r0, babyjub_scalar_modulus[0], 0);
    let (w1, borrow) = sbb(r1, babyjub_scalar_modulus[1], borrow);
    let (w2, borrow) = sbb(r2, babyjub_scalar_modulus[2], borrow);
    let (w3, borrow) = sbb(r3, babyjub_scalar_modulus[3], borrow);
    let (w4, borrow) = sbb(r4, 0, borrow);

    // If underflow occurred on the final limb, borrow = 0xfff...fff, otherwise
    // borrow = 0x000...000. Thus, we use it as a mask to conditionally add the
    // BabyjubScalar_modulus.
    let (w0, carry) = adc(w0, babyjub_scalar_modulus[0] & borrow, 0);
    let (w1, carry) = adc(w1, babyjub_scalar_modulus[1] & borrow, carry);
    let (w2, carry) = adc(w2, babyjub_scalar_modulus[2] & borrow, carry);
    let (w3, carry) = adc(w3, babyjub_scalar_modulus[3] & borrow, carry);
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
