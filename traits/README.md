# Traits

This crate includes the traits which are defined for the implementation of arithmetic operations over the fields along with the trait for the algebraic hash functions. These traits include :

* **Field trait** : used to define the function over the fields like `random()`, `square()`, `cube()`,`invert()`, `sqrt()`, `power_by()` ,`double()`,`triple()`, `is_zero()`, `is_one()`,` CONST ZERO`, `CONST ONE` , type of the base field element along with the bounds like Clone, Debug ,Eq, Sized etc.
* **PrimeField trait** : used to define the function like `is_odd()`, `is_even()`, `CONST MODULUS`,`CONST NUM_BITS`(number. of bits used to represent the field element), `CONST GENERATOR`(generator of multiplicative group of the field), `CONST TWO_ADIC_ROOT_OF_UNITY` ,`CONST TWO_ADICITY`,`get_root_of_unity` along with the bounds like Copy, Default , Sync , AsRef<[ u8 ]> , AsMut<[ u8 ]> , 'static.
* **Extensible trait** : this trait is defined for the implementation of the extension fields. It includes functions like 
`mul()`, `mul_base()` (multiplication with base field), `square()`,`invert()`, `sqrt()`.
* **Extension of trait** : this trait is defined for the implementation of the extension field on itself as a field is always an extension of itself. It includes function `mul_base()`.
* **Hasher Trait** : it is defined for the algebraic hash functions such as rescue hash, poseidon hash and GMIMC hash. It is used to compute the hash of the field elements using the `hash()` function defined in the trait along with the  `hash_and_store_states()` function which computes the rescue hash and also store intermediate states.
* **PoseidonParameter trait** : it includes the parameters used in the poseidon hash algorithm such as 
  state width, alpha, rate, total number of rounds, number of full rounds in beginning, number of partial rounds, total number of full rounds, MDS matrix, ineternal MDS matrix and round constants.
* **RescueParameter trait** : it includes the parameters used in the rescue hash algorithm such as state width, alpha, inverse alpha , rate , number of rounds, MDS matrix, inverse MDS matrix and round constants.
* **GMIMCParameter trait** : it includes the parameters used in the GMIMC hash algorithm such as rate, number of rounds, state width and round constants.