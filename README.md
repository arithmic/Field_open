# Field

The Field repository is a rust implementation for arithmetic operations of multiple fields which are further used for implementing elliptic curves. The fields implemented in this repository are:
* Stark-252 field [https://github.com/arithmic/field_trait/tree/main/stark252/src]
* BLS12-381 field [https://github.com/arithmic/field_trait/tree/main/bls381/src]
* Bandersnatch field [https://github.com/arithmic/field_trait/tree/main/bls381/src]
* Jubjub field [https://github.com/arithmic/field_trait/tree/main/bls381/src]
* Cheetah64 field [https://github.com/arithmic/field_trait/tree/main/cheetah64/src]
* 128bit prime field [https://github.com/arithmic/field_trait/tree/main/f128/src]
* BN254 field  [https://github.com/arithmic/field_trait/tree/main/bn254/src]
* Baby Jubjub field  [https://github.com/arithmic/field_trait/tree/main/bn254/src]


In this repository algebraic hash functions : 
* Rescue hash, 
* Poseison hash,
* GMIMC hash <br>
are also defined in the hash crate [https://github.com/arithmic/field_trait/tree/main/hash/src] along with the fields.

### Testing
To run all the tests of the repository use the command : 

```
cargo test

```
in the Cargo.toml file.

### How to use the field trait crates in your project
Add the crates into your project by adding the specific crate name(as in Cargo.toml file) as <br>
 "crate_name" = {git = "ssh://git@github.com/arithmic/field_trait.git" , branch = "main"} <br>
in your Cargo.toml file and run `cargo update` command.
