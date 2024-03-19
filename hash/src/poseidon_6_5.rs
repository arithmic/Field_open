use traits::traits::{Field, PoseidonParameter, PrimeField};

// Implementation of Poseidon hash algorithm
pub fn poseidon2<F: Field + PrimeField+ PoseidonParameter<F>>(value: &mut Vec<F>) -> F {
    let mut state = vec![F::ZERO; F::STATE_WIDTH_6_5];
    if value.len() % F::RATE_6_5 != 0 {
        for _ in 0..(F::RATE_6_5 - value.len() % F::RATE_6_5) {
            value.append(&mut [F::ZERO].to_vec());
        }
    }
    let mut i = 0;
    while i < value.len() {
        for j in 0..F::RATE_6_5 {
            state[j] += value[i + j];
        }
        // Linear layer at beginning
        apply_mds(&mut state);
        for round in 0..F::ROUNDS_F_BEGINNING {
            add_constants(&mut state, round * F::STATE_WIDTH_6_5);
            // full round
            apply_sbox(&mut state);
            //  MDS for full round
            apply_mds(&mut state);
        }
        for round in F::ROUNDS_F_BEGINNING..F::ROUNDS_F_BEGINNING + F::PARTIAL_ROUNDS_6_5 {
            add_constants(&mut state, round * F::STATE_WIDTH_6_5);
            // partial round
            if F::ALPHA == 3 {
                state[F::STATE_WIDTH_6_5 - 1] =
                    state[F::STATE_WIDTH_6_5 - 1].square() * state[F::STATE_WIDTH_6_5 - 1];
            } else if F::ALPHA == 5 {
                state[F::STATE_WIDTH_6_5 - 1] =
                    state[F::STATE_WIDTH_6_5 - 1].square().square() * state[F::STATE_WIDTH_6_5 - 1];
            } else {
                state[F::STATE_WIDTH_6_5 - 1] =
                    state[F::STATE_WIDTH_6_5 - 1].power_by([F::ALPHA, 0, 0, 0]);
            }
            // MDS for partial round
            apply_internal_mds(&mut state);
        }

        for round in F::ROUNDS_F_BEGINNING + F::PARTIAL_ROUNDS_6_5..F::TOTAL_NUM_ROUNDS_6_5 {
            add_constants(&mut state, round * F::STATE_WIDTH_6_5);
            // full round
            apply_sbox(&mut state);
            //  MDS for full round
            apply_mds(&mut state);
        }
        i += F::RATE_6_5;
    }
    state[0]
}

// HELPER FUNCTIONS
// ================================================================================================
#[inline(always)]
#[allow(clippy::needless_range_loop)]
fn add_constants<F: Field + PrimeField+ PoseidonParameter<F>>(state: &mut [F], offset: usize) {
    for i in 0..F::STATE_WIDTH_6_5 {
        state[i] += F::round_constants_6_5()[offset + i];
    }
}

#[inline(always)]
#[allow(clippy::needless_range_loop)]
pub fn apply_sbox<F: Field + PrimeField+ PoseidonParameter<F>>(state: &mut [F]) {
    for i in 0..F::STATE_WIDTH_6_5 {
        if F::ALPHA == 3 {
            state[i] = state[i].square() * state[i];
        } else if F::ALPHA == 5 {
            state[i] = state[i].square().square() * state[i];
        } else {
            state[i] = state[i].power_by([F::ALPHA, 0, 0, 0]);
        }
    }
}

#[inline(always)]
#[allow(clippy::needless_range_loop)]
pub fn apply_mds<F: Field + PrimeField+ PoseidonParameter<F>>(state: &mut [F]) {
    let mut result = vec![F::ZERO; F::STATE_WIDTH_6_5];
    for i in 0..F::STATE_WIDTH_6_5 {
        for j in 0..F::STATE_WIDTH_6_5 {
            result[i] = result[i] + F::mds_6_5()[i * F::STATE_WIDTH_6_5 + j] * state[j];
        }
    }
    state.copy_from_slice(&result);
}

#[inline(always)]
#[allow(clippy::needless_range_loop)]
pub fn apply_internal_mds<F: Field + PrimeField+ PoseidonParameter<F>>(state: &mut [F]) {
    let mut result = vec![F::ZERO; F::STATE_WIDTH_6_5];
    for i in 0..F::STATE_WIDTH_6_5 {
        for j in 0..F::STATE_WIDTH_6_5 {
            result[i] = result[i] + F::internal_mds_6_5()[i * F::STATE_WIDTH_6_5 + j] * state[j];
        }
    }
    state.copy_from_slice(&result);
}
