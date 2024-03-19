extern crate core;
use core::{Randomizable, Serializable, Deserializable, ByteWriter, DeserializationError, ByteReader, AsBytes, SliceReader};
use std::{
    convert::{TryFrom, TryInto},
    fmt::{self, Debug, Display, Formatter},
    hash::{Hash, Hasher},
    ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Neg, Sub, SubAssign},
    slice
};
use crypto_bigint::{U256, rand_core::OsRng, Random};
use subtle::{Choice, ConditionallySelectable, ConstantTimeEq, CtOption};

use traits::traits::{Field, PrimeField};

use utilities::{mul64_with_carry, add64_with_carry, sub64_with_carry, shl64_by_u32_with_carry};
use crate::{ fp::{Fp, MODULUS}, fp6::Fp6};
use bitvec::{
    order::Lsb0,
    slice::BitSlice
};

// CONSTANTS
// =======================================`=========================================================
//const SCALAR_MODULUS="7AF2599B3B3F22D0563FBF0F990A37B5327AA72330157722D443623EAED4ACCF"
//                    = 55610362957290864006699123731285679659474893560816383126640993521607086746831
pub const ELEMENT_BYTES: usize = std::mem::size_of::<u64>()*4;
pub const SCALAR_MODULUS:[u64; 4] = [0xD443623EAED4ACCF,0x327AA72330157722,0x563FBF0F990A37B5,0x7AF2599B3B3F22D0];

pub const M: Scalar = Scalar([
    0xd443623eaed4accf, //15295176780841004239
    0x327aa72330157722, //3637403418627503906
    0x563fbf0f990a37b5, //6214896084507572149
    0x7af2599b3b3f22d0, //8859241940239983312
]);

// 2^256 mod M; this is used for conversion of elements into Montgomery representation.
pub const R: Scalar = Scalar([
    0x57793b82a256a662,
    0x9b0ab1b99fd511ba,
    0x538081e0cdeb9095,
    0x0a1b4cc98981ba5f,
]);

// 2^512 mod M; this is used for conversion of elements into Montgomery representation.
pub const R2: Scalar = Scalar([
    0x5a93b1562a974d84,
    0x5a5314649f39eecd,
    0xa2780be8a30fdfc6,
    0x2a68a265fd96d1bb,
]);

// 2^768 mod M; this is used for conversion of elements into Montgomery representation.
pub const R3: Scalar = Scalar([
    0x2d80dd65de381f7a,
    0x3a30adaf7b70fb37,
    0x5154791dc29dc568,
    0x7219959569912364,
]);

// Multiplicative generator g of order q-1
// g = 6
//   = 0x3ca3ccb9390a5e3bf5030b44d3856381a2402a59befe6a5e0cd7650fce07e64c in Montgomery form
//converted using (a*(2^256))%M
const GENERATOR: Scalar = Scalar([
    0x0cd7650fce07e64c,//lsd in montgomerty form
    0xa2402a59befe6a5e,
    0xf5030b44d3856381,
    0x3ca3ccb9390a5e3b,//msd
]);

// Two-adicity of the field: (q-1) % 2^1 = 0
pub const TWO_ADICITY: u32 = 1;

// 2^1 root of unity = 0x7af2599b3b3f22d0563fbf0f990a37b5327aa72330157722d443623eaed4acce  = -1 in field
//                   = 0x70d70cd1b1bd687102bf3d2ecb1ea71f976ff569904065687cca26bc0c7e066d in Montgomery form
pub const TWO_ADIC_ROOT_OF_UNITY: Scalar = Scalar([
    0x7cca26bc0c7e066d,
    0x976ff56990406568,
    0x02bf3d2ecb1ea71f,
    0x70d70cd1b1bd6871,
]);

/// -M^{-1} mod 2^64; this is used during element multiplication.
const U: u64 = 7208734935082542545;

// SCALAR FIELD ELEMENT
// ================================================================================================
/// Represents a scalar field element.
/// Internal values are stored in Montgomery representation.
/// The backing type is `[u64; 4]`.
#[derive(Copy, Clone, Eq)]
pub struct Scalar(pub [u64; 4]);

impl Scalar {
    /// Creates a new field element from a [u64; 4] value.
    /// The value is converted to Montgomery form by computing
    /// (a.R^0 * R^2) / R = a.R
    pub fn new(value: [u64; 4]) -> Self {
        (Scalar(value))*R2
    }

    /// Returns zero, the additive identity.
    pub const fn zero() -> Self {
        Scalar([0, 0, 0, 0])
    }

    /// Returns one, the multiplicative identity.
    pub const fn one() -> Self {
        R
    }

    const ELEMENT_BYTES: usize = std::mem::size_of::<u64>()*4;

    //Converts scalar element to a field element.
    pub fn to_field(a: Scalar) -> Fp6<Fp> {
        Fp6 {
            c0: Fp(a.0[0]),
            c1: Fp(a.0[1]),
            c2: Fp(a.0[2]),
            c3: Fp(a.0[3]),
            c4: Fp(0),
            c5: Fp(0),
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

    pub fn montgomery_reduce(r0: u64,r1: u64, r2: u64, r3: u64, r4: u64, r5: u64, r6: u64, r7: u64) -> Self {
        let k = r0.wrapping_mul(U);
        let (_, carry) = mul64_with_carry(r0, k, M.0[0], 0);
        let (r1, carry) = mul64_with_carry(r1, k, M.0[1], carry);
        let (r2, carry) = mul64_with_carry(r2, k, M.0[2], carry);
        let (r3, carry) = mul64_with_carry(r3, k, M.0[3], carry);
        let (r4, carry2) = add64_with_carry(r4, 0, carry);

        let k = r1.wrapping_mul(U);
        let (_, carry) = mul64_with_carry(r1, k, M.0[0], 0);
        let (r2, carry) = mul64_with_carry(r2, k, M.0[1], carry);
        let (r3, carry) = mul64_with_carry(r3, k, M.0[2], carry);
        let (r4, carry) = mul64_with_carry(r4, k, M.0[3], carry);
        let (r5, carry2) = add64_with_carry(r5, carry2, carry);

        let k = r2.wrapping_mul(U);
        let (_, carry) = mul64_with_carry(r2, k, M.0[0], 0);
        let (r3, carry) = mul64_with_carry(r3, k, M.0[1], carry);
        let (r4, carry) = mul64_with_carry(r4, k, M.0[2], carry);
        let (r5, carry) = mul64_with_carry(r5, k, M.0[3], carry);
        let (r6, carry2) = add64_with_carry(r6, carry2, carry);

        let k = r3.wrapping_mul(U);
        let (_, carry) = mul64_with_carry(r3, k, M.0[0], 0);
        let (r4, carry) = mul64_with_carry(r4, k, M.0[1], carry);
        let (r5, carry) = mul64_with_carry(r5, k, M.0[2], carry);
        let (r6, carry) = mul64_with_carry(r6, k, M.0[3], carry);
        let (r7, _) = add64_with_carry(r7, carry2, carry);

        // The result may be within M of the correct value,
        // hence subtracting the modulus
        (Scalar([r4, r5, r6, r7]))-M
    }

    

    /// Converts a 256-bit little endian integer into
    /// a `Scalar`. The element does ***NOT*** have to
    /// be canonical.
    /// This is to be used when providing an unconstrained
    /// slice of bytes, for instance a hash digest.
    pub fn from_bytes_non_canonical(bytes: &[u8; 32]) -> Self {
        let mut tmp = Scalar([0, 0, 0, 0]);

        tmp.0[0] = u64::from_le_bytes(<[u8; 8]>::try_from(&bytes[0..8]).unwrap());
        tmp.0[1] = u64::from_le_bytes(<[u8; 8]>::try_from(&bytes[8..16]).unwrap());
        tmp.0[2] = u64::from_le_bytes(<[u8; 8]>::try_from(&bytes[16..24]).unwrap());
        tmp.0[3] = u64::from_le_bytes(<[u8; 8]>::try_from(&bytes[24..32]).unwrap());

        // Convert to Montgomery form by computing
        // (a.R^0 * R^2) / R = a.R
        tmp * R2
    }

    /// Converts a little-endian bit sequence into a Scalar element
    pub fn from_bits(bit_slice: &BitSlice<u8, Lsb0>) -> Scalar {
        assert_eq!(bit_slice.len(), 256);

        let mut result = Scalar::zero();
        for i in (0..256).rev() {
            result = result.double();
            let tmp = Scalar::conditional_select(
                &Scalar::zero(),
                &Scalar::one(),
                Choice::from(bit_slice[i] as u8),
            );
            result += tmp;
        }

        result
    }

    /// Converts a little-endian bit sequence into a Scalar element
    ///
    /// **This operation is variable time with respect
    /// to the binary slice.** If the slice is fixed,
    /// this operation is effectively constant time.
    pub fn from_bits_vartime(bit_slice: &BitSlice<u8, Lsb0>) -> Scalar {
        assert_eq!(bit_slice.len(), 256);

        let mut result = Scalar::zero();
        for i in (0..256).rev() {
            result = result.double();
            if bit_slice[i] {
                result += Scalar::one();
            }
        }

        result
    }

    /// Outputs the internal representation as 4 64-bit limbs after Montgomery reduction
    /// E.g. println!("{:?}",a); gives [6, 0, 0, 0] where a= Scalar::output_reduced_limbs(&GENERATOR);
    pub fn output_reduced_limbs(&self) -> [u64; 4] {
        //Converts montgomery to normal form
        Scalar::montgomery_reduce(self.0[0], self.0[1], self.0[2], self.0[3], 0, 0, 0, 0).0
    }

    /// Outputs the internal representation as 4 64-bit limbs without Montgomery reduction
    /// This is intended for uses like re-interpreting the type containing the internal value.
    /// E.g.  println!("{:?}",a); outputs [925319367003465292, 11691391197598607966, 17654967354456892289, 4369561159377509947]
    ///  which is ame as hex form given while declaring constant GENERATOR
    pub fn output_unreduced_limbs(&self) -> [u64; 4] {
        self.0
    }

    /// Converts a 512-bit little endian integer into
    /// a `Scalar` by reducing by the modulus.
    pub fn from_bytes_wide(bytes: &[u8; 64]) -> Self {
        Scalar::from_u512([
            u64::from_le_bytes(<[u8; 8]>::try_from(&bytes[0..8]).unwrap()),
            u64::from_le_bytes(<[u8; 8]>::try_from(&bytes[8..16]).unwrap()),
            u64::from_le_bytes(<[u8; 8]>::try_from(&bytes[16..24]).unwrap()),
            u64::from_le_bytes(<[u8; 8]>::try_from(&bytes[24..32]).unwrap()),
            u64::from_le_bytes(<[u8; 8]>::try_from(&bytes[32..40]).unwrap()),
            u64::from_le_bytes(<[u8; 8]>::try_from(&bytes[40..48]).unwrap()),
            u64::from_le_bytes(<[u8; 8]>::try_from(&bytes[48..56]).unwrap()),
            u64::from_le_bytes(<[u8; 8]>::try_from(&bytes[56..64]).unwrap()),
        ])
    }

    /// Attempts to convert a little-endian byte representation of
    /// a scalar into a `Scalar`, failing if the input is not canonical.
   pub  fn from_bytes(bytes: &[u8; 32]) -> CtOption<Scalar> {
        let mut tmp = Scalar([0, 0, 0, 0]);

        tmp.0[0] = u64::from_le_bytes(<[u8; 8]>::try_from(&bytes[0..8]).unwrap());
        tmp.0[1] = u64::from_le_bytes(<[u8; 8]>::try_from(&bytes[8..16]).unwrap());
        tmp.0[2] = u64::from_le_bytes(<[u8; 8]>::try_from(&bytes[16..24]).unwrap());
        tmp.0[3] = u64::from_le_bytes(<[u8; 8]>::try_from(&bytes[24..32]).unwrap());

        // Try to subtract the modulus
        let (_, borrow) = sub64_with_carry(tmp.0[0], M.0[0], 0);
        let (_, borrow) = sub64_with_carry(tmp.0[1], M.0[1], borrow);
        let (_, borrow) = sub64_with_carry(tmp.0[2], M.0[2], borrow);
        let (_, borrow) = sub64_with_carry(tmp.0[3], M.0[3], borrow);

        // If the element is smaller than M then the
        // subtraction will underflow, producing a borrow value
        // of 0xffff...ffff. Otherwise, it'll be zero.
        let is_some = (borrow as u8) & 1;

        // Convert to Montgomery form by computing
        // (a.R^0 * R^2) / R = a.R
        tmp *= R2;

        CtOption::new(tmp, Choice::from(is_some))
    }

    /// Converts a `Scalar` element into a byte representation in
    /// little-endian byte order.
    fn to_bytes(&self) -> [u8; 32] {
        // Turn into canonical form by computing
        // (a.R) / R = a
        let tmp = Scalar::montgomery_reduce(self.0[0], self.0[1], self.0[2], self.0[3], 0, 0, 0, 0);

        let mut res = [0; 32];
        res[0..8].copy_from_slice(&tmp.0[0].to_le_bytes());
        res[8..16].copy_from_slice(&tmp.0[1].to_le_bytes());
        res[16..24].copy_from_slice(&tmp.0[2].to_le_bytes());
        res[24..32].copy_from_slice(&tmp.0[3].to_le_bytes());

        res
    }

    fn from_u512(limbs: [u64; 8]) -> Self {
        // We reduce an arbitrary 512-bit number by decomposing it into two 256-bit digits
        // with the higher bits multiplied by 2^256. Thus, we perform two reductions
        //
        // 1. the lower bits are multiplied by R^2, as normal
        // 2. the upper bits are multiplied by R^2 * 2^256 = R^3
        //
        // and computing their sum in the field.
        // The reduction works so long as the product is less than R=2^256 multiplied by
        // the modulus. This holds because for any `c` smaller than the modulus, we have
        // that (2^256 - 1)*c is an acceptable product for the reduction. Therefore, the
        // reduction always works so long as `c` is in the field; in this case it is either the
        // constant `R2` or `R3`.
        let d0 = Scalar([limbs[0], limbs[1], limbs[2], limbs[3]]);
        let d1 = Scalar([limbs[4], limbs[5], limbs[6], limbs[7]]);

        // Convert to Montgomery form
        d0 * R2 + d1 * R3
    }

    /// Converts a `Scalar` element given as byte representation into a radix-16
    /// representation, where each resulting coefficient is in [-8; 8).
    ///
    /// The resulting decomposition `[a_0, ..., a_63]` is such that
    /// `sum(a_j * 2^(j * 4)) == a`.
    pub fn bytes_to_radix_16(bytes: &[u8; 32]) -> [i8; 64] {
        let mut result = [0i8; 64];

        // Convert from bytes to radix-16
        for i in 0..32 {
            result[2 * i] = (bytes[i] & 0xf) as i8;
            result[2 * i + 1] = ((bytes[i] >> 4) & 0xf) as i8;
        }

        // Shift every coefficients from [0; 16) to [-8; 8)
        for i in 0..63 {
            let carry = (result[i] + 8) >> 4;
            result[i] -= carry << 4;
            result[i + 1] += carry;
        }

        result
    }

    /// Converts a `Scalar` element given as byte representation into a radix-256
    /// representation, where each resulting coefficient is in [-128; 128).
    ///
    /// The resulting decomposition `[a_0, ..., a_31]` is such that
    /// `sum(a_j * 2^(j * 8)) == a`.
    pub fn bytes_to_radix_256(bytes: &[u8; 32]) -> [i8; 32] {
        let mut result = [0i16; 32];

        // Convert to signed integers
        for i in 0..32 {
            result[i] = bytes[i] as i16;
        }

        // Shift every coefficients from [0; 255) to [-128; 128)
        for i in 0..31 {
            let carry = (result[i] + 128) >> 8;
            result[i] -= carry << 8;
            result[i + 1] += carry;
        }

        let mut result_i8 = [0i8; 32];
        for i in 0..32 {
            result_i8[i] = result[i] as i8;
        }

        result_i8
    }

    /// Converts a `Scalar` element given as byte representation into a w-NAF
    /// representation, where each resulting coefficient is odd and in (-2^(w-1); 2^(w-1)).
    /// In addition, the leading coefficient is non-zero, and there cannot be
    /// more than one non-zero coefficient in any w consecutive set of coefficients.
    ///
    /// **This operation is variable time with respect to the scalar.**
    /// If the scalar is fixed, this operation is effectively constant time.
    pub fn bytes_to_wnaf_vartime(bytes: &[u8; 32], w: usize) -> [i8; 256] {
        // Taken from https://github.com/dalek-cryptography/curve25519-dalek/blob/main/src/scalar.rs
        // from an adaptation of Algorithm 3.35 in Guide to Elliptic Curve Cryptography by
        // Hankerson, Menezes and Vanstone.

        debug_assert!(w >= 2);
        debug_assert!(w <= 8);

        let mut naf = [0i8; 256];

        let mut x_u64 = [0u64; 5];

        x_u64[0] = u64::from_le_bytes(<[u8; 8]>::try_from(&bytes[0..8]).unwrap());
        x_u64[1] = u64::from_le_bytes(<[u8; 8]>::try_from(&bytes[8..16]).unwrap());
        x_u64[2] = u64::from_le_bytes(<[u8; 8]>::try_from(&bytes[16..24]).unwrap());
        x_u64[3] = u64::from_le_bytes(<[u8; 8]>::try_from(&bytes[24..32]).unwrap());

        let width = 1 << w;
        let window_mask = width - 1;

        let mut pos = 0;
        let mut carry = 0;
        while pos < 256 {
            // Construct a buffer of bits of the scalar, starting at bit `pos`
            let u64_idx = pos / 64;
            let bit_idx = pos % 64;
            let bit_buf = if bit_idx < 64 - w {
                // This window's bits are contained in a single u64
                x_u64[u64_idx] >> bit_idx
            } else {
                // Combine the current u64's bits with the bits from the next u64
                (x_u64[u64_idx] >> bit_idx) | (x_u64[1 + u64_idx] << (64 - bit_idx))
            };

            // Add the carry into the current window
            let window = carry + (bit_buf & window_mask);

            if window & 1 == 0 {
                // If the window value is even, preserve the carry and continue.
                // Why is the carry preserved?
                // If carry == 0 and window & 1 == 0, then the next carry should be 0
                // If carry == 1 and window & 1 == 0, then bit_buf & 1 == 1 so the next carry should be 1
                pos += 1;
                continue;
            }

            if window < width / 2 {
                carry = 0;
                naf[pos] = window as i8;
            } else {
                carry = 1;
                naf[pos] = (window as i8).wrapping_sub(width as i8);
            }

            pos += w;
        }

        naf
    }

    /// Returns whether or not this element is strictly lexicographically
    /// larger than its negation.
    pub fn lexicographically_largest(&self) -> Choice {
        // This can be determined by checking to see if the element is
        // larger than (M - 1) // 2. If we subtract by ((M - 1) // 2) + 1
        // and there is no underflow, then the element must be larger than
        // (M - 1) // 2.

        // First, because self is in Montgomery form we need to reduce it
        let tmp = Scalar::montgomery_reduce(self.0[0], self.0[1], self.0[2], self.0[3], 0, 0, 0, 0);
        let (_, borrow) = sub64_with_carry(tmp.0[0], 0x6a21b11f576a5668, 0);
        let (_, borrow) = sub64_with_carry(tmp.0[1], 0x993d5391980abb91, borrow);
        let (_, borrow) = sub64_with_carry(tmp.0[2], 0x2b1fdf87cc851bda, borrow);
        let (_, borrow) = sub64_with_carry(tmp.0[3], 0x3d792ccd9d9f9168, borrow);

        // If the element was smaller, the subtraction will underflow
        // producing a borrow value of 0xffff...ffff, otherwise it will
        // be zero. We create a Choice representing true if there was
        // overflow (and so this element is not lexicographically larger
        // than its negation) and then negate it.

        !Choice::from((borrow as u8) & 1)
    }

    /// Constructs a `Scalar` element without checking that it is
    /// canonical.
    pub const fn from_raw_unchecked(v: [u64; 4]) -> Self {
        Scalar(v)
    }
}

impl Field for Scalar{
    /// Generates a random element.
    fn random() -> Self {
        let mut r = U256::random(&mut OsRng);
        if r >= U256::from_be_hex("7AF2599B3B3F22D0563FBF0F990A37B5327AA72330157722D443623EAED4ACCF"){
            r = r >>2
        }
        let r : [u64; 4] = r.to_words();
        return Scalar(r)
    }

    //Returns the square of a scalar element.
    fn square(self) -> Self {
        square(&self)
    }

    // returns the cube of field element
    fn cube(self)->Self{
        self*(self.square())
    }

    //Returns the inverse of a scalar element.
    fn invert(self)->CtOption<Self> {
        invert(&self)
    }

    /// Computes the square root of this element, if it exists.
    fn sqrt(self) -> CtOption<Self>{
        // Tonelli-Shank's algorithm for q mod 4 = 3
        // See https://eprint.iacr.org/2012/685.pdf
        // M+1/4= 13902590739322716001674780932821419914868723390204095781660248380401771686708
        // Compute s^((M+1)/4)
        let s = self.power_by([
            0xb510d88fabb52b34,
            0x4c9ea9c8cc055dc8,
            0x158fefc3e6428ded,
            0x1ebc9666cecfc8b4,
        ]);
        CtOption::new(s, (s.square()).ct_eq(&self))
    }

    /// Exponentiates `self` by `power`, where `power` is a
    /// little-endian order integer exponent.
    ///
    /// **This operation is variable time with respect
    /// to the exponent.** If the exponent is fixed,
    /// this operation is effectively constant time.
    fn power_by<S:AsRef<[u64]>>(self, power: S)->Self{
        let mut res = Self::ONE;
        for e in power.as_ref().iter().rev(){
            for i in (0..64).rev() {
                res = res.square();

                if ((*e >> i) & 1) == 1 {
                    res.mul_assign(self);
                }
            }
        }
    res

    }

    /// Returns the double of a scalar element
    fn double(self) -> Self {
        let (d0, carry) = shl64_by_u32_with_carry(self.0[0], 1, 0);
        let (d1, carry) = shl64_by_u32_with_carry(self.0[1], 1, carry);
        let (d2, carry) = shl64_by_u32_with_carry(self.0[2], 1, carry);
        let (d3, _carry) = shl64_by_u32_with_carry(self.0[3], 1, carry);
        // Attempt to subtract the modulus, to ensure the value
        // is smaller than the modulus.
        (&Scalar([d0, d1, d2, d3])).sub(M)
    }

    //Returns the  triple of a scalar element.
    fn triple(self)->Self {
        self+self.double()
    }

    /// Checks whether `self` is zero or not
    fn is_zero(self) -> bool {
        self.ct_eq(&Scalar::zero()).into()
    }

    fn is_one(self) -> bool {
        self.ct_eq(&Scalar::one()).into()
    }
    

    fn to_curve_bytes(&self) -> &[u8] {
        self.as_bytes()
    }

    fn to_words(&self) -> Vec<u64> {
        self.0.to_vec()
    }
    
    const ZERO:Self= Self::zero();

    const ONE:Self = Self::one();
    // element bytes
    const ELEMENT_BYTES: usize = ELEMENT_BYTES;

    type BaseField= Scalar;

    fn from_uint_reduced(w: Scalar) -> Self {
        Scalar(Scalar::output_reduced_limbs(&Scalar(w.0.into())))
    }

    fn from_words( a: &Vec<u64>) -> Self {
        let k = [a[0],a[1],a[2],a[3]];
        let value = U256::from_words(k);
        Scalar(value.into())
    }

    fn get_windows(&self, window_bits : usize)->Vec<usize> {

        let int = U256::from_words(self.0);

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

impl PrimeField for Scalar{
    type Repr= [u8; 32];

    fn is_odd(self)->Choice {
        (self.to_bytes()[0] & 1).ct_eq(&1)
    }

    /// Outputs a `Scalar` element of multiplicative order equals to 2^n
    fn get_root_of_unity(n: u32) -> Self {
        assert!(n==1, "2^{n}th root does not exist");
        TWO_ADIC_ROOT_OF_UNITY
    }

    const MODULUS:&'static str= "7AF2599B3B3F22D0563FBF0F990A37B5327AA72330157722D443623EAED4ACCF";
    const NUM_BITS:u32= 255;

    const GENERATOR:Self= GENERATOR;

    const TWO_ADDICITY: u32= 1;

  
    const TWO_ADIC_ROOT: &'static str="0x7af2599b3b3f22d0563fbf0f990a37b5327aa72330157722d443623eaed4acce";


    fn is_even(self)->Choice{
        !self.is_odd()
    }

}

impl From<[u64; 6]> for Scalar{
    fn from(_value: [u64; 6]) -> Self {
        unimplemented!()
    }
}

impl From<u128> for Scalar {
    /// Converts a 128-bit value into a field element. If the value is greater than or equal to
    /// the field modulus, modular reduction is silently performed.
    fn from(value: u128) -> Self {
        let value_high: u64 = (value >> 64).try_into().unwrap();
        let value_low: u64 = (value & (u64::MAX as u128)).try_into().unwrap();
        Scalar::new([value_low, value_high, 0, 0])
    }
}

impl From<u64> for Scalar {
    /// Converts a 64-bit value into a field element.
    fn from(value: u64) -> Self {
        Scalar([value, 0, 0, 0]) * R2
    }
}

impl From<u32> for Scalar {
    /// Converts a 32-bit value into a field element.
    fn from(value: u32) -> Self {
        Scalar([value as u64, 0, 0, 0])*R2
    }
}

impl From<u16> for Scalar {
    /// Converts a 16-bit value into a field element.
    fn from(value: u16) -> Self {
        Scalar([value as u64, 0, 0, 0])*R2
    }
}

impl From<u8> for Scalar {
    /// Converts an 8-bit value into a field element.
    fn from(value: u8) -> Self {
        Scalar([value as u64, 0, 0, 0])*R2
    }
}


impl From<U256> for Scalar {
    /// Converts an 8-bit value into a field element.
    fn from(value: U256) -> Self {
        Scalar(value.into())*R2
    }
}


impl PartialOrd for Scalar {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}
impl Ord for Scalar {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
       let a = U256::from(self.0);
       let b = U256::from(other.0);
        a.cmp(&b)
    }
}




//TRAIT BOUNDS
//================================================================================================
impl Default for Scalar {
    fn default() -> Self {
        Self::zero()
    }
}

impl Debug for Scalar {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        let tmp = Scalar::to_bytes(self);
        write!(f, "0x")?;
        for &b in tmp.iter().rev() {
            write!(f, "{:02x}", b)?;
        }
        Ok(())
    }
}

impl Display for Scalar {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl Hash for Scalar {
    fn hash<H: Hasher>(&self, hasher: &mut H) {
        self.0.hash(hasher);
    }
}

impl ConstantTimeEq for Scalar {
    fn ct_eq(&self, other: &Self) -> Choice {
        self.0[0].ct_eq(&other.0[0])
            & self.0[1].ct_eq(&other.0[1])
            & self.0[2].ct_eq(&other.0[2])
            & self.0[3].ct_eq(&other.0[3])
    }
}

impl PartialEq for Scalar {
    #[inline]
    fn eq(&self, other: &Self) -> bool {
        bool::from(self.ct_eq(other))
    }
}

impl ConditionallySelectable for Scalar {
    fn conditional_select(a: &Self, b: &Self, choice: Choice) -> Self {
        Scalar([
            u64::conditional_select(&a.0[0], &b.0[0], choice),
            u64::conditional_select(&a.0[1], &b.0[1], choice),
            u64::conditional_select(&a.0[2], &b.0[2], choice),
            u64::conditional_select(&a.0[3], &b.0[3], choice),
        ])
    }
}

impl Add for Scalar{
    type Output=Scalar;
    fn add(self,rhs: Self) -> Self::Output {
        add(&self,&rhs)
    }
}

impl Sub for Scalar{
    type Output = Scalar;
    fn sub(self, rhs: Self) -> Self::Output {
        sub(&self,&rhs)
    }
}

impl Mul<Scalar> for Scalar{
    type Output=Scalar;
    fn mul(self, rhs: Scalar) -> Self::Output {
        mul(&self,&rhs)
    }
}

impl Div for Scalar{
    type Output=Scalar;
    fn div(self, rhs: Scalar) -> Self::Output {
        div(&self,&rhs)
    }
}

impl AddAssign<Scalar> for Scalar{
    fn add_assign(&mut self, other: Self){
        *self=add(self, &other);
    }
}

impl SubAssign for Scalar{
    fn sub_assign(&mut self, other: Self){
        *self=sub(self,&other);
    }
}

impl MulAssign<Scalar> for Scalar{
    fn mul_assign(&mut self, rhs: Self){
        *self=mul(self, &rhs)  //doubt
    }
}

impl DivAssign for Scalar{
    fn div_assign(&mut self, rhs: Self){
        *self=div(self, &rhs);
    }
}

impl Neg for Scalar{
    type Output= Scalar;
    fn neg(self) -> Self::Output {
        neg(&self)
    }
}

impl Randomizable for Scalar {
    const VALUE_SIZE: usize = Scalar::ELEMENT_BYTES;
    fn from_random_bytes(bytes: &[u8]) -> Option<Self> {
        Self::try_from(bytes).ok()
    }
}

impl Serializable for Scalar {
    fn write_into<W: ByteWriter>(&self, target: &mut W) {
        target.write_u8_slice(&Scalar::to_bytes(self));
    }
}

impl Deserializable for Scalar {
    fn read_from<R: ByteReader>(source: &mut R) -> Result<Self, DeserializationError> {
        let value = source.read_u64();
        // if value >=  {
        //     return Err(DeserializationError::InvalidValue(format!(
        //         "invalid field element: value {value} is greater than or equal to the field modulus"
        //     )));
        // }
        Ok(Scalar([value.unwrap(),0,0,0]))
    }
}

impl AsBytes for Scalar{
    fn as_bytes(&self) -> &[u8] {
       let ptr: *const Scalar= self;
       unsafe{slice::from_raw_parts(ptr as *const u8, Scalar::ELEMENT_BYTES)}
        
    }
}

impl<'a> TryFrom<&'a [u8]> for Scalar {
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


/// Computes the summation of two scalar elements
pub fn add(a: &Scalar, b: &Scalar) -> Scalar {
    let (d0, carry) = add64_with_carry(a.0[0], b.0[0], 0);
    let (d1, carry) = add64_with_carry(a.0[1], b.0[1], carry);
    let (d2, carry) = add64_with_carry(a.0[2], b.0[2], carry);
    let (d3, _) = add64_with_carry(a.0[3], b.0[3], carry);

    // Attempt to subtract the modulus, to ensure the value
    // is smaller than the modulus.
    (&Scalar([d0, d1, d2, d3])).sub(M)
}

/// Computes the difference of two scalar elements
pub fn sub(a: &Scalar, b: &Scalar) -> Scalar {
    let (d0, borrow) = sub64_with_carry(a.0[0], b.0[0], 0);
    let (d1, borrow) = sub64_with_carry(a.0[1], b.0[1], borrow);
    let (d2, borrow) = sub64_with_carry(a.0[2], b.0[2], borrow);
    let (d3, borrow) = sub64_with_carry(a.0[3], b.0[3], borrow);

    // If underflow occurred on the final limb,
    // borrow = 0xfff...fff, otherwise borrow = 0x000...000.
    let (d0, carry) = add64_with_carry(d0, M.0[0] & borrow, 0);
    let (d1, carry) = add64_with_carry(d1, M.0[1] & borrow, carry);
    let (d2, carry) = add64_with_carry(d2, M.0[2] & borrow, carry);
    let (d3, _) = add64_with_carry(d3, M.0[3] & borrow, carry);

    Scalar([d0, d1, d2, d3])
}

/// Computes the negation of a scalar element
pub fn neg(a: &Scalar) -> Scalar {
    // Subtract `self` from `M` to negate. Ignore the final
    // borrow because it cannot underflow; self is guaranteed to
    // be in the field.
    let (d0, borrow) = sub64_with_carry(M.0[0], a.0[0], 0);
    let (d1, borrow) = sub64_with_carry(M.0[1], a.0[1], borrow);
    let (d2, borrow) = sub64_with_carry(M.0[2], a.0[2], borrow);
    let (d3, _) = sub64_with_carry(M.0[3], a.0[3], borrow);

    // `tmp` could be `M` if `self` was zero. Create a mask that is
    // zero if `self` was zero, and `u64::max_value()` if self was nonzero.
    let mask = (((a.0[0] | a.0[1] | a.0[2] | a.0[3]) == 0) as u64).wrapping_sub(1);

    Scalar([d0 & mask, d1 & mask, d2 & mask, d3 & mask])
}

/// Computes the multiplication of two scalar elements
pub fn mul(a: &Scalar, b: &Scalar) -> Scalar {
    // Schoolbook multiplication

    let (r0, carry) = mul64_with_carry(0, a.0[0], b.0[0], 0);
    let (r1, carry) = mul64_with_carry(0, a.0[0], b.0[1], carry);
    let (r2, carry) = mul64_with_carry(0, a.0[0], b.0[2], carry);
    let (r3, r4) = mul64_with_carry(0, a.0[0], b.0[3], carry);

    let (r1, carry) = mul64_with_carry(r1, a.0[1], b.0[0], 0);
    let (r2, carry) = mul64_with_carry(r2, a.0[1], b.0[1], carry);
    let (r3, carry) = mul64_with_carry(r3, a.0[1], b.0[2], carry);
    let (r4, r5) = mul64_with_carry(r4, a.0[1], b.0[3], carry);

    let (r2, carry) = mul64_with_carry(r2, a.0[2], b.0[0], 0);
    let (r3, carry) = mul64_with_carry(r3, a.0[2], b.0[1], carry);
    let (r4, carry) = mul64_with_carry(r4, a.0[2], b.0[2], carry);
    let (r5, r6) = mul64_with_carry(r5, a.0[2], b.0[3], carry);

    let (r3, carry) = mul64_with_carry(r3, a.0[3], b.0[0], 0);
    let (r4, carry) = mul64_with_carry(r4, a.0[3], b.0[1], carry);
    let (r5, carry) = mul64_with_carry(r5, a.0[3], b.0[2], carry);
    let (r6, r7) = mul64_with_carry(r6, a.0[3], b.0[3], carry);

    Scalar::montgomery_reduce(r0, r1, r2, r3, r4, r5, r6, r7)
}

/// Computes the multiplicative inverse of this element,
/// failing if the element is zero.
pub fn invert(a: &Scalar) -> CtOption<Scalar> {
    //found using https://github.com/kwantam/addchain for M - 2
    //assert!(a.is_zero()==false,"invalid input"); //handles the inverse of 0
    let mut t0 = a.square(); //    1: 2
    let t13 = t0.square(); //    2: 4
    let t4 = t13 * (*a); //    3: 5
    let t12 = t4 * t0; //    4: 7
    let t14 = t4 * t13; //    5: 9
    let t9 = t12 * t13; //    6: 11
    let t1 = t14 * t13; //    7: 13
    let t6 = t9 * t13; //    8: 15
    let t7 = t1 * t13; //    9: 17
    let t2 = t6 * t13; //   10: 19
    let t3 = t7 * t13; //   11: 21
    let t15 = t2 * t13; //   12: 23
    let t10 = t3 * t13; //   13: 25
    let t5 = t15 * t13; //   14: 27
    let t11 = t10 * t13; //   15: 29
    t0 = t6.square(); //   16: 30
    let t13 = t5 * t13; //   17: 31
    square_assign_multi(&mut t0, 5); //   22: 960
    t0 *= t15; //   23: 983
    square_assign_multi(&mut t0, 4); //   27: 15728
    t0 *= t14; //   28: 15737
    square_assign_multi(&mut t0, 6); //   34: 1007168
    t0 *= t9; //   35: 1007179
    square_assign_multi(&mut t0, 7); //   42: 128918912
    t0 *= t10; //   43: 128918937
    square_assign_multi(&mut t0, 4); //   47: 2062702992
    t0 *= t9; //   48: 2062703003
    square_assign_multi(&mut t0, 7); //   55: 264025984384
    t0 *= t11; //   56: 264025984413
    square_assign_multi(&mut t0, 5); //   61: 8448831501216
    t0 *= t2; //   62: 8448831501235
    square_assign_multi(&mut t0, 4); //   66: 135181304019760
    t0 *= t6; //   67: 135181304019775
    square_assign_multi(&mut t0, 7); //   74: 17303206914531200
    t0 *= t7; //   75: 17303206914531217
    square_assign_multi(&mut t0, 5); //   80: 553702621264998944
    t0 *= t1; //   81: 553702621264998957
    square_assign_multi(&mut t0, 10); //   91: 566991484175358931968
    t0 *= t3; //   92: 566991484175358931989
    square_assign_multi(&mut t0, 5); //   97: 18143727493611485823648
    t0 *= t7; //   98: 18143727493611485823665
    square_assign_multi(&mut t0, 5); //  103: 580599279795567546357280
    t0 *= t13; //  104: 580599279795567546357311
    square_assign_multi(&mut t0, 4); //  108: 9289588476729080741716976
    t0 *= t9; //  109: 9289588476729080741716987
    square_assign_multi(&mut t0, 4); //  113: 148633415627665291867471792
    t0 *= t6; //  114: 148633415627665291867471807
    square_assign_multi(&mut t0, 9); //  123: 76100308801364629436145565184
    t0 *= t13; //  124: 76100308801364629436145565215
    square_assign_multi(&mut t0, 7); //  131: 9740839526574672567826632347520
    t0 *= t10; //  132: 9740839526574672567826632347545
    square_assign_multi(&mut t0, 7); //  139: 1246827459401558088681808940485760
    t0 *= t4; //  140: 1246827459401558088681808940485765
    square_assign_multi(&mut t0, 8); //  148: 319187829606798870702543088764355840
    t0 *= t5; //  149: 319187829606798870702543088764355867
    square_assign_multi(&mut t0, 5); //  154: 10214010547417563862481378840459387744
    t0 *= t5; //  155: 10214010547417563862481378840459387771
    square_assign_multi(&mut t0, 4); //  159: 163424168758681021799702061447350204336
    t0 *= t4; //  160: 163424168758681021799702061447350204341
    square_assign_multi(&mut t0, 7); //  167: 20918293601111170790361863865260826155648
    t0 *= t10; //  168: 20918293601111170790361863865260826155673
    square_assign_multi(&mut t0, 6); //  174: 1338770790471114930583159287376692873963072
    t0 *= t6; //  175: 1338770790471114930583159287376692873963087
    square_assign_multi(&mut t0, 6); //  181: 85681330590151355557322194392108343933637568
    t0 *= t3; //  182: 85681330590151355557322194392108343933637589
    square_assign_multi(&mut t0, 5); //  187: 2741802578884843377834310220547467005876402848
    t0 *= t12; //  188: 2741802578884843377834310220547467005876402855
    square_assign_multi(&mut t0, 7); //  195: 350950730097259952362791708230075776752179565440
    t0 *= t7; //  196: 350950730097259952362791708230075776752179565457
    square_assign_multi(&mut t0, 5); //  201: 11230423363112318475609334663362424856069746094624
    t0 *= t2; //  202: 11230423363112318475609334663362424856069746094643
    square_assign_multi(&mut t0, 12); //  214: 45999814095308056476095834781132492210461680003657728
    t0 *= t3; //  215: 45999814095308056476095834781132492210461680003657749
    square_assign_multi(&mut t0, 6); //  221: 2943988102099715614470133425992479501469547520234095936
    t0 *= t11; //  222: 2943988102099715614470133425992479501469547520234095965
    square_assign_multi(&mut t0, 5); //  227: 94207619267190899663044269631759344047025520647491070880
    t0 *= t10; //  228: 94207619267190899663044269631759344047025520647491070905
    square_assign_multi(&mut t0, 7); //  235: 12058575266200435156869666512865196038019266642878857075840
    t0 *= t9; //  236: 12058575266200435156869666512865196038019266642878857075851
    square_assign_multi(&mut t0, 4); //  240: 192937204259206962509914664205843136608308266286061713213616
    t0 *= t4; //  241: 192937204259206962509914664205843136608308266286061713213621
    square_assign_multi(&mut t0, 4); //  245: 3086995268147311400158634627293490185732932260576987411417936
    t0 *= *a; //  246: 3086995268147311400158634627293490185732932260576987411417937
    square_assign_multi(&mut t0, 9); //  255: 1580541577291423436881220929174266975095261317415417554645983744
    t0 *= t5; //  256: 1580541577291423436881220929174266975095261317415417554645983771
    square_assign_multi(&mut t0, 8); //  264: 404618643786604399841592557868612345624386897258346893989371845376
    t0 *= t7; //  265: 404618643786604399841592557868612345624386897258346893989371845393
    square_assign_multi(&mut t0, 4); //  269: 6473898300585670397465480925897797529990190356133550303829949526288
    t0 *= t6; //  270: 6473898300585670397465480925897797529990190356133550303829949526303
    square_assign_multi(&mut t0, 6); //  276: 414329491237482905437790779257459041919372182792547219445116769683392
    t0 *= t3; //  277: 414329491237482905437790779257459041919372182792547219445116769683413
    square_assign_multi(&mut t0, 5); //  282: 13258543719599452974009304936238689341419909849361511022243736629869216
    t0 *= t5; //  283: 13258543719599452974009304936238689341419909849361511022243736629869243
    square_assign_multi(&mut t0, 4); //  287: 212136699513591247584148878979819029462718557589784176355899786077907888
    t0 *= t4; //  288: 212136699513591247584148878979819029462718557589784176355899786077907893
    square_assign_multi(&mut t0, 7); //  295: 27153497537739679690771056509416835771227975371492374573555172617972210304
    t0 *= t3; //  296: 27153497537739679690771056509416835771227975371492374573555172617972210325
    square_assign_multi(&mut t0, 5); //  301: 868911921207669750104673808301338744679295211887755986353765523775110730400
    t0 *= t2; //  302: 868911921207669750104673808301338744679295211887755986353765523775110730419
    square_assign_multi(&mut t0, 6); //  308: 55610362957290864006699123731285679659474893560816383126640993521607086746816
    t0 *= t1; //  309: 55610362957290864006699123731285679659474893560816383126640993521607086746829

    
    CtOption::new(t0, !a.ct_eq(&Scalar::zero()))
}

/// Computes the square of a scalar element
pub fn square(a: &Scalar) -> Scalar {
    let (r1, carry) = mul64_with_carry(0, a.0[0], a.0[1], 0);
    let (r2, carry) = mul64_with_carry(0, a.0[0], a.0[2], carry);
    let (r3, r4) = mul64_with_carry(0, a.0[0], a.0[3], carry);

    let (r3, carry) = mul64_with_carry(r3, a.0[1], a.0[2], 0);
    let (r4, r5) = mul64_with_carry(r4, a.0[1], a.0[3], carry);

    let (r5, r6) = mul64_with_carry(r5, a.0[2], a.0[3], 0);

    let r7 = r6 >> 63;
    let r6 = (r6 << 1) | (r5 >> 63);
    let r5 = (r5 << 1) | (r4 >> 63);
    let r4 = (r4 << 1) | (r3 >> 63);
    let r3 = (r3 << 1) | (r2 >> 63);
    let r2 = (r2 << 1) | (r1 >> 63);
    let r1 = r1 << 1;

    let (r0, carry) = mul64_with_carry(0, a.0[0], a.0[0], 0);
    let (r1, carry) = add64_with_carry(0, r1, carry);
    let (r2, carry) = mul64_with_carry(r2, a.0[1], a.0[1], carry);
    let (r3, carry) = add64_with_carry(0, r3, carry);
    let (r4, carry) = mul64_with_carry(r4, a.0[2], a.0[2], carry);
    let (r5, carry) = add64_with_carry(0, r5, carry);
    let (r6, carry) = mul64_with_carry(r6, a.0[3], a.0[3], carry);
    let (r7, _) = add64_with_carry(0, r7, carry);

    Scalar::montgomery_reduce(r0, r1, r2, r3, r4, r5, r6, r7)
}

//Computes a/b or a*(b^{-1})
pub fn div(a:&Scalar,b:&Scalar)->Scalar{
    let rhs:Scalar=b.invert().unwrap();
    (*a)*(rhs)
}   

//Computes (2^num_times) power of a square element.
pub fn square_assign_multi(n: &mut Scalar, num_times: usize) {
    for _ in 0..num_times {
        *n = n.square();
    }
}



// SERDE SERIALIZATION
// ================================================================================================

#[cfg(feature = "serialize")]
impl Serialize for Scalar {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        use serde::ser::SerializeTuple;
        let mut tup = serializer.serialize_tuple(32)?;
        for byte in self.to_bytes().iter() {
            tup.serialize_element(byte)?;
        }
        tup.end()
    }
}

#[cfg(feature = "serialize")]
impl<'de> Deserialize<'de> for Scalar {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct ScalarVisitor;

        impl<'de> Visitor<'de> for ScalarVisitor {
            type Value = Scalar;

            fn expecting(&self, formatter: &mut Formatter) -> fmt::Result {
                formatter.write_str("a valid field element")
            }

            fn visit_seq<A>(self, mut seq: A) -> Result<Scalar, A::Error>
            where
                A: serde::de::SeqAccess<'de>,
            {
                let mut bytes = [0u8; 32];
                for (i, byte) in bytes.iter_mut().enumerate() {
                    *byte = seq
                        .next_element()?
                        .ok_or_else(|| serde::de::Error::invalid_length(i, &"expected 32 bytes"))?;
                }
                let elem = Scalar::from_bytes(&bytes);
                if bool::from(elem.is_none()) {
                    Err(serde::de::Error::custom("decompression failed"))
                } else {
                    Ok(elem.unwrap())
                }
            }
        }

        deserializer.deserialize_tuple(32, ScalarVisitor)
    }
}
