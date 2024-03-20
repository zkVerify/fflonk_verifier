use substrate_bn::{arith::U256, Fq, Fq2, Fr, G1, G2};

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

impl From<VerificationKey> for AugmentedVerificationKey {
    fn from(vk: VerificationKey) -> Self {
        let w3 = [vk.w3, vk.w3 * vk.w3];
        let w4_2 = vk.w4 * vk.w4;
        let w4 = [vk.w4, w4_2, vk.w4 * w4_2];
        let mut w8: [Fr; 7] = [Fr::zero(); 7];
        w8[0] = vk.w8;
        for i in 1..7 {
            w8[i] = w8[i - 1] * vk.w8;
        }
        Self {
            n: 2.into_fr().pow((vk.power as u64).into_fr()),
            k1: (vk.k1 as u64).into_fr(),
            k2: (vk.k2 as u64).into_fr(),
            w: vk.w,
            w3,
            w4,
            w8,
            wr: vk.wr,
            x2: vk.x2,
            c0: vk.c0,
        }
    }
}

#[derive(PartialEq, Eq, Debug)]
pub struct AugmentedVerificationKey {
    pub n: Fr,
    pub k1: Fr,
    pub k2: Fr,
    pub w: Fr,
    pub w3: [Fr; 2],
    pub w4: [Fr; 3],
    pub w8: [Fr; 7],
    pub wr: Fr,
    pub x2: G2,
    pub c0: G1,
}

#[cfg(test)]
mod should {
    use super::*;

    #[test]
    fn compute_default_augmented_vk_from_default_vk() {
        assert_eq!(
            AugmentedVerificationKey::default(),
            VerificationKey::default().into()
        )
    }
}

impl Default for AugmentedVerificationKey {
    fn default() -> Self {
        const N: U256 = u256!("0000000000000000000000000000000000000000000000000000000001000000"); // 2^24 = 16777216

        Self {
            n: N.into_fr(),
            k1: 2.into_fr(),
            k2: 3.into_fr(),
            w: u256!("0c9fabc7845d50d2852e2a0371c6441f145e0db82e8326961c25f1e3e32b045b").into_fr(),
            w3: [
                u256!("30644e72e131a029048b6e193fd84104cc37a73fec2bc5e9b8ca0b2d36636f23").into_fr(),
                u256!("0000000000000000b3c4d79d41a917585bfc41088d8daaa78b17ea66b99c90dd").into_fr(),
            ],
            w4: [
                u256!("30644e72e131a029048b6e193fd841045cea24f6fd736bec231204708f703636").into_fr(),
                u256!("30644e72e131a029b85045b68181585d2833e84879b9709143e1f593f0000000").into_fr(),
                u256!("0000000000000000b3c4d79d41a91758cb49c3517c4604a520cff123608fc9cb").into_fr(),
            ],
            w8: [
                u256!("2b337de1c8c14f22ec9b9e2f96afef3652627366f8170a0a948dad4ac1bd5e80").into_fr(),
                u256!("30644e72e131a029048b6e193fd841045cea24f6fd736bec231204708f703636").into_fr(),
                u256!("1d59376149b959ccbd157ac850893a6f07c2d99b3852513ab8d01be8e846a566").into_fr(),
                u256!("30644e72e131a029b85045b68181585d2833e84879b9709143e1f593f0000000").into_fr(),
                u256!("0530d09118705106cbb4a786ead16926d5d174e181a26686af5448492e42a181").into_fr(),
                u256!("0000000000000000b3c4d79d41a91758cb49c3517c4604a520cff123608fc9cb").into_fr(),
                u256!("130b17119778465cfb3acaee30f81dee20710ead41671f568b11d9ab07b95a9b").into_fr(),
            ],
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
