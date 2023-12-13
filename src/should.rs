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
        "17f87a599ca7d3a86ffd7de8cf12adcfd418136c14aec5ced91f4a49b2975c2c",
        "1287c8ed2b2009c6fe9031e272439442d8ccda251de9c8737c2e5af3689a1767",
        "1b74f0d660e9e88f0f8f87c6e32be65cb71204e4fd385c29fa93f3aa043c26ba",
        "2581eb0d9e2b5942ec8ffc9d61650e05d049c8d35b986f1224b6876d12b6194b",
        "1da25c0ab8021a9b52681e5510be5f2e38bc5daf6ade3d58a0d54711aa33c534",
        "1fed05884b416a93d551d27a6fdf683972568ff0d2c9a26c8425d0604c3b77a9",
        "2ca037535c6e9d94a8cf15511dd38a5a43377816242ce93846d8f882306f39a3",
        "17daf2a44ced35aa8ac02921c5f8c0557f30290d5940f52e2d1fa4608ea5b1db",
        "0954ff268194b6e09677a8e930a1cf8e38b5315807ed8b393954b626263896d9",
        "184f910581d502641cd8cff4512b1d4e382932b55dc8d816484b9de0c9c43630",
        "1a345a58b9a9f87ac671a3f7bb17032c41a75a537f9101a5aeb83009feeef401",
        "0c74addd2dbe0ee47fcfc2b1cf5cec3c5e86692ef48f1c0235fad1d7a01c668e",
        "03372f5c6df30567156e9a2788f8a404033b4cc12591084918018425b36c85e1",
        "20120c2975a7dfb730fdae333a771049473e4c13eb3ccd85911d8a6e1a8ec19f",
        "00e2b3945fa3224f8f395791ed78709d153044397bf0a48cc41a2007b5228086",
        "012b96cd44c4f4ea2fdc8beb2414e0bb5b3c9de9df1a938044e522e1c6fff631",
        "25008ebe0c16aac088bc38cbb5f487b5601673421aa31462869c8c992e4ca321",
        "181f1c35924e14d4b3aa39a55331f016e7a1bda6b0562f227493c38f2bcd94aa",
        "1ea83ce07e30d84945c0a665d1f9e0e93fd2db9f3a61fd9c05f33e753715dbec",
        "1deed29feb3a59387ea9b087fc0c6b36b2a69124da7ced65b852d1535a385b64",
        "1a950c68fe0cd92b6f4e83890b62a8e115f126ba0399084b6def365ed80fe360",
        "27887a2f0b8a87c873b171d74db622cd77e67291bee1c59a9fa7f00ca0b87e95",
        "09c6dfcc7db43ceee36998f660efa5e1c485a083a43c497b8e1061ab2b9bc0c2",
        "1948698c7b7f3b4c2b6f8ca07f6ca519c27dc72e87e67bbe4675a92a92371897",
    ];

    const PUBS: U256 = u256!("2a2e8a3d4897c9ef79f20daf88ce801f240a3bfb97b4e8e6faf831fbd9f26706");
}

struct AdditionalValidTestCase {}

impl TestCase for AdditionalValidTestCase {
    const PROOF: ProofData = u256s![
        "2732c7efa4f1938c8bac3514e55fb376727f00576080ed3921af8f532fb3cdb6",
        "1b0344230a558e2167fba348e7b9e46a0c806fd1e01c9d53428c0133f83251f0",
        "22e0561804b8c6254f889107c0c3b603371df11f9565b674bde8a80f038b0195",
        "22c4c73a3ad89e5d490237e93368e489f40c9be4bd6d24254f0701e9104a6794",
        "1f51867dc86078187fa3aebdb914146c2292e1090bf84e74ce16921587608a58",
        "0999f819f5036dad1e32e1f8fde327f724c6be523b13a9a541fb90610b80eb3a",
        "2e5f680d3530df5a5089437821a9fc83be7500a912a1de6eda2d3a81fe6aad7d",
        "0b1fef8b68021a74049ff3df59530226ec764c4f0dfb1c18b9e4f5315dbe9ab2",
        "1b880d65bbe3e371fd388bd75ca07498f26b75f24558d15cd9e8e08325c12437",
        "08b8a6fea41e4f294c57f36a039b11927fa9f1f0a45365f675046bf4cf5c49f6",
        "147ef6b658605e74c2d00d49f62f3c1ebdbbb23b1170d3318e57d185fa9b750a",
        "06c73b7ca86fb73d945d75b65ca8e5559c9d7f5179870153ed9c4fa3fab706ae",
        "13f7f52694412e70bcd775b8a2e0d3c8f51b596ec514722d3cfe7c70ea8648ec",
        "2300d7d67912564bb1772e01fbb9a56c541e190614561146dcc036d4444b0d61",
        "1ac97ac7e6a31b2b4c7a879fcc4561bda9c0731081482499ca9cbf4ed85e4569",
        "0d7ad14261a398019c2c6e0fb1c863ccdf97ad9c7ed644a3c9e4492ee29b98b8",
        "1d0c06e454b695487dd9729fddb319058f87c0f0201c74939229b6e8b326030f",
        "15c2a941d31e7e2e29f64ef97b5fc541527b1d6a1b17552436630ce900500afc",
        "2aba2354aef6ee19ebb106bd7833077d323d95b9bf7b581d71866dfe1ddce509",
        "24be0a6cc6cbc0c81a218a40868b7f3f6154b735144d9f67973f48d47604a0a7",
        "12070afbb6ff38f2a88ac10b57ddbbd98cedd16581981fb0060e49d653859391",
        "16862b06cd159a74371193db2581a02f540f7f647e28c2573c16b2a3428ad9d9",
        "2d7fda81c6bdd963c947d0158cca06f620aa56f6b379982313e900ff61b971d2",
        "06b9fd32e64b1c607714b056477d1add7db7ba43982f431f4341004f8bb01fd3",
    ];

    const PUBS: U256 = u256!("0d48225cca2578cfab88b55e6ce0193f454af85e1ddf3fdf74492f1838a3f9b3");
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
