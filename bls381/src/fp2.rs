use core::{Serializable, AsBytes, Randomizable, ByteWriter, Deserializable, ByteReader, DeserializationError};
use std::{ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Neg, Sub, SubAssign}, slice, fmt::{Display, Formatter}};
use crypto_bigint::{
    U384,
    subtle::{CtOption, Choice, ConstantTimeEq, ConditionallySelectable}, U768, generic_array::GenericArray, U256};

use traits::traits::{Field, Extensible, PrimeField, ExtensionOf};

use crate::{fp::{Fp, ELEMENT_BYTES, MODULUS, MODULUS_MINUS_ONE, MODULUS_MINUS_ONE_DIV_TWO}, fp6::Fp6};

pub type FieldSize = <U768 as crypto_bigint::ArrayEncoding>::ByteSize;
/// Byte representation of a base/scalar field element of a given curve.
pub type FieldBytes = GenericArray<u8, FieldSize>;

#[derive(Clone, Copy, Debug,Default, PartialEq, Eq, PartialOrd, Ord)]
// Fp2 is the quadratic extension of basefield Fp
/// An element of Fp2, represented by c0 + c1 * u.

pub struct Fp2<B: Extensible<2>> {
    pub c0: B,
    pub c1: B,
}

impl <B: Extensible<2>>Fp2<B>{
    //NONRESIDUE = -1
    pub const FP2_NONRESIDUE: Fp = Fp(MODULUS_MINUS_ONE).to_montgomery();

    pub const FROBENIUS_COEFF_C1: &'static [Fp] = &[
        // Fq(-1)**(((q^0) - 1) / 2)
        Fp::ONE,
        // Fq(-1)**(((q^1) - 1) / 2)
        Fp(MODULUS_MINUS_ONE).to_montgomery(),
    ];
    //Additive identity 
    pub const ZERO:Fp2<B>=Self { c0: B::ZERO, c1: B::ZERO };
    // Multiplicative identity
    pub const ONE:Fp2<B>=  Self { c0: B::ONE, c1: B::ZERO };
    // returns the Fp2 element c0+c1*u if two field elements c0 and c1 are passed into it.
    pub const fn new(c0: B, c1: B) -> Self{
       return Self{c0,c1};
    }
    // returns the norm of  an Fp2 element i.e norm of a+b*u is a^2+b^2
    pub fn norm(&self) -> B {
        let mut t0 = self.c0;
        let mut t1 = self.c1;
        t0=t0.square();
        t1=t1.square();
        t1 += t0;

        t1
    }
    // returns the zero(additive identity) of Fp2 
    pub const fn zero() -> Self {
        
        Self { c0: B::ZERO, c1: B::ZERO }
    }
    // returns the one (multiplicative identity) of Fp2
    pub const fn one() -> Self {
        
        Self { c0: B::ONE, c1: B::ZERO }
    }
    fn mul_fp_by_nonresidue_in_place(fe: &mut B) -> &mut B{
        *fe *= B::from(Self::FP2_NONRESIDUE.0.to_words());
        fe
    }
    pub fn mul_base_field_by_frob_coeff(fe: &mut B, power: usize) {
        *fe *= B::from(Self::FROBENIUS_COEFF_C1[power % 2].0.to_words());
    }
    // Multiply the Fp2 element by the cubic and quadratic nonresidue 1 + u.
    pub fn mul_by_nonresidue(&mut self) -> Self{
        let t0 = self.c0;
        self.c0 -= self.c1;
        self.c1 += t0;
        *self
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
    // scalar multiplication of Fp2 element with Fp element
    pub fn mul_scalar(&self, s: B) ->  Self {
        //Self { c0: self.c0*s, c1: self.c1*s }
        let res=<B as Extensible<2>>::mul_base([self.c0,self.c1], s);
        Fp2{ c0: res[0], c1: res[1] }
    }
    

    // checks whether the given element in Fp2 is square or not.
    pub fn is_square(mut x: Fp2<B>)-> bool{
        let mut tv1 = x.c0.square();
        let  tv2 = Self::mul_fp_by_nonresidue_in_place(&mut x.c1);
        let  tv = &mut tv2.square();
        tv1 = tv1 - *tv;
        tv1 = tv1.power_by( &MODULUS_MINUS_ONE_DIV_TWO.from_montgomery().0.to_words());
        let e1 = tv1 != B::from(Fp(MODULUS_MINUS_ONE).to_montgomery().0.to_words());
        e1
         
    }
    // convert the Fp2 element into a vector of bytes(bytes representation)
    pub fn to_bytes(a:&Fp2<B>) -> Vec<u8>{
        let mut vec_bytes = Vec::new();
        vec_bytes.push(a.c0.to_bytes());
        vec_bytes.push(a.c1.to_bytes());
        let mut final_bytes = Vec::new();
        for bytes in vec_bytes{
            for byte in bytes{
                final_bytes.push(byte);
            }
        }
        final_bytes
    }

  pub  fn frobenius_map(&mut self, power: usize) {
        let a=Self::FROBENIUS_COEFF_C1[power % 2].from_montgomery().0.to_words();
        self.c1.mul_assign(B::from(a));
    }




}
impl <B:Extensible<2>> Field for Fp2<B>{
    // checks whether the given field element of Fp2 is zero or not 
    fn is_zero(self) -> bool {
        
        self==Self::ZERO
    }
    // checks whether the given field element of Fp2 is one or not 
    fn is_one(self) -> bool {
    
        self==Self::ONE
    }
    // returns the twice of element passed in it 
    fn double(self) -> Self {
        (self)+(self)
    }
    // returns the thrice of element of Fp2
    fn triple(self) -> Self {
        (self)+(self)+(self)
    }
    // returns the square of self in Fp2
    fn square(self) ->Self {
        let a=[self.c0,self.c1];
        let res=<B as Extensible<2>>::square(a);
        Self { c0:res[0], c1:res[1] }
    }
    // returns the inverse of self
    fn invert(self) -> CtOption<Self> {
        let a=[self.c0,self.c1];
        let res=<B as Extensible<2>>::invert(a);
        if res.is_some().unwrap_u8()==1{
        let out =Fp2 { c0: res.unwrap()[0], c1: res.unwrap()[1]};
        CtOption::new(out, res.is_some())}
        else {
            CtOption::new(Self::ZERO,res.is_some())
        }
    }
    // additive and multiplicative identity in Fp2 i.e xero and one
    const ZERO:Self=Self::ZERO;
    const ONE:Self=Self::ONE;
    // returns the random element in Fp2
    fn random()->Self {
        Self{
            c0:B::random(),
            c1:B::random(),
        }
    }

    fn sqrt(self)->CtOption<Self> {
        let a=[self.c0,self.c1];
        let res=<B as Extensible<2>>::sqrt(a);
        if bool::from(res.is_some()){
        let out =Fp2{ c0: res.unwrap()[0], c1: res.unwrap()[1]};
        CtOption::new(out, res.is_some())}
        else {
            CtOption::new(Self::ZERO,res.is_some())
        }
    }
    // Exponentiates the Fp2 element by number which is given as array of u64(i.e in words)
    fn power_by<S:AsRef<[u64]>>(self,pow:S)->Self {
        let mut res =Fp2{c0:B::from(U384::ONE.to_words()),c1:B::ZERO};
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
        let mut  a = Vec::new();
        for i in 0..self.c0.to_words().len(){
          a.push(self.c0.to_words()[i])
        }
        for i in 0..self.c1.to_words().len(){
            a.push(self.c1.to_words()[i])
          }
          a
    }
    type BaseField=Fp2<B>;
    // element bytes
    const ELEMENT_BYTES: usize = ELEMENT_BYTES;

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
impl <B:Extensible<2>>PrimeField for Fp2<B>{
    // checks whether the self is odd or not
    fn is_odd(self)->Choice{
        self.c0.is_odd()
    }
    fn get_root_of_unity(_k:u32)->Self {
        unimplemented!()
    }

    const MODULUS:&'static str="1a0111ea397fe69a4b1ba7b6434bacd764774b84f38512bf6730d2a0f6b0f6241eabfffeb153ffffb9feffffffffaaab";
                                
    const NUM_BITS:u32=(ELEMENT_BYTES * 8 * 2) as u32;
    // u + 2; Fp2{c0 :2 , c1: 1}
    const GENERATOR:Self = unimplemented!();

    const TWO_ADDICITY: u32 = 3;

    fn is_even(self)->Choice{
        !self.is_odd()
    }

    type Repr=FieldBytes;

    const TWO_ADIC_ROOT: & 'static str = unimplemented!();

}
impl <B:Extensible<2>+Extensible<3>> Extensible<3> for Fp2<B>{
    /// Returns a product of `a` and `b` in the field defined by this extension.
    //irreducible polynomial is v^3 - u -1
    fn mul(a: [Self; 3], b: [Self; 3]) -> [Self; 3] {
        let mut a_a = a[0];
        let mut b_b = a[1];
        let mut c_c = a[2];
        a_a = a_a.mul(b[0]);
        b_b = b_b.mul(b[1]);
        c_c = c_c.mul(b[2]);

        let mut t1 = b[1];
        t1 = t1.add(b[2]);
        {
        let mut tmp = a[1];
        tmp = tmp.add(a[2]);

        t1 = t1.mul(tmp);
        t1 = t1.sub(b_b);
        t1 = t1.sub(c_c);
        t1 = t1.mul_by_nonresidue();
        t1 = t1.add(a_a);
    }

        let mut t3 = b[0];
        t3 = t3.add(b[2]);

    {
        let mut tmp = a[0];
        tmp = tmp.add(a[2]);

        t3 = t3.mul(tmp);
        t3 = t3.sub(a_a);
        t3 = t3.add(b_b);
        t3 = t3.sub(c_c);
    }

        let mut t2 = b[0];
        t2 = t2.add(b[1]);
    {
        let mut tmp = a[0];
        tmp = tmp.add(a[1]);

        t2 = t2.mul(tmp);
        t2 = t2.sub(a_a);
        t2 = t2.sub(b_b);
       let c_c =  c_c.mul_by_nonresidue();

        t2 = t2.add(c_c);
    }
        [t1,t2,t3]     
    }
 // this function multiplies the Fp6 element by an Fp2 element
    fn mul_base(a: [Self; 3], b: Self) -> [Self; 3] {
        let mut b_b = a[1];
        b_b = b_b.mul(b);

        let mut t1 = b;
        {
            let mut tmp = a[1];
            tmp = tmp.add(a[2]);

            t1 = t1.mul(tmp);
            t1 = t1.sub(b_b);
            t1 = t1.mul_by_nonresidue();
        }

        let mut t2 = b;
        {
            let mut tmp = a[0];
            tmp = tmp.add(a[1]);

            t2 = t2.mul(tmp);
            t2 = t2.sub(b_b);
        }
        [t1,t2,b_b]
      
    }
// returns the square of Fp6 element
    fn square(a: [Self; 3])->[Self; 3] {
        let mut m = a;
        let mut s0 = a[0];
        s0 = s0.square();
        let mut ab = a[0];
        ab = ab.mul(a[1]);
        let mut s1 = ab;
        s1 = s1.double();
        let mut s2 = a[0];
        s2 = s2.sub(a[1]);
        s2 = s2.add(a[2]);
        s2 = s2.square();
        let mut bc = a[1];
        bc = bc.mul(a[2]);
        let mut s3 = bc;
        s3 = s3.double();
        let mut s4 = a[2];
        s4 = s4.square();

        m[0] = s3;
        m[0] = m[0].mul_by_nonresidue();
        m[0] = m[0].add(s0);

        m[1] = s4;
        m[1] = m[1].mul_by_nonresidue();
        m[1] = m[1].add(s1);

        m[2] = s1;
        m[2] = m[2].add(s2);
        m[2] = m[2].add(s3);
        m[2] = m[2].sub(s0);
        m[2] = m[2].sub(s4);
        m
    }

    fn sqrt(_a: [Self; 3])->CtOption<[Self; 3]> {
        unimplemented!()
    }
// returns the invert of Fp6 element
    fn invert(a: [Self; 3])->CtOption<[Self; 3]> {
        // invese of zero is not defined . So we are panicking when zero is given as input.
        if a[0].is_zero()&a[1].is_zero()&a[2].is_zero()
            { return CtOption::new(a,Choice::from(0))}
    let mut c0 = a[2];

    c0 = c0.mul_by_nonresidue();
    c0 = c0.mul(a[1]);
    c0 = c0.neg();
    {
        let mut c0s = a[0];
        c0s = c0s.square();
        c0 = c0.add(c0s);
    }
    let mut c1 = a[2];
    c1 = c1.square();
    c1 = c1.mul_by_nonresidue();

    {
        let mut c01 = a[0];
        c01 = c01.mul(a[1]);
        c1 = c1.sub(c01);
    }
    let mut c2 = a[1];
    c2 = c2.square();
    {
        let mut c02 = a[0];
        c02 = c02.mul(a[2]);
        c2 = c2.sub(c02);
    }

    let mut tmp1 = a[2];
    tmp1 = tmp1.mul(c1);
    let mut tmp2 = a[1];
    tmp2 = tmp2.mul(c2);
    tmp1 = tmp1.add(tmp2);
    tmp1 = tmp1.mul_by_nonresidue( );
    tmp2 = a[0];
    tmp2 = tmp2.mul(c0);
    tmp1 = tmp1.add(tmp2);

    let t = tmp1.invert().unwrap();
    let mut tmp = Fp6 {
        c0: t,
        c1: t,
        c2: t,
    };
    tmp.c0 = tmp.c0.mul(c0);
    tmp.c1 = tmp.c1.mul(c1);
    tmp.c2 = tmp.c2.mul(c2);

    CtOption::new([tmp.c0,tmp.c1,tmp.c2], Choice::from(1))
    }

    fn is_supported() -> bool {
        true
    }
}
impl<B:Extensible<2>> Extensible<2> for Fp2<B>{
    fn mul(_a: [Self; 2], _b: [Self; 2]) -> [Self; 2] {
        unimplemented!()
    }

    fn mul_base(_a: [Self; 2], _b: Self) -> [Self; 2] {
        unimplemented!()
    }
    fn square(_a: [Self; 2])->[Self; 2] {
     unimplemented!()
    }

    fn sqrt(_a: [Self; 2])->CtOption<[Self; 2]> {
     unimplemented!()
    }

    fn invert(_a: [Self; 2])->CtOption<[Self; 2]> {
     unimplemented!()
    }
}
impl<B:Extensible<2>> ExtensionOf<B> for Fp2<B> where Fp2<B>: From<B> {
    fn mul_base(self, other: B) -> Self {
        B::mul_base([self.c0, self.c1], other).into()
    }
}

impl <B:Extensible<2>> Display for Fp2<B>{
    fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
        write!(f, "{}", self)
    }
}
// Basic arithmetic operators and assignment operators on Fp2 like + , - , * , /
impl <B:Extensible<2>> Add for Fp2<B>{
    type Output=Self;
    fn add(self, other: Self) -> Self{
        add::<B>(&self,&other)
    }
}

impl <B:Extensible<2>> Sub for Fp2<B>{
    type Output = Self;
    fn sub(self, other: Self) -> Self::Output{
        sub::<B>(&self,&other)
    }
}

impl<B:Extensible<2>>Mul for Fp2<B> {
    type Output = Self;
    // returns a*b in Fp2 using the irreducible polynomial u^2+1=0 like (a+b*u)(c+d*u)=(ac-bd)+(ad+bc)*u
    fn mul(self, rhs: Self) -> Self{
        let aa=[self.c0,self.c1];
        let bb=[rhs.c0,rhs.c1];
        let res=<B as Extensible<2>>::mul(aa, bb);
        Fp2{c0:res[0],c1:res[1]}
       
    }
}
impl <B:Extensible<2>> Div for Fp2<B> {
    type Output = Self;
    fn div(self, rhs: Self) -> Self::Output{
        div::<B>(&self,&rhs)
    }
}
impl <B:Extensible<2>>AddAssign for Fp2<B>{
    fn add_assign(&mut self, other: Self) {
        *self=add::<B>(self,&other);
    }
}
impl <B:Extensible<2>>SubAssign for Fp2<B>{
    fn sub_assign(&mut self, other: Self) {
        *self=sub(self,&other);
    }
}
impl<B:Extensible<2>> MulAssign for Fp2<B>{
    
   fn mul_assign(&mut self, rhs: Self){
        *self=*self*rhs
   }
}
impl <B:Extensible<2>>DivAssign for Fp2<B>{
    fn div_assign(&mut self, rhs: Self){
        *self=div(self,&rhs)
    }
}
impl <B:Extensible<2>>Neg for Fp2<B>{
   type Output = Self;
   fn neg(self) -> Self::Output{
       neg(&self)
   }
}

// Conversions .....
// conversion of u128,u64,u32,u16,u8,Fp to Fp2


impl  <B:Extensible<2>>From<&B> for Fp2<B>{
    fn from(num: &B) -> Self {
        Self { c0: *num, c1: B::ZERO }
    }
}
impl From<Fp> for Fp2<Fp>{
    fn from(value: Fp) -> Self {
        Fp2::<Fp>::new(Fp::from(value), Fp::ZERO)
    }
}
impl <B:Extensible<2>> From<u128> for Fp2<B>{
    fn from(num: u128) -> Self{
        Self{c0:B::from(num),c1:B::ZERO}
    }
}
impl <B:Extensible<2>> From<u64> for Fp2<B>{
    fn from(num: u64) -> Self{
        Self{c0:B::from(num),c1:B::ZERO}
    
    }
}
impl <B:Extensible<2>> From<u32> for Fp2<B>{
    fn from(num: u32) -> Self{
        Self{c0:B::from(num),c1:B::ZERO}
    
    }
}
impl <B:Extensible<2>>From<u16> for Fp2<B>{
    fn from(num: u16) -> Self{
        Self{c0:B::from(num),c1:B::ZERO}
    
    }
}
impl <B:Extensible<2>>From<u8> for Fp2<B>{
    fn from(num: u8) -> Self{
        Self{c0:B::from(num),c1:B::ZERO}
    
    }
}
//Conversion from double of field elements
impl<B:From<U>+Extensible<2>,U:Copy> From<[U; 2]> for Fp2<B> {
    fn from(value: [U; 2]) -> Self {
        Fp2::new(B::from(value[0]),B::from(value[1]))
    }
}
impl<B: Extensible<2>> From<[u64; 6]> for Fp2<B> {
    fn from(value: [u64; 6]) -> Self {
        Self {
            c0: B::from(value),
            c1: B::ZERO,
        }
    }
}

impl<B: Extensible<2>> From<U256> for Fp2<B> {
    fn from(value: U256) -> Self {
        Self {
            c0: B::from(value),
            c1: B::ZERO,
        }
    }
}

impl <'a, B:Extensible<2>> TryFrom<&'a [u8]> for Fp2<B>{
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
        Ok(Fp2{c0:B::from(value.0.to_words()),c1:B::ZERO})
    }
}
impl <B:Extensible<2>>AsBytes for Fp2<B>{
    fn as_bytes(&self) -> &[u8] {
    // TODO: take endianness into account
        let self_ptr: *const Self = self;
        unsafe { slice::from_raw_parts(self_ptr as *const u8, ELEMENT_BYTES)}
    }
}
impl <B:Extensible<2>+ConstantTimeEq>ConstantTimeEq for Fp2<B> {
    fn ct_eq(&self, other: &Self) -> Choice {
        self.c0.ct_eq(&other.c0) & self.c1.ct_eq(&other.c1)
    }
}
impl <B:Extensible<2>>Randomizable for Fp2<B>{
    const VALUE_SIZE: usize = ELEMENT_BYTES;

    fn from_random_bytes(bytes: &[u8]) -> Option<Self> {
        Self::try_from(bytes).ok()
    }
}
impl <B:Extensible<2>>Serializable for Fp2<B>{
    fn write_into<W: ByteWriter>(&self, target: &mut W) {
        target.write_u8_slice(&self.c0.to_bytes());
    }
}
impl <B:Extensible<2>>Deserializable for Fp2<B>{
    fn read_from<R: ByteReader>(_source: &mut R) -> Result<Self, DeserializationError> {
         unimplemented!()
    }
}
impl <B:Extensible<2>+ConditionallySelectable> ConditionallySelectable for Fp2<B> {
    fn conditional_select(a: &Self, b: &Self, choice: Choice) -> Self {
        Self{
            c0:B::conditional_select(&a.c0, &b.c0, choice),
            c1:B::conditional_select(&a.c1, &b.c1, choice)
    }
}
}
// returns the addition of a and b in Fp2 field. (similar to component wise addition)
fn add<B:Extensible<2>>(a: &Fp2<B>, b: &Fp2<B>) -> Fp2<B> {
    Fp2 {
        c0: a.c0 + b.c0,
        c1: a.c1 + b.c1,
    }
}
// returns the additive inverse of element in Fp2.
fn neg<B:Extensible<2>>(a: &Fp2<B>) -> Fp2<B>{
    Fp2 { c0: -a.c0,
         c1: -a.c1 }
}
// return a - b in Fp2 ( similar to component wise subtraction) where a and b are elements of Fp2
fn sub<B:Extensible<2>>(a: &Fp2<B>, b: &Fp2<B>) -> Fp2<B> {
    Fp2 {
        c0: a.c0 - b.c0,
        c1: a.c1 - b.c1,
    }
}
// computes a/b in Fp2 means a*(Multiplicative inverse of b in Fp2)
fn div<B:Extensible<2>>(a:&Fp2<B>,b:&Fp2<B>)->Fp2<B>{
    let c=b.invert().unwrap();
    (*a)*(c)
}