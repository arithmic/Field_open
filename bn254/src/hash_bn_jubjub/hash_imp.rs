use super::{
    poseidon_bn_jubjub::{
        internal_mds_4_2, internal_mds_6_5, mds_4_2, mds_6_5, round_constants_4_2,
        round_constants_6_5,
    },
    rescue_bn_jubjub::{rescue_ark, rescue_inv_mds, rescue_mds},
};

use crate::scalar::Scalar as Fp;
use crypto_bigint::U256;
use traits::traits::{Field, GMIMCParameter, PoseidonParameter, RescueParameter, PrimeField};

// Implementation of PoseidonParameter trait  for babyjubjub field
impl<F: Field+ PrimeField> PoseidonParameter<F> for Fp {
    const STATE_WIDTH_4_2: usize = 4;

    const STATE_WIDTH_6_5: usize = 6;

    const ALPHA: u64 = 5;

    const RATE_4_2: usize = 2;

    const RATE_6_5: usize = 5;

    const TOTAL_NUM_ROUNDS_4_2: usize = 64;

    const TOTAL_NUM_ROUNDS_6_5: usize = 65;
    
    const ROUNDS_F_BEGINNING: usize = 4;

    const PARTIAL_ROUNDS_4_2: usize = 56;

    const PARTIAL_ROUNDS_6_5: usize = 57;

    const TOTAL_FULL_ROUNDS: usize = 8;

    fn mds_4_2() -> Vec<F> {
        mds_4_2::<F>().to_vec()
    }

    fn mds_6_5() -> Vec<F> {
        mds_6_5::<F>().to_vec()
    }

    fn round_constants_4_2() -> Vec<F> {
        round_constants_4_2::<F>().to_vec()
    }

    fn round_constants_6_5() -> Vec<F> {
        round_constants_6_5::<F>().to_vec()
    }

    fn internal_mds_4_2() -> Vec<F> {
        internal_mds_4_2::<F>().to_vec()
    }

    fn internal_mds_6_5() -> Vec<F> {
        internal_mds_6_5::<F>().to_vec()
    }
}

// Implementation of RescueParameter trait  for babyjubjub field
impl<F: Field+ PrimeField> RescueParameter<F> for Fp {
    const RESCUE_STATE_WIDTH: usize = 4;

    const RESCUE_ALPHA: u64 = 5;

    const RESCUE_RATE: usize = 2;
    const RESCUE_NO_OF_ROUNDS: usize = 15;
    const RESCUE_INV_ALPHA: U256 =
        U256::from_be_hex("26b6a528b427b35493736af8679aad17535cb9d394945a0dcfe7f7a98ccccccd");

    fn rescue_mds() -> Vec<F> {
        rescue_mds::<F>().to_vec()
    }

    fn rescue_inv_mds() -> Vec<F> {
        rescue_inv_mds::<F>().to_vec()
    }

    fn rescue_ark() -> Vec<Vec<F>> {
        rescue_ark::<F>().to_vec()
    }
}

// Implementation of GMIMCParameter trait  for babyjubjub field
impl<F: Field+ PrimeField> GMIMCParameter<F> for Fp {
    const GMIMC_RATE: usize = 5;
    const GMIMC_NO_OF_ROUNDS: usize = 255;
    const GMIMC_STATE_WIDTH: usize = 6;
    const NO_OF_COLUMNS: usize = 68;
    fn gmimc_constants() -> Vec<F> {
        super::gmimc::gmimc_constants::<F>().to_vec()
    }
}
