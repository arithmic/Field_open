use crypto_bigint::subtle::{CtOption, Choice, ConstantTimeEq, ConditionallySelectable};
use crypto_bigint::U256;
use traits::traits::{Field, Extensible, PrimeField};
use crate::fp::{Fp, MODULUS, ELEMENT_BYTES,  FieldBytes};
use crate::fp12::Fp12;
use crate::fp2::Fp2;
use core::{Serializable, AsBytes, Randomizable, ByteWriter, Deserializable, ByteReader, DeserializationError};
use std::{
    convert::TryFrom,
    fmt::{Display,Formatter},
    ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Neg, Sub, SubAssign},
    slice,
};


//implement cubic extension oer Fp2
// An element of Fp6, represented by c0 + c1 * v + c2 * v^(2).
#[derive(Clone, Copy, Debug,Default, PartialEq, Eq, PartialOrd, Ord)]
pub struct Fp6<B: Extensible<3> + Extensible<2>> {
    pub c0:Fp2<B>,
    pub c1:Fp2<B>,
    pub c2:Fp2<B>,
}
impl <B:Extensible<3>+ Extensible<2>>  Fp6<B>{

    //non residue = (u + 9)
    pub fn fp6_nonresidue() -> Fp2<B>{
        Fp2::<B>::new(
            B::from(9u8),
            B::ONE,
        )
    }
    // Additive identity 0+0v+0v^2
    pub const ZERO:Fp6<B>=Fp6{
        c0:<Fp2<B>>::ZERO,
        c1:<Fp2<B>>::ZERO,
        c2:<Fp2<B>>::ZERO,
    };
    // Multiplicative identity in Fp6 : 1+0*v + 0*v^2
    pub const ONE:Fp6<B>=Fp6{
        c0:<Fp2<B>>::ONE,
        c1:<Fp2<B>>::ZERO,
        c2:<Fp2<B>>::ZERO,
    };
    // returns the Fp6 element using the paramaters c0,c1,c2 passed to it : c0+c1*v+c2*v^2
    pub const fn new(c0: Fp2<B>, c1: Fp2<B>, c2: Fp2<B>) -> Self {
        return Self { c0, c1, c2 };
    }
    // this function multiplies the Fp6 element by an Fp2 element
    pub fn mul_by_1(self, c1: &Fp2<B>) -> Self {
        let aa=[self.c0,self.c1,self.c2];
        let res=<Fp2<B> as Extensible<3>>::mul_base(aa, *c1);
        Self { c0: res[0], c1: res[1], c2: res[2] }

    }
    // elements as bytes
    pub fn elements_as_bytes(elements: &[Self]) -> &[u8] {
        unsafe {
            slice::from_raw_parts(
                elements.as_ptr() as *const u8,
                elements.len() * ELEMENT_BYTES,
            )
        }
    }

    pub fn frobenius_coeff_fp6_c1() -> [Fp2<B>; 6] {
        [
            // Fp2::NONRESIDUE^(((q^0) - 1) / 3)
            Fp2::<B>::new(B::ONE, B::ZERO),
            // Fp2::NONRESIDUE^(((q^1) - 1) / 3)
            Fp2::<B>::new(
                B::from(U256::from_be_hex("2FB347984F7911F74C0BEC3CF559B143B78CC310C2C3330C99E39557176F553D")),
                B::from(U256::from_be_hex("16C9E55061EBAE204BA4CC8BD75A079432AE2A1D0B7C9DCE1665D51C640FCBA2")),
            ),
        
            // Fp2::NONRESIDUE^(((q^2) - 1) / 3)
            Fp2::<B>::new(
                B::from(U256::from_be_hex(
                    "30644E72E131A0295E6DD9E7E0ACCCB0C28F069FBB966E3DE4BD44E5607CFD48",
                )),
                B::ZERO,
            ),
            // Fp2::NONRESIDUE^(((q^3) - 1) / 3)
            Fp2::<B>::new(
                B::from(U256::from_be_hex(
                    "0856E078B755EF0ABAFF1C77959F25AC805FFD3D5D6942D37B746EE87BDCFB6D",
                )),
                B::from(U256::from_be_hex(
                    "04F1DE41B3D1766FA9F30E6DEC26094F0FDF31BF98FF2631380CAB2BAAA586DE",
                )),
            ),
            // Fp2::NONRESIDUE^(((q^4) - 1) / 3)
            Fp2::<B>::new(
                B::from(U256::from_be_hex(
                    "000000000000000059E26BCEA0D48BACD4F263F1ACDB5C4F5763473177FFFFFE",
                )),
                B::ZERO,
            ),
            // Fp2::NONRESIDUE^(((q^5) - 1) / 3)
            Fp2::<B>::new(
                B::from(U256::from_be_hex(
                    "28BE74D4BB943F51699582B87809D9CAF71614D4B0B71F3A62E913EE1DADA9E4",
                )),
                B::from(U256::from_be_hex(
                    "14A88AE0CB747B99C2B86ABCBE01477A54F40EB4C3F6068DEDAE0BCEC9C7AAC7",
                )),
            ),
        ]
    }

    pub fn frobenius_coeff_fp6_c2() -> [Fp2<B>; 6] {
        [
            // Fq2(u + 1)**(((2q^0) - 2) / 3)
            Fp2::<B>::new(B::ONE, B::ZERO),
            // Fq2(u + 1)**(((2q^1) - 2) / 3)
            Fp2::<B>::new(
                B::from(U256::from_be_hex("05B54F5E64EEA80180F3C0B75A181E84D33365F7BE94EC72848A1F55921EA762")),
                B::from(U256::from_be_hex("2C145EDBE7FD8AEE9F3A80B03B0B1C923685D2EA1BDEC763C13B4711CD2B8126")),
            ),
        
            // Fq2(u + 1)**(((2q^2) - 2) / 3)
            Fp2::<B>::new(
                B::from(U256::from_be_hex(
                    "000000000000000059E26BCEA0D48BACD4F263F1ACDB5C4F5763473177FFFFFE",
                )),
                B::ZERO,
            ),
            // Fq2(u + 1)**(((2q^3) - 2) / 3)
            Fp2::<B>::new(
                B::from(U256::from_be_hex(
                    "0BC58C6611C08DAB19BEE0F7B5B2444EE633094575B06BCB0E1A92BC3CCBF066",
                )),
                B::from(U256::from_be_hex(
                    "23D5E999E1910A12FEB0F6EF0CD21D04A44A9E08737F96E55FE3ED9D730C239F",
                )),
            ),
            // Fq2(u + 1)**(((2q^4) - 2) / 3)
            Fp2::<B>::new(
                B::from(U256::from_be_hex(
                    "30644E72E131A0295E6DD9E7E0ACCCB0C28F069FBB966E3DE4BD44E5607CFD48",
                )),
                B::ZERO,
            ),
            // Fq2(u + 1)**(((2q^5) - 2) / 3)
            Fp2::<B>::new(
                B::from(U256::from_be_hex(
                    "1EE972AE6A826A7D1D9DA40771B6F589DE1AFB54342C724FA97BDA050992657F",
                )),
                B::from(U256::from_be_hex(
                    "10DE546FF8D4AB51D2B513CDBB25772454326430418536D15721E37E70C255C9",
                )),
            ),
        ]
    }


   

    pub fn mul_fp2_by_nonresidue_in_place(fe: &mut Fp2<B>) -> &mut Fp2<B> {
        let t0 = fe.c0;
        fe.c0 -= fe.c1;
        fe.c1 += t0;
        fe
    }

    pub fn mul_by_nonresidue(&mut self) -> Self {
        use std::mem::swap;
        swap(&mut self.c0, &mut self.c1);
        swap(&mut self.c0, &mut self.c2);
        self.c0 = self.c0.mul_by_nonresidue();
        *self
    }    
    pub fn mul_scalar(&self, s: B) -> Self {
        let d0 = self.c0.mul_scalar(s);
        let d1 = self.c1.mul_scalar(s);
        let d2 = self.c2.mul_scalar(s);
        Self {
            c0: d0,
            c1: d1,
            c2: d2,
        }
    }
    pub fn to_bytes(&self) -> Vec<u8> {
        let mut vec_bytes = Vec::new();
        vec_bytes.push(self.c0.to_bytes());
        vec_bytes.push(self.c1.to_bytes());
        vec_bytes.push(self.c2.to_bytes());
        let mut final_bytes = Vec::new();
        for bytes in vec_bytes {
            for byte in bytes {
                final_bytes.push(byte);
            }
        }
        final_bytes
    }


  pub fn frobenius_map(&mut self, power: usize) {
        self.c0.frobenius_map(power);
        self.c1.frobenius_map(power);
        self.c2.frobenius_map(power);
        let a = Self::frobenius_coeff_fp6_c1();
        let b = Self::frobenius_coeff_fp6_c2();
        self.c1.mul_assign(a[power % 6]);

        self.c2.mul_assign(b[power % 6]);
    }
 }


impl <B:Extensible<3>  + Extensible<2>> Field for Fp6<B>{
    // checks whether the given Fp6 element is zero or not .
    fn is_zero(self) -> bool {
        self==Self::ZERO
    }
    // checks whether the given Fp6 element is one or not .
    fn is_one(self) -> bool {
        self==Self::ONE
    }
    // returns the twice of Fp6 element passed to it
    fn double(self) -> Self {
        (self)+(self)
    }
    // returns the thrice of Fp6 element
    fn triple(self) -> Self {
        (self)+(self)+(self)
    }
    // returns the square of Fp6 element 
    fn square(self) ->Self {
        let a=[self.c0,self.c1,self.c2];
        let res=<Fp2<B> as Extensible<3>>::square(a);
        Self { c0:res[0], c1:res[1],c2:res[2] }
    }
    const ZERO:Self=Self::ZERO;
    // element bytes
    const ELEMENT_BYTES: usize = ELEMENT_BYTES;
    
    const ONE:Self = Self::ONE;

// returns the random Fp6 element
    fn random()->Self {
        Self{c0:<Fp2<B>>::random(),
            c1:<Fp2<B>>::random(),
            c2:<Fp2<B>>::random(),
            }
    }

    fn invert(self) -> CtOption<Self> {
        let a=[self.c0,self.c1,self.c2];
        let res=<Fp2<B> as Extensible<3>>::invert(a);
        if res.is_some().unwrap_u8()==1{
        let out =Fp6 { c0: res.unwrap()[0], c1: res.unwrap()[1],c2:res.unwrap()[2]};
        CtOption::new(out, res.is_some())}
        else {
            CtOption::new(Self::ZERO,res.is_some())
        }
    }

    fn sqrt(self)->CtOption<Self> {
        unimplemented!()
    }
    // computes self ** pow where power is given as array of u64(word)
    fn power_by<S:AsRef<[u64]>>(self,pow:S)->Self {
        let mut res = Fp6::<B>::ONE;
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

    type BaseField=Fp6<B>;

    fn  from_uint_reduced(_w: Self) -> Self {
     unimplemented!()
    }

    fn from_words( _a: &Vec<u64>) -> Self {
     unimplemented!()
    }

    fn get_windows(&self, _exp: usize)->Vec<usize> {
     unimplemented!()
    }

    fn cube(self) -> Self {
        self * (self.square())
    }

    const IS_CANONICAL: bool = true;
}

impl<B:Extensible<3> + Extensible<2>> PrimeField for Fp6<B>{
    // checks whether the self is odd or not
    fn is_odd(self)->Choice{
        self.c0.is_odd()
    }
    fn get_root_of_unity(_k:u32)->Self {
        unimplemented!()
    }

    const MODULUS:&'static str="30644E72E131A029B85045B68181585D97816A916871CA8D3C208C16D87CFD47s";

    const NUM_BITS:u32=(ELEMENT_BYTES*8*6) as u32;

    const GENERATOR:Self=unimplemented!();

    const TWO_ADDICITY: u32=unimplemented!();

    fn is_even(self)->Choice{
        !self.is_odd()
    }

    type Repr=FieldBytes;

    const TWO_ADIC_ROOT: & 'static str = unimplemented!();

}
impl <B:Extensible<3> + Extensible<2>> Extensible<2> for Fp6<B>{
    // computes multiplication of a and b (of Fp12 )using irreducible polynomial w^2-v=0 , so (a+bw)(c+dw)=(ac-bd*v)+(ad+bc)*w

    fn mul(a: [Self; 2], b: [Self; 2]) -> [Self; 2] {
         //let mut m = self;
    let mut aa = a[0];
    aa = aa.mul(b[0]);
    let mut bb = a[1];
    bb = bb.mul(b[1]);
    let mut o = b[0];
    o = o.add(b[1]);

    let mut c1 = a[1].add(a[0]);
    c1 = c1.mul(o);
    c1 = c1.sub(aa);
    c1 = c1.sub(bb);

    let mut c0 = bb;
    c0 = c0.mul_by_nonresidue();

    c0 = c0.add(aa);
    [c0,c1]
    }

    fn mul_base(_a: [Self; 2], _b: Self) -> [Self; 2] {
        unimplemented!()
    }

    fn square(a: [Self; 2])->[Self; 2] {
         //let mut m = self;
         let mut ab = a[0];
         ab = ab.mul(a[1]);
         let mut c0c1 = a[0];
         c0c1 = c0c1.add(a[1]);
         let mut c0 = a[1];
         c0 = c0.mul_by_nonresidue();
         c0 = c0.add(a[0]);
         c0 = c0.mul(c0c1);
         c0 = c0.sub(ab);
         let mut c1 = ab;
         c1 = c1.add(ab);
         ab = ab.mul_by_nonresidue();
         c0 = c0.sub(ab);
         [c0,c1]
    }

    fn sqrt(_a: [Self; 2])-> CtOption<[Self; 2]> {
        unimplemented!()
    }
    // returns the multiplicative inverse of a of Fp12.
    fn invert(a:[Self; 2])-> CtOption<[Self; 2]> {
    // invese of zero is not defined . So we are panicking when zero is given as input.
    if a[0].is_zero()&a[1].is_zero(){ return CtOption::new([a[0],a[1]], Choice::from(0))}
    let mut c0s = a[0];
    c0s = c0s.square();
    let mut c1s = a[1];
    c1s = c1s.square();
    c1s = c1s.mul_by_nonresidue();

    c0s = c0s.sub(c1s);

    let t = c0s.invert().unwrap();
    let mut tmp = Fp12::<B> { c0: t, c1: t };
    tmp.c0 = tmp.c0.mul(a[0]);
    tmp.c1 = tmp.c1.mul(a[1]);
    tmp.c1 = tmp.c1.neg();

    let inv=Fp12 {
        c0: (tmp.c0),
        c1: (tmp.c1),
    };
    CtOption::new([inv.c0,inv.c1], Choice::from(1))
    }

    fn is_supported() -> bool {
        true
    }
}

impl <B:Extensible<3>+ Extensible<2>>Display for Fp6<B> {
    fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
        write!(f, "{}", self)
    }
}
// Basic arithmetic operators and assignment operators on Fp6 like + , - , * , /
impl <B:Extensible<3>+ Extensible<2>>Add for Fp6<B>{
    type Output=Self;
    fn add(self, other: Self) -> Self{
        add(&self,&other)
    }
}
impl <B:Extensible<3>+ Extensible<2>>Sub for Fp6<B>{
    type Output = Self;
    fn sub(self, other: Self) -> Self::Output{
        sub(&self,&other)
    }
}
impl <B:Extensible<3>+ Extensible<2>>Mul for Fp6<B> {
    type Output = Self;
    fn mul(self, rhs: Self) -> Self{
        let aa=[self.c0,self.c1,self.c2];
        let bb=[rhs.c0,rhs.c1,rhs.c2];
        let res=<Fp2<B> as Extensible<3>>::mul(aa, bb);
        Fp6{c0:res[0],c1:res[1],c2:res[2]}
    }
}
impl <B:Extensible<3>+ Extensible<2>>Div for Fp6<B> {
    type Output = Self;
    fn div(self, rhs: Self) -> Self::Output{
        div(&self,&rhs)
    }
}
impl <B:Extensible<3>+ Extensible<2>>AddAssign for Fp6<B>{
    fn add_assign(&mut self, other: Self) {
        *self=add(self,&other);
    }
}
impl <B:Extensible<3>+ Extensible<2>>SubAssign for Fp6<B>{
    fn sub_assign(&mut self, other: Self) {
        *self=sub(&self,&other);
    }
}
impl <B:Extensible<3>+ Extensible<2>>MulAssign for Fp6<B> {
   fn mul_assign(&mut self, rhs: Self){
       *self=(*self)*rhs
   }
}
impl <B:Extensible<3>+ Extensible<2>> DivAssign for Fp6<B>{
    fn div_assign(&mut self, rhs: Self){
        *self=div(self,&rhs)
    }
}
impl <B:Extensible<3>+ Extensible<2>>Neg for Fp6<B> {
   type Output = Self;
   fn neg(self) -> Self::Output{
       neg(&self)
   }
}
// conversion of u128,u64,u32,u16,u8,Fp2,Fp to Fp6
impl <B:Extensible<3>+ Extensible<2>>From<u128> for Fp6<B>{
    fn from(num: u128) -> Self{
        Self{c0:<Fp2<B>>::from(num),c1:<Fp2<B>>::ZERO,c2:<Fp2<B>>::ZERO}
    }
}
impl <B:Extensible<3>+ Extensible<2>>From<u64> for Fp6<B>{
    fn from(num: u64) -> Self{
        Self{c0:<Fp2<B>>::from(num),c1:<Fp2<B>>::ZERO,c2:<Fp2<B>>::ZERO}
    }
}
impl <B:Extensible<3>+ Extensible<2>>From<u32> for Fp6<B>{
    fn from(num: u32) -> Self{
        Self{c0:<Fp2<B>>::from(num),c1:<Fp2<B>>::ZERO,c2:<Fp2<B>>::ZERO}
    }
}
impl <B:Extensible<3>+ Extensible<2>>From<u16> for Fp6<B>{
    fn from(num: u16) -> Self{
        Self{c0:<Fp2<B>>::from(num),c1:<Fp2<B>>::ZERO,c2:<Fp2<B>>::ZERO}
    }
}
impl <B:Extensible<3>+ Extensible<2>>From<u8> for Fp6<B>{
    fn from(num:u8) -> Self{
        Self{c0:<Fp2<B>>::from(num),c1:<Fp2<B>>::ZERO,c2:<Fp2<B>>::ZERO}
    }
}
impl <B:Extensible<3>+ Extensible<2>>From<Fp2<B>> for Fp6<B>{
    fn from(num: Fp2<B>) -> Self {
        Self { c0: num,
               c1: <Fp2<B>>::ZERO, 
               c2: <Fp2<B>>::ZERO }
    }
}
impl <B:Extensible<3>+ Extensible<2>> From<&B> for Fp6<B>{
    fn from(value: &B) -> Self {
        Self { c0: <Fp2<B>>::from(value), c1:<Fp2<B>>::ZERO,c2:<Fp2<B>>::ZERO}
    }
}
impl<B: Extensible<3>+ Extensible<2>> From<[u64; 6]> for Fp6<B> {
    fn from(value: [u64; 6]) -> Self {
        Self {
            c0: <Fp2<B>>::from(value),
            c1: <Fp2<B>>::ZERO,
            c2: <Fp2<B>>::ZERO
        }
    }
}

impl<B: Extensible<3>+ Extensible<2>> From<U256> for Fp6<B> {
    fn from(value: U256) -> Self {
        Self {
            c0: <Fp2<B>>::from(value),
            c1: <Fp2<B>>::ZERO,
            c2: <Fp2<B>>::ZERO
        }
    }
}


impl<'a, B: Extensible<3>+ Extensible<2>> TryFrom<&'a [u8]> for Fp6<B>{
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
        Ok(Fp6{c0:<Fp2<B>>::from(value.0),c1:<Fp2<B>>::ZERO,c2:<Fp2<B>>::ZERO})
    }
}
impl <B:Extensible<3>+ Extensible<2>> AsBytes for Fp6<B>{
    fn as_bytes(&self) -> &[u8] {
    // TODO: take endianness into account
        let self_ptr: *const Self = self;
        unsafe { slice::from_raw_parts(self_ptr as *const u8, ELEMENT_BYTES)}
    }
}
impl <B:Extensible<3>+ Extensible<2>>Randomizable for Fp6<B> {
    const VALUE_SIZE: usize = ELEMENT_BYTES;

    fn from_random_bytes(bytes: &[u8]) -> Option<Self> {
        Self::try_from(bytes).ok()
    }
}
impl <B:Extensible<3>+ Extensible<2>>Serializable for Fp6<B>{
    fn write_into<W: ByteWriter>(&self, target: &mut W) {
        target.write_u8_slice(&self.c0.c0.to_bytes());
    }
}
impl <B:Extensible<3> + Extensible<2>>Deserializable for Fp6<B>{
    fn read_from<R: ByteReader>(_source: &mut R) -> Result<Self, DeserializationError> {
     unimplemented!()
    }
}
impl <B:Extensible<3> + Extensible<2> + ConstantTimeEq> ConstantTimeEq for Fp6<B> {
    fn ct_eq(&self, other: &Self) -> Choice {
        self.c0.ct_eq(&other.c0) & self.c1.ct_eq(&other.c1) & self.c2.ct_eq(&other.c2)
    }
}
impl <B:Extensible<3> + Extensible<2> +ConditionallySelectable> ConditionallySelectable for Fp6<B> {
    fn conditional_select(a: &Self, b: &Self, choice: Choice) -> Self {
        Self{
            c0:Fp2::<B>::conditional_select(&a.c0, &b.c0, choice),
            c1:Fp2::<B>::conditional_select(&a.c1, &b.c1, choice),
            c2:Fp2::<B>::conditional_select(&a.c2, &b.c2, choice)
        }
    }
}
// returns a+b where a and b are Fp6 elements ( (a+b*v+c*v^2)+(a'+b'*v+c'*v^2)  =  (a+a') + (b+b')*v +(c+c')&v^2 )
fn add<B:Extensible<3> + Extensible<2>>(a:&Fp6<B>,b:&Fp6<B>)->Fp6<B>{
    Fp6{
        c0:a.c0+b.c0,
        c1:a.c1+b.c1,
        c2:a.c2+b.c2,
    }
}
// returns a-b ( similar to component wise subtraction)
fn sub<B:Extensible<3> + Extensible<2>>(a:&Fp6<B>,b:&Fp6<B>)->Fp6<B>{
    Fp6{
        c0:a.c0-b.c0,
        c1:a.c1-b.c1,
        c2:a.c2-b.c2,
    }
}
// returns the multiplicative inverse of a
fn neg<B:Extensible<3>+ Extensible<2>>(a:&Fp6<B>)->Fp6<B>{
    Fp6 { c0: -(a.c0), c1: -(a.c1), c2: -(a.c2)}
}
fn div<B:Extensible<3> + Extensible<2>>(a:&Fp6<B>,b:&Fp6<B>)->Fp6<B>{
    let c=b.invert().unwrap();
    (*a)*c
}