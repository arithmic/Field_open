use core::{AsBytes, Randomizable, Serializable, ByteWriter, Deserializable, ByteReader, DeserializationError};
use std::fmt::{Display, Formatter};
use std::ops::{Add, Sub, Mul, Div, AddAssign, SubAssign, MulAssign, DivAssign, Neg};
use std::slice;

use crypto_bigint::subtle::{Choice, CtOption, ConstantTimeEq, ConditionallySelectable};
use crypto_bigint::{ U384, U256};
use traits::traits::{Field, Extensible, PrimeField};

use crate::fp::{ELEMENT_BYTES, self, Fp, MODULUS, MODULUS_MINUS_ONE, FieldBytes};
use crate::scalar::Scalar;
use crate::{fp6::Fp6, fp2::Fp2};
// Fp12 is a quadratic extension over Fp6.
//An element of Fp12, represented by c0 + c1 * w
#[derive(Clone,Copy, Debug,Default, PartialEq, Eq, PartialOrd, Ord)]
pub struct Fp12<B:Extensible<2>+Extensible<3>>{
    pub c0:Fp6<B>,
    pub c1:Fp6<B>,
}
impl <B:Extensible<2>+Extensible<3>>Fp12<B>{
    
    pub const NONRESIDUE: Fp6<Fp2<Fp>> = Fp6::new(Fp2::zero(), Fp2::one(), Fp2::zero());
    const ZERO:Fp12<B>=Fp12{c0:<Fp6<B>>::ZERO,c1:<Fp6<B>>::ZERO};
    const ONE:Fp12<B>=Fp12{c0:<Fp6<B>>::ONE,c1:<Fp6<B>>::ZERO};

    pub fn frobenius_coeff_fp12_c1() ->[Fp2<B>;12] {[
        // Fp2::NONRESIDUE^(((q^0) - 1) / 6)
        Fp2::<B>::new(B::ONE, B::ZERO),
        // Fp2::NONRESIDUE^(((q^1) - 1) / 6)
        Fp2::<B>::new(
            B::from(U256::from_be_hex("1284B71C2865A7DFE8B99FDD76E68B605C521E08292F2176D60B35DADCC9E470")),
            B::from(U256::from_be_hex("246996F3B4FAE7E6A6327CFE12150B8E747992778EEEC7E5CA5CF05F80F362AC")),
        ),
        // Fp2::NONRESIDUE^(((q^2) - 1) / 6)
        Fp2::<B>::new(
            B::from(U256::from_be_hex("30644E72E131A0295E6DD9E7E0ACCCB0C28F069FBB966E3DE4BD44E5607CFD49")),
            B::ZERO,
        ),
        // Fp2::NONRESIDUE^(((q^3) - 1) / 6)
        Fp2::<B>::new(
            B::from(U256::from_be_hex("19DC81CFCC82E4BBEFE9608CD0ACAA90894CB38DBE55D24AE86F7D391ED4A67F")),
            B::from(U256::from_be_hex("00ABF8B60BE77D7306CBEEE33576139D7F03A5E397D439EC7694AA2BF4C0C101")),
        ),
        // Fp2::NONRESIDUE^(((q^4) - 1) / 6)
        Fp2::<B>::new(
            B::from(U256::from_be_hex("30644E72E131A0295E6DD9E7E0ACCCB0C28F069FBB966E3DE4BD44E5607CFD48")),
            B::ZERO,
        ),
        // Fp2::NONRESIDUE^(((q^5) - 1) / 6)
        Fp2::<B>::new(
            B::from(U256::from_be_hex("0757CAB3A41D3CDC072FC0AF59C61F302CFA95859526B0D41264475E420AC20F")),
            B::from(U256::from_be_hex("0CA6B035381E35B618E9B79BA4E2606CA20B7DFD71573C93E85845E34C4A5B9C")),
        ),
        // Fp2::NONRESIDUE^(((q^6) - 1) / 6)
        Fp2::<B>::new(
            B::from(MODULUS_MINUS_ONE),
            B::ZERO,
        ),
        // Fp2::NONRESIDUE^(((q^7) - 1) / 6)
         Fp2::<B>::new(
            B::from(U256::from_be_hex("1DDF9756B8CBF849CF96A5D90A9ACCFD3B2F4C893F42A9166615563BFBB318D7")),
            B::from(U256::from_be_hex("0BFAB77F2C36B843121DC8B86F6C4CCF2307D819D98302A771C39BB757899A9B")),
        ),
        // Fp2::NONRESIDUE^(((q^8) - 1) / 6)
        Fp2::<B>::new(
            B::from(U256::from_be_hex("000000000000000059E26BCEA0D48BACD4F263F1ACDB5C4F5763473177FFFFFE")),
            B::ZERO,
        ),
        // Fp2::NONRESIDUE^(((q^9) - 1) / 6)
        Fp2::<B>::new(
            B::from(U256::from_be_hex("1687CCA314AEBB6DC866E529B0D4ADCD0E34B703AA1BF84253B10EDDB9A856C8")),
            B::from(U256::from_be_hex("2FB855BCD54A22B6B18456D34C0B44C0187DC4ADD09D90A0C58BE1EAE3BC3C46")),
        ),
        // Fp2::NONRESIDUE^(((q^10) - 1) / 6)
        Fp2::<B>::new(
            B::from(U256::from_be_hex("000000000000000059E26BCEA0D48BACD4F263F1ACDB5C4F5763473177FFFFFF")),
            B::ZERO,
        ),
        // Fp2::NONRESIDUE^(((q^11) - 1) / 6)
        Fp2::<B>::new(
            B::from(U256::from_be_hex("290C83BF3D14634DB120850727BB392D6A86D50BD34B19B929BC44B896723B38")),
            B::from(U256::from_be_hex("23BD9E3DA9136A739F668E1ADC9EF7F0F575EC93F71A8DF953C846338C32A1AB")),
        ),
    ]}
    // returns the Fp12 element : c0+c1*w when two Fp6 elements c0 and c1 are passed
    pub fn new(c0:Fp6<B>,c1:Fp6<B>)->Self{
            Self { c0, c1 }
    }
    // returns the zero / additive identity of fp12
    pub fn zero()->Self{
        Self::ZERO
    }
    // returns the one / multiplicative identity of Fp12
    pub fn one()->Self{
        Self::ONE
    }
    // returns the conjugate of self i.e if (a+b*w) is passed in it , then it will return a-b*w
    pub fn conjugate(self) -> Self {
        Self { c0: self.c0, c1:-(self.c1)  }
    }
    // Exponentiates self by exp ( exp is belonging to Scalar field)
    pub fn pow(&self, exp: &Scalar) -> Self {
        self.power_by(exp.0.as_words())
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

    pub fn mul_fp6_by_nonresidue_in_place(fe: &mut Fp6<Fp2<B>>) -> &mut Fp6<Fp2<B>> {
        // see [[DESD06, Section 6.1]](https://eprint.iacr.org/2006/471.pdf).
        let old_c1 = fe.c1;
        fe.c1 = fe.c0;
        fe.c0 = fe.c2;
        fe.c0 = *<Fp6<Fp2<B>>>::mul_fp2_by_nonresidue_in_place(&mut fe.c0);
        fe.c2 = old_c1;
        fe
    }

   //this function multiply a Fp12 element with a scalar
   pub fn mul_scalar(&self, s: B) -> Self {
    let d0=self.c0.mul_scalar(s);
    let d1=self.c1.mul_scalar(s);
    Self { c0: d0, c1: d1 }
    }
    pub fn cyclotomic_inverse(&self) -> Self {
        self.conjugate()
    }
    pub fn cyclotomic_square(&self) -> Self {
        // Faster Squaring in the Cyclotomic Subgroup of Sixth Degree Extensions
        // - Robert Granger and Michael Scott
        //
        let fp2_nr = Fp6::mul_fp2_by_nonresidue_in_place;

        let r0 = self.c0.c0;
        let r4 = self.c0.c1;
        let mut r3 = self.c0.c2;
        let r2 = self.c1.c0;
        let mut r1 = self.c1.c1;
        let mut r5 = self.c1.c2;

        // t0 + t1*y = (z0 + z1*y)^2 = a^2
        let mut tmp = r0 * r1;
        let t0 = (r0 + r1) * (*fp2_nr(&mut r1) + r0) - tmp - *fp2_nr(&mut tmp);
        let t1 = tmp.double();

        // t2 + t3*y = (z2 + z3*y)^2 = b^2
        tmp = r2 * r3;
        let t2 = (r2 + r3) * (*fp2_nr(&mut r3) + r2) - tmp - *fp2_nr(&mut tmp);
        let t3 = tmp.double();

        // t4 + t5*y = (z4 + z5*y)^2 = c^2
        tmp = r4 * r5;
        let t4 = (r4 + r5) * (*fp2_nr(&mut r5) + r4) - tmp - *fp2_nr(&mut tmp);
        let mut t5 = tmp.double();

        let mut z0 = self.c0.c0;
        let mut z4 = self.c0.c1;
        let mut z3 = self.c0.c2;
        let mut z2 = self.c1.c0;
        let mut z1 = self.c1.c1;
        let mut z5 = self.c1.c2;

        // for A

        // z0 = 3 * t0 - 2 * z0
        z0 = t0 - z0;
        z0 = z0.double();
        z0 += t0;

        // z1 = 3 * t1 + 2 * z1
        z1 = t1 + z1;
        z1 = z1.double();
        z1 += t1;

        // for B

        // z2 = 3 * (xi * t5) + 2 * z2
        tmp = *fp2_nr(&mut t5);
        z2 += tmp;
        z2 = z2.double();
        z2 += tmp;

        // z3 = 3 * t4 - 2 * z3
        z3 = t4 - z3;
        z3 = z3.double();
        z3 += t4;

        // for C

        // z4 = 3 * t2 - 2 * z4
        z4 = t2 - z4;
        z4 = z4.double();
        z4 += t2;

        // z5 = 3 * t3 + 2 * z5
        z5 += t3;
        z5 = z5.double();
        z5 += t3;

        Self {
            c0: Fp6::new(z0, z1, z2),
            c1: Fp6::new(z3, z4, z5),
        }
    }
     

    // Shamir's trick
    pub fn multi_exponentiation(point: Vec<Fp12<B>>, exponent: Vec<Scalar>) -> Self {
        // Convert Vec<Scalar> into Vec<[u64; 4]
        let mut exp = Vec::new();
        for value in exponent {
            exp.push(value.0.to_words());
        }
        let mut result = Self::one();
        for outer_idx in (0..4).rev() {
            for inner_idx in (0..64).rev() {
                result = result.square();
                for (idx, value) in exp.iter().enumerate() {
                    if ((value[outer_idx] >> inner_idx) & 1) == 1 {
                        result = result.mul(point[idx])
                    }
                }
            }
        }
        result
    }

    
    pub fn to_bytes(&self) -> Vec<u8> {
        let mut vec_bytes = Vec::new();
        vec_bytes.push(self.c0.to_bytes());
        vec_bytes.push(self.c1.to_bytes());
        let mut final_bytes = Vec::new();
        for bytes in vec_bytes {
            for byte in bytes {
                final_bytes.push(byte);
            }
        }
        final_bytes
    }



    // Returns the mapping a -> a^{p^12}
   pub fn frobenius_map(&mut self, power: usize) {
        self.c0.frobenius_map(power);
        self.c1.frobenius_map(power);

        self.c1
            .c0
            .mul_assign(Self::frobenius_coeff_fp12_c1()[power % 12]);
        self.c1
            .c1
            .mul_assign(Self::frobenius_coeff_fp12_c1()[power % 12]);
        self.c1
            .c2
            .mul_assign(Self::frobenius_coeff_fp12_c1()[power % 12]);
    }






}
impl <B:Extensible<2>+Extensible<3>> Field for Fp12<B>{
    // checks whether the given Fp12 element is zero or  not
    fn is_zero(self) -> bool {
        self==Self::ZERO
    }
    // checks whether the given Fp12 element is one or not
    fn is_one(self) -> bool {
        self==Self::ONE
    }
    // returns the twice of given Fp12 element
    fn double(self) -> Self {
        self+self
    }
    // returns the thrice of given fp12 element
    fn triple(self) -> Self {
        self+self+self
    }
    // returns self*self in Fp12
    fn square(self) ->Self {
        let a=[self.c0,self.c1];
        let res=<Fp6<B> as Extensible<2>>::square(a);
        Self { c0:res[0], c1:res[1] }
    }
    // returns the multiplicative inverse of self
    fn invert(self) -> CtOption<Self> {
        let a=[self.c0,self.c1];
        let res=<Fp6<B> as Extensible<2>>::invert(a);
        if res.is_some().unwrap_u8()==1{
        let out =Fp12 { c0: res.unwrap()[0], c1: res.unwrap()[1]};
        CtOption::new(out, res.is_some())}
        else {
            CtOption::new(Self::ZERO,res.is_some())
        }
    }
    // additive identity of Fp12 i.e Zero element
    const ZERO:Self=Self::ZERO;
    // multiplicative identity of Fp12 element i.e One element
    const ONE:Self=Self::ONE;
    // element bytes
    const ELEMENT_BYTES: usize = ELEMENT_BYTES;
// returns the random Fp12 element
    fn random()->Self {
        Self { c0: <Fp6<B>>::random(), c1: <Fp6<B>>::random() }
    }

    fn sqrt(self)->CtOption<Self> {
     unimplemented!()
    }
   // returns the self**pow where pow is given as array of u64(word)
    fn power_by<S:AsRef<[u64]>>(self,pow:S)->Self {
        let mut res = Fp12{c0:<Fp6<B>>::from(U384::ONE.to_words()),c1:<Fp6<B>>::ZERO};
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
    
    fn to_curve_bytes(&self) -> &[u8] {
        self.as_bytes()
    }

    fn to_words(&self) -> Vec<u64> {
        unimplemented!()
    }
    type BaseField = Fp12<B>;

    fn cube(self) -> Self{
        self*(self.square())
    }

    const IS_CANONICAL: bool = true;

    fn  from_uint_reduced(_w: Self) -> Self {
     unimplemented!()
    }

    fn from_words( _a: &Vec<u64>) -> Self {
     unimplemented!()
    }

    fn get_windows(&self, _exp: usize)->Vec<usize> {
     unimplemented!()
    }

   
}
impl <B:Extensible<2>+Extensible<3>>PrimeField for Fp12<B>{
    // checks whether the self is odd or not
    fn is_odd(self)->Choice{
        self.c0.is_odd()
    }
    fn get_root_of_unity(_k: u32)->Self {
        unimplemented!()
    }

    const MODULUS:&'static str="30644E72E131A029B85045B68181585D97816A916871CA8D3C208C16D87CFD47";

    const NUM_BITS:u32=(ELEMENT_BYTES*8*12) as u32;

    const GENERATOR:Self=unimplemented!();

    const TWO_ADDICITY: u32=unimplemented!();

    fn is_even(self)->Choice{
        !self.is_odd()
    }

    type Repr=FieldBytes;

    const TWO_ADIC_ROOT: & 'static str = unimplemented!();

}
impl <B:Extensible<2>+Extensible<3>> Display for Fp12<B>{
    fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
        write!(f, "{}", self)
    }
}
// Basic arithmetic operators and assignment operators on Fp12 like + , - , * , /
impl<B:Extensible<2>+Extensible<3>> Add for Fp12<B>{
    type Output=Self;
    fn add(self, other: Self) -> Self{
        add(&self,&other)
    }
}
impl <B:Extensible<2>+Extensible<3>>Sub for Fp12<B>{
    type Output = Self;
    fn sub(self, other: Self) -> Self::Output{
        sub(&self,&other)
    }
}
impl <B:Extensible<2>+Extensible<3>> Mul for Fp12<B>{
    type Output = Self;
    // computes multiplication of a and b (of Fp12 )using irreducible polynomial w^2-v=0 , so (a+bw)(c+dw)=(ac-bd*v)+(ad+bc)*w

    fn mul(self, rhs: Self) -> Self{
        let aa=[self.c0,self.c1];
        let bb=[rhs.c0,rhs.c1];
        let res=<Fp6<B> as Extensible<2>>::mul(aa, bb);
        Fp12{c0:res[0],c1:res[1]}
    }
}
impl <B:Extensible<2>+Extensible<3>>Div for Fp12<B>{
    type Output = Self;
    fn div(self, rhs: Self) -> Self::Output{
        div(&self,&rhs)
    }
}
impl <B:Extensible<2>+Extensible<3>>AddAssign for Fp12<B>{
    fn add_assign(&mut self, other: Self) {
        *self=add(self,&other);
    }
}
impl <B:Extensible<2>+Extensible<3>>SubAssign for Fp12<B>{
    fn sub_assign(&mut self, other: Self) {
        *self=sub(self,&other);
    }
}
impl <B:Extensible<2>+Extensible<3>>MulAssign for Fp12<B> {
   fn mul_assign(&mut self, rhs: Self){
        *self=(*self)*rhs
   }
}
impl <B:Extensible<2>+Extensible<3>>DivAssign for Fp12<B>{
    fn div_assign(&mut self, rhs: Self){
        *self=div(self,&rhs)
    }
}
impl <B:Extensible<2>+Extensible<3>>Neg for Fp12<B>{
   type Output = Self;
   fn neg(self) -> Self::Output{
       neg(&self)
   }
}
// conversion of u128,u64,u32,u16,u8,Fp6,Fp2,Fp to Fp12

impl <B:Extensible<2>+Extensible<3>>From<u128> for Fp12<B>{
    fn from(num: u128) -> Self{
        Self{c0:<Fp6<B>>::from(num),c1:<Fp6<B>>::ZERO}
    }
}
impl <B:Extensible<2>+Extensible<3>>From<u64> for Fp12<B>{
    fn from(num: u64) -> Self{
        Self{c0:<Fp6<B>>::from(num),c1:<Fp6<B>>::ZERO}
    }
}
impl <B:Extensible<2>+Extensible<3>>From<u32> for Fp12<B>{
    fn from(num: u32) -> Self{
        Self{c0:<Fp6<B>>::from(num),c1:<Fp6<B>>::ZERO}
    }
}
impl <B:Extensible<2>+Extensible<3>>From<u16> for Fp12<B>{
    fn from(num: u16) -> Self{
        Self{c0:<Fp6<B>>::from(num),c1:<Fp6<B>>::ZERO}
    }
}
impl <B:Extensible<2>+Extensible<3>>From<u8> for Fp12<B>{
    fn from(num:u8) -> Self{
        Self{c0:<Fp6<B>>::from(num),c1:<Fp6<B>>::ZERO}
    }
}
impl <B:Extensible<3>+ Extensible<2>>From<Fp6<B>> for Fp12<B>{
    fn from(num: Fp6<B>) -> Self {
        Self { c0: num,
               c1: <Fp6<B>>::ZERO }
    }
}
impl <B:Extensible<3>+ Extensible<2>>From<Fp2<B>> for Fp12<B>{
    fn from(num: Fp2<B>) -> Self {
        Self { c0: <Fp6<B>>::from(num),
               c1: <Fp6<B>>::ZERO }
    }
}
impl <B:Extensible<3>+ Extensible<2>>From<&B> for Fp12<B>{
    fn from(num: &B) -> Self {
        Self { c0: <Fp6<B>>::from(num),
               c1: <Fp6<B>>::ZERO }
    }
}
impl<B: Extensible<2>+Extensible<3>> From<[u64; 6]> for Fp12<B> {
    fn from(value: [u64; 6]) -> Self {
        Self {
            c0: <Fp6<B>>::from(value),
            c1: <Fp6<B>>::ZERO,
        }
    }
}


impl<B: Extensible<2>+Extensible<3>> From<U256> for Fp12<B> {
    fn from(value: U256) -> Self {
        Self {
            c0: <Fp6<B>>::from(value),
            c1: <Fp6<B>>::ZERO,
        }
    }
}



impl<'a, B: Extensible<2>+Extensible<3>> TryFrom<&'a [u8]> for Fp12<B>{
    type Error = String;

    /// Converts a slice of bytes into a field element; returns error if the value encoded in bytes
    /// is not a valid field element. The bytes are assumed to be in little-endian byte order.
    fn try_from(bytes: &[u8]) -> Result<Self, Self::Error> {
        let value = bytes
            .try_into()
            .map(Fp::from_repr)
            .map_err(|error| format!("{error}"))?;
        if value >= Fp(MODULUS) {
            return Err(format!(
                "cannot convert bytes into a field element: \
                value {value} is greater or equal to the field modulus"
            ));
        }
        Ok(Fp12{c0:<Fp6<B>>::from(value.0),c1:<Fp6<B>>::ZERO})
    }
}
impl <B:Extensible<2>+Extensible<3>>AsBytes for Fp12<B>{
    fn as_bytes(&self) -> &[u8] {
    // TODO: take endianness into account
        let self_ptr: *const Self = self;
        unsafe { slice::from_raw_parts(self_ptr as *const u8, ELEMENT_BYTES)}
    }
}
impl <B:Extensible<2>+Extensible<3>>Randomizable for Fp12<B>{
    const VALUE_SIZE: usize = fp::ELEMENT_BYTES;

    fn from_random_bytes(bytes: &[u8]) -> Option<Self> {
        Self::try_from(bytes).ok()
    }
}
impl <B:Extensible<2>+Extensible<3>>Serializable for Fp12<B>{
    fn write_into<W: ByteWriter>(&self, target: &mut W) {
        target.write_u8_slice(&self.c0.to_bytes());
    }
}
impl <B:Extensible<2>+Extensible<3>>Deserializable for Fp12<B>{
    fn read_from<R: ByteReader>(_source: &mut R) -> Result<Self, DeserializationError> {
     unimplemented!()
    }
}

impl <B:Extensible<3> + Extensible<2> + ConstantTimeEq> ConstantTimeEq for Fp12<B> {
    fn ct_eq(&self, other: &Self) -> Choice {
        self.c0.ct_eq(&other.c0) & self.c1.ct_eq(&other.c1)
    }
}
impl<B: Extensible<2>+Extensible<3>+ConditionallySelectable>  ConditionallySelectable for Fp12<B> {
    fn conditional_select(a: &Self, b: &Self, choice: Choice) -> Self {
        Self{
            c0:Fp6::<B>::conditional_select(&a.c0, &b.c0, choice),
            c1:Fp6::<B>::conditional_select(&a.c1, &b.c1, choice)
        }
    }
}

// returns a+b ( (a+b*w) + (c+d*w) = (a+c) + (b+d)*w // component wise addition
fn add<B:Extensible<2>+Extensible<3>>(a:&Fp12<B>,b:&Fp12<B>)->Fp12<B>{
    Fp12 { c0: a.c0+b.c0, c1: a.c1+b.c1 }
}
// returns a-b in Fp12 ( similar to component wise subtraction)
fn sub<B:Extensible<2>+Extensible<3>>(a:&Fp12<B>,b:&Fp12<B>)->Fp12<B>{
    Fp12 { c0: a.c0-b.c0, c1: a.c1-b.c1 }
}
// returns additive inverse of a in Fp12
fn neg<B:Extensible<2>+Extensible<3>>(a:&Fp12<B>)->Fp12<B>{
    Fp12 { c0: -(a.c0), c1: -(a.c1) }
}
// returns a/b means a*(multiplicative inverse of b)
fn div<B:Extensible<2>+Extensible<3>>(a:&Fp12<B>,b:&Fp12<B>)->Fp12<B>{
    *a*b.invert().unwrap()
}
