// Copyright 2024, The Horizen Foundation
// Copyright 2021 0KIMS association.
//
// fflonk_verifier is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.
//
// fflonk_verifier is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU General Public License for more details.
//
// You should have received a copy of the GNU General Public License
// along with fflonk_verifier.  If not, see <http://www.gnu.org/licenses/>.

#![cfg_attr(not(feature = "std"), no_std)]
#![doc = include_str!("../README.md")]

use macros::u256;
use snafu::Snafu;
use substrate_bn::{arith::U256, pairing_batch, AffineG1, AffineG2, Fq, Fq2, Fr, Gt, G1, G2};

use hash::Hasher as _;
use utils::IntoFq as _;
use utils::IntoFr as _;

pub(crate) mod hash;
mod macros;
mod proof_input;
pub(crate) mod utils;

/// The proof data as `U256` fixed array.
pub use proof_input::ProofData;
/// The proof data as fixed size bytes array.
pub use proof_input::ProofRawData;

/// The public input data.
pub struct Public(U256);

impl From<U256> for Public {
    fn from(inner: U256) -> Self {
        Self(inner)
    }
}

impl From<[u8; 32]> for Public {
    fn from(inner: [u8; 32]) -> Self {
        Self(U256::from_slice(&inner).expect("Cannot fails: is a 32-byte array"))
    }
}

impl TryFrom<&[u8]> for Public {
    type Error = core::array::TryFromSliceError;

    fn try_from(inner: &[u8]) -> Result<Self, Self::Error> {
        <[u8; 32]>::try_from(inner).map(Into::into)
    }
}

#[derive(Debug)]
enum ProofFields {
    C1,
    C2,
    W1,
    W2,
    Ql,
    Qr,
    Qm,
    Qo,
    Qc,
    S1,
    S2,
    S3,
    A,
    B,
    C,
    Z,
    Zw,
    T1w,
    T2w,
    Inv,
}

/// The Proof data: use the implemented conversion traits `TryFrom` to build it.
pub struct Proof {
    pub c1: G1,
    pub c2: G1,
    pub w1: G1,
    pub w2: G1,

    pub ql: Fr,
    pub qr: Fr,
    pub qm: Fr,
    pub qo: Fr,
    pub qc: Fr,
    pub s1: Fr,
    pub s2: Fr,
    pub s3: Fr,
    pub a: Fr,
    pub b: Fr,
    pub c: Fr,
    pub z: Fr,
    pub zw: Fr,
    pub t1w: Fr,
    pub t2w: Fr,
    pub inv: Fr,
}

impl Proof {
    /// Verifies the proof against the given public inputs. Can fail if:
    /// - the provided inverse in the proof is wrong
    /// - the pair checking is wrong
    pub fn verify(&self, pubs: Public) -> Result<(), VerifyError> {
        let challenges: Challenges = (self, &pubs).into();
        let (inverse, l1) = challenges.compute_inverse(self.inv)?;
        let pi = Self::compute_pi(&pubs, l1);
        let r0 = self.compute_r0(&challenges, &inverse.li_s0_inv);
        let r1 = self.compute_r1(&challenges, pi, inverse.zh_inv, &inverse.li_s1_inv);
        let r2 = self.compute_r2(&challenges, l1, inverse.zh_inv, &inverse.li_s2_inv);

        let (f, e, j) = self.compute_fej(&challenges, r0, r1, r2, inverse.den_h1, inverse.den_h2);

        self.check_paring(&challenges, f, e, j)
    }

    /// Compute public input polynomial evaluation PI(xi)
    fn compute_pi(p: &Public, l1: Fr) -> Fr {
        -l1 * p.0.into_fr()
    }

    /// Compute r0(y) by interpolating the polynomial r0(X) using 8 points (x,y)
    /// where x = {h9, h0w8, h0w8^2, h0w8^3, h0w8^4, h0w8^5, h0w8^6, h0w8^7}
    /// and   y = {C0(h0), C0(h0w8), C0(h0w8^2), C0(h0w8^3), C0(h0w8^4), C0(h0w8^5), C0(h0w8^6), C0(h0w8^7)}
    /// and computing C0(xi)
    fn compute_r0(&self, challenges: &Challenges, li_s0_inv: &LiS0) -> Fr {
        let base = challenges.y.pow(8_u64.into_fr()) - challenges.xi;

        let coefficients = [
            self.ql, self.qr, self.qo, self.qm, self.qc, self.s1, self.s2, self.s3,
        ];

        // Compute c0Value = ql + (h0w8[i]) qr + (h0w8[i])^2 qo + (h0w8[i])^3 qm + (h0w8[i])^4 qc +
        //                      + (h0w8[i])^5 S1 + (h0w8[i])^6 S2 + (h0w8[i])^7 S3
        polynomial_eval(base, &coefficients, &challenges.h0_w8, li_s0_inv, None)
    }

    /// Compute r1(y) by interpolating the polynomial r1(X) using 4 points (x,y)
    /// where x = {h1, h1w4, h1w4^2, h1w4^3}
    /// and   y = {C1(h1), C1(h1w4), C1(h1w4^2), C1(h1w4^3)}
    /// and computing T0(xi)
    fn compute_r1(&self, challenges: &Challenges, pi: Fr, zh_inv: Fr, li_s1_inv: &LiS1) -> Fr {
        let base = challenges.y.pow(4_u64.into_fr()) - challenges.xi;

        let t0 = ((self.ql * self.a)
            + (self.qr * self.b)
            + (self.qm * self.a * self.b)
            + (self.qo * self.c)
            + self.qc
            + pi)
            * zh_inv;
        let coefficients = [self.a, self.b, self.c, t0];

        polynomial_eval(base, &coefficients, &challenges.h1_w4, li_s1_inv, None)
    }

    /// Compute r2(y) by interpolating the polynomial r2(X) using 6 points (x,y)
    /// where x = {[h2, h2w3, h2w3^2], [h3, h3w3, h3w3^2]}
    /// and   y = {[C2(h2), C2(h2w3), C2(h2w3^2)], [CChallenges::C0x.into_fr()2(h3), C2(h3w3), C2(h3w3^2)]}
    /// and computing T1(xi) and T2(xi)
    fn compute_r2(&self, challenges: &Challenges, l1: Fr, zh_inv: Fr, li_s2_inv: &LiS2) -> Fr {
        let base = challenges.y.pow(6_u64.into_fr())
            - (challenges.y.pow(3_u64.into_fr()) * challenges.xi * (Fr::one() + Challenges::w1()))
            + (challenges.xi * challenges.xi * Challenges::w1());

        let beta_xi = challenges.beta * challenges.xi;
        let t1 = (self.z - Fr::one()) * l1 * zh_inv;
        let t2 = (((self.a + beta_xi + challenges.gamma)
            * (self.b + beta_xi * Challenges::k1() + challenges.gamma)
            * (self.c + beta_xi * Challenges::k2() + challenges.gamma)
            * self.z)
            - ((self.a + challenges.beta * self.s1 + challenges.gamma)
                * (self.b + challenges.beta * self.s2 + challenges.gamma)
                * (self.c + challenges.beta * self.s3 + challenges.gamma)
                * self.zw))
            * zh_inv;

        let coefficients = [self.z, t1, t2];
        let gamma = polynomial_eval(
            base,
            &coefficients,
            &challenges.h2_w3,
            &li_s2_inv[..3],
            None,
        );

        let coefficients = [self.zw, self.t1w, self.t2w];
        polynomial_eval(
            base,
            &coefficients,
            &challenges.h3_w3,
            &li_s2_inv[3..],
            Some(gamma),
        )
    }

    fn compute_fej(
        &self,
        challenges: &Challenges,
        r0: Fr,
        r1: Fr,
        r2: Fr,
        den_h1: Fr,
        den_h2: Fr,
    ) -> (G1, G1, G1) {
        let numerator = challenges
            .h0_w8
            .iter()
            .fold(Fr::one(), |acc, h0_w8_i| acc * (challenges.y - *h0_w8_i));
        let quotient1 = challenges.alpha * numerator * den_h1;
        let quotient2 = challenges.alpha * challenges.alpha * numerator * den_h2;
        let f = self.c1 * quotient1 + self.c2 * quotient2 + Challenges::f();
        let e = Challenges::g1() * (r0 + quotient1 * r1 + quotient2 * r2);
        let j = self.w1 * numerator;

        (f, e, j)
    }

    fn check_paring(
        &self,
        challenges: &Challenges,
        f: G1,
        e: G1,
        j: G1,
    ) -> Result<(), VerifyError> {
        let f = f - e - j + self.w2 * challenges.y;
        if pairing_batch(&[
            (f, Challenges::g2_pair()),
            (-self.w2, Challenges::x2_pair()),
        ]) == Gt::one()
        {
            Ok(())
        } else {
            Err(VerifyError::NotPairing)
        }
    }
}

fn polynomial_eval(
    base: Fr,
    coefficients: &[Fr],
    challenges: &[Fr],
    inv: &[Fr],
    acc: Option<Fr>,
) -> Fr {
    let mut acc = acc.unwrap_or(Fr::zero());
    for (i, root) in challenges.iter().enumerate() {
        let mut h = Fr::one();
        let mut c1_value = Fr::zero();
        for c in coefficients {
            c1_value = c1_value + (*c) * h;
            h = h * *root;
        }
        acc = acc + c1_value * base * inv[i];
    }
    acc
}

/// Verification Error
#[derive(Snafu, Debug)]
pub enum VerifyError {
    /// The provided inverse is wrong
    #[snafu(display(
        "Invalid provided inverse is {inverse:?} that's not the inverse of {computed:?}"
    ))]
    InvalidInverse { inverse: Fr, computed: Fr },
    /// Cannot verify the pairing for this proof
    #[snafu(display("Cannot verify paring"))]
    NotPairing,
}

trait Constants {
    const N: U256 = u256!("0000000000000000000000000000000000000000000000000000000001000000"); // 2^24 = 16777216
    fn n() -> Fr {
        Self::N.into_fr()
    }

    // Plonk k1 multiplicative factor to force distinct cosets of H
    fn k1() -> Fr {
        2.into_fr()
    }
    // Plonk k2 multiplicative factor to force distinct cosets of H
    fn k2() -> Fr {
        3.into_fr()
    }

    const C0X: U256 = u256!("10711a639fed66ba6cd6001188b8fe7285cb9bd01afc1f90598223550aa57e36");
    const C0Y: U256 = u256!("28c937a4cb758326763015d30fff3568f5cbed932cdc7c411a435d3de04549ef");
    fn f() -> G1 {
        let x = Fq::from_u256(Self::C0X).expect("C0x should be a valid Fq point");
        let y = Fq::from_u256(Self::C0Y).expect("C0y should be a valid Fq point");
        AffineG1::new(x, y)
            .expect("(C0x, C0y) Should be a valid G1 point")
            .into()
    }

    fn g1() -> G1 {
        AffineG1::new(Fq::one(), 2.into_fq())
            .expect("(1, 2) Should be a valid G1 point")
            .into()
    }

    const G2_X1: U256 = u256!("1800deef121f1e76426a00665e5c4479674322d4f75edadd46debd5cd992f6ed");
    const G2_X2: U256 = u256!("198e9393920d483a7260bfb731fb5d25f1aa493335a9e71297e485b7aef312c2");
    const G2_Y1: U256 = u256!("12c85ea5db8c6deb4aab71808dcb408fe3d1e7690c43d37b4ce6cc0166fa7daa");
    const G2_Y2: U256 = u256!("090689d0585ff075ec9e99ad690c3395bc4b313370b38ef355acdadcd122975b");

    fn g2_pair() -> G2 {
        let g2x1 = Fq::from_u256(Self::G2_X1).expect("G2x1 should be a valid Fq point");
        let g2x2 = Fq::from_u256(Self::G2_X2).expect("G2x2 should be a valid Fq point");
        let g2y1 = Fq::from_u256(Self::G2_Y1).expect("G2y1 should be a valid Fq point");
        let g2y2 = Fq::from_u256(Self::G2_Y2).expect("G2y2 should be a valid Fq point");
        AffineG2::new(Fq2::new(g2x1, g2x2), Fq2::new(g2y1, g2y2))
            .expect("Should be on curve")
            .into()
    }

    const X2_X1: U256 = u256!("30441fd1b5d3370482c42152a8899027716989a6996c2535bc9f7fee8aaef79e");
    const X2_X2: U256 = u256!("26186a2d65ee4d2f9c9a5b91f86597d35f192cd120caf7e935d8443d1938e23d");
    const X2_Y1: U256 = u256!("054793348f12c0cf5622c340573cb277586319de359ab9389778f689786b1e48");
    const X2_Y2: U256 = u256!("1970ea81dd6992adfbc571effb03503adbbb6a857f578403c6c40e22d65b3c02");

    fn x2_pair() -> G2 {
        let x2x1 = Fq::from_u256(Self::X2_X1).expect("X2x1 should be a valid Fq point");
        let x2x2 = Fq::from_u256(Self::X2_X2).expect("X2x2 should be a valid Fq point");
        let x2y1 = Fq::from_u256(Self::X2_Y1).expect("X2y1 should be a valid Fq point");
        let x2y2 = Fq::from_u256(Self::X2_Y2).expect("X2y2 should be a valid Fq point");
        AffineG2::new(Fq2::new(x2x1, x2x2), Fq2::new(x2y1, x2y2))
            .expect("Should be on curve")
            .into()
    }

    const W1: U256 = u256!("0c9fabc7845d50d2852e2a0371c6441f145e0db82e8326961c25f1e3e32b045b");
    fn w1() -> Fr {
        Self::W1.into_fr()
    }
    const WR: U256 = u256!("283ce45a2e5b8e4e78f9fbaf5f6a348bfcfaf76dd28e5ca7121b74ef68fdec2e");
    fn wr() -> Fr {
        Self::WR.into_fr()
    }

    const W3: U256 = u256!("30644e72e131a029048b6e193fd84104cc37a73fec2bc5e9b8ca0b2d36636f23");
    fn w3() -> Fr {
        Self::W3.into_fr()
    }
    const W3_2: U256 = u256!("0000000000000000b3c4d79d41a917585bfc41088d8daaa78b17ea66b99c90dd");
    fn w3_2() -> Fr {
        Self::W3_2.into_fr()
    }

    const W4: U256 = u256!("30644e72e131a029048b6e193fd841045cea24f6fd736bec231204708f703636");
    fn w4() -> Fr {
        Self::W4.into_fr()
    }
    const W4_2: U256 = u256!("30644e72e131a029b85045b68181585d2833e84879b9709143e1f593f0000000");
    fn w4_2() -> Fr {
        Self::W4_2.into_fr()
    }
    const W4_3: U256 = u256!("0000000000000000b3c4d79d41a91758cb49c3517c4604a520cff123608fc9cb");
    fn w4_3() -> Fr {
        Self::W4_3.into_fr()
    }

    const W8_1: U256 = u256!("2b337de1c8c14f22ec9b9e2f96afef3652627366f8170a0a948dad4ac1bd5e80");
    fn w8_1() -> Fr {
        Self::W8_1.into_fr()
    }
    const W8_2: U256 = u256!("30644e72e131a029048b6e193fd841045cea24f6fd736bec231204708f703636");
    fn w8_2() -> Fr {
        Self::W8_2.into_fr()
    }
    const W8_3: U256 = u256!("1d59376149b959ccbd157ac850893a6f07c2d99b3852513ab8d01be8e846a566");
    fn w8_3() -> Fr {
        Self::W8_3.into_fr()
    }
    const W8_4: U256 = u256!("30644e72e131a029b85045b68181585d2833e84879b9709143e1f593f0000000");
    fn w8_4() -> Fr {
        Self::W8_4.into_fr()
    }
    const W8_5: U256 = u256!("0530d09118705106cbb4a786ead16926d5d174e181a26686af5448492e42a181");
    fn w8_5() -> Fr {
        Self::W8_5.into_fr()
    }
    const W8_6: U256 = u256!("0000000000000000b3c4d79d41a91758cb49c3517c4604a520cff123608fc9cb");
    fn w8_6() -> Fr {
        Self::W8_6.into_fr()
    }
    const W8_7: U256 = u256!("130b17119778465cfb3acaee30f81dee20710ead41671f568b11d9ab07b95a9b");
    fn w8_7() -> Fr {
        Self::W8_7.into_fr()
    }
}

#[derive(Debug)]
struct Challenges {
    beta: Fr,
    gamma: Fr,
    h0_w8: [Fr; 8],
    h1_w4: [Fr; 4],
    h2_w3: [Fr; 3],
    h3_w3: [Fr; 3],
    xi: Fr,
    zh: Fr,
    alpha: Fr,
    y: Fr,
}

type LiS0 = [Fr; 8];
type LiS1 = [Fr; 4];
type LiS2 = [Fr; 6];

#[derive(Debug, PartialEq)]
struct Inverse {
    li_s0_inv: LiS0,
    li_s1_inv: LiS1,
    li_s2_inv: LiS2,
    den_h1: Fr,
    den_h2: Fr,
    zh_inv: Fr,
}

impl Challenges {
    fn compute_li_s0(&self) -> LiS0 {
        let den1 = self.h0_w8[0].pow(6_u64.into_fr()) * 8_u64.into_fr();
        [
            den1 * self.h0_w8[0] * (self.y - self.h0_w8[0]),
            den1 * self.h0_w8[7] * (self.y - self.h0_w8[1]),
            den1 * self.h0_w8[6] * (self.y - self.h0_w8[2]),
            den1 * self.h0_w8[5] * (self.y - self.h0_w8[3]),
            den1 * self.h0_w8[4] * (self.y - self.h0_w8[4]),
            den1 * self.h0_w8[3] * (self.y - self.h0_w8[5]),
            den1 * self.h0_w8[2] * (self.y - self.h0_w8[6]),
            den1 * self.h0_w8[1] * (self.y - self.h0_w8[7]),
        ]
    }

    fn compute_li_s1(&self) -> LiS1 {
        let den1 = self.h1_w4[0] * self.h1_w4[0] * 4_u64.into_fr();
        [
            den1 * self.h1_w4[0] * (self.y - self.h1_w4[0]),
            den1 * self.h1_w4[3] * (self.y - self.h1_w4[1]),
            den1 * self.h1_w4[2] * (self.y - self.h1_w4[2]),
            den1 * self.h1_w4[1] * (self.y - self.h1_w4[3]),
        ]
    }

    fn compute_li_s2(&self) -> LiS2 {
        let den1_0 = 3_u64.into_fr() * self.h2_w3[0] * (self.xi - self.xi * Self::w1());
        let den1_1 = 3_u64.into_fr() * self.h3_w3[0] * (self.xi * Self::w1() - self.xi);
        [
            den1_0 * self.h2_w3[0] * (self.y - self.h2_w3[0]),
            den1_0 * self.h2_w3[2] * (self.y - self.h2_w3[1]),
            den1_0 * self.h2_w3[1] * (self.y - self.h2_w3[2]),
            den1_1 * self.h3_w3[0] * (self.y - self.h3_w3[0]),
            den1_1 * self.h3_w3[2] * (self.y - self.h3_w3[1]),
            den1_1 * self.h3_w3[1] * (self.y - self.h3_w3[2]),
        ]
    }

    fn compute_eval_l1_base(&self) -> Fr {
        Self::n() * (self.xi - Fr::one())
    }

    fn compute_den_h1_base(&self) -> Fr {
        let w = self.y - self.h1_w4[0];
        let w = w * (self.y - self.h1_w4[1]);
        let w = w * (self.y - self.h1_w4[2]);
        w * (self.y - self.h1_w4[3])
    }

    fn compute_den_h2_base(&self) -> Fr {
        let w = self.y - self.h2_w3[0];
        let w = w * (self.y - self.h2_w3[1]);
        let w = w * (self.y - self.h2_w3[2]);
        let w = w * (self.y - self.h3_w3[0]);
        let w = w * (self.y - self.h3_w3[1]);
        w * (self.y - self.h3_w3[2])
    }

    fn compute_inverse(&self, expected: Fr) -> Result<(Inverse, Fr), VerifyError> {
        let den_h1_base = self.compute_den_h1_base();
        let den_h2_base = self.compute_den_h2_base();
        let mut data = [Fr::zero(); 22];
        data[0] = self.zh;
        data[1] = data[0] * den_h1_base;
        data[2] = data[1] * den_h2_base;
        let mut cursor = 3;
        let li_s0 = self.compute_li_s0();
        for elem in li_s0 {
            data[cursor] = data[cursor - 1] * elem;
            cursor += 1;
        }
        let li_s1 = self.compute_li_s1();
        for elem in li_s1 {
            data[cursor] = data[cursor - 1] * elem;
            cursor += 1;
        }
        let li_s2 = self.compute_li_s2();
        for elem in li_s2 {
            data[cursor] = data[cursor - 1] * elem;
            cursor += 1;
        }

        let eval_l1_base = self.compute_eval_l1_base();
        data[cursor] = data[cursor - 1] * eval_l1_base;
        let value = data[cursor];

        if Fr::one() != value * expected {
            return Err(VerifyError::InvalidInverse {
                inverse: expected,
                computed: value,
            });
        }
        data[cursor] = expected;
        cursor -= 1;
        // We get l1 from batches and we compute the polynomial evaluation L1(x)
        let l1 = data[cursor + 1] * data[cursor] * self.zh;
        data[cursor] = data[cursor + 1] * eval_l1_base;
        cursor -= 1;
        let mut li_s2_inv = [Fr::zero(); 6];
        for (pos, elem) in li_s2.into_iter().enumerate().rev() {
            li_s2_inv[pos] = data[cursor + 1] * data[cursor];
            data[cursor] = data[cursor + 1] * elem;
            cursor -= 1;
        }
        let mut li_s1_inv = [Fr::zero(); 4];
        for (pos, elem) in li_s1.into_iter().enumerate().rev() {
            li_s1_inv[pos] = data[cursor + 1] * data[cursor];
            data[cursor] = data[cursor + 1] * elem;
            cursor -= 1;
        }
        let mut li_s0_inv = [Fr::zero(); 8];
        for (pos, elem) in li_s0.into_iter().enumerate().rev() {
            li_s0_inv[pos] = data[cursor + 1] * data[cursor];
            data[cursor] = data[cursor + 1] * elem;
            cursor -= 1;
        }
        let den_h2 = data[cursor + 1] * data[cursor];
        data[cursor] = data[cursor + 1] * den_h2_base;
        cursor -= 1;
        let den_h1 = data[cursor + 1] * data[cursor];
        data[cursor] = data[cursor + 1] * den_h1_base;
        let zh_inv = data[cursor];
        Ok((
            Inverse {
                li_s0_inv,
                li_s1_inv,
                li_s2_inv,
                den_h1,
                den_h2,
                zh_inv,
            },
            l1,
        ))
    }
}

impl Constants for Challenges {}

impl From<(&Proof, &Public)> for Challenges {
    fn from((proof, public): (&Proof, &Public)) -> Self {
        let beta = [
            Self::C0X,
            Self::C0Y,
            public.0,
            proof.c1.x().into_u256(),
            proof.c1.y().into_u256(),
        ]
        .hash()
        .into_fr();
        let gamma = [beta.into_u256()].hash().into_fr();
        let xi_seed = [
            gamma.into_u256(),
            proof.c2.x().into_u256(),
            proof.c2.y().into_u256(),
        ]
        .hash()
        .into_fr();
        let xi_seed_2 = xi_seed * xi_seed;
        let xi_seed_3 = xi_seed * xi_seed_2;
        let h0_w8 = [
            xi_seed_3,
            xi_seed_3 * Self::w8_1(),
            xi_seed_3 * Self::w8_2(),
            xi_seed_3 * Self::w8_3(),
            xi_seed_3 * Self::w8_4(),
            xi_seed_3 * Self::w8_5(),
            xi_seed_3 * Self::w8_6(),
            xi_seed_3 * Self::w8_7(),
        ];
        let xi_seed_6 = xi_seed_3 * xi_seed_3;
        let h1_w4 = [
            xi_seed_6,
            xi_seed_6 * Self::w4(),
            xi_seed_6 * Self::w4_2(),
            xi_seed_6 * Self::w4_3(),
        ];
        let xi_seed_8 = xi_seed_6 * xi_seed_2;
        let h2_w3 = [xi_seed_8, xi_seed_8 * Self::w3(), xi_seed_8 * Self::w3_2()];
        let h3_w3_0 = xi_seed_8 * Self::wr();
        let h3_w3 = [h3_w3_0, h3_w3_0 * Self::w3(), h3_w3_0 * Self::w3_2()];
        let xi = xi_seed_8 * xi_seed_8 * xi_seed_8;
        let zh = xi.pow(Self::n()) - Fr::one();
        let alpha = [
            xi_seed.into_u256(),
            proof.ql.into_u256(),
            proof.qr.into_u256(),
            proof.qm.into_u256(),
            proof.qo.into_u256(),
            proof.qc.into_u256(),
            proof.s1.into_u256(),
            proof.s2.into_u256(),
            proof.s3.into_u256(),
            proof.a.into_u256(),
            proof.b.into_u256(),
            proof.c.into_u256(),
            proof.z.into_u256(),
            proof.zw.into_u256(),
            proof.t1w.into_u256(),
            proof.t2w.into_u256(),
        ]
        .hash()
        .into_fr();
        let y = [
            alpha.into_u256(),
            proof.w1.x().into_u256(),
            proof.w1.y().into_u256(),
        ]
        .hash()
        .into_fr();
        Self {
            beta,
            gamma,
            h0_w8,
            h1_w4,
            h2_w3,
            h3_w3,
            xi,
            zh,
            alpha,
            y,
        }
    }
}

#[cfg(test)]
mod should;
