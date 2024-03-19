use core::{AsBytes, Deserializable, Randomizable, Serializable};
use crypto_bigint::{
    subtle::{Choice, CtOption},
    U256,
};
use std::{
    fmt::{Debug, Display},
    ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Neg, Sub, SubAssign},
    u128, usize,
};

pub trait Field:
    Copy
    + Clone
    + Debug
    + Display
    + Default
    + Send
    + Sync
    + Eq
    + PartialEq
    + Sized
    + Add<Self, Output = Self>
    + Sub<Self, Output = Self>
    + Mul<Self, Output = Self>
    + Div<Self, Output = Self>
    + AddAssign<Self>
    + SubAssign<Self>
    + MulAssign<Self>
    + DivAssign<Self>
    + Neg<Output = Self>
    + From<u128>
    + From<u64>
    + From<u32>
    + From<u16>
    + From<u8>
    + From<[u64; 6]>
    + From<U256>
    + for<'a> TryFrom<&'a [u8]>
    + AsBytes
    + Randomizable
    + Serializable
    + Deserializable
    + Ord
    + PartialOrd
{
    /// Returns random field element
    fn random() -> Self;

    ///Returns the square of field element
    fn square(self) -> Self {
        self * self
    }

    /// Returns the cube of field element
    fn cube(self) -> Self {
        self * (self.square())
    }

    ///Returns multiplicative inverse of field element
    fn invert(self) -> CtOption<Self>;

    ///Returns the square root of self
    fn sqrt(self) -> CtOption<Self>;

    //Exponentiates the self by pow
    fn power_by<S: AsRef<[u64]>>(self, exp: S) -> Self;

    //Doubles field element
    fn double(self) -> Self {
        self + self
    }

    //Triples field element
    fn triple(self) -> Self {
        self + self + self
    }

    //Indicates whether field element is 0
    fn is_zero(self) -> bool;

    //Indicates whether field element is 1
    fn is_one(self) -> bool;

    //curve bytes conversion
    fn to_curve_bytes(&self) -> &[u8];

    fn to_words(&self) -> Vec<u64>;

    fn from_words(a: &Vec<u64>) -> Self;

    fn get_windows(&self, exp: usize) -> Vec<usize>;
    // Returns the value after modulus reduction
    fn from_uint_reduced(w: Self) -> Self;

    //Defines the additive identity element of the field
    const ZERO: Self;

    //Defines the multiplicative identity element of the field
    const ONE: Self;

    const ELEMENT_BYTES: usize;
    /// True if internal representation of the element is the same as its canonical representation.
    /// Elements in all fields are taken from new function and is in canonical representation
    const IS_CANONICAL: bool = true;

    type BaseField: PrimeField;
}

pub trait PrimeField: Field<BaseField = Self> {
    type Repr: Copy + Default + Send + Sync + AsRef<[u8]> + AsMut<[u8]>;

    // Checks whether the self is odd or not
    fn is_odd(self) -> Choice;

    // Checks whether the self is even or not
    fn is_even(self) -> Choice {
        !self.is_odd()
    }

    fn get_root_of_unity(n: u32) -> Self;

    // fn to_repr(&self)->Self::Repr;

    //Modulus of the PrimeField
    const MODULUS: &'static str;

    // No. of bits used to represent the field element
    const NUM_BITS: u32;

    //Generator of multiplicative group of the field
    const GENERATOR: Self;

    // Two addicity of the prime field
    const TWO_ADDICITY: u32;

    const TWO_ADIC_ROOT: &'static str;
}

pub trait Extensible<const N: usize>: PrimeField + Copy {
    /// Returns the product of `a` and `b` in the field defined by this extension.
    fn mul(a: [Self; N], b: [Self; N]) -> [Self; N];

    /// Returns a product of `a` and `b` in the field defined by this extension. `b` represents
    /// an element in the base field.
    fn mul_base(a: [Self; N], b: Self) -> [Self; N];

    /// Returns true if this extension is supported for the underlying base field.
    fn is_supported() -> bool {
        true
    }
    ///Returns the square of field element.
    fn square(a: [Self; N]) -> [Self; N];

    //Returns the square root of the field element.
    fn sqrt(a: [Self; N]) -> CtOption<[Self; N]>;

    //Returns the inverse of field element.
    fn invert(a: [Self; N]) -> CtOption<[Self; N]>;
}

// A field is always an extension of itself.
pub trait ExtensionOf<E: Field>: From<E> {
    fn mul_base(self, other: E) -> Self;
}
impl<E: Field> ExtensionOf<E> for E {
    #[inline(always)]
    fn mul_base(self, other: E) -> Self {
        self * other
    }
}

// HASHER TRAITS
// ================================================================================================
// This trait define hash for the vector of field elements
pub trait Hasher: Clone + Eq + PartialEq + Debug {
    // Specifies a FieldElement type
    type FieldElement: Field + PrimeField;
    // Returns a hash of the provided field elements.
    fn hash(value: &mut Vec<Self::FieldElement>) -> Self::FieldElement;

    // Compute the rescue hash and also store intermediate states
    fn hash_and_store_states(
        value: &mut [Self::FieldElement],
        result: &mut [Self::FieldElement],
        hash_data: &mut Vec<Vec<Self::FieldElement>>,
    );
}

// trait defined for the parameters used in poseidon hash
pub trait PoseidonParameter<F: Field + PrimeField> {
    const STATE_WIDTH_4_2: usize;
    const STATE_WIDTH_6_5: usize;
    const ALPHA: u64;
    const RATE_4_2: usize;
    const RATE_6_5: usize;
    const TOTAL_NUM_ROUNDS_4_2: usize;
    const TOTAL_NUM_ROUNDS_6_5: usize;
    const ROUNDS_F_BEGINNING: usize;
    const PARTIAL_ROUNDS_4_2: usize;
    const PARTIAL_ROUNDS_6_5: usize;
    const TOTAL_FULL_ROUNDS: usize;
    fn mds_4_2() -> Vec<F>;
    fn mds_6_5() -> Vec<F>;
    fn round_constants_4_2() -> Vec<F>;
    fn round_constants_6_5() -> Vec<F>;
    fn internal_mds_4_2() -> Vec<F>;
    fn internal_mds_6_5() -> Vec<F>;
}

// trait defined for the parameters used in rescue hash
pub trait RescueParameter<F: Field + PrimeField> {
    const RESCUE_STATE_WIDTH: usize;
    const RESCUE_ALPHA: u64;
    const RESCUE_INV_ALPHA: U256;
    const RESCUE_RATE: usize;
    const RESCUE_NO_OF_ROUNDS: usize;
    fn rescue_mds() -> Vec<F>;
    fn rescue_inv_mds() -> Vec<F>;
    fn rescue_ark() -> Vec<Vec<F>>;
}

// trait defined for the parameters used in gmimc hash
pub trait GMIMCParameter<F: Field + PrimeField> {
    const GMIMC_RATE: usize;
    const GMIMC_NO_OF_ROUNDS: usize;
    const GMIMC_STATE_WIDTH: usize;
    const NO_OF_COLUMNS: usize;
    fn gmimc_constants() -> Vec<F>;
}
