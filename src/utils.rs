use substrate_bn::{arith::U256, Fq, Fr};

use crate::macros::u256;
pub(crate) trait FrModule {
    const _RFIELD: U256 = u256!("30644e72e131a029b85045b68181585d2833e84879b9709143e1f593f0000001");

    fn fr_module(self) -> U256;
}

impl FrModule for U256 {
    fn fr_module(mut self) -> U256 {
        while self >= Self::_RFIELD {
            self.add(&U256::zero(), &Self::_RFIELD);
        }
        self
    }
}

pub(crate) trait IntoFq {
    fn into_fq(self) -> Fq;
}

impl IntoFq for u64 {
    fn into_fq(self) -> Fq {
        Fq::from_u256(U256::from(self)).expect("BUG: u64 is always a member of Fq")
    }
}

impl IntoFq for Fr {
    fn into_fq(self) -> Fq {
        Fq::from_u256(self.into_u256()).expect("BUG: F is always a member of Fq")
    }
}

pub(crate) trait IntoFr {
    fn into_fr(self) -> Fr;
}

impl IntoFr for &[u8; 32] {
    fn into_fr(self) -> Fr {
        Fr::from_slice(self).expect("BUG: should be hardcoded")
    }
}
impl IntoFr for U256 {
    fn into_fr(self) -> Fr {
        self.into_bytes().into_fr()
    }
}

impl IntoFr for u64 {
    fn into_fr(self) -> Fr {
        U256::from(self).into_fr()
    }
}

pub(crate) trait IntoU256 {
    fn into_u256(self) -> U256;
}

impl IntoU256 for &[u8; 32] {
    fn into_u256(self) -> U256 {
        U256::from_slice(self).expect("BUG: should be hardcoded")
    }
}

impl IntoU256 for [u8; 32] {
    fn into_u256(self) -> U256 {
        (&self).into_u256()
    }
}

pub(crate) trait IntoBytes {
    fn into_bytes(self) -> [u8; 32];
}

impl IntoBytes for U256 {
    fn into_bytes(self) -> [u8; 32] {
        let mut out = [0; 32];
        self.to_big_endian(&mut out)
            .expect("BUG: should be hardcoded");
        out
    }
}

impl IntoBytes for Fr {
    fn into_bytes(self) -> [u8; 32] {
        let mut out = [0; 32];
        self.to_big_endian(&mut out)
            .expect("BUG: should be hardcoded");
        out
    }
}
