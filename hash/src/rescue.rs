use crypto_bigint::U256;
use traits::traits::{Field, Hasher, PrimeField, RescueParameter};

// HASH FUNCTION
// ================================================================================================

//==== struct def =====
// struct defination
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Rescue<F: Field + PrimeField + RescueParameter<F> + Sized> {
    _option: Option<F>,
}

// Implementation of the Hasher trait for Rescue hash function
impl<F: Field + PrimeField + RescueParameter<F>> Hasher for Rescue<F> {
    //  Returns a hash of the provided field elements.
    fn hash(value: &mut Vec<Self::FieldElement>) -> Self::FieldElement {
        Self::rescue_hash(value)
    }

    type FieldElement = F;

    // Compute the rescue hash and also store intermediate states
    fn hash_and_store_states(
        value: &mut [Self::FieldElement],
        result: &mut [Self::FieldElement],
        hash_data: &mut Vec<Vec<Self::FieldElement>>,
    ) {
        Self::rescue_hash_account(value, result, hash_data);
    }
}

// Implementation of Rescue hash function
impl<F: Field + PrimeField + RescueParameter<F>> Rescue<F> {
    pub fn rescue_hash(value: &mut Vec<F>) -> F {
        let mut state = vec![F::ZERO; F::RESCUE_STATE_WIDTH];

        if value.len() % F::RESCUE_RATE != 0 {
            value.append(&mut [F::ZERO].to_vec());
        }
        let mut i = 0;
        while i < value.len() {
            for j in 0..F::RESCUE_RATE {
                state[j] += value[i + j];
            }
            for k in 0..F::RESCUE_NO_OF_ROUNDS {
                apply_round::<F>(&mut state, k);
            }
            i += F::RESCUE_RATE;
        }
        state[0]
    }

    // Compute the rescue hash account and also store intermediate states
    pub fn rescue_hash_account(value: &mut [F], result: &mut [F], hash_data: &mut Vec<Vec<F>>) {
        let mut state = vec![F::ZERO; F::RESCUE_STATE_WIDTH];
        if value.len() % F::RESCUE_RATE != 0 {
            value[value.len()] = F::ZERO;
        }
        let mut i = 0;
        while i < value.len() {
            for j in 0..F::RESCUE_RATE {
                state[j] = state[j].add(value[i + j]);
            }
            for k in 0..F::RESCUE_NO_OF_ROUNDS {
                if k == 0 {
                    hash_data.push(state.clone());
                }
                apply_round::<F>(&mut state, k);
                hash_data.push(state.clone());
            }
            i += F::RESCUE_RATE;
        }
        result.copy_from_slice(&state[..1]);
    }

    // Compute the rescue hash and also store intermediate states.
    pub fn rescue_modified_hash(value: Vec<F>, result: &mut [F], hash_data: &mut Vec<Vec<F>>) {
        let mut state = vec![F::ZERO; F::RESCUE_STATE_WIDTH];
        let amount = value[1];
        //default nonce
        let nonce = value[2];
        //trim 100bits of the balance
        let mut amount_bits = Vec::new();
        for i in 0..100 {
            // amount_bits.push(amount.0.bit(i));
            let a = amount.to_words();
            amount_bits.push(U256::from_words([a[0], a[1], a[2], a[3]]).bit(i));
        }
        //trim 100 bit of the nonce.
        let mut nonce_bits = Vec::new();
        for i in 0..100 {
            let n = nonce.to_words();
            nonce_bits.push(U256::from_words([n[0], n[1], n[2], n[3]]).bit(i));
        }
        //Concatenates the bits of the balance and nonce
        // where nonce bits are LSB and balance bits are MSB
        let mut concatenated_bits = [0_u64; 200];
        for i in 0..100 {
            concatenated_bits[i] = nonce_bits[i];
            concatenated_bits[i + 100] = amount_bits[i];
        }
        // Convert bits into limbs.
        let mut limbs = [0_u64; 4];
        for i in 0..200 {
            limbs[i / 64] =
                limbs[i / 64] + (2_u64.pow((i % 64).try_into().unwrap()) * concatenated_bits[i]);
        }
        state[0] = value[0];
        state[1] = F::from(U256::from_words(limbs));
        for k in 0..F::RESCUE_NO_OF_ROUNDS {
            apply_round::<F>(&mut state, k);
            hash_data.push(state.clone());
        }
        result.copy_from_slice(&state[..1]);
    }
}

// TRACE
// ================================================================================================
pub fn apply_round<F: Field + PrimeField + RescueParameter<F>>(state: &mut [F], step: usize) {
    // determine which round constants to use
    let ark = &F::rescue_ark()[step];
    // apply first half of Rescue round
    apply_sbox::<F>(state);
    apply_mds::<F>(state);
    add_constants::<F>(state, &ark, 0);
    // apply second half of Rescue round
    apply_inv_sbox::<F>(state);
    apply_mds::<F>(state);
    add_constants::<F>(state, &ark, F::RESCUE_STATE_WIDTH);
}

// HELPER FUNCTIONS
// ================================================================================================
#[inline(always)]
#[allow(clippy::needless_range_loop)]
fn add_constants<F: Field + PrimeField + RescueParameter<F>>(
    state: &mut [F],
    ark: &[F],
    offset: usize,
) {
    state.par_iter_mut().enumerate().for_each(|(i, state_element)| {
        *state_element += ark[offset + i];
    });
}
#[inline(always)]
#[allow(clippy::needless_range_loop)]
pub fn apply_sbox<F: Field + PrimeField + RescueParameter<F>>(state: &mut [F]) {
    state.par_iter_mut().for_each(|element| {
        if F::RESCUE_ALPHA == 3 {
            *element = element.square() * *element;
        } else if F::RESCUE_ALPHA == 5 {
            *element = element.square().square() * *element;
        } else {
            *element = element.power_by([F::RESCUE_ALPHA, 0, 0, 0]);
        }
    });
}
#[inline(always)]
#[allow(clippy::needless_range_loop)]
pub fn apply_inv_sbox<F: Field + PrimeField + RescueParameter<F>>(state: &mut [F]) {
    state.par_iter_mut().for_each(|element| {
        *element = element.power_by(&F::RESCUE_INV_ALPHA.to_words());
    });
}

use rayon::prelude::*;
#[inline(always)]
#[allow(clippy::needless_range_loop)]
pub fn apply_mds<F: Field + PrimeField + RescueParameter<F>>(state: &mut [F]) {
    let mut result = vec![F::ZERO; F::RESCUE_STATE_WIDTH];

    result.par_iter_mut().enumerate().for_each(|(i, res)| {
        let start_index = i * F::RESCUE_STATE_WIDTH;
        *res = (0..F::RESCUE_STATE_WIDTH).fold(F::ZERO, |acc, j| {
            acc + F::rescue_mds()[start_index + j] * state[j]
        });
    });

    state.copy_from_slice(&result);
}
#[inline(always)]
#[allow(clippy::needless_range_loop)]
pub fn apply_inv_mds<F: Field + PrimeField + RescueParameter<F>>(state: &mut [F]) {
    let mut result = vec![F::ZERO; F::RESCUE_STATE_WIDTH];

    result.par_iter_mut().enumerate().for_each(|(i, res)| {
        let start_index = i * F::RESCUE_STATE_WIDTH;
        *res = (0..F::RESCUE_STATE_WIDTH).fold(F::ZERO, |acc, j| {
            acc + F::rescue_inv_mds()[start_index + j] * state[j]
        });
    });

    state.copy_from_slice(&result);
}
