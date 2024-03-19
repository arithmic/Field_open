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
    rand_core::OsRng,
    subtle::{Choice, ConditionallySelectable, ConstantTimeEq, ConstantTimeLess, CtOption},
    ArrayEncoding, Encoding, Integer, Limb, Random, Zero, U256,
};
use traits::traits::{Extensible, Field, PrimeField};

use crate::fp2::Fp2;

// Size of field elements of this elliptic curve.
pub type FieldSize = <U256 as crypto_bigint::ArrayEncoding>::ByteSize;

/// Byte representation of a base/Fp field element of a given curve.
pub type FieldBytes = GenericArray<u8, FieldSize>;

pub const ELEMENT_BYTES: usize = std::mem::size_of::<U256>();
// modulus of the base field , p= 21888242871839275222246405745257275088696311157297823662689037894645226208583
pub(crate) const MODULUS: U256 =
    U256::from_be_hex("30644E72E131A029B85045B68181585D97816A916871CA8D3C208C16D87CFD47");
//modulus-1
pub const MODULUS_MINUS_ONE: U256 =
    U256::from_be_hex("30644E72E131A029B85045B68181585D97816A916871CA8D3C208C16D87CFD46");

pub const M_PLUS_ONE_FOUR: Fp = Fp(U256::from_be_hex(
    "0c19139cb84c680a6e14116da060561765e05aa45a1c72a34f082305b61f3f52",
));
pub const MODULUS_MINUS_ONE_DIV_TWO: Fp = Fp(MODULUS_MINUS_ONE.shr_vartime(1));

//two adic root of unity
pub const TWO_ADIC_ROOT_OF_UNITY: U256 =
    U256::from_be_hex("30644E72E131A029B85045B68181585D97816A916871CA8D3C208C16D87CFD46");


// 2^{512}/p
//0x54a47462623a04a7ab074a5868073013ae965e1767cd4c086f3aed8a19bf90e51
pub const MU: [u64; 5] = [
    0xf3ae_d8a1_9bf9_0e51,
    0xe965_e176_7cd4_c086,
    0xb074_a586_8073_013a,
    0x4a47_4626_23a0_4a7a,
    0x0000_0000_0000_0005,
];

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, PartialOrd, Ord)]
pub struct Fp(pub U256); //defining the Fp struct

impl Fp {
    // constants zero and one
    pub const ZERO: Fp = Fp(U256::ZERO);
    pub const ONE: Fp = Fp(U256::ONE);

    // Returns self  mod n
    pub fn new(other: U256) -> Self {
        Fp(CtOption::new(other, other.ct_lt(&MODULUS)).unwrap())
    }

    // Shift right by one bit
    pub fn shr1(&mut self) {
        self.0 >>= 1;
    }

    // returns the Fp field element from its byte representation.
    pub fn from_repr(bytes: &FieldBytes) -> Fp {
        let inner = U256::from_be_byte_array(*bytes);
        <Fp as Field>::from_uint_reduced(Fp(inner))
    }

    // returns the remainder obtained when self is divided by d.
    pub fn rem(self, d: Self) -> Self {
        let mut a = self;
        while a >= d {
            a -= d;
        }
        a
    }
    // gives the bytes representation of given Fp field element
    pub fn to_bytes(a: &Fp) -> FieldBytes {
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
impl Field for Fp {
    // checks whether the given scakar field element is zero or  not.
    fn is_zero(self) -> bool {
        self == Self::ZERO
    }
    // checks ehether the given Fp field element is one or not
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
        let out: Fp = Fp(U256::from_words(words));
        out
    }
    // returns the multiplicative inverse of the self
    fn invert(self) -> CtOption<Fp> {
        invert(&self)
    }
    // Additive and Multiplicative identity of the Fp field i.e zero and one
    const ZERO: Self = Self::ZERO;

    const ONE: Self = Self::ONE;
    const ELEMENT_BYTES: usize = ELEMENT_BYTES;

    // returns the random Fp field element
    fn random() -> Self {
        let r = U256::random(&mut OsRng);
        if r < MODULUS {
            return Fp(r);
        } else {
            return Fp(r >> 3);
        }
    }
    fn sqrt(self) -> CtOption<Self> {
        let k = self.power_by(&M_PLUS_ONE_FOUR.0.to_words());
        assert_eq!(k.square(), self);
        CtOption::new(k, k.square().0.ct_eq(&self.0))
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
    type BaseField = Fp;

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
        Fp(value)
    }

    fn from_uint_reduced(w: Self) -> Self {
        let (r, underflow) = w.0.sbb(&MODULUS, Limb::ZERO);
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

impl PrimeField for Fp {
    type Repr = FieldBytes;

    fn is_odd(self) -> Choice {
        self.0.is_odd()
    }

    fn get_root_of_unity(k: u32) -> Self {
        assert!(k == 1, "2^{k}th root does not exist");
        Fp(TWO_ADIC_ROOT_OF_UNITY)
    }

    const MODULUS: &'static str =
        "30644E72E131A029B85045B68181585D97816A916871CA8D3C208C16D87CFD47";

    const NUM_BITS: u32 = (ELEMENT_BYTES * 8) as u32;

    const GENERATOR: Self = Fp(U256::from_u32(3));

    const TWO_ADDICITY: u32 = 1;

    fn is_even(self) -> Choice {
        !self.is_odd()
    }
    //two_adic_root= 21888242871839275222246405745257275088696311157297823662689037894645226208582
    const TWO_ADIC_ROOT: &'static str =
        "30644E72E131A029B85045B68181585D97816A916871CA8D3C208C16D87CFD46";
}

impl Extensible<2> for Fp {
    /// Returns a product of `a` and `b` in the field defined by this extension.
    fn mul(a: [Self; 2], b: [Self; 2]) -> [Self; 2] {
        let mut aa = a[0];
        aa.mul_assign(b[0]);
        let mut bb = a[1];
        bb.mul_assign(b[1]);
        let mut o = b[0];
        o.add_assign(b[1]);
        let mut d1 = a[1];
        d1.add_assign(a[0]);
        d1.mul_assign(o);
        d1.sub_assign(aa);
        d1.sub_assign(bb);
        let mut d0 = aa;
        d0.sub_assign(bb);
        [d0, d1]
    }
    // Returns a product of `a` and `b` in the field defined by this extension. `b` represents
    /// an element in the base field.
    fn mul_base(a: [Self; 2], b: Self) -> [Self; 2] {
        [a[0] * b, a[1] * b]
    }
    // returns the square of element of extension Fp2
    fn square(a: [Self; 2]) -> [Self; 2] {
        let mut ab = a[0];
        ab.mul_assign(a[1]);
        let mut c0c1 = a[0];
        c0c1.add_assign(a[1]);
        let mut c0 = a[1];
        c0 = c0.neg();
        c0.add_assign(a[0]);
        c0.mul_assign(c0c1);
        let mut c1 = ab;
        c1.add_assign(ab);
        [c0, c1]
    }
    // returns the square root of element
    fn sqrt(a: [Self; 2]) -> CtOption<[Self; 2]> {
        // Algorithm 9, https://eprint.iacr.org/2012/685.pdf

        if a[0].is_zero() & a[1].is_zero() {
            CtOption::new(a, Choice::from(1))
        } else {
            // a1 = self^((q - 3) / 4)
            let value = U256::from_be_hex(
                "0c19139cb84c680a6e14116da060561765e05aa45a1c72a34f082305b61f3f51",
            );
            let mut a1 = Fp2 { c0: a[0], c1: a[1] }.power_by(&value.to_words());
            let mut alpha = a1;
            alpha = alpha.square();
            alpha.mul_assign(Fp2 { c0: a[0], c1: a[1] });
            let mut a0 = alpha;
            a0.frobenius_map(1); //
            a0.mul_assign(alpha);

            let neg1 = Fp2 {
                c0: Fp(MODULUS_MINUS_ONE),
                c1: Self::ZERO,
            };

            if a0 == neg1 {
                CtOption::new([a[0], a[1]], Choice::from(0))
            } else {
                a1.mul_assign(Fp2 { c0: a[0], c1: a[1] });

                if alpha == neg1 {
                    a1.mul_assign(Fp2 {
                        c0: Self::ZERO,
                        c1: Self::ONE,
                    });
                } else {
                    alpha.add_assign(Fp2::one());
                    // alpha = alpha^((q - 1) / 2)
                    alpha = alpha.power_by(
                        &U256::from_be_hex(
                            "183227397098d014dc2822db40c0ac2ecbc0b548b438e5469e10460b6c3e7ea3",
                        )
                        .to_words(),
                    );
                    a1.mul_assign(alpha);
                }
                CtOption::new([a1.c0, a1.c1], Choice::from(1))
            }
        }
    }

    // returns the inverse of extension element Fp2
    fn invert(a: [Self; 2]) -> CtOption<[Self; 2]> {
        if a[0].is_zero() & a[1].is_zero() {
            CtOption::new([a[0], a[1]], Choice::from(0u8))
        } else {
            let x: Fp2<Fp> = Fp2::new(a[0], a[1]);
            let numerator: Fp2<Fp> = Fp2::new(x.c0, -x.c1);
            let norm = x.norm();
            let denom_inv = norm.invert().unwrap();
            let d0 = numerator.c0 * denom_inv;
            let d1 = numerator.c1 * denom_inv;
            let inv = Fp2 { c0: d0, c1: d1 };
            CtOption::new([inv.c0, inv.c1], Choice::from(1))
        }
    }
}

impl Extensible<3> for Fp {
    /// Returns a product of `a` and `b` in the field defined by this extension.
    fn mul(_a: [Self; 3], _b: [Self; 3]) -> [Self; 3] {
        unimplemented!()
    }
    // Returns a product of `a` and `b` in the field defined by this extension. `b` represents
    /// an element in the base field.
    fn mul_base(_a: [Self; 3], _b: Self) -> [Self; 3] {
        unimplemented!()
    }

    fn square(_a: [Self; 3]) -> [Self; 3] {
        unimplemented!()
    }

    fn sqrt(_a: [Self; 3]) -> CtOption<[Self; 3]> {
        unimplemented!()
    }

    fn invert(_a: [Self; 3]) -> CtOption<[Self; 3]> {
        unimplemented!()
    }
}

impl Display for Fp {
    fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

// Basic arithmetic operators and assignment operators on FieldElement like + , - , * , /
impl Add for Fp {
    type Output = Fp;

    fn add(self, other: Self) -> Self {
        add(&self, &other)
    }
}
impl Sub for Fp {
    type Output = Fp;
    fn sub(self, other: Self) -> Self::Output {
        sub(&self, &other)
    }
}
impl Mul for Fp {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self {
        mul(&self, &rhs)
    }
}
impl Div for Fp {
    type Output = Self;
    fn div(self, rhs: Self) -> Self::Output {
        mul(&self, &rhs.invert().unwrap())
    }
}
impl AddAssign for Fp {
    fn add_assign(&mut self, other: Self) {
        *self = add(self, &other);
    }
}
impl SubAssign for Fp {
    fn sub_assign(&mut self, other: Self) {
        *self = sub(self, &other);
    }
}
impl MulAssign for Fp {
    fn mul_assign(&mut self, rhs: Self) {
        *self = mul(self, &rhs)
    }
}
impl DivAssign for Fp {
    fn div_assign(&mut self, rhs: Self) {
        let rhsinv = (&rhs).invert().unwrap();
        *self = mul(self, &rhsinv)
    }
}
impl Neg for Fp {
    type Output = Self;
    fn neg(self) -> Self::Output {
        neg(&self)
    }
}
// conversions from u128,u64,u32,u16,u8 to field element.
impl From<u128> for Fp {
    fn from(num: u128) -> Self {
        Self::new(U256::from_u128(num))
    }
}
impl From<u64> for Fp {
    fn from(num: u64) -> Self {
        Self(U256::from_u64(num))
    }
}
impl From<u32> for Fp {
    fn from(num: u32) -> Self {
        Self(U256::from_u32(num))
    }
}
impl From<u16> for Fp {
    fn from(num: u16) -> Self {
        Self(U256::from_u16(num))
    }
}
impl From<u8> for Fp {
    fn from(num: u8) -> Self {
        Self(U256::from_u8(num))
    }
}
impl From<[u64; 6]> for Fp {
    fn from(value: [u64; 6]) -> Self {
        let value = U256::from_words([value[0], value[1], value[2], value[3]]);
        Self::new(value)
    }
}

impl From<U256> for Fp {
    fn from(value: U256) -> Self {
        let value = U256::from_words(value.into());
        Self::new(value)
    }
}

impl<'a> TryFrom<&'a [u8]> for Fp {
    type Error = String;

    /// Converts a slice of bytes into a field element; returns error if the value encoded in bytes
    /// is not a valid field element. The bytes are assumed to be in little-endian byte order.
    fn try_from(bytes: &[u8]) -> Result<Self, Self::Error> {
        let value = bytes
            .try_into()
            .map(Self::from_repr)
            .map_err(|error| format!("{error}"))?;
        if value >= Fp(MODULUS) {
            return Err(format!(
                "cannot convert bytes into a field element: \
                value {value} is greater or equal to the field modulus"
            ));
        }
        Ok(value)
    }
}
impl AsBytes for Fp {
    fn as_bytes(&self) -> &[u8] {
        // TODO: take endianness into account
        let self_ptr: *const Fp = self;
        unsafe { slice::from_raw_parts(self_ptr as *const u8, ELEMENT_BYTES) }
    }
}
impl Randomizable for Fp {
    const VALUE_SIZE: usize = ELEMENT_BYTES;

    fn from_random_bytes(bytes: &[u8]) -> Option<Self> {
        Self::try_from(bytes).ok()
    }
}
impl AsRef<[u8]> for Fp {
    #[inline]
    fn as_ref(&self) -> &[u8] {
        self.as_bytes()
    }
}

impl ConstantTimeEq for Fp {
    fn ct_eq(&self, other: &Self) -> Choice {
        self.0.ct_eq(&other.0)
    }
}

pub trait Reduce<U256: Integer>: Sized {
    /// Perform a modular reduction, returning a field element.
    fn from_uint_reduced(n: U256) -> Self;
}
impl Reduce<U256> for Fp {
    fn from_uint_reduced(w: U256) -> Self {
        let (r, underflow) = w.sbb(&MODULUS, Limb::ZERO);
        let underflow = Choice::from((underflow.0 >> (Limb::BITS - 1)) as u8);
        Self(U256::conditional_select(&w, &r, !underflow))
    }
}
impl ConditionallySelectable for Fp {
    fn conditional_select(a: &Self, b: &Self, choice: Choice) -> Self {
        Self(U256::conditional_select(&a.0, &b.0, choice))
    }
}

// Returns a+b mod n
fn add(a: &Fp, b: &Fp) -> Fp {
    Fp(a.0.add_mod(&b.0, &MODULUS))
}

// Returns a-b mod n
pub const fn sub(a: &Fp, b: &Fp) -> Fp {
    Fp(a.0.sub_mod(&b.0, &MODULUS))
}

// Returns -a mod n
pub const fn neg(a: &Fp) -> Fp {
    Fp(a.0.neg_mod(&MODULUS))
}
//Returns a * b mod n
pub const fn mul(a: &Fp, b: &Fp) -> Fp {
    let product = a.0.mul_wide(&b.0);
    let limbs = barrett_reduce(product.0, product.1);
    let words: [u64; 4] = limbs.to_words();
    let out: Fp = Fp(U256::from_words(words));
    out
}
// Returns the multiplicative inverse
#[allow(non_snake_case)]
pub fn invert(a: &Fp) -> CtOption<Fp> {
    let mut u = *a;
    let mut v = Fp(MODULUS);
    let mut A = Fp::ONE;
    let mut C = Fp::ZERO;

    while !bool::from(u.0.is_zero()) {
        while bool::from(u.0.is_even()) {
            u.shr1();

            let was_odd: bool = A.0.is_odd().into();
            A.shr1();

            if was_odd {
                A += MODULUS_MINUS_ONE_DIV_TWO;
                A += Fp::ONE;
            }
        }

        while bool::from(v.0.is_even()) {
            v.shr1();

            let was_odd: bool = C.0.is_odd().into();
            C.shr1();

            if was_odd {
                C += MODULUS_MINUS_ONE_DIV_TWO;
                C += Fp::ONE;
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
impl Serialize for Fp {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: ser::Serializer,
    {
        FpCore::from(self).serialize(serializer)
    }
}

#[cfg(feature = "serde")]
#[cfg_attr(docsrs, doc(cfg(feature = "serde")))]
impl<'de> Deserialize<'de> for Fp {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: de::Deserializer<'de>,
    {
        Ok(FpCore::deserialize(deserializer)?.into())
    }
}

#[cfg(feature = "bits")]
#[cfg_attr(docsrs, doc(cfg(feature = "bits")))]
impl From<&Fp> for FpBits {
    fn from(Fp: &Fp) -> FpBits {
        Fp.0.to_words().into()
    }
}

impl Serializable for Fp {
    fn write_into<W: ByteWriter>(&self, target: &mut W) {
        target.write_u8_slice(&self.0.to_le_bytes());
    }
}
impl Deserializable for Fp {
    fn read_from<R: ByteReader>(source: &mut R) -> Result<Self, DeserializationError> {
        let value = source.read_u8_array()?;
        let value = U256::from_le_byte_array(value.into());
        if value >= MODULUS {
            return Err(DeserializationError::InvalidValue(format!(
                "invalid field element: value {} is greater than or equal to the field modulus",
                value
            )));
        }
        Ok(Fp(value))
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

    let fp_modulus = MODULUS.as_words();

    let (w0, carry) = mac(0, q3[0], fp_modulus[0], 0);
    let (w1, carry) = mac(0, q3[0], fp_modulus[1], carry);
    let (w2, carry) = mac(0, q3[0], fp_modulus[2], carry);
    let (w3, carry) = mac(0, q3[0], fp_modulus[3], carry);
    let (w4, _) = mac(0, q3[0], 0, carry);

    let (w1, carry) = mac(w1, q3[1], fp_modulus[0], 0);
    let (w2, carry) = mac(w2, q3[1], fp_modulus[1], carry);
    let (w3, carry) = mac(w3, q3[1], fp_modulus[2], carry);
    let (w4, _) = mac(w4, q3[1], fp_modulus[3], carry);

    let (w2, carry) = mac(w2, q3[2], fp_modulus[0], 0);
    let (w3, carry) = mac(w3, q3[2], fp_modulus[1], carry);
    let (w4, _) = mac(w4, q3[2], fp_modulus[2], carry);

    let (w3, carry) = mac(w3, q3[3], fp_modulus[0], 0);
    let (w4, _) = mac(w4, q3[3], fp_modulus[1], carry);

    let (w4, _) = mac(w4, q3[4], fp_modulus[0], 0);

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
    let fp_modulus = MODULUS.as_words();

    let (w0, borrow) = sbb(r0, fp_modulus[0], 0);
    let (w1, borrow) = sbb(r1, fp_modulus[1], borrow);
    let (w2, borrow) = sbb(r2, fp_modulus[2], borrow);
    let (w3, borrow) = sbb(r3, fp_modulus[3], borrow);
    let (w4, borrow) = sbb(r4, 0, borrow);

    // If underflow occurred on the final limb, borrow = 0xfff...fff, otherwise
    // borrow = 0x000...000. Thus, we use it as a mask to conditionally add the
    // Fp_modulus.
    let (w0, carry) = adc(w0, fp_modulus[0] & borrow, 0);
    let (w1, carry) = adc(w1, fp_modulus[1] & borrow, carry);
    let (w2, carry) = adc(w2, fp_modulus[2] & borrow, carry);
    let (w3, carry) = adc(w3, fp_modulus[3] & borrow, carry);
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
