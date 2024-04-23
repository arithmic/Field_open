use crypto_bigint::{
    generic_array::GenericArray,
    rand_core::OsRng,
    subtle::{Choice, ConditionallySelectable, ConstantTimeEq, ConstantTimeLess, CtOption},
    ArrayEncoding, Encoding, Integer, Limb, Random, Uint, Zero, U256, U384,
};
use std::{
    convert::{TryFrom, TryInto},
    fmt::{Display, Formatter},
    ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Neg, Sub, SubAssign},
    slice,
};

use crypto_bigint::uint::modular::reduction;
use traits::traits::{Extensible, Field, PrimeField};

use core::{
    AsBytes, ByteReader, ByteWriter, Deserializable, DeserializationError, Randomizable,
    Serializable,
};

use crate::fp2::Fp2;
// Size of field elements of this elliptic curve.
pub type FieldSize = <U384 as crypto_bigint::ArrayEncoding>::ByteSize;
/// Byte representation of a base/scalar field element of a given curve.
pub type FieldBytes = GenericArray<u8, FieldSize>;
// Number of bytes needed to represent field element
pub const ELEMENT_BYTES: usize = std::mem::size_of::<U384>();
// modulus of the base field , p=4002409555221667393417789825735904156556882819939007885332058136124031650490837864442687629129015664037894272559787
pub(crate) const MODULUS: U384 = U384::from_be_hex("1a0111ea397fe69a4b1ba7b6434bacd764774b84f38512bf6730d2a0f6b0f6241eabfffeb153ffffb9feffffffffaaab");
//modulus-1
pub const MODULUS_MINUS_ONE: U384 = U384::from_be_hex("1A0111EA397FE69A4B1BA7B6434BACD764774B84F38512BF6730D2A0F6B0F6241EABFFFEB153FFFFB9FEFFFFFFFFAAAA");

pub const MODULUS_MINUS_ONE_DIV_TWO: Fp = Fp(MODULUS_MINUS_ONE.shr_vartime(1)).to_montgomery();

pub(crate) const P: U384 = U384::from_be_hex("1a0111ea397fe69a4b1ba7b6434bacd764774b84f38512bf6730d2a0f6b0f6241eabfffeb153ffffb9feffffffffaaab");
//2^384-MODULUS
pub(crate) const P_INV: Limb = Limb::from_u64(9940570264628428797);
//
pub const TWO_ADIC_ROOT: & 'static str = "1A0111EA397FE69A4B1BA7B6434BACD764774B84F38512BF6730D2A0F6B0F6241EABFFFEB153FFFFB9FEFFFFFFFFAAAA";
pub const TWO_ADDICITY: u32 = 1;

pub(crate) const P_MINUS_2: [u64; 6] = [
    0xb9feffffffffaaa9,
    0x1eabfffeb153ffff,
    0x6730d2a0f6b0f624,
    0x64774b84f38512bf,
    0x4b1ba7b6434bacd7,
    0x1a0111ea397fe69a,
];
pub(crate) const R :U384 = U384::from_be_hex("15f65ec3fa80e4935c071a97a256ec6d77ce5853705257455f48985753c758baebf4000bc40c0002760900000002fffd");
// R2=2708263910654730174793787626328176511836455197166317677006154293982164122222515399004018013397331347120527951271750
pub(crate) const R2 :U384 = U384::from_be_hex("11988fe592cae3aa9a793e85b519952d67eb88a9939d83c08de5476c4c95b6d50a76e6a609d104f1f4df1f341c341746");
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, PartialOrd, Ord)]
//implement struct FieldElement where base type is U384

pub struct Fp(pub U384);

impl Fp {
    // additive identity
    pub const ZERO: Fp = Fp(U384::ZERO);
    // multiplicative identity
    pub const ONE: Fp = Fp(U384::ONE).to_montgomery();
    // returns the field element from its byte representation
    pub fn from_repr(bytes: &FieldBytes) -> Fp {
        let inner = U384::from_be_byte_array(*bytes);
        <Fp as Field>::from_uint_reduced(Fp(inner))
    }
    // Returns other mod p
    pub fn new(other: U384) -> Self {
        //out = CtOption::new(other, other.ct_lt(&MODULUS)).unwrap();
        let product = other.mul_wide(&R2);
        let limbs: Uint<6> = reduction::montgomery_reduction(product, MODULUS, P_INV);
        let words: [u64; 6] = limbs.to_words();
        let out = U384::from_words(words);
        // out
        Fp(CtOption::new(out, out.ct_lt(&MODULUS)).unwrap())
    }
    // converts the field element into its montgomery form
    pub const fn to_montgomery(&self) -> Self {
        let product = self.0.mul_wide(&R2);
        let limbs: Uint<6> = reduction::montgomery_reduction(product, MODULUS, P_INV);
        let words: [u64; 6] = limbs.to_words();
        let out = Fp(U384::from_words(words));
        out
    }
    // converts the montgomery form into normal form of field element.
    pub const fn from_montgomery(&self) -> Self {
        let product = self.0.mul_wide(&(U384::ONE));
        let limbs: Uint<6> = reduction::montgomery_reduction(product, MODULUS, P_INV);
        let words: [u64; 6] = limbs.to_words();
        let out = Fp(U384::from_words(words));
        out
    }
    // returns 1  if the field element is quadratic residue and -1 if it is not a quadratic residue.
    pub fn legendre(&self) -> i32 {
        // s = self^((MODULUS - 1) // 2)
        let s = self.power_by(&MODULUS_MINUS_ONE_DIV_TWO.0.to_words());
        if bool::from(s.is_zero()) {
            0
        } else if s == Fp(U384::from_u8(1)) {
            1 //  quadratic residue
        } else {
            -1 // non-quadratic residue
        }
    }

    // checks whether the given field elemenet is perfect square or not in the field.
    pub fn is_square(&self) -> bool {
        let a = (self.sqrt()).unwrap_or(Fp::ZERO);
        if a.square() == *self {
            true
        } else {
            false
        }
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

    pub fn rem(self, d: Self) -> Self {
        let mut a = self;
        while a >= d {
            a -= d;
        }
        a
    }

    // convert the fieldelement 'a' into bytes representation.
    pub fn to_bytes(a: &Fp) -> FieldBytes {
        a.0.to_be_byte_array()
    }
}

impl Field for Fp {
    // checks whether the given input is zero or not
    fn is_zero(self) -> bool {
        self == Fp::ZERO
    }
    // checks whether the given input is one or not
    fn is_one(self) -> bool {
        self.ct_eq(&Fp::ONE).into()
    }
    // returns the twice of field element passed in it.
    fn double(self) -> Self {
        add(&self, &self)
    }
    // returns the thrice of field element passed in it.
    fn triple(self) -> Self {
        add(&self.double(), &self)
    }
    // returns the square of the field element i.e self*self mod p
    fn square(self) -> Self {
        let product = self.0.square_wide();
        let limbs: Uint<6> = reduction::montgomery_reduction(product, MODULUS, P_INV);
        let words: [u64; 6] = limbs.to_words();
        let out: Fp = Self(U384::from_words(words));
        out
    }
    // returns the cube of field element
    fn cube(self) -> Self {
        self * self.square()
    }
    // returns the multiplicative inverse of the field element
    fn invert(self) -> CtOption<Self> {
        invert(&self)
    }
    // Additive identity of the field Fp
    const ZERO: Self = Self::ZERO;
    // Multiplicative identity of the field Fp
    const ONE: Self = Self::ONE;
    // element bytes
    const ELEMENT_BYTES: usize = ELEMENT_BYTES;

    // returns the square root of field element if it exists.
    fn sqrt(self) -> CtOption<Self> {
        let w = self.power_by(&[
            0xee7f_bfff_ffff_eaaa,
            0x07aa_ffff_ac54_ffff,
            0xd9cc_34a8_3dac_3d89,
            0xd91d_d2e1_3ce1_44af,
            0x92c6_e9ed_90d2_eb35,
            0x0680_447a_8e5f_f9a6,
        ]);
        // v is the number of leading zeros bit in the bit representation of q-1
        let mut v = 1 as u32;
        let mut x = self * w;
        let mut b = x * w;
        //1a0111ea397fe69a4b1ba7b6434bacd764774b84f38512bf6730d2a0f6b0f6241eabfffeb153ffffb9feffffffffaaaa
        //1a0111ea397fe69a
        let multiplicative_generator = Fp(U384::from_u32(2));
        let mut z = multiplicative_generator.power_by(&[
            0xb9fe_ffff_ffff_aaaa,
            0x1eab_fffe_b153_ffff,
            0x6730_d2a0_f6b0_f624,
            0x6477_4b84_f385_12bf,
            0x4b1b_a7b6_434b_acd7,
            0x1a01_11ea_397f_e69a,
        ]);
        for max_v in (1..=1).rev() {
            let mut k = 1;
            let mut tmp = b.square();
            let mut j_less_than_v = Choice::from(1);
            for j in 2..max_v {
                let tmp_is_one = tmp.ct_eq(&Fp(U384::from_u8(1)));
                let squared = Self::conditional_select(&tmp, &z, tmp_is_one).square();
                tmp = Self::conditional_select(&squared, &tmp, tmp_is_one);
                let new_z = Self::conditional_select(&z, &squared, tmp_is_one);
                j_less_than_v &= !j.ct_eq(&v);
                k = u32::conditional_select(&j, &k, tmp_is_one);
                z = Self::conditional_select(&z, &new_z, j_less_than_v);
            }
            let result = x * z;
            x = Self::conditional_select(&result, &x, b.ct_eq(&Fp(U384::from_u8(1))));
            z = z.square();
            b = b * z;
            v = k;
        }
        CtOption::new(x, x.square().ct_eq(&self))
    }

    // Exponentiates the self by pow
    fn power_by<S: AsRef<[u64]>>(self, pow: S) -> Self {
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
    type BaseField = Self;

    fn random() -> Self {
        let r = U384::random(&mut OsRng);
        if r < MODULUS {
            return Fp(r);
        } else {
            return Fp(r >> 4);
        }
    }

    fn to_curve_bytes(&self) -> &[u8] {
        self.as_bytes()
    }
    fn to_words(&self) -> Vec<u64> {
        self.0.to_words().into()
    }

    fn from_uint_reduced(w: Fp) -> Self {
        let (r, underflow) = w.0.sbb(&MODULUS, Limb::ZERO);
        let underflow = Choice::from((underflow.0 >> (Limb::BITS - 1)) as u8);
        Fp(U384::conditional_select(&w.0, &r, !underflow))
    }

    fn from_words(a: &Vec<u64>) -> Self {
        Self(U384::from_words([a[0], a[1], a[2], a[3], a[4], a[5]]))
    }

    fn get_windows(&self, _exp: usize) -> Vec<usize> {
        todo!()
    }

    const IS_CANONICAL: bool = true;
}
impl PrimeField for Fp {
    // checks whether the given field element is odd or not.
    fn is_odd(self) -> Choice {
        // we are calling is_odd function from crypto_bigint crate and method is defined for Uint
        self.0.is_odd()
    }

    fn get_root_of_unity(k: u32) -> Self {
        assert!(k == 1, "2^{k}th root does not exist");
        Fp::new(U384::from_be_hex(TWO_ADIC_ROOT))
    }
    // modulus of the prime field
    const MODULUS:&'static str="1a0111ea397fe69a4b1ba7b6434bacd764774b84f38512bf6730d2a0f6b0f6241eabfffeb153ffffb9feffffffffaaab";

    const GENERATOR: Self = Fp(U384::from_u32(2));
    const NUM_BITS: u32 = ELEMENT_BYTES as u32;
    const TWO_ADIC_ROOT: & 'static str = "1A0111EA397FE69A4B1BA7B6434BACD764774B84F38512BF6730D2A0F6B0F6241EABFFFEB153FFFFB9FEFFFFFFFFAAAA";
    const TWO_ADDICITY: u32 = 1;

    fn is_even(self) -> Choice {
        !self.is_odd()
    }
    type Repr = FieldBytes;
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
            let mut a1 = Fp2 { c0: a[0], c1: a[1] }.power_by(&[
                0xee7fbfffffffeaaa,
                0x7aaffffac54ffff,
                0xd9cc34a83dac3d89,
                0xd91dd2e13ce144af,
                0x92c6e9ed90d2eb35,
                0x680447a8e5ff9a6,
            ]);
            let mut alpha = a1;
            alpha = alpha.square();
            alpha.mul_assign(Fp2 { c0: a[0], c1: a[1] });
            let mut a0 = alpha;
            a0.frobenius_map(1);
            a0.mul_assign(alpha);

            let neg1 = Fp2 {
                c0: Fp(U384::from_words([
                    4897101644811774638,
                    3654671041462534141,
                    569769440802610537,
                    17053147383018470266,
                    17227549637287919721,
                    291242102765847046,
                ])),
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
                    alpha = alpha.power_by(&[
                        0xdcff7fffffffd555,
                        0xf55ffff58a9ffff,
                        0xb39869507b587b12,
                        0xb23ba5c279c2895f,
                        0x258dd3db21a5d66b,
                        0xd0088f51cbff34d,
                    ]);
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
        div(&self, &rhs)
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
        Self::new(U384::from_u128(num))
    }
}
impl From<u64> for Fp {
    fn from(num: u64) -> Self {
        Self(U384::from_u64(num))
    }
}
impl From<u32> for Fp {
    fn from(num: u32) -> Self {
        Self(U384::from_u32(num))
    }
}
impl From<u16> for Fp {
    fn from(num: u16) -> Self {
        Self(U384::from_u16(num))
    }
}
impl From<u8> for Fp {
    fn from(num: u8) -> Self {
        Self(U384::from_u8(num))
    }
}
impl From<[u64; 6]> for Fp {
    fn from(value: [u64; 6]) -> Self {
        let value = U384::from_words(value);
        Self::new(value)
    }
}

impl From<U256> for Fp {
    fn from(value: U256) -> Self {
        let v = value.to_words();
        Self::from([v[0], v[1], v[2], v[3], 0, 0])
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

// SERIALIZATION / DESERIALIZATION
// ------------------------------------------------------------------------------------------------
impl Serializable for Fp {
    fn write_into<W: ByteWriter>(&self, target: &mut W) {
        target.write_u8_slice(&self.0.to_le_bytes());
    }
}
impl Deserializable for Fp {
    fn read_from<R: ByteReader>(source: &mut R) -> Result<Self, DeserializationError> {
        unimplemented!()
        // let value = source.read_u8_array()?;
        // let result = U384::from_le_byte_array(value);
        // if result >= MODULUS {
        //     return Err(DeserializationError::InvalidValue(format!(
        //         "invalid field element: value {} is greater than or equal to the field modulus",
        //         result
        //     )));
        // }
        // Ok(Fp(result))
    }
}

impl ConstantTimeEq for Fp {
    fn ct_eq(&self, other: &Self) -> Choice {
        self.0.ct_eq(&other.0)
    }
}
impl ConditionallySelectable for Fp {
    fn conditional_select(a: &Self, b: &Self, choice: Choice) -> Self {
        Self(U384::conditional_select(&a.0, &b.0, choice))
    }
}
pub trait Reduce<U384: Integer>: Sized {
    /// Perform a modular reduction, returning a field element.
    fn from_uint_reduced(n: U384) -> Self;
}
impl Reduce<U384> for Fp {
    fn from_uint_reduced(w: U384) -> Fp {
        let (r, underflow) = w.sbb(&MODULUS, Limb::ZERO);
        let underflow = Choice::from((underflow.0 >> (Limb::BITS - 1)) as u8);
        Fp(U384::conditional_select(&w, &r, !underflow))
    }
}
// Returns a+b mod p
fn add(a: &Fp, b: &Fp) -> Fp {
    // we are calling add_mod function of crypto_bigint crate which will return a.0(u384 type) + b.0(u384 type) mod p in U384
    Fp(a.0.add_mod(&b.0, &P))
}
// Returs (a-b) mod p
fn sub(a: &Fp, b: &Fp) -> Fp {
    Fp(a.0.sub_mod(&b.0, &P))
}
//Returns -a mod p i.e additive inverse of a in the field.
fn neg(a: &Fp) -> Fp {
    Fp(a.0.neg_mod(&P))
}
//returns a*b mod p
fn mul(a: &Fp, b: &Fp) -> Fp {
    if a == &Fp::ONE || b == &Fp::ONE {
        if a == &Fp::ONE {
            return b.clone();
        } else {
            return a.clone();
        }
    }
    if a == &Fp::ZERO || b == &Fp::ZERO {
        return Fp::ZERO;
    }
    let product = a.0.mul_wide(&b.0);
    let limbs: Uint<6> = reduction::montgomery_reduction(product, MODULUS, P_INV);
    let words: [u64; 6] = limbs.to_words();
    let out: Fp = Fp(U384::from_words(words));
    out
}
// returns multiplicative inverse of a
fn invert(a: &Fp) -> CtOption<Fp> {
    // invese of zero is not defined . So we are panicking when zero is given as input.
    let inv = a.power_by(&P_MINUS_2);
    CtOption::new(inv, !a.0.is_zero())
}
// returns a/b which means a*(Multiplicative inverse of b) mod p
fn div(a: &Fp, b: &Fp) -> Fp {
    mul(a, &invert(b).unwrap())
}
