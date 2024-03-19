use super::{
    poseidonhash::{
        internal_mds_4_2, internal_mds_6_5, mds_4_2, mds_6_5, round_constants_4_2,
        round_constants_6_5,
    },
    stark_rescue::{rescue_ark, rescue_inv_mds, rescue_mds},
};
use crate::field::Fp;
use crypto_bigint::U256;
use traits::traits::{Field, GMIMCParameter, PoseidonParameter, RescueParameter, PrimeField};

// Implementation of PoseidonParameter trait  for stark field
impl<F: Field + PrimeField> PoseidonParameter<F> for Fp {
    const STATE_WIDTH_4_2: usize = 4;

    const STATE_WIDTH_6_5: usize = 6;

    const ALPHA: u64 = 3;

    const RATE_4_2: usize = 2;

    const RATE_6_5: usize = 5;

    

    const ROUNDS_F_BEGINNING: usize = 4;

    const PARTIAL_ROUNDS_4_2: usize = 84;
    const PARTIAL_ROUNDS_6_5: usize = 84;
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

    const TOTAL_NUM_ROUNDS_4_2: usize = 92;

    const TOTAL_NUM_ROUNDS_6_5: usize = 92;
}

// Implementation of RescueParameter trait  for stark field
impl<F: Field + PrimeField> RescueParameter<F> for Fp {
    const RESCUE_STATE_WIDTH: usize = 4;

    const RESCUE_ALPHA: u64 = 3;

    const RESCUE_RATE: usize = 2;
    const RESCUE_NO_OF_ROUNDS: usize = 15;
    const RESCUE_INV_ALPHA: U256 =
        U256::from_be_hex("0555555555555560aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaab");

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


// Implementation of GMIMCParameter trait  for stark field
impl<F: Field+ PrimeField> GMIMCParameter<F> for Fp {
    const GMIMC_RATE: usize = 5;
    const GMIMC_NO_OF_ROUNDS: usize = 255;
    const GMIMC_STATE_WIDTH: usize = 6;
    const NO_OF_COLUMNS: usize = 69;
    fn gmimc_constants() -> Vec<F> {
        super::gmimc::gmimc_constants::<F>().to_vec()
    }
}
