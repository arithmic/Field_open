use traits::traits::{Field, Hasher, PoseidonParameter, PrimeField};

use crate::{poseidon_4_2::{self, poseidon2_hash_account}, poseidon_6_5};

//==== struct def =====
// struct defination
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct PoseidonHash<F: Field + PrimeField+ PoseidonParameter<F>> {
    _option: Option<F>,
}

// Implementation of the Hasher trait for Poseidon hash function
impl<F: Field +PrimeField+ PoseidonParameter<F>> Hasher for PoseidonHash<F> {
    type FieldElement = F;
    fn hash(value: &mut Vec<Self::FieldElement>) -> Self::FieldElement {
        Self::poseidon_hash(value)
    }

    fn hash_and_store_states(
        value: &mut [Self::FieldElement],
        result: &mut [Self::FieldElement],
        hash_data: &mut Vec<Vec<Self::FieldElement>>,
    ) {
        poseidon2_hash_account(&mut value.to_vec(),  result, hash_data ) 
    }
}

// Implementation of the Poseidon hash function
impl<F: Field+PrimeField + PoseidonParameter<F>> PoseidonHash<F> {
    pub fn poseidon_hash(value: &mut Vec<F>) -> F {
        if value.len() <= F::STATE_WIDTH_4_2 {
            poseidon_4_2::poseidon2::<F>(value)
        } else {
            poseidon_6_5::poseidon2::<F>(value)
        }
    }
}
