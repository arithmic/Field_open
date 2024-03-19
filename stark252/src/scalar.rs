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

//super
use super::{
    field::Reduce,
    util::{adc, mac, sbb},
};

// Size of field elements of this elliptic curve.
pub type FieldSize = <U256 as crypto_bigint::ArrayEncoding>::ByteSize;

/// Byte representation of a base/scalar field element of a given curve.
pub type FieldBytes = GenericArray<u8, FieldSize>;

// Number of bytes needed to represent field element
const ELEMENT_BYTES: usize = std::mem::size_of::<U256>();

// n= 3618502788666131213697322783095070105526743751716087489154079457884512865583
pub const SCALAR_MODULUS: U256 =
    U256::from_be_hex("0800000000000010FFFFFFFFFFFFFFFFB781126DCAE7B2321E66A241ADC64D2F");

// two adic root
pub const TWO_ADIC_ROOT: U256 =
    U256::from_be_hex("0800000000000010FFFFFFFFFFFFFFFFB781126DCAE7B2321E66A241ADC64D2E");

// SCALAR_MODULUS/2
const FRAC_SCALAR_MODULUS_2: Scalar = Scalar(SCALAR_MODULUS.shr_vartime(1));

//mu = 1f_0xffff_ffff_ffff_bc00_0x0000_0000_0090_8121_0xfbb6_48d3_2e22_6718_0x9ec1_7538_4d77_b0ff
#[allow(unused)]
pub const MU: [u64; 5] = [
    0x9EC1_7538_4D77_B0FD,
    0xFBB6_48D3_2E22_6718,
    0x0000_0000_0090_8121,
    0xFFFF_FFFF_FFFF_BC00,
    0x0000_0000_0000_001F,
];
//====== struct implementation =======
#[derive(Debug, Copy, Clone, Default)]
pub struct Scalar(pub U256); //defining the scalar struct

//implementing the scalar struct
impl Scalar {
    //implement zero and one
    pub const ZERO: Self = Self(U256::ZERO);
    pub const ONE: Self = Self(U256::ONE);
    pub const SCALAR_MODULUS_1: Self = Self(U256::from_be_hex(
        "0800000000000010FFFFFFFFFFFFFFFFB781126DCAE7B2321E66A241ADC64D2E",
    ));

    //return the new instance of the struct it takes any element as input
    //convert on U256 bit and check if the value is greater or less than modulus
    //if more than modulus, it uses mul_wide and barrett reduce to bring the
    //value in the field range. It return self % p
    pub fn new(other: U256) -> Self {
        let prod = other.mul_wide(&U256::ONE);
        let res = barrett_reduce(prod.0, prod.1);
        Scalar(CtOption::new(res, res.ct_lt(&SCALAR_MODULUS)).unwrap())
    }
    pub fn from_repr(bytes: &FieldBytes) -> Scalar {
        let inner = U256::from_be_byte_array(*bytes);
        <Scalar as Field>::from_uint_reduced(Scalar(inner))
    }
    //zeroed vector
   pub  fn zeroed_vector(n: usize) -> Vec<Self> {
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
//implement Field for Scalar
impl Field for Scalar {
    type BaseField = Scalar;
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
    // uses the add function of the scalar field
    fn double(self) -> Self {
        add(self, self)
    }
    // triple functions
    // it take a field element and return 3 times the element in the scalar field.
    //depends on double function
    fn triple(self) -> Self {
        add(self, self.double())
    }

    fn cube(self) -> Self {
        self * self.double()
    }
    //square of element
    //it takes a self (field element) and returns the square of the scalar field element.
    //it uses square_wide of the crypto_bigint to give a tuple output (lo, hi)
    //which is reduced using the barrett_reduce function
    fn square(self) -> Self {
        let res = self.0.square_wide();
        let prod = barrett_reduce(res.0, res.1);
        Self(prod)
    }
    //inverting or inverse of an element function
    // takes a self and returns the inverse element of the self in the Scalar field
    // uses is_odd, is_even, add_mod, bit shift (shr1), sub_mod functions
    fn invert(self) -> CtOption<Scalar> {
        let a = invert(self);
        // convert scalar to ctoption
        a
    }
    // Exponentiates self by exp
    // Need to exponentiate the self (U256 bit) value with another 256 bit
    // value as power. The power is given in an array of u64 with len 4.
    // we start with ONE of Scalar. we loop in the size of the array. We
    // then loop in the bit length of the value. and do the square of the value
    // but in this loop if bit is 1 then the value need to be multiplied with the base value.
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
    // square root function computes the square roort of the field element
    // depends on exp, square, new takes an input in the field.
    fn sqrt(self) -> CtOption<Self> {
        // Tonelli-Shank's algorithm for q mod 4 = 3
        // See https://eprint.iacr.org/2012/685.pdf
        // M+1/4= 904625697166532803424330695773767526381685937929021872288519864471128216396
        // Compute s^((M+1)/4)
        let s = self.power_by([
            0x8799a8906b71934c,
            0xede0449b72b9ec8c,
            0x3fffffffffffffff,
            0x0200000000000004,
            //0200000000000004_3fffffffffffffff_ede0449b72b9ec8c_8799a8906b71934c
        ]);

        CtOption::new(s, (s.square()).ct_eq(&self))
    }

    //it generates the random values of U256 bits using the random OsRng function of the crypto
    //bigint. The r value stores the random value. It can be greater then the modulus. We
    // convert it into Scalar element using the mul_wide function from crypto_bigint. Then
    // do barret reduce to reduce the value from modulus and bring it in the Scalar as Scalar element.
    fn random() -> Self {
        //return random Scalar element
        let mut r = Scalar(U256::random(&mut OsRng));
        let m = Scalar(SCALAR_MODULUS);
        while r >= m {
            r -= m;
        }
        r
    }
    // not required for scalar field
    fn to_curve_bytes(&self) -> &[u8] {
        unimplemented!()
    }
    fn to_words(&self) -> Vec<u64> {
        self.0.to_words().into()
    }

    fn from_uint_reduced(w: Scalar) -> Self {
        let (r, underflow) = w.0.sbb(&SCALAR_MODULUS, Limb::ZERO);
        let underflow = Choice::from((underflow.0 >> (Limb::BITS - 1)) as u8);
        Self(U256::conditional_select(&w.0, &r, !underflow))
    }

    fn from_words(a: &Vec<u64>) -> Self {
        let k = [a[0], a[1], a[2], a[3]];
        let value = U256::from_words(k);
        Scalar(value)
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
//impl base field on scalar
impl PrimeField for Scalar {
    type Repr = FieldBytes;
    const TWO_ADIC_ROOT: &'static str =
        "0800000000000010FFFFFFFFFFFFFFFFB781126DCAE7B2321E66A241ADC64D2E";
    const GENERATOR: Self = unimplemented!();
    const NUM_BITS: u32 = (ELEMENT_BYTES * 8) as u32; // every byte is 8 bits
    const MODULUS: &'static str =
        "0800000000000010FFFFFFFFFFFFFFFFB781126DCAE7B2321E66A241ADC64D2F";
    const TWO_ADDICITY: u32 = 1;
    // the function takes a self argument and check if the input is odd or not
    // it uses the is_odd of the crypto bigint to analyse the value.
    //return a bool value - true or false
    fn is_odd(self) -> Choice {
        let a = self.0.is_odd();
        a
    }
    // not possible for scalar.
    // the root_of_unity does not exist for scalar field
    fn get_root_of_unity(n: u32) -> Self {
        assert!(n == 1, "2^{n}th root does not exist");
        Scalar(TWO_ADIC_ROOT)
    }
}
// ======== From implementation ==========
// implementing the From bound
impl From<u8> for Scalar {
    // uses the from_u64 crypto_bigint function to return field element from u8 bit
    fn from(value: u8) -> Self {
        Self(U256::from_u8(value))
    }
}
impl From<u16> for Scalar {
    // uses the from_u64 crypto_bigint function to return field element from u16 bit
    fn from(value: u16) -> Self {
        Self(U256::from_u16(value))
    }
}
impl From<u32> for Scalar {
    // uses the from_u64 crypto_bigint function to return field element from u32 bit
    fn from(value: u32) -> Self {
        Self(U256::from_u32(value))
    }
}
impl From<u64> for Scalar {
    // uses the from_u64 crypto_bigint function to return field element from u64 bit
    fn from(value: u64) -> Self {
        Self(U256::from_u64(value))
    }
}
impl From<u128> for Scalar {
    // uses the from_u64 crypto_bigint function to return field element from u128 bit
    fn from(value: u128) -> Self {
        Self(U256::from_u128(value))
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
        let value = U256::from_words(value.into());
        Self::new(value)
    }
}

//=========bound implementation========
impl AsRef<[u8]> for Scalar {
    fn as_ref(&self) -> &[u8] {
        self.as_ref()
    }
}
impl AsMut<u8> for Scalar {
    fn as_mut(&mut self) -> &mut u8 {
        self.as_mut()
    }
}
impl Add<Scalar> for Scalar {
    type Output = Scalar;
    // uses the field add function to add two (a,b) elements
    fn add(self, rhs: Self) -> Self::Output {
        add(self, rhs)
    }
}
impl Add<&Scalar> for &Scalar {
    type Output = Scalar;
    // uses the field add function to add two (a, &b) elements
    fn add(self, rhs: &Scalar) -> Self::Output {
        add(*self, *rhs)
    }
}
impl Add<&Scalar> for Scalar {
    type Output = Scalar;
    // uses the field add function to add two (a,b) elements
    fn add(self, rhs: &Scalar) -> Self::Output {
        add(self, *rhs)
    }
}
impl Sub<Scalar> for Scalar {
    type Output = Scalar;
    // uses the field sub function to subtract two (a,b) elements
    fn sub(self, rhs: Scalar) -> Self::Output {
        sub(self, rhs)
    }
}
//sub trait implement
impl Sub<&Scalar> for &Scalar {
    //borrowed struct scalar
    type Output = Scalar;
    // uses the field sub function to subtract two (a,b) elements
    fn sub(self, rhs: &Scalar) -> Self::Output {
        sub(*self, *rhs)
    }
}

//implementing multiplcation for Scalar
impl Mul<Scalar> for Scalar {
    type Output = Scalar;
    // uses the field mul function to multiply two (a,b) elements
    fn mul(self, rhs: Scalar) -> Self::Output {
        mul(self, rhs)
    }
}
impl Mul<&Scalar> for Scalar {
    type Output = Scalar;
    // uses the field mul function to multiply two (a,b) elements
    fn mul(self, rhs: &Scalar) -> Self::Output {
        mul(self, *rhs)
    }
}
impl Mul<&Scalar> for &Scalar {
    type Output = Scalar;
    // uses the field mul function to multiply two (a,b) elements
    fn mul(self, rhs: &Scalar) -> Self::Output {
        mul(*self, *rhs)
    }
}
impl Div<Scalar> for Scalar {
    type Output = Scalar;
    // uses the field div function to division two (a,b) elements
    fn div(self, rhs: Scalar) -> Self::Output {
        div(self, rhs)
    }
}
impl Div<&Scalar> for &Scalar {
    type Output = Scalar;
    // uses the field div function to division two (a,b) elements
    fn div(self, rhs: &Scalar) -> Self::Output {
        div(*self, *rhs)
    }
}
//
impl AddAssign<Scalar> for Scalar {
    // uses the field add function to add two (a,b) elements
    //assign a the value of the sum
    fn add_assign(&mut self, rhs: Scalar) {
        *self = add(*self, rhs)
    }
}
impl AddAssign<&Scalar> for Scalar {
    // uses the field add function to add two (a,b) elements
    //assign a the value of the sum
    fn add_assign(&mut self, rhs: &Scalar) {
        *self = add(*self, *rhs)
    }
}
impl SubAssign<Scalar> for Scalar {
    // uses the field sub function to subtract two (a,b) elements
    //assign a the value of the subtraction
    fn sub_assign(&mut self, rhs: Scalar) {
        *self = sub(*self, rhs)
    }
}
impl MulAssign<Scalar> for Scalar {
    // uses the field mul function to multiply two (a,b) elements
    //assign a the value of the multiplication
    fn mul_assign(&mut self, rhs: Scalar) {
        *self = mul(*self, rhs)
    }
}
impl DivAssign<Scalar> for Scalar {
    // uses the field div function to divsion two (a,b) elements
    //assign a the value of the division
    fn div_assign(&mut self, rhs: Scalar) {
        *self = div(*self, rhs)
    }
}

impl Neg for Scalar {
    type Output = Scalar;
    // uses the neg Scalar function to return negative of the scalar element
    fn neg(self) -> Self::Output {
        neg(self)
    }
}

//implementing the deserializable bound for scalar
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
    fn read_batch_from<R: ByteReader>(
        source: &mut R,
        num_elements: usize,
    ) -> Result<Vec<Self>, DeserializationError> {
        let mut result = Vec::new();
        for i in 0..num_elements {
            let value = source.read_u8_array()?;
            let value = U256::from_le_byte_array(value.into());
            if value >= SCALAR_MODULUS {
                return Err(DeserializationError::InvalidValue(format!(
                    "invalid field element: value {}, {} is greater than or equal to the  scalar modulus",
                    value,i
                )));
            }
            result.push(Scalar(value))
        }

        Ok(result)
    }
}

//Implementing the serializable for the base field
impl Serializable for Scalar {
    fn write_into<W: ByteWriter>(&self, target: &mut W) {
        target.write_u8_slice(&self.0.to_le_bytes());
    }
}
//Implementing the randomizable bound for scalar
impl Randomizable for Scalar {
    const VALUE_SIZE: usize = 256;
    fn from_random_bytes(source: &[u8]) -> Option<Self> {
        Self::try_from(source).ok()
    }
}
impl AsBytes for Scalar {
    fn as_bytes(&self) -> &[u8] {
        // TODO: take endianness into account
        let self_ptr: *const Scalar = self;
        unsafe { slice::from_raw_parts(self_ptr as *const u8, ELEMENT_BYTES) } // element bytes are what?
    }
}
impl Eq for Scalar {}
impl ConstantTimeEq for Scalar {
    fn ct_eq(&self, other: &Self) -> Choice {
        self.0.ct_eq(&other.0)
    }
}
impl PartialEq for Scalar {
    fn eq(&self, other: &Self) -> bool {
        self.ct_eq(other).into()
    }
}
impl Display for Scalar {
    //function output the writing format for the input given
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}
impl From<[u8; 32]> for Scalar {
    /// Converts the value encoded in an array of 16 bytes into a field element. The bytes
    /// are assumed to be in little-endian byte order. If the value is greater than or equal
    /// to the field modulus, modular reduction is silently performed.
    fn from(bytes: [u8; 32]) -> Self {
        let value = Self::from_repr(&bytes.try_into().unwrap());
        value
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

        let s = Scalar(SCALAR_MODULUS);
        if value >= s {
            return Err(format!(
                "cannot convert bytes into a scalar element: \
                value {value} is greater or equal to the scalar modulus"
            ));
        }
        Ok(value)
    }
}
impl PartialOrd for Scalar {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}
impl Ord for Scalar {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.0.cmp(&other.0)
    }
}
//
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
//============ global fucntions============
//Addition
// it takes two scalar elements as inputs and
// returns a + b %p as output. It uses the add_mod
// function of the crypto_bigint function to perform calculations
pub fn add(a: Scalar, b: Scalar) -> Scalar {
    Scalar(a.0.add_mod(&b.0, &SCALAR_MODULUS))
}
// Subtraction
// it takes two Scalar elements as inputs and
// returns a - b %p as output. It uses the sub_mod
// function of the crypto_bigint function to perform calculations
pub fn sub(a: Scalar, b: Scalar) -> Scalar {
    Scalar(a.0.sub_mod(&b.0, &SCALAR_MODULUS))
}
// Negation
// it takes one scalar element as input and
// return the negation of the element as -a mod p. It uses the neg_mod
// function of the crypto_bigint function to perform calculations
pub fn neg(a: Scalar) -> Scalar {
    Scalar(a.0.neg_mod(&SCALAR_MODULUS))
}
// Multiplication
// it takes two field elements as inputs and
// returns a * b %p as output. It first performs the mul wide
// operation using mul_wide function --> (lo, hi) output. Then do barrett_reduction
// to bring the value in the field if greater than the modulus.
pub fn mul(a: Scalar, b: Scalar) -> Scalar {
    let prod = a.0.mul_wide(&b.0);
    let res = barrett_reduce(prod.0, prod.1);
    Scalar(res)
}
// Division
// it takes two field elements as inputs and
// returns a / b %p as output. It uses the mul and
// invert functions. It inverts second element,
//b --> invert(b) then multiply with the first element,
// a to give the output of division.
pub fn div(a: Scalar, b: Scalar) -> Scalar {
    // a /b  is assumed while division
    let res = mul(a, invert(b).unwrap());
    res
}
// Square function
// it take a Scalar element. Uses the square method implemented
// to return the square of the element.
pub fn square(a: Scalar) -> Scalar {
    a.square()
}

// Shift right by one bit
// it takes an element and shift the bits
// to right by one -- meaning -- diving by 2
pub fn shr1(a: &mut Scalar) {
    a.0 >>= 1;
}

pub fn invert(a: Scalar) -> CtOption<Scalar> {
    #[allow(non_snake_case)]
    let element = a;
    let mut u = element; //removed the dereference
    let mut v = Scalar(SCALAR_MODULUS);
    let mut a = Scalar(U256::from_u8(1));
    let mut c = Scalar(U256::ZERO);
    while !bool::from(u.is_zero()) {
        while bool::from(u.0.is_even()) {
            shr1(&mut u);
            let was_odd: bool = a.is_odd().into();
            shr1(&mut a);
            if was_odd {
                a = Scalar(a.0.add_mod(&FRAC_SCALAR_MODULUS_2.0, &SCALAR_MODULUS));
                a = Scalar(a.0.add_mod(&U256::ONE, &SCALAR_MODULUS));
            }
        }
        while bool::from(v.0.is_even()) {
            shr1(&mut v);
            let was_odd: bool = c.is_odd().into();
            shr1(&mut c);
            if was_odd {
                c = Scalar(c.0.add_mod(&FRAC_SCALAR_MODULUS_2.0, &SCALAR_MODULUS));
                c = Scalar(c.0.add_mod(&U256::ONE, &SCALAR_MODULUS));
            }
        }
        if u >= v {
            u = Scalar(u.0.sub_mod(&v.0, &SCALAR_MODULUS));
            a = Scalar(a.0.sub_mod(&c.0, &SCALAR_MODULUS));
        } else {
            v = Scalar(v.0.sub_mod(&u.0, &SCALAR_MODULUS));
            c = Scalar(c.0.sub_mod(&a.0, &SCALAR_MODULUS));
        }
    }
    CtOption::new(Scalar(c.0), !element.0.is_zero())
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
