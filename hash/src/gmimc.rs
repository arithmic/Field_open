use traits::traits::{Field, GMIMCParameter, Hasher, PrimeField};

//==== struct def =====
// struct defination
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct GMIMCHASH<F: Field + PrimeField + GMIMCParameter<F>> {
    _option: Option<F>,
}

// Implementation of the Hasher trait for Gmimc hash function
impl<F: Field + PrimeField + GMIMCParameter<F>> Hasher for GMIMCHASH<F> {
    type FieldElement = F;
    fn hash(value: &mut Vec<Self::FieldElement>) -> Self::FieldElement {
        Self::gmimic(value.to_vec())
    }

    fn hash_and_store_states(
        _value: &mut [Self::FieldElement],
        _result: &mut [Self::FieldElement],
        _hash_data: &mut Vec<Vec<Self::FieldElement>>,
    ) {
        unimplemented!()
    }
}

// Implementation of Gmimc hash algorithm
impl<F: Field + PrimeField + GMIMCParameter<F>> GMIMCHASH<F> {
    pub fn gmimic(trace_value: Vec<F>) -> F {
        let mut state = vec![F::ZERO; F::GMIMC_STATE_WIDTH];
        for j in 0..(F::NO_OF_COLUMNS / F::GMIMC_RATE) {
            for i in 0..(F::GMIMC_RATE) {
                state[i] += trace_value[i + (j * F::GMIMC_RATE)];
            }
            for k in 0..F::GMIMC_NO_OF_ROUNDS {
                gmimc_apply_round::<F>(&mut state, k);
            }
        }

        for i in 0..(F::NO_OF_COLUMNS % F::GMIMC_RATE) {
            state[i] += trace_value[i + F::NO_OF_COLUMNS - F::GMIMC_RATE + 1];
        }
        for k in 0..F::GMIMC_NO_OF_ROUNDS {
            gmimc_apply_round::<F>(&mut state, k);
        }

        state[0]
    }
}

// HELPER FUNCTIONS
// ================================================================================================
pub fn gmimc_apply_round<F: Field + PrimeField + GMIMCParameter<F>>(
    state: &mut Vec<F>,
    round_number: usize,
) {
    let stored_value = state[0];
    let result = (state[0] * state[0] * state[0]) + F::gmimc_constants()[round_number];
    for i in 0..F::GMIMC_STATE_WIDTH - 1 {
        state[i] = state[i + 1] + result;
    }
    state[F::GMIMC_STATE_WIDTH - 1] = stored_value;
}
