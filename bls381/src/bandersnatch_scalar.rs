use core::{
    AsBytes, ByteReader, ByteWriter, Deserializable, DeserializationError, Randomizable,
    Serializable,
};
use crypto_bigint::{
    generic_array::GenericArray,
    subtle::{Choice, ConditionallySelectable, ConstantTimeEq, ConstantTimeLess, CtOption},
    ArrayEncoding, Integer, Limb, Zero, U256,
};
use crypto_bigint::{Encoding, Random};
use rand::rngs::OsRng;
use std::{
    fmt::Display,
    mem,
    ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Neg, Sub, SubAssign},
    slice,
};
use traits::traits::{Field, PrimeField};

use crate::{
    fp::Reduce,
    scalar::{adc, mac, sbb},
};

// Size of field elements of this elliptic curve.
pub type FieldSize = <U256 as crypto_bigint::ArrayEncoding>::ByteSize;

/// Byte representation of a base/BandScalar field element of a given curve.
pub type FieldBytes = GenericArray<u8, FieldSize>;

// Number of bytes needed to represent field element
const ELEMENT_BYTES: usize = std::mem::size_of::<U256>();

pub const GENERATOR: u32 = 7;
// n=13108968793781547619861935127046491459309155893440570251786403306729687672801
pub const BANDSCALAR_MODULUS: U256 =
    U256::from_be_hex("1CFB69D4CA675F520CCE760202687600FF8F87007419047174FD06B52876E7E1");

// BANDSCALAR_MODULUS/2
const FRAC_BANDSCALAR_MODULUS_2: BandScalar = BandScalar(BANDSCALAR_MODULUS.shr_vartime(1));
pub const TWO_ADIC_ROOT: U256 =
    U256::from_be_hex("19470B7EFE802F9B36B6675F52C7008234BB3E0CB7ED22AEC65A62A1234BD960");

#[allow(unused)]
// MU = floor(2^512 / n)
// MU = 8D54253B7FB78DDF0E2D772DC1F823B679936F818566267B5D321F1DA00070AD5
pub const MU: [u64; 5] = [
    0xD321_F1DA_0007_0AD5,
    0x9936_F818_5662_67B5,
    0xE2D7_72DC_1F82_3B67,
    0xD542_53B7_FB78_DDF0,
    0x0000_0000_0000_0008,
];
//====== struct implementation =======
#[derive(Debug, Copy, Clone, Default)]
pub struct BandScalar(pub U256); //defining the BandScalar struct

//implementing the BandScalar struct
impl BandScalar {
    //implement zero and one
    pub const ZERO: Self = Self(U256::ZERO);
    pub const ONE: Self = Self(U256::ONE);
    pub const BANDSCALAR_MODULUS_1: Self = Self(U256::from_be_hex(
        "1CFB69D4CA675F520CCE760202687600FF8F87007419047174FD06B52876E7E0",
    ));

    //return the new instance of the struct it takes any element as input
    //convert on U256 bit and check if the value is greater or less than modulus
    //if more than modulus, it uses mul_wide and barrett reduce to bring the
    //value in the field range. It return self % p
    pub fn new(other: U256) -> Self {
        let prod = other.mul_wide(&U256::ONE);
        let res = barrett_reduce(prod.0, prod.1);
        BandScalar(CtOption::new(res, res.ct_lt(&BANDSCALAR_MODULUS)).unwrap())
    }
    pub fn from_repr(bytes: &FieldBytes) -> BandScalar {
        let inner = U256::from_be_byte_array(*bytes);
        <BandScalar as Field>::from_uint_reduced(BandScalar(inner))
    }
    //zeroed vector
    pub fn zeroed_vector(n: usize) -> Vec<Self> {
        // this uses a specialized vector initialization code which requests zero-filled memory
        // from the OS; unfortunately, this works only for built-in types and we can't use
        // Self::ZERO here as much less efficient initialization procedure will be invoked.
        // We also use u128 to make sure the memory is aligned correctly for our element size.
        debug_assert_eq!(ELEMENT_BYTES, mem::size_of::<u128>());
        let result = vec![0u128; n];
        // translate a zero-filled vector of u128s into a vector of base field elements
        let mut v = std::mem::ManuallyDrop::new(result);
        let p = v.as_mut_ptr();
        let len = v.len();
        let cap = v.capacity();
        unsafe { Vec::from_raw_parts(p as *mut Self, len, cap) }
    }
    //elements as bytes. It converts input into u8 array
    // converts using the as_ptr of crypto_bigint
    // and slices using the from_raw_parts
    pub fn elements_as_bytes(elements: &[Self]) -> &[u8] {
        // TODO: take endianness into account
        let p = elements.as_ptr();
        let len = elements.len() * ELEMENT_BYTES;
        unsafe { slice::from_raw_parts(p as *const u8, len) }
    }
}

//=====Field implementation =======
//implement Field for BandScalar
impl Field for BandScalar {
    type BaseField = BandScalar;
    const ONE: Self = Self::ONE;
    const ZERO: Self = Self::ZERO;
    const ELEMENT_BYTES: usize = ELEMENT_BYTES;
    // checks if the given value if zero or not
    // returns true or one when value is zero
    fn is_zero(self) -> bool {
        self == Self::ZERO
    }
    // checks if the given value if zero or not
    // returns true or one when value is zero
    fn is_one(self) -> bool {
        self == Self::ONE
    }
    // double function
    // it takes a field element and returns the double of the element
    // uses the add function of the BandScalar field
    fn double(self) -> Self {
        add(self, self)
    }
    // triple functions
    // it take a field element and return 3 times the element in the BandScalar field.
    //depends on double function
    fn triple(self) -> Self {
        add(self, self.double())
    }

    fn cube(self) -> Self {
        self * self.double()
    }
    //square of element
    //it takes a self (field element) and returns the square of the BandScalar field element.
    //it uses square_wide of the crypto_bigint to give a tuple output (lo, hi)
    //which is reduced using the barrett_reduce function
    fn square(self) -> Self {
        let res = self.0.square_wide();
        let prod = barrett_reduce(res.0, res.1);
        Self(prod)
    }
    //inverting or inverse of an element function
    // takes a self and returns the inverse element of the self in the BandScalar field
    // uses is_odd, is_even, add_mod, bit shift (shr1), sub_mod functions
    fn invert(self) -> CtOption<BandScalar> {
        let a = invert(self);
        // convert BandScalar to ctoption
        a
    }
    // Exponentiates self by exp
    // Need to exponentiate the self (U256 bit) value with another 256 bit
    // value as power. The power is given in an array of u64 with len 4.
    // we start with ONE of BandScalar. we loop in the size of the array. We
    // then loop in the bit length of the value. and do the square of the value
    // but in this loop if bit is 1 then the value need to be multiplied with the base value.
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
    // square root function computes the square roort of the field element
    // depends on exp, square, new takes an input in the field.
    //

    //it generates the random values of U256 bits using the random OsRng function of the crypto
    //bigint. The r value stores the random value. It can be greater then the modulus. We
    // convert it into BandScalar element using the mul_wide function from crypto_bigint. Then
    // do barret reduce to reduce the value from modulus and bring it in the BandScalar as BandScalar element.
    fn random() -> Self {
        let r = U256::random(&mut OsRng);
        if r < BANDSCALAR_MODULUS {
            return BandScalar(r);
        } else {
            return BandScalar(r >> 4);
        }
    }

    fn sqrt(self) -> CtOption<Self> {
        //self^((t-1)/2)
        let mut w = self.power_by(
            U256::from_be_hex("0073eda753299d7d483339d80809a1d803fe3e1c01d06411c5d3f41ad4a1db9f")
                .to_words(),
        );
        // v is the number of leading zeros bit in the bit representation of q-1
        let mut v = 5 as u32; //v
        let mut x = self * w; //x
        let mut b = x * w; //b
        let multiplicative_generator = BandScalar(U256::from_u32(GENERATOR));
        //g^((q-1)/2^5)
        let mut z = multiplicative_generator.power_by(
            U256::from_be_hex("00e7db4ea6533afa906673b0101343b007fc7c3803a0c8238ba7e835a943b73f")
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

    fn to_curve_bytes(&self) -> &[u8] {
        self.as_bytes()
    }
    fn to_words(&self) -> Vec<u64> {
        self.0.to_words().into()
    }

    fn from_words(a: &Vec<u64>) -> Self {
        let k = [a[0], a[1], a[2], a[3]];
        let value = U256::from_words(k);
        BandScalar(value)
    }

    fn from_uint_reduced(w: BandScalar) -> Self {
        let (r, underflow) = w.0.sbb(&BANDSCALAR_MODULUS, Limb::ZERO);
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
/// ========= prime field implementation =========
//impl base field on BandScalar
impl PrimeField for BandScalar {
    type Repr = FieldBytes;
    const TWO_ADIC_ROOT: &'static str =
        "19470B7EFE802F9B36B6675F52C7008234BB3E0CB7ED22AEC65A62A1234BD960";
    const GENERATOR: Self = BandScalar(U256::from_u32(7));
    const NUM_BITS: u32 = (ELEMENT_BYTES * 8) as u32; // every byte is 8 bits
    const MODULUS: &'static str =
        "1CFB69D4CA675F520CCE760202687600FF8F87007419047174FD06B52876E7E1";
    const TWO_ADDICITY: u32 = 5;
    // the function takes a self argument and check if the input is odd or not
    // it uses the is_odd of the crypto bigint to analyse the value.
    //return a bool value - true or false
    fn is_odd(self) -> Choice {
        let a = self.0.is_odd();
        a
    }
    // not possible for BandScalar.
    // the root_of_unity does not exist for BandScalar field
    fn get_root_of_unity(k: u32) -> Self {
        //if n == 0 ; 2^0 root does not exist
        // if n is more than 5 no root exist
        assert!(k == 0 || k <= 5, "2^{:?} th root does not exist", k);
        if k == 5 {
            println!("{:?}", BandScalar(TWO_ADIC_ROOT))
        };
        //TWO_ADIC_ROOT: & 'static str = "005282DB87529CFA3F0464519C8B0FA5AD187148E11A61616070024F42F8EF94";
        BandScalar(TWO_ADIC_ROOT).power_by((U256::ONE << ((5 - k) as usize)).to_words())
    }
}

// ======== From implementation ==========
// implementing the From bound
impl From<u8> for BandScalar {
    // uses the from_u64 crypto_bigint function to return field element from u8 bit
    fn from(value: u8) -> Self {
        Self(U256::from_u8(value))
    }
}
impl From<u16> for BandScalar {
    // uses the from_u64 crypto_bigint function to return field element from u16 bit
    fn from(value: u16) -> Self {
        Self(U256::from_u16(value))
    }
}
impl From<u32> for BandScalar {
    // uses the from_u64 crypto_bigint function to return field element from u32 bit
    fn from(value: u32) -> Self {
        Self(U256::from_u32(value))
    }
}
impl From<u64> for BandScalar {
    // uses the from_u64 crypto_bigint function to return field element from u64 bit
    fn from(value: u64) -> Self {
        Self(U256::from_u64(value))
    }
}
impl From<u128> for BandScalar {
    // uses the from_u64 crypto_bigint function to return field element from u128 bit
    fn from(value: u128) -> Self {
        Self(U256::from_u128(value))
    }
}
impl From<[u64; 6]> for BandScalar {
    fn from(value: [u64; 6]) -> Self {
        let value = U256::from_words([value[0], value[1], value[2], value[3]]);
        Self::new(value)
    }
}

impl From<U256> for BandScalar {
    fn from(value: U256) -> Self {
        let value = U256::from_words(value.into());
        Self::new(value)
    }
}

//=========bound implementation========
impl AsRef<[u8]> for BandScalar {
    fn as_ref(&self) -> &[u8] {
        self.as_ref()
    }
}
impl AsMut<u8> for BandScalar {
    fn as_mut(&mut self) -> &mut u8 {
        self.as_mut()
    }
}
impl Add<BandScalar> for BandScalar {
    type Output = BandScalar;
    // uses the field add function to add two (a,b) elements
    fn add(self, rhs: Self) -> Self::Output {
        add(self, rhs)
    }
}
impl Add<&BandScalar> for &BandScalar {
    type Output = BandScalar;
    // uses the field add function to add two (a, &b) elements
    fn add(self, rhs: &BandScalar) -> Self::Output {
        add(*self, *rhs)
    }
}
impl Add<&BandScalar> for BandScalar {
    type Output = BandScalar;
    // uses the field add function to add two (a,b) elements
    fn add(self, rhs: &BandScalar) -> Self::Output {
        add(self, *rhs)
    }
}
impl Sub<BandScalar> for BandScalar {
    type Output = BandScalar;
    // uses the field sub function to subtract two (a,b) elements
    fn sub(self, rhs: BandScalar) -> Self::Output {
        sub(self, rhs)
    }
}
//sub trait implement
impl Sub<&BandScalar> for &BandScalar {
    //borrowed struct BandScalar
    type Output = BandScalar;
    // uses the field sub function to subtract two (a,b) elements
    fn sub(self, rhs: &BandScalar) -> Self::Output {
        sub(*self, *rhs)
    }
}

//implementing multiplcation for BandScalar
impl Mul<BandScalar> for BandScalar {
    type Output = BandScalar;
    // uses the field mul function to multiply two (a,b) elements
    fn mul(self, rhs: BandScalar) -> Self::Output {
        mul(self, rhs)
    }
}
impl Mul<&BandScalar> for BandScalar {
    type Output = BandScalar;
    // uses the field mul function to multiply two (a,b) elements
    fn mul(self, rhs: &BandScalar) -> Self::Output {
        mul(self, *rhs)
    }
}
impl Mul<&BandScalar> for &BandScalar {
    type Output = BandScalar;
    // uses the field mul function to multiply two (a,b) elements
    fn mul(self, rhs: &BandScalar) -> Self::Output {
        mul(*self, *rhs)
    }
}
impl Div<BandScalar> for BandScalar {
    type Output = BandScalar;
    // uses the field div function to division two (a,b) elements
    fn div(self, rhs: BandScalar) -> Self::Output {
        div(self, rhs)
    }
}
impl Div<&BandScalar> for &BandScalar {
    type Output = BandScalar;
    // uses the field div function to division two (a,b) elements
    fn div(self, rhs: &BandScalar) -> Self::Output {
        div(*self, *rhs)
    }
}
//
impl AddAssign<BandScalar> for BandScalar {
    // uses the field add function to add two (a,b) elements
    //assign a the value of the sum
    fn add_assign(&mut self, rhs: BandScalar) {
        *self = add(*self, rhs)
    }
}
impl AddAssign<&BandScalar> for BandScalar {
    // uses the field add function to add two (a,b) elements
    //assign a the value of the sum
    fn add_assign(&mut self, rhs: &BandScalar) {
        *self = add(*self, *rhs)
    }
}
impl SubAssign<BandScalar> for BandScalar {
    // uses the field sub function to subtract two (a,b) elements
    //assign a the value of the subtraction
    fn sub_assign(&mut self, rhs: BandScalar) {
        *self = sub(*self, rhs)
    }
}
impl MulAssign<BandScalar> for BandScalar {
    // uses the field mul function to multiply two (a,b) elements
    //assign a the value of the multiplication
    fn mul_assign(&mut self, rhs: BandScalar) {
        *self = mul(*self, rhs)
    }
}
impl DivAssign<BandScalar> for BandScalar {
    // uses the field div function to divsion two (a,b) elements
    //assign a the value of the division
    fn div_assign(&mut self, rhs: BandScalar) {
        *self = div(*self, rhs)
    }
}

impl Neg for BandScalar {
    type Output = BandScalar;
    // uses the neg BandScalar function to return negative of the BandScalar element
    fn neg(self) -> Self::Output {
        neg(self)
    }
}
//implementing the deserializable bound for BandScalar
impl Deserializable for BandScalar {
    fn read_from<R: ByteReader>(source: &mut R) -> Result<Self, DeserializationError> {
        let value = source.read_u8_array()?;
        let value = U256::from_le_byte_array(value.into());
        if value >= BANDSCALAR_MODULUS {
            return Err(DeserializationError::InvalidValue(format!(
                "invalid field element: value {} is greater than or equal to the field modulus",
                value
            )));
        }
        Ok(BandScalar(value))
    }
}
//implementing the serializable bound for BandScalar
// implementing the serializable for the base field
impl Serializable for BandScalar {
    fn write_into<W: ByteWriter>(&self, target: &mut W) {
        target.write_u8_slice(&self.0.to_le_bytes());
    }
}
//implementing the randomizable bound for BandScalar
impl Randomizable for BandScalar {
    const VALUE_SIZE: usize = 256;
    fn from_random_bytes(source: &[u8]) -> Option<Self> {
        Self::try_from(source).ok()
    }
}
impl AsBytes for BandScalar {
    fn as_bytes(&self) -> &[u8] {
        // TODO: take endianness into account
        let self_ptr: *const BandScalar = self;
        unsafe { slice::from_raw_parts(self_ptr as *const u8, ELEMENT_BYTES) } // element bytes are what?
    }
}
impl Eq for BandScalar {}
impl ConstantTimeEq for BandScalar {
    fn ct_eq(&self, other: &Self) -> Choice {
        self.0.ct_eq(&other.0)
    }
}
impl PartialEq for BandScalar {
    fn eq(&self, other: &Self) -> bool {
        self.ct_eq(other).into()
    }
}
impl Display for BandScalar {
    //function output the writing format for the input given
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}
impl From<[u8; 32]> for BandScalar {
    /// Converts the value encoded in an array of 16 bytes into a field element. The bytes
    /// are assumed to be in little-endian byte order. If the value is greater than or equal
    /// to the field modulus, modular reduction is silently performed.
    fn from(bytes: [u8; 32]) -> Self {
        let value = Self::from_repr(&bytes.try_into().unwrap());
        value
    }
}
impl<'a> TryFrom<&'a [u8]> for BandScalar {
    type Error = String;
    /// Converts a slice of bytes into a field element; returns error if the value encoded in bytes
    /// is not a valid field element. The bytes are assumed to be in little-endian byte order.
    fn try_from(bytes: &[u8]) -> Result<Self, Self::Error> {
        let value = bytes
            .try_into()
            .map(Self::from_repr)
            .map_err(|error| format!("{error}"))?;

        let s = BandScalar(BANDSCALAR_MODULUS);
        if value >= s {
            return Err(format!(
                "cannot convert bytes into a BandScalar element: \
                value {value} is greater or equal to the BandScalar modulus"
            ));
        }
        Ok(value)
    }
}
impl PartialOrd for BandScalar {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}
impl Ord for BandScalar {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.0.cmp(&other.0)
    }
}
//
impl Reduce<U256> for BandScalar {
    fn from_uint_reduced(w: U256) -> Self {
        let (r, underflow) = w.sbb(&BANDSCALAR_MODULUS, Limb::ZERO);
        let underflow = Choice::from((underflow.0 >> (Limb::BITS - 1)) as u8);
        Self(U256::conditional_select(&w, &r, !underflow))
    }
}
impl ConditionallySelectable for BandScalar {
    fn conditional_select(a: &Self, b: &Self, choice: Choice) -> Self {
        Self(U256::conditional_select(&a.0, &b.0, choice))
    }
}
//============ global fucntions============
//Addition
// it takes two BandScalar elements as inputs and
// returns a + b %p as output. It uses the add_mod
// function of the crypto_bigint function to perform calculations
pub fn add(a: BandScalar, b: BandScalar) -> BandScalar {
    BandScalar(a.0.add_mod(&b.0, &BANDSCALAR_MODULUS))
}
// Subtraction
// it takes two BandScalar elements as inputs and
// returns a - b %p as output. It uses the sub_mod
// function of the crypto_bigint function to perform calculations
pub fn sub(a: BandScalar, b: BandScalar) -> BandScalar {
    BandScalar(a.0.sub_mod(&b.0, &BANDSCALAR_MODULUS))
}
// Negation
// it takes one BandScalar element as input and
// return the negation of the element as -a mod p. It uses the neg_mod
// function of the crypto_bigint function to perform calculations
pub fn neg(a: BandScalar) -> BandScalar {
    BandScalar(a.0.neg_mod(&BANDSCALAR_MODULUS))
}
// Multiplication
// it takes two field elements as inputs and
// returns a * b %p as output. It first performs the mul wide
// operation using mul_wide function --> (lo, hi) output. Then do barrett_reduction
// to bring the value in the field if greater than the modulus.
pub fn mul(a: BandScalar, b: BandScalar) -> BandScalar {
    let prod = a.0.mul_wide(&b.0);
    let res = barrett_reduce(prod.0, prod.1);
    BandScalar(res)
}
// Division
// it takes two field elements as inputs and
// returns a / b %p as output. It uses the mul and
// invert functions. It inverts second element,
//b --> invert(b) then multiply with the first element,
// a to give the output of division.
pub fn div(a: BandScalar, b: BandScalar) -> BandScalar {
    // a /b  is assumed while division
    let res = mul(a, invert(b).unwrap());
    res
}
// Square function
// it take a BandScalar element. Uses the square method implemented
// to return the square of the element.
pub fn square(a: BandScalar) -> BandScalar {
    a.square()
}

// Shift right by one bit
// it takes an element and shift the bits
// to right by one -- meaning -- diving by 2
pub fn shr1(a: &mut BandScalar) {
    a.0 >>= 1;
}

pub fn invert(a: BandScalar) -> CtOption<BandScalar> {
    #[allow(non_snake_case)]
    let element = a;
    let mut u = element; //removed the dereference
    let mut v = BandScalar(BANDSCALAR_MODULUS);
    let mut a = BandScalar(U256::from_u8(1));
    let mut c = BandScalar(U256::ZERO);
    while !bool::from(u.is_zero()) {
        while bool::from(u.0.is_even()) {
            shr1(&mut u);
            let was_odd: bool = a.is_odd().into();
            shr1(&mut a);
            if was_odd {
                a = BandScalar(a.0.add_mod(&FRAC_BANDSCALAR_MODULUS_2.0, &BANDSCALAR_MODULUS));
                a = BandScalar(a.0.add_mod(&U256::ONE, &BANDSCALAR_MODULUS));
            }
        }
        while bool::from(v.0.is_even()) {
            shr1(&mut v);
            let was_odd: bool = c.is_odd().into();
            shr1(&mut c);
            if was_odd {
                c = BandScalar(c.0.add_mod(&FRAC_BANDSCALAR_MODULUS_2.0, &BANDSCALAR_MODULUS));
                c = BandScalar(c.0.add_mod(&U256::ONE, &BANDSCALAR_MODULUS));
            }
        }
        if u >= v {
            u = BandScalar(u.0.sub_mod(&v.0, &BANDSCALAR_MODULUS));
            a = BandScalar(a.0.sub_mod(&c.0, &BANDSCALAR_MODULUS));
        } else {
            v = BandScalar(v.0.sub_mod(&u.0, &BANDSCALAR_MODULUS));
            c = BandScalar(c.0.sub_mod(&a.0, &BANDSCALAR_MODULUS));
        }
    }
    CtOption::new(BandScalar(c.0), !element.0.is_zero())
}

#[cfg(feature = "serde")]
#[cfg_attr(docsrs, doc(cfg(feature = "serde")))]
impl Serialize for BandScalar {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: ser::Serializer,
    {
        BandScalarCore::from(self).serialize(serializer)
    }
}
#[cfg(feature = "serde")]
#[cfg_attr(docsrs, doc(cfg(feature = "serde")))]
impl<'de> Deserialize<'de> for BandScalar {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: de::Deserializer<'de>,
    {
        Ok(BandScalarCore::deserialize(deserializer)?.into())
    }
}
#[cfg(feature = "bits")]
#[cfg_attr(docsrs, doc(cfg(feature = "bits")))]
impl From<&BandScalar> for BandScalarBits {
    fn from(BandScalar: &BandScalar) -> BandScalarBits {
        BandScalar.0.to_words().into()
    }
}
//=========================================
///
/// The general algorithm is:
/// ```text
/// p = n = order of group
/// b = 2^64 = 64bit machine word
/// k = 4
/// a \in [0, 2^512]
/// mu := floor(b^{2k} / p)
/// q1 := floor(a / b^{k - 1})
/// q2 := q1 * mu
/// q3 := <- floor(q2/ b^{k +1})
/// r1 := a mod b^{k + 1}
/// r2 := q3 * m mod b^{k + 1}
/// r := r1 - r2
///
/// if r < 0: r := r + b^{k + 1}
/// while r >= p: do r := r - p (at most twice)
/// ```
///
/// References:
/// - Handbook of Applied Cryptography, Chapter 14
///   Algorithm 14.42
///   http://cacr.uwaterloo.ca/hac/about/chap14.pdf
///
/// - Efficient and Secure Elliptic Curve Cryptography Implementation of Curve P-256
///   Algorithm 6) t Reduction modulo p
///   https://csrc.nist.gov/csrc/media/events/workshop-on-elliptic-curve-cryptography-standards/documents/papers/session6-adalier-mehmet.pdf
///
///
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
    let bandscalar_modulus = BANDSCALAR_MODULUS.as_words();
    let (w0, carry) = mac(0, q3[0], bandscalar_modulus[0], 0);
    let (w1, carry) = mac(0, q3[0], bandscalar_modulus[1], carry);
    let (w2, carry) = mac(0, q3[0], bandscalar_modulus[2], carry);
    let (w3, carry) = mac(0, q3[0], bandscalar_modulus[3], carry);
    let (w4, _) = mac(0, q3[0], 0, carry);
    let (w1, carry) = mac(w1, q3[1], bandscalar_modulus[0], 0);
    let (w2, carry) = mac(w2, q3[1], bandscalar_modulus[1], carry);
    let (w3, carry) = mac(w3, q3[1], bandscalar_modulus[2], carry);
    let (w4, _) = mac(w4, q3[1], bandscalar_modulus[3], carry);
    let (w2, carry) = mac(w2, q3[2], bandscalar_modulus[0], 0);
    let (w3, carry) = mac(w3, q3[2], bandscalar_modulus[1], carry);
    let (w4, _) = mac(w4, q3[2], bandscalar_modulus[2], carry);
    let (w3, carry) = mac(w3, q3[3], bandscalar_modulus[0], 0);
    let (w4, _) = mac(w4, q3[3], bandscalar_modulus[1], carry);
    let (w4, _) = mac(w4, q3[4], bandscalar_modulus[0], 0);
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
    let bandscalar_modulus = BANDSCALAR_MODULUS.as_words();
    let (w0, borrow) = sbb(r0, bandscalar_modulus[0], 0);
    let (w1, borrow) = sbb(r1, bandscalar_modulus[1], borrow);
    let (w2, borrow) = sbb(r2, bandscalar_modulus[2], borrow);
    let (w3, borrow) = sbb(r3, bandscalar_modulus[3], borrow);
    let (w4, borrow) = sbb(r4, 0, borrow);
    // If underflow occurred on the final limb, borrow = 0xfff...fff, otherwise
    // borrow = 0x000...000. Thus, we use it as a mask to conditionally add the
    // BANDSCALAR_MODULUS.
    let (w0, carry) = adc(w0, bandscalar_modulus[0] & borrow, 0);
    let (w1, carry) = adc(w1, bandscalar_modulus[1] & borrow, carry);
    let (w2, carry) = adc(w2, bandscalar_modulus[2] & borrow, carry);
    let (w3, carry) = adc(w3, bandscalar_modulus[3] & borrow, carry);
    let (w4, _carry) = adc(w4, 0, carry);
    [w0, w1, w2, w3, w4]
}
