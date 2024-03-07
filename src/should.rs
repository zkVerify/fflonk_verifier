use rand::Rng;
use rstest::{fixture, rstest};

use crate::macros::u256s;

use super::*;

trait TestCase {
    const PROOF: ProofData;
    const PUBS: U256;

    fn valid_proof() -> Proof {
        Self::PROOF.try_into().unwrap()
    }

    fn valid_pubs() -> Public {
        Self::PUBS.into()
    }
}

struct ValidTestCase {}

impl TestCase for ValidTestCase {
    const PROOF: ProofData = u256s![
        "283e3f25323d02dabdb94a897dc2697a3b930d8781381ec574af89a201a91d5a",
        "2c2808c59f5c736ff728eedfea58effc2443722e78b2eb4e6759a278e9246d60",
        "0f9c56dc88e043ce0b90c402e96b1f4b1a246f4d0d69a4c340bc910e1f2fd805",
        "19e465e01bd7629f175931feed102cb6459a1be7b08018b93c142e961d0352d8",
        "0b8e5d340df28c2f454c5a2535ca01a230bb945ee24b1171481a9a2c6496fed6",
        "1cf8878e40adb52dc27da5e79718f118467319d15d64fed460d69d951376ac63",
        "1a6c44faaec76e296b43fe720d700a63fd530f9064878b5f72f2ffe7458c2f03",
        "1ac6ed8c1e0758dfb3702ed29bbc0c14b5e727c164b3ade07b9f164af0be54b0",
        "143b1a6534b2dcf2bd660e1b5b420d86c0c350fd9d614b639c5df98009f1375e",
        "141259679021d0a6a3aa3aae2516bace4a4a651265217ec0ea7c0d7f89b98710",
        "0abcc93d98ff40bae16eff6c29955f7a37155bb25672b12eb5074dcb7c3e2b00",
        "1718a257cca21ee593d1ba9f8e91e5168aed8e0b1893e11a6b583d975e747f80",
        "08a8c2150a04d8f867945ca1740dc3fc3b2fc4daff61b4725fb294435a1b9010",
        "1803690ae70fc212b7e929de9a22a4642ef4772546cf93ffd1b1196a3d9113a3",
        "009c506755578932ca3630508ca1ed6ee83df5ec9e26cb0b5800a70967a1a93a",
        "04d142b6a532935a31d84f75d16929df6d38c3a210ac4f435a8024dfb7e6c1f3",
        "246d58038a943f237325b44f03d106e523adfec4324615a2dd09e1e5b9143b41",
        "1c1cf09ee411cf9864d30df4904099920cee9ae8134d45dfeb29e46115d2e740",
        "098674b8fc2ca31fac6fcc9302860654fdc1b522b7e064b0759bc5924f332fa9",
        "21121b5af880f83fbce02f19dabb8f684593e7322fb80bfc0d054797b1d4eff4",
        "11b01bf68f81f2032ae4f7fc514bd76ca1b264f3989a92e6b3d74cda4f8a7149",
        "20e4c02f5a71082a8bcf5be0b5750a244bd040a776ec541dfc2c8ae73180e924",
        "0ada5414d66387211eec80d7d9d48498efa1e646d64bb1bf8775b3796a9fd0bf",
        "0fdf8244018ce57b018c093e2f75ed77d8dbdb1a7b60a2da671de2efe5f6b9d7",
    ];

    const PUBS: U256 = u256!("0d69b94acdfaca5bacc248a60b35b925a2374644ce0c1205db68228c8921d9d9");
}

struct AdditionalValidTestCase {}

impl TestCase for AdditionalValidTestCase {
    const PROOF: ProofData = u256s![
        "2ecc31435ec6d6963d463c38ea5662d9c94a67e441e7bc611598ebcc59f57188",
        "0768291fd5d95fcf02bce7e4fde1f048b843bbffab1f242904e82d443a4ebb61",
        "150c3a4afdbb62d034320da390e3585a30ba13f4df73798b78e5a75655d3350d",
        "19fca02cc5838405f9ae4177ac7117971af2cb5006d7a46436f644410d6e7c52",
        "099f803c0f18d4b44fe22f3100d1fc80ccb7309fa7168f51bc64f3fc0f1dbd24",
        "0b0573f3593238e56b23e75246d9d0f6f6a5cf824700667e3482ca9fecf74cdc",
        "0b308e6a8f69dccb9ca00d540543441f7030928da766406a152427bdf31b4405",
        "1b6b5198b34006f9ac34c6c857e450cc11f5c6b6d21119fe283738581c0ad8bd",
        "0f8cbdbc574f64da884f6a02e00669f3eef10138266f3d7fa278aef1b1c60171",
        "005c0c2b8b2429c5003c5ab24af44cb1ab81cdc96dcaf6004a0f74406bb10f45",
        "233b13015cef8c40c491a7770efd0a8d8a64186d4f3827e74972bfc25b11f1f0",
        "02550a5e253c923c5783026c7439601595477f1a212de449c64a8ae5e2fc0313",
        "127bf9cd5146217e531196ce65ccef3249375450d6932151f923c39e6a735882",
        "23d90f5bf230eee5a6cb6463f161602cc37fe538e2954ebef695b926b76e3fae",
        "299c60c1952aa4b1f246204ac7c22c0156ede30aeb73444ee40d69c0f131fa47",
        "1fdca090abfd38541c88ee73624657a695155748643f7834b80d1c0481079e67",
        "0033817252b24575a4e6007f08f37c34462d5e9fd50b1e83ec8cfc86149400d5",
        "19e224ee11831ac393e3a09730be6f385ae5c9e14446fde5069fea751fb6b482",
        "11c85268b8017de7981eb1bd78526bc20d5f863ad3abe249728ca7b75b2146c1",
        "254465d6100a911213d95f800779e74f6701b1dfa0b6660642108fd2c7cd2f13",
        "1d9163eeebe9d8aabdf8d37fde4451f762be478d117688e0a6ed2648dbe025e8",
        "2a4b13ee629a73d1efa6f269747506058746aa589bb961c1385bb2b30e0086f0",
        "10ef87535f2137a04f19fe5aa7c4f348c32ce6f5b0b45bb503895673a8a51d7f",
        "1b0228693fbfb38be718b04c9fdf116a97d7f30e670db84d21bb0d12fc576454",
    ];

    const PUBS: U256 = u256!("15950a3fab52ee1557ac7b895deeca2eb27bacfc3b9e26a39b1875149680611d");
}

#[fixture]
fn valid_proof() -> Proof {
    ValidTestCase::valid_proof()
}

#[fixture]
fn valid_pubs() -> Public {
    ValidTestCase::valid_pubs()
}

#[rstest]
fn compute_valid_check_paring(valid_proof: Proof, valid_pubs: Public) {
    let challenges = Challenges::from((&valid_proof, &valid_pubs));
    let (inverse, l1) = challenges.compute_inverse(valid_proof.inv).unwrap();
    let pi = Proof::compute_pi(&valid_pubs, l1);
    let r0 = valid_proof.compute_r0(&challenges, &inverse.li_s0_inv);
    let r1 = valid_proof.compute_r1(&challenges, pi, inverse.zh_inv, &inverse.li_s1_inv);
    let r2 = valid_proof.compute_r2(&challenges, l1, inverse.zh_inv, &inverse.li_s2_inv);

    let (f, e, j) =
        valid_proof.compute_fej(&challenges, r0, r1, r2, inverse.den_h1, inverse.den_h2);

    let result = valid_proof.check_paring(&challenges, f, e, j);

    assert!(result.is_ok())
}

#[rstest]
#[case(ValidTestCase {})]
#[case(AdditionalValidTestCase {})]
fn verify_valid_proof<TC: TestCase>(#[case] _a: TC) {
    assert!(TC::valid_proof().verify(TC::valid_pubs()).is_ok())
}

mod reject {
    use super::*;

    #[fixture]
    fn rng() -> impl Rng {
        rand::thread_rng()
    }

    impl ProofFields {
        fn perturbed(&self, mut proof: Proof, rng: &mut impl Rng) -> Proof {
            let random = Fr::random(rng);
            match self {
                ProofFields::C1 => {
                    proof.c1 = proof.c1 * random;
                }
                ProofFields::C2 => {
                    proof.c2 = proof.c2 * random;
                }
                ProofFields::W1 => {
                    proof.w1 = proof.w1 * random;
                }
                ProofFields::W2 => {
                    proof.w2 = proof.w2 * random;
                }
                ProofFields::Ql => {
                    proof.ql = random;
                }
                ProofFields::Qr => {
                    proof.qr = random;
                }
                ProofFields::Qm => {
                    proof.qm = random;
                }
                ProofFields::Qo => {
                    proof.qo = random;
                }
                ProofFields::Qc => {
                    proof.qc = random;
                }
                ProofFields::S1 => {
                    proof.s1 = random;
                }
                ProofFields::S2 => {
                    proof.s2 = random;
                }
                ProofFields::S3 => {
                    proof.s3 = random;
                }
                ProofFields::A => {
                    proof.a = random;
                }
                ProofFields::B => {
                    proof.b = random;
                }
                ProofFields::C => {
                    proof.c = random;
                }
                ProofFields::Z => {
                    proof.z = random;
                }
                ProofFields::Zw => {
                    proof.zw = random;
                }
                ProofFields::T1w => {
                    proof.t1w = random;
                }
                ProofFields::T2w => {
                    proof.t2w = random;
                }
                ProofFields::Inv => {
                    proof.inv = random;
                }
            }
            proof
        }
    }
    use ProofFields::*;

    #[rstest]
    fn an_invalid_proof(
        mut rng: impl Rng,
        valid_proof: Proof,
        valid_pubs: Public,
        #[values(
            C1, C2, W1, W2, Ql, Qr, Qm, Qo, Qc, S1, S2, S3, A, B, C, Z, Zw, T1w, T2w, Inv
        )]
        change: ProofFields,
    ) {
        let perturbed_proof = change.perturbed(valid_proof, &mut rng);
        assert!(perturbed_proof.verify(valid_pubs).is_err())
    }

    #[rstest]
    fn an_invalid_public_input(mut rng: impl Rng, valid_proof: Proof) {
        assert!(valid_proof
            .verify(
                U256::random(
                    &mut rng,
                    &u256!("ffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff")
                )
                .into()
            )
            .is_err())
    }

    #[rstest]
    #[should_panic(expected = "InvalidInverse")]
    fn an_invalid_inverse(
        mut rng: impl Rng,
        #[from(valid_proof)] mut proof: Proof,
        valid_pubs: Public,
    ) {
        proof.inv = Fr::random(&mut rng);
        proof.verify(valid_pubs).unwrap()
    }
}
