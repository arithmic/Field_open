//crate imports
use crypto_bigint::Random;
use traits::traits::Field;
use traits::traits::{Extensible, PrimeField};

//bigint crate
use core::{
    AsBytes, ByteReader, ByteWriter, Deserializable, DeserializationError, Randomizable,
    Serializable,
};
// use crypto_bigint::consts::{U2, U25}; //consts
use crypto_bigint::{
    generic_array::GenericArray,
    subtle::{Choice, ConditionallySelectable, ConstantTimeEq, ConstantTimeLess, CtOption},
    ArrayEncoding, Encoding, Integer, Limb,  Zero, U256,
};
use rand::rngs::OsRng;
use std::{
    convert::TryInto,
    fmt::{Display, Formatter},
    mem,
    ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Neg,  Sub, SubAssign},
    slice,
};
//allow macro for dealing with non used imports
use super::util::{adc, mac, sbb};

//======constants
// Size of field elements of this elliptic curve.
pub type FieldSize = <U256 as crypto_bigint::ArrayEncoding>::ByteSize;
/// Byte representation of a base/scalar field element of a given curve.
pub type FieldBytes = GenericArray<u8, FieldSize>;
// Number of bytes needed to represent field element
const ELEMENT_BYTES: usize = std::mem::size_of::<U256>();
/// Constant representing the modulus
/// p = 2^{251} +17. 2^{192} + 1
// be_hex("0800000000000011000000000000000000000000000000000000000000000001");
///p = 3618502788666131213697322783095070105623107215331596699973092056135872020481
pub const MODULUS: U256 =
    U256::from_be_hex("0800000000000011000000000000000000000000000000000000000000000001");
/// p-1
pub const MODULUS_1: U256 =
    U256::from_be_hex("0800000000000011000000000000000000000000000000000000000000000000");
//2 adicity
pub const TWO_ADICITY: u32 = 192;
//TWO ADIC ROOT
// k = 005282DB87529CFA3F0464519C8B0FA5AD187148E11A61616070024F42F8EF94
pub const TWO_ADIC_ROOT: U256 =
    U256::from_be_hex("005282DB87529CFA3F0464519C8B0FA5AD187148E11A61616070024F42F8EF94");
// modulus/2
const FRAC_MODULUS_2: Fp = Fp(MODULUS.shr_vartime(1));
// 2^{512}/p=370534685559411814428248451066667753690889841777638999842330124194418215878553
//mu = 001f_0xffff_ffff_ffff_bc00_0x0000_0000_0090_7fff_0xffff_fffe_ccf0_0000_0x0000_028c_81ff_fbff
pub const MU: [u64; 5] = [
    0x0000_028c_81ff_fbff,
    0xffff_fffe_ccf0_0000,
    0x0000_0000_0090_7fff,
    0xffff_ffff_ffff_bc00,
    0x0000_0000_0000_001f,
];

//==== struct def =====
// struct defination
#[derive(Debug, Clone, Default, Copy)]
pub struct Fp(pub U256); //structure of the struct

//===== implement struct =======
// struct implementation
impl Fp {
    pub const ZERO: Fp = Self(U256::ZERO);
    pub const ONE: Fp = Self(U256::ONE);

    //return the new instance of the struct it takes any element as input
    //convert on U256 bit and check if the value is greater or less than modulus
    //if more than modulus, it uses mul_wide and barrett reduce to bring the
    //value in the field range. It return self % p
    #[allow(unused)]
    pub fn new(other: U256) -> Self {
        let prod = other.mul_wide(&U256::ONE);
        let res = barrett_reduce(prod.0, prod.1);
        Fp(CtOption::new(res, res.ct_lt(&MODULUS)).unwrap())
    }

    //from_repr takes a field element and return the field element.
    // by performing a reduction of the field element.
    pub fn from_repr(bytes: &FieldBytes) -> Fp {
        let inner = U256::from_be_byte_array(*bytes);
        <Fp as Field>::from_uint_reduced(Fp(inner))
    }
    // //to_bytes returns a field element from self.
    // //doing serializable of the input in a big arra
    // #[allow(unused)]
    // fn to_bytes(&self) -> FieldBytes {
    //     self.0.to_be_byte_array()
    // }
    //
    pub fn to_repr(&self) -> FieldBytes {
        self.0.to_be_bytes().into()
    }
    //zeroed vector
    // this uses a specialized vector initialization code which requests zero-filled memory
    // from the OS; unfortunately, this works only for built-in types and we can't use
    // Self::ZERO here as much less efficient initialization procedure will be invoked.
    // We also use u128 to make sure the memory is aligned correctly for our element size.
    pub fn zeroed_vector(n: usize) -> Vec<Self> {
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

//========common Field impl======
impl Field for Fp {
    type BaseField = Fp; // need to change this to Fp

    const ZERO: Self = Self::ZERO; // cont ZERO
    const ONE: Self = Self::ONE; // const ONE
                                 // element bytes
    const ELEMENT_BYTES: usize = ELEMENT_BYTES;
    // checks if the given value if zero or not
    // returns true or one when value is zero
    fn is_zero(self) -> bool {
        self == Self::ZERO
    }
    // checks if the given value if ONE or not
    // returns true or one when value is zero
    fn is_one(self) -> bool {
        self == Self::ONE
    }
    // double function
    // it takes a field element and returns the double of the element
    // uses the add function of the field
    fn double(self) -> Self {
        add(self, self)
    }
    // triple functions
    // it take a field element and return 3 times the element in the field.
    //depends on double function
    fn triple(self) -> Self {
        add(self, self.double())
    }
    // cube functions
    // it take a field element and return cube of the element.
    //depends on double function
    fn cube(self) -> Self {
        self * self.square()
    }
    //square of element
    //it takes a self (field element) and returns the square of the field element.
    //it uses square_wide of the crypto_bigint to give a tuple output (lo, hi)
    //which is reduced using the barrett_reduce function
    fn square(self) -> Self {
        let sq = self.0.square_wide(); // add barret
        let prod = barrett_reduce(sq.0, sq.1);
        Fp(prod)
    }
    //inverting or inverse of an element functionz
    // takes a self and returns the inverse element of the self in the field
    // uses is_odd, is_even, add_mod, bit shift (shr1), sub_mod functions
    fn invert(self) -> CtOption<Self> {
        let a = invert(self);
        a
    }
    // Exponentiates self by exp
    // Need to exponentiate the self (U256 bit) value with another 256 bit
    // value as power. The power is given in an array of u64 with len 4.
    // we start with ONE of field. we loop in the size of the array. We
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
        let w = self.power_by(&[
            0x0400_0000_0000_0008,
            0x0000_0000_0000_0000,
            0x0000_0000_0000_0000,
            0x0000_0000_0000_0000,
        ]);
        // v is the number of leading zeros bit in the bit representation of q-1
        let mut v = 192 as u32; //v
        let mut x = self * w; //x
        let mut b = x * w; //b
                           //800_0000_0000_0011
        let multiplicative_generator = Fp(U256::from_u32(3));
        let mut z = multiplicative_generator.power_by(&[
            0x0800_0000_0000_0011,
            0x0000_0000_0000_0000,
            0x0000_0000_0000_0000,
            0x0000_0000_0000_0000,
        ]); // z
        for max_v in (1..=192).rev() {
            let mut k = 1;
            let mut tmp = square(b); //square
            let mut j_less_than_v = Choice::from(1);
            for j in 2..max_v {
                let tmp_is_one = tmp.ct_eq(&Self::ONE);
                let squared = square(Self::conditional_select(&tmp, &z, tmp_is_one));
                tmp = Self::conditional_select(&squared, &tmp, tmp_is_one);
                let new_z = Self::conditional_select(&z, &squared, tmp_is_one);
                j_less_than_v &= !j.ct_eq(&v);
                k = u32::conditional_select(&j, &k, tmp_is_one);
                z = Self::conditional_select(&z, &new_z, j_less_than_v);
            }
            let result = x * z;
            x = Self::conditional_select(&result, &x, b.ct_eq(&Self::ONE));
            z = square(z);
            b = b * z;
            v = k;
        }
        CtOption::new(x, square(x).ct_eq(&self))
    }
    //rand function
    //it generates the random values of U256 bits using the random OsRng function of the crypto
    //bigint. The r value stores the random value. It can be greater then the modulus. We
    // convert it into field element using the mul_wide function from crypto_bigint. Then
    // do barret reduce to reduce the value from modulus and bring it in the field as field element.
    fn random() -> Self {
        let mut r = Fp(U256::random(&mut OsRng));
        let m = Fp(MODULUS);
        while r >= m {
            r -= m;
        }
        r
    }
    fn to_curve_bytes(&self) -> &[u8] {
        &self.as_bytes()
    }

    fn to_words(&self) -> Vec<u64> {
        self.0.to_words().into()
    }

    fn from_words(a: &Vec<u64>) -> Self {
        let k = [a[0], a[1], a[1], a[2]];
        let value = U256::from_words(k);
        Fp(value)
    }

    fn from_uint_reduced(w: Fp) -> Self {
        let (r, underflow) = w.0.sbb(&MODULUS, Limb::ZERO);
        let underflow = Choice::from((underflow.0 >> (Limb::BITS - 1)) as u8);
        Fp(U256::conditional_select(&w.0, &r, !underflow))
    }

    fn get_windows(&self, _window_bits: usize) -> Vec<usize> {
     unimplemented!()
    }

    const IS_CANONICAL: bool = true;
}

//======= implement Prime Field ======
impl PrimeField for Fp {
    type Repr = FieldBytes;
    const GENERATOR: Self = Fp(U256::from_u32(3));
    const NUM_BITS: u32 = (ELEMENT_BYTES * 8) as u32;
    const MODULUS: &'static str =
        "0800000000000011000000000000000000000000000000000000000000000001";
    const TWO_ADDICITY: u32 = 192;
    const TWO_ADIC_ROOT: &'static str =
        "005282DB87529CFA3F0464519C8B0FA5AD187148E11A61616070024F42F8EF94";

    // get_root_of_unity function
    fn get_root_of_unity(k: u32) -> Self {
        //if n == 0 ; 2^0 root does not exist
        // if n is more than 192 no root exist
        assert!(k == 0 || k <= 192, "2^{:?} th root does not exist", k);
        if k == TWO_ADICITY {
            println!("{:?}", Fp(TWO_ADIC_ROOT))
        };
        //TWO_ADIC_ROOT: & 'static str = "005282DB87529CFA3F0464519C8B0FA5AD187148E11A61616070024F42F8EF94";
        Fp(TWO_ADIC_ROOT).power_by((U256::ONE << ((TWO_ADICITY - k) as usize)).to_words())
    }
    // the function takes a self argument and check if the input is odd or not
    // it uses the is_odd of the crypto bigint to analyse the value.
    //return a bool value - true or false
    fn is_odd(self) -> Choice {
        let a = self.0.is_odd();
        a
    }
}
//=====Extensible for size 1 ===========
impl Extensible<1> for Fp {
    fn mul(_a: [Self; 1], _b: [Self; 1]) -> [Self; 1] {
        unimplemented!()
    }

    fn mul_base(_a: [Self; 1], _b: Self) -> [Self; 1] {
        unimplemented!()
    }

    fn is_supported() -> bool {
        false
    }

    fn square(_a: [Self; 1]) -> [Self; 1] {
        unimplemented!()
    }

    fn sqrt(_a: [Self; 1]) -> CtOption<[Self; 1]> {
        unimplemented!()
    }

    fn invert(_a: [Self; 1]) -> CtOption<[Self; 1]> {
        unimplemented!()
    }
}
//====impl From Traits =========
// impl u8
// it creates a unit from the field element
impl From<u8> for Fp {
    fn from(v: u8) -> Self {
        Self(U256::from_u8(v))
    }
}
//impl u16
// it creates a unit from the field element
impl From<u16> for Fp {
    fn from(value: u16) -> Self {
        Self(U256::from_u16(value))
    }
}
//impl u32
// it creates a unit from the field element
impl From<u32> for Fp {
    fn from(value: u32) -> Self {
        Self(U256::from_u32(value))
    }
}
//impl u64
// it creates a unit from the field element
impl From<u64> for Fp {
    fn from(value: u64) -> Self {
        Self(U256::from_u64(value))
    }
}
//impl u128
// it creates a unit from the field element
impl From<u128> for Fp {
    fn from(value: u128) -> Self {
        Self(U256::from_u128(value))
    }
}
//Implement from trait
impl From<[u8; 32]> for Fp {
    /// Converts the value encoded in an array of 16 bytes into a field element. The bytes
    /// are assumed to be in little-endian byte order. If the value is greater than or equal
    /// to the field modulus, modular reduction is silently performed.
    fn from(bytes: [u8; 32]) -> Self {
        let value = Self::from_repr(&bytes.try_into().unwrap());
        value
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
//======= Implement trait bounds ======
impl AsRef<[u8]> for Fp {
    #[inline]
    fn as_ref(&self) -> &[u8] {
        self.as_bytes()
    }
}
impl AsMut<u8> for Fp {
    fn as_mut(&mut self) -> &mut u8 {
        self.as_mut()
    }
}
//Randomizable
//it converts u8 input to
impl Randomizable for Fp {
    const VALUE_SIZE: usize = ELEMENT_BYTES;
    fn from_random_bytes(bytes: &[u8]) -> Option<Self> {
        Self::try_from(bytes).ok()
    }
}
// implement Display for Field256
// the function writes the output in the format
impl Display for Fp {
    fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

//Implementing Add, Sub, AddAssign, SubAssign, Mul, MulAssign, Div, DivAssign
impl Add<Fp> for Fp {
    type Output = Fp;
    // uses the field add function to add two (a,b) elements
    fn add(self, rhs: Fp) -> Self::Output {
        add(self, rhs)
    }
}
impl Add<&Fp> for Fp {
    type Output = Fp;
    // uses the field add function to add two (a, borrowed b) elements
    fn add(self, rhs: &Fp) -> Self::Output {
        add(self, *rhs)
    }
}
//implement AddAssign
impl AddAssign<Fp> for Fp {
    // uses the field add function to add two (a, b) elements
    // assigns a with the sum value a = a + b
    fn add_assign(&mut self, rhs: Self) {
        *self = add(*self, rhs)
    }
}
impl AddAssign<&Fp> for Fp {
    // uses the field add function to add two (a, &b) elements
    // assigns a with the sum value a = a + b
    fn add_assign(&mut self, rhs: &Self) {
        *self = add(*self, *rhs)
    }
}
//implement Sub
impl Sub<Fp> for Fp {
    type Output = Fp;
    // uses the field subtraction function to sub two (a, b) elements
    fn sub(self, rhs: Fp) -> Self::Output {
        sub(self, rhs)
    }
}
impl Sub<&Fp> for Fp {
    type Output = Fp;
    // uses the field sub function to sub two (a, &b) elements
    fn sub(self, rhs: &Fp) -> Self::Output {
        sub(self, *rhs)
    }
}
//implement SubAssign
impl SubAssign<Fp> for Fp {
    // uses the field sub function to sub two (a, b) elements
    // assigns a with the sum value a = a - b
    fn sub_assign(&mut self, rhs: Fp) {
        *self = sub(*self, rhs)
    }
}
//implement Neg for field256
impl Neg for Fp {
    type Output = Fp;
    // uses the field neg function to negate the field element
    fn neg(self) -> Self::Output {
        neg(self)
    }
}
//implement Mul for F256
impl Mul<Fp> for Fp {
    type Output = Fp;
    // uses the field mul function to mul two (a, b) elements
    fn mul(self, rhs: Fp) -> Self::Output {
        mul(self, rhs)
    }
}
impl Mul<&Fp> for Fp {
    type Output = Fp;
    // uses the field mul function to mul two (a, &b) elements
    fn mul(self, rhs: &Fp) -> Self::Output {
        mul(self, *rhs)
    }
}
//implement MulAssign for F256
impl MulAssign<Fp> for Fp {
    // uses the field mul function to mul two (a, b) elements
    // assigns a with the multiplication value a = a * b
    fn mul_assign(&mut self, rhs: Fp) {
        *self = mul(*self, rhs)
    }
}
impl MulAssign<&Fp> for Fp {
    // uses the field mul function to multiplication two (a, &b) elements
    // assigns a with the mul value a = a * b
    fn mul_assign(&mut self, rhs: &Fp) {
        *self = mul(*self, *rhs)
    }
}

//implement Div for F256
impl Div for Fp {
    type Output = Self;
    // uses the field div function to div two (a, b) elements
    fn div(self, rhs: Self) -> Self::Output {
        div(self, rhs)
    }
}
//implement DivAssign for F256
impl DivAssign for Fp {
    // uses the field div function to div two (a, b) elements
    // assigns a with the division value a = a / b
    fn div_assign(&mut self, rhs: Self) {
        *self = div(*self, rhs)
    }
}

impl Eq for Fp {}
impl ConstantTimeEq for Fp {
    fn ct_eq(&self, other: &Self) -> Choice {
        self.0.ct_eq(&other.0)
    }
}
impl PartialEq for Fp {
    fn eq(&self, other: &Self) -> bool {
        self.ct_eq(other).into()
    }
}
impl PartialOrd for Fp {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}
impl Ord for Fp {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.0.cmp(&other.0)
    }
}
// SERIALIZATION / DESERIALIZATION
// ------------------------------------------------------------------------------------------------
// implementing the serializable for the base field
impl Serializable for Fp {
    fn write_into<W: ByteWriter>(&self, target: &mut W) {
        target.write_u8_slice(&self.0.to_le_bytes());
    }
}
// implementing the deserizable for the base field
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

// TYPE CONVERSIONS
// ================================================================================================
impl AsBytes for Fp {
    fn as_bytes(&self) -> &[u8] {
        // TODO: take endianness into account
        let self_ptr: *const Fp = self;
        unsafe { slice::from_raw_parts(self_ptr as *const u8, ELEMENT_BYTES) }
    }
}
// ====== global functions =====

//Addition
// it takes two field elements as inputs and
// returns a + b %p as output. It uses the add_mod
// function of the crypto_bigint function to perform calculations
pub fn add(a: Fp, b: Fp) -> Fp {
    Fp(a.0.add_mod(&b.0, &MODULUS))
}
// Subtraction
// it takes two field elements as inputs and
// returns a - b %p as output. It uses the sub_mod
// function of the crypto_bigint function to perform calculations
pub fn sub(a: Fp, b: Fp) -> Fp {
    Fp(a.0.sub_mod(&b.0, &MODULUS))
}
// Negation
// it takes one field element as input and
// return the negation of the element as -a mod p. It uses the neg_mod
// function of the crypto_bigint function to perform calculations
pub fn neg(a: Fp) -> Fp {
    Fp(a.0.neg_mod(&MODULUS))
}
// Multiplication
// it takes two field elements as inputs and
// returns a * b %p as output. It first performs the mul wide
// operation using mul_wide function --> (lo, hi) output. Then do barrett_reduction
// to bring the value in the field if greater than the modulus.
pub fn mul(a: Fp, b: Fp) -> Fp {
    let prod = a.0.mul_wide(&b.0);
    let bprod = barrett_reduce(prod.0, prod.1);
    Fp(bprod)
}
// Division
// it takes two field elements as inputs and
// returns a / b %p as output. It uses the mul and
// invert functions. It inverts second element,
//b --> invert(b) then multiply with the first element,
// a to give the output of division.
pub fn div(a: Fp, b: Fp) -> Fp {
    // a /b  is assumed while division
    let res = mul(a, invert(b).unwrap());
    res
}
// Square function
// it take a field element. Uses the square method implemented
// to return the square of the element.
pub fn square(a: Fp) -> Fp {
    a.square()
}
// double function
// it take a field element. Uses the square method implemented
// to return the square of the element.
pub fn double(a: Fp) -> Fp {
    a.double()
}

pub fn invert(input: Fp) -> CtOption<Fp> {
    #[allow(non_snake_case)]
    let element = input;
    let mut u = element.0; //removed the dereference
    let mut v = MODULUS;
    let mut a = U256::from_u8(1);
    let mut c = U256::ZERO;
    while !bool::from(u.is_zero()) {
        while bool::from(u.is_even()) {
            shr1(&mut u);
            let was_odd: bool = a.is_odd().into();
            shr1(&mut a);
            if was_odd {
                a = a.add_mod(&FRAC_MODULUS_2.0, &MODULUS);
                a = a.add_mod(&U256::ONE, &MODULUS);
            }
        }
        while bool::from(v.is_even()) {
            shr1(&mut v);
            let was_odd: bool = c.is_odd().into();
            shr1(&mut c);
            if was_odd {
                c = c.add_mod(&FRAC_MODULUS_2.0, &MODULUS);
                c = c.add_mod(&U256::ONE, &MODULUS);
            }
        }
        if u >= v {
            u = u.sub_mod(&v, &MODULUS);
            a = a.sub_mod(&c, &MODULUS);
        } else {
            v = v.sub_mod(&u, &MODULUS);
            c = c.sub_mod(&a, &MODULUS);
        }
    }
    CtOption::new(Fp(c), !element.0.is_zero())
}

// Shift right by one bit
// it takes an element and shift the bits
// to right by one -- meaning -- diving by 2
fn shr1(value: &mut U256) {
    *value >>= 1;
}

//shift left by one bit
// it takes an element and shift the bits
// to left by one -- meaning -- multiplying by 2
pub fn shl1(value: &mut U256) {
    *value <<= 1;
}

impl Reduce<U256> for Fp {
    fn from_uint_reduced(w: U256) -> Fp {
        let (r, underflow) = w.sbb(&MODULUS, Limb::ZERO);
        let underflow = Choice::from((underflow.0 >> (Limb::BITS - 1)) as u8);
        Fp(U256::conditional_select(&w, &r, !underflow))
    }
}
pub trait Reduce<U256: Integer>: Sized {
    /// Perform a modular reduction, returning a field element.
    fn from_uint_reduced(n: U256) -> Self;
}
impl ConditionallySelectable for Fp {
    fn conditional_select(a: &Self, b: &Self, choice: Choice) -> Self {
        Self(U256::conditional_select(&a.0, &b.0, choice))
    }
}

/// Barrett Reduction
///

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
#[inline]
#[allow(unused)]
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
    //r.cmp(&MODULUS.to_words());
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
    let modulus = MODULUS.as_words();
    let (w0, carry) = mac(0, q3[0], modulus[0], 0);
    let (w1, carry) = mac(0, q3[0], modulus[1], carry);
    let (w2, carry) = mac(0, q3[0], modulus[2], carry);
    let (w3, carry) = mac(0, q3[0], modulus[3], carry);
    let (w4, _) = mac(0, q3[0], 0, carry);
    let (w1, carry) = mac(w1, q3[1], modulus[0], 0);
    let (w2, carry) = mac(w2, q3[1], modulus[1], carry);
    let (w3, carry) = mac(w3, q3[1], modulus[2], carry);
    let (w4, _) = mac(w4, q3[1], modulus[3], carry);
    let (w2, carry) = mac(w2, q3[2], modulus[0], 0);
    let (w3, carry) = mac(w3, q3[2], modulus[1], carry);
    let (w4, _) = mac(w4, q3[2], modulus[2], carry);
    let (w3, carry) = mac(w3, q3[3], modulus[0], 0);
    let (w4, _) = mac(w4, q3[3], modulus[1], carry);
    let (w4, _) = mac(w4, q3[4], modulus[0], 0);
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
    [w0, w1, w2, w3, w4]
}
#[inline]
#[allow(unused)]
#[allow(clippy::too_many_arguments)]
const fn subtract_n_if_necessary(r0: u64, r1: u64, r2: u64, r3: u64, r4: u64) -> [u64; 5] {
    let modulus = MODULUS.as_words();
    let (w0, borrow) = sbb(r0, modulus[0], 0);
    let (w1, borrow) = sbb(r1, modulus[1], borrow);
    let (w2, borrow) = sbb(r2, modulus[2], borrow);
    let (w3, borrow) = sbb(r3, modulus[3], borrow);
    let (w4, borrow) = sbb(r4, 0, borrow);
    // If underflow occurred on the final limb, borrow = 0xfff...fff, otherwise
    // borrow = 0x000...000. Thus, we use it as a mask to conditionally add the
    // modulus.
    let (w0, carry) = adc(w0, modulus[0] & borrow, 0);
    let (w1, carry) = adc(w1, modulus[1] & borrow, carry);
    let (w2, carry) = adc(w2, modulus[2] & borrow, carry);
    let (w3, carry) = adc(w3, modulus[3] & borrow, carry);
    let (w4, _carry) = adc(w4, 0, carry);
    [w0, w1, w2, w3, w4]
}
