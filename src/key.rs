use substrate_bn::{Fq, Fq2, Fr, G1, G2};

use crate::{macros::u256, utils::IntoFr};

#[derive(PartialEq, Eq, Debug)]
#[cfg_attr(feature = "serde", derive(::serde::Serialize, ::serde::Deserialize))]
pub struct VerificationKey {
    pub power: u8,
    #[cfg_attr(feature = "serde", serde(with = "self::serde::fr"))]
    pub k1: Fr,
    #[cfg_attr(feature = "serde", serde(with = "self::serde::fr"))]
    pub k2: Fr,
    #[cfg_attr(feature = "serde", serde(with = "self::serde::fr"))]
    pub w: Fr,
    #[cfg_attr(feature = "serde", serde(with = "self::serde::fr"))]
    pub w3: Fr,
    #[cfg_attr(feature = "serde", serde(with = "self::serde::fr"))]
    pub w4: Fr,
    #[cfg_attr(feature = "serde", serde(with = "self::serde::fr"))]
    pub w8: Fr,
    #[cfg_attr(feature = "serde", serde(with = "self::serde::fr"))]
    pub wr: Fr,
    #[cfg_attr(feature = "serde", serde(with = "self::serde::g2", rename = "X_2"))]
    pub x2: G2,
    #[cfg_attr(feature = "serde", serde(with = "self::serde::g1", rename = "C0"))]
    pub c0: G1,
}

#[cfg(feature = "serde")]
mod serde {
    pub mod fr {
        use substrate_bn::{arith::U256, Fr};

        use crate::utils::{IntoBytes, IntoFr};

        pub fn serialize<S>(fr: &Fr, s: S) -> Result<S::Ok, S::Error>
        where
            S: serde::Serializer,
        {
            let u256 = fr.into_u256();
            if s.is_human_readable() {
                ethnum::serde::decimal::serialize(
                    &ethnum::U256::from_words(u256.0[1], u256.0[0]),
                    s,
                )
            } else {
                s.serialize_bytes(&u256.into_bytes())
            }
        }

        pub fn deserialize<'de, D>(data: D) -> Result<Fr, D::Error>
        where
            D: serde::Deserializer<'de>,
        {
            if data.is_human_readable() {
                let u256: ethnum::U256 = ethnum::serde::decimal::deserialize(data)?;
                Ok(U256([u256.0[0], u256.0[1]]).into_fr())
            } else {
                <[u8; 32] as serde::Deserialize>::deserialize(data).map(IntoFr::into_fr)
            }
        }
    }

    mod fq {
        use substrate_bn::{arith::U256, Fq};

        use crate::utils::{IntoBytes, IntoU256};

        pub fn serialize<S>(fq: &Fq, s: S) -> Result<S::Ok, S::Error>
        where
            S: serde::Serializer,
        {
            let u256 = fq.into_u256();
            if s.is_human_readable() {
                ethnum::serde::decimal::serialize(
                    &ethnum::U256::from_words(u256.0[1], u256.0[0]),
                    s,
                )
            } else {
                s.serialize_bytes(&u256.into_bytes())
            }
        }

        pub fn deserialize<'de, D>(data: D) -> Result<Fq, D::Error>
        where
            D: serde::Deserializer<'de>,
        {
            let u256 = if data.is_human_readable() {
                let u256: ethnum::U256 = ethnum::serde::decimal::deserialize(data)?;
                U256([u256.0[0], u256.0[1]])
            } else {
                <[u8; 32] as serde::Deserialize>::deserialize(data).map(IntoU256::into_u256)?
            };
            Fq::from_u256(u256).map_err(|_e| serde::de::Error::custom("Invalid Fq value"))
        }
    }

    mod fq2 {
        use serde::{Deserialize, Serialize};
        use substrate_bn::{Fq, Fq2};

        #[derive(Serialize, Deserialize)]
        struct Fq2Serde(
            #[serde(with = "super::fq")] Fq,
            #[serde(with = "super::fq")] Fq,
        );

        pub fn serialize<S>(fq2: &Fq2, s: S) -> Result<S::Ok, S::Error>
        where
            S: serde::Serializer,
        {
            Fq2Serde(fq2.real(), fq2.imaginary()).serialize(s)
        }

        pub fn deserialize<'de, D>(data: D) -> Result<Fq2, D::Error>
        where
            D: serde::Deserializer<'de>,
        {
            let fq2 = Fq2Serde::deserialize(data)?;
            Ok(Fq2::new(fq2.0, fq2.1))
        }
    }

    pub mod g2 {
        use serde::{Deserialize, Serialize};
        use substrate_bn::{AffineG2, Fq2, Group, G2};

        #[derive(Serialize, Deserialize)]
        struct G2Serde(
            #[serde(with = "super::fq2")] Fq2,
            #[serde(with = "super::fq2")] Fq2,
            #[serde(with = "super::fq2")] Fq2,
        );

        pub fn serialize<S>(g2: &G2, s: S) -> Result<S::Ok, S::Error>
        where
            S: serde::Serializer,
        {
            G2Serde(g2.x(), g2.y(), g2.z()).serialize(s)
        }

        fn check_point<'de, D>(point: &G2) -> Result<(), D::Error>
        where
            D: serde::Deserializer<'de>,
        {
            let mut check = point.clone();
            check.normalize();
            AffineG2::new(check.x(), check.y())
                .map_err(|_e| serde::de::Error::custom("Invalid G2 point"))?;
            Ok(())
        }

        pub fn deserialize<'de, D>(data: D) -> Result<G2, D::Error>
        where
            D: serde::Deserializer<'de>,
        {
            let g2 = G2Serde::deserialize(data)?;
            let candidate = G2::new(g2.0, g2.1, g2.2);
            check_point::<D>(&candidate)?;
            Ok(candidate)
        }
    }

    pub mod g1 {
        use serde::{Deserialize, Serialize};
        use substrate_bn::{AffineG1, Fq, Group, G1};

        #[derive(Serialize, Deserialize)]
        struct G1Serde(
            #[serde(with = "super::fq")] Fq,
            #[serde(with = "super::fq")] Fq,
            #[serde(with = "super::fq")] Fq,
        );

        pub fn serialize<S>(g1: &G1, s: S) -> Result<S::Ok, S::Error>
        where
            S: serde::Serializer,
        {
            G1Serde(g1.x(), g1.y(), g1.z()).serialize(s)
        }

        fn check_point<'de, D>(point: &G1) -> Result<(), D::Error>
        where
            D: serde::Deserializer<'de>,
        {
            let mut check = point.clone();
            check.normalize();
            AffineG1::new(check.x(), check.y())
                .map_err(|_e| serde::de::Error::custom("Invalid G1 point"))?;
            Ok(())
        }

        pub fn deserialize<'de, D>(data: D) -> Result<G1, D::Error>
        where
            D: serde::Deserializer<'de>,
        {
            let g1 = G1Serde::deserialize(data)?;
            let candidate = G1::new(g1.0, g1.1, g1.2);
            check_point::<D>(&candidate)?;
            Ok(candidate)
        }
    }

    #[cfg(test)]
    mod should {
        use ::serde::Deserialize;
        use pretty_assertions::assert_eq;

        use super::super::*;

        // Just because `json!` macro need `vec!` macro.
        #[cfg(feature = "std")]
        #[test]
        fn serialize_the_valid_json() {
            let vk = VerificationKey::default();

            let serialized = serde_json::to_string(&vk).unwrap();

            let v: serde_json::Value = serde_json::from_str(&serialized).unwrap();
            let expected = serde_json::json!({
            "power": 24,
            "k1": "2",
            "k2": "3",
            "w": "5709868443893258075976348696661355716898495876243883251619397131511003808859",
            "w3": "21888242871839275217838484774961031246154997185409878258781734729429964517155",
            "w4": "21888242871839275217838484774961031246007050428528088939761107053157389710902",
            "w8": "19540430494807482326159819597004422086093766032135589407132600596362845576832",
            "wr": "18200100796661656210024324131237448517259556535315737226009542456080026430510",
            "X_2": [
            [
            "21831381940315734285607113342023901060522397560371972897001948545212302161822",
            "17231025384763736816414546592865244497437017442647097510447326538965263639101"
            ],
            [
            "2388026358213174446665280700919698872609886601280537296205114254867301080648",
            "11507326595632554467052522095592665270651932854513688777769618397986436103170"
            ],
            [
            "1",
            "0"
            ]
            ],
            "C0": [
            "7436841426934271843999872946312645822871802402068881571108027575346498207286",
            "18448034242258174646222819724328439025708531082946938915005051387020977719791",
            "1"
            ]
            });
            assert_eq!(expected, v);
        }

        #[test]
        fn deserialize_the_verification_key_json() {
            let json = r#"
        {
            "protocol": "fflonk",
            "curve": "bn128",
            "nPublic": 1,
            "power": 24,
            "k1": "2",
            "k2": "3",
            "w": "5709868443893258075976348696661355716898495876243883251619397131511003808859",
            "w3": "21888242871839275217838484774961031246154997185409878258781734729429964517155",
            "w4": "21888242871839275217838484774961031246007050428528088939761107053157389710902",
            "w8": "19540430494807482326159819597004422086093766032135589407132600596362845576832",
            "wr": "18200100796661656210024324131237448517259556535315737226009542456080026430510",
            "X_2": [
            [
            "21831381940315734285607113342023901060522397560371972897001948545212302161822",
            "17231025384763736816414546592865244497437017442647097510447326538965263639101"
            ],
            [
            "2388026358213174446665280700919698872609886601280537296205114254867301080648",
            "11507326595632554467052522095592665270651932854513688777769618397986436103170"
            ],
            [
            "1",
            "0"
            ]
            ],
            "C0": [
            "7436841426934271843999872946312645822871802402068881571108027575346498207286",
            "18448034242258174646222819724328439025708531082946938915005051387020977719791",
            "1"
            ]
        }
        "#;
            let vk: VerificationKey = serde_json::from_str(json).unwrap();

            assert_eq!(VerificationKey::default(), vk);
        }

        #[test]
        fn serialize_deserialize_default_key() {
            let vk = VerificationKey::default();
            let json = serde_json::to_string(&vk).unwrap();
            let other = serde_json::from_str(&json).unwrap();

            assert_eq!(vk, other);
        }

        #[test]
        fn serialize_deserialize_in_a_non_human_readable_format() {
            let vk = VerificationKey::default();
            let mut buffer = [0_u8; 600];
            ciborium::into_writer(&vk, buffer.as_mut_slice()).unwrap();
            let other = ciborium::from_reader(buffer.as_slice()).unwrap();
            assert_eq!(vk, other);
        }

        #[test]
        #[should_panic(expected = "Invalid G1 point")]
        fn raise_error_if_try_to_deserialize_an_invalid_g1_point() {
            let json = r#"["1", "2", "3"]"#;
            #[derive(Deserialize)]
            #[allow(dead_code)]
            struct Test(#[cfg_attr(feature = "serde", serde(with = "super::g1"))] G1);

            serde_json::from_str::<Test>(json).unwrap();
        }

        #[test]
        #[should_panic(expected = "Invalid G2 point")]
        fn raise_error_if_try_to_deserialize_an_invalid_g2_point() {
            let json = r#"[["1", "2"], ["3", "4"], ["5", "6"]]"#;
            #[derive(Deserialize)]
            #[allow(dead_code)]
            struct Test(#[cfg_attr(feature = "serde", serde(with = "super::g2"))] G2);

            serde_json::from_str::<Test>(json).unwrap();
        }
    }
}

impl Default for VerificationKey {
    fn default() -> Self {
        Self {
            power: 24,
            k1: substrate_bn::arith::U256::from(2).into_fr(),
            k2: substrate_bn::arith::U256::from(3).into_fr(),
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
