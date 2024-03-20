use substrate_bn::{Fq, Fq2, Fr, G1, G2};

use crate::{macros::u256, utils::IntoFr};

pub struct VerificationKey {
    pub power: u8,
    pub k1: u8,
    pub k2: u8,
    pub w: Fr,
    pub w3: Fr,
    pub w4: Fr,
    pub w8: Fr,
    pub wr: Fr,
    pub x2: G2,
    pub c0: G1,
}

impl Default for VerificationKey {
    fn default() -> Self {
        Self {
            power: 24,
            k1: 2,
            k2: 3,
            w: u256!("0c9fabc7845d50d2852e2a0371c6441f145e0db82e8326961c25f1e3e32b045b").into_fr(),
            w3: u256!("30644e72e131a029048b6e193fd84104cc37a73fec2bc5e9b8ca0b2d36636f23").into_fr(),
            w4: u256!("30644e72e131a029048b6e193fd841045cea24f6fd736bec231204708f703636").into_fr(),
            w8: u256!("2b337de1c8c14f22ec9b9e2f96afef3652627366f8170a0a948dad4ac1bd5e80").into_fr(),
            wr: u256!("283ce45a2e5b8e4e78f9fbaf5f6a348bfcfaf76dd28e5ca7121b74ef68fdec2e").into_fr(),
            x2: {
                let x2x1 = Fq::from_u256(u256!(
                    "30441fd1b5d3370482c42152a8899027716989a6996c2535bc9f7fee8aaef79e"
                ))
                .expect("X2x1 should be a valid Fq point");
                let x2x2 = Fq::from_u256(u256!(
                    "26186a2d65ee4d2f9c9a5b91f86597d35f192cd120caf7e935d8443d1938e23d"
                ))
                .expect("X2x2 should be a valid Fq point");
                let x2y1 = Fq::from_u256(u256!(
                    "054793348f12c0cf5622c340573cb277586319de359ab9389778f689786b1e48"
                ))
                .expect("X2y1 should be a valid Fq point");
                let x2y2 = Fq::from_u256(u256!(
                    "1970ea81dd6992adfbc571effb03503adbbb6a857f578403c6c40e22d65b3c02"
                ))
                .expect("X2y2 should be a valid Fq point");
                G2::new(Fq2::new(x2x1, x2x2), Fq2::new(x2y1, x2y2), Fq2::one())
            },
            c0: {
                let x = Fq::from_u256(u256!(
                    "10711a639fed66ba6cd6001188b8fe7285cb9bd01afc1f90598223550aa57e36"
                ))
                .expect("C0x should be a valid Fq point");
                let y = Fq::from_u256(u256!(
                    "28c937a4cb758326763015d30fff3568f5cbed932cdc7c411a435d3de04549ef"
                ))
                .expect("C0y should be a valid Fq point");
                G1::new(x, y, Fq::one())
            },
        }
    }
}
