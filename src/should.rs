use pretty_assertions::assert_eq;

use rand::Rng;
use rstest::{fixture, rstest};

use crate::macros::{fr, g1, u256s};

use super::*;

#[fixture]
fn valid_proof_data() -> ProofData {
    u256s![
        "1592b15070ee070dcec2a5e1e20a020976a21df73165ae5e462aa8d8b41c5323",
        "2cb15a95b1fe542bfb52320223dbbd9c9637404752880456f97ae718baf60f93",
        "0dcf1e0d545fb4684facd1009e5bbdd63a0220a2bb57bfdabb66199b1081c11a",
        "016314a75659deb23bf5252d4a6eea02120c8cf951116fb88d5d8524fe1da94b",
        "1f924a36446ef0bee0529d5f16cebedfe4eff9c004dac713546c8b817a227e06",
        "181c39853bb47989464a289ec5150230b6dea91ba8f34ab05b8363a6bd3f1795",
        "0c676f4dac5690b05695d332b6287cb4e4c39cc40ca24545218f960528dc0823",
        "0ed29ae6c74ac6e7fa7ecbb16cd796b4cd4d65ce2cca0b434e97af0cc442ff96",
        "090c4ff97cb0d01ee2b1a6396149e407c115920bf3956009b2a3cd6832b66dfa",
        "2fca50be168b4ce837e93ea419b521e8cbf0c3b5b72e9d85c072e82b5f619f91",
        "065fa71d25e993865bf9883fcf0c91f54411b1c97ada45c3e25c0caae389c647",
        "0188d5441830ba5e88f1f40e9beaa8127bda5ecea61eab1f3a3830c705bfab96",
        "1fb344c4278f0189db466b91fd12ce63d5e9152fe1bab6d26be79bb2bc64d880",
        "05dc42697d566da181418c12f6355ad050c5d4f4237fbb3d9d23643c27e12ba3",
        "236c777c03450f05e48de21b65b2ca0df1e368609bdb94e17b88e21f13e34faf",
        "01c8afe9bf70777b0ff54bef8b45ab992ef240e9f267529731803e2b467359e4",
        "12f892b9ece35a8756ebee70d80788dfe63b0a08cd554b2a003289a477a4cf12",
        "23b237b76cc10a9ee7c720628e544ba46d86bdb698c25f5b59c4bd14e66e2908",
        "0e5d909645d14fe84ae66ac8dfdfa6e1bbb83b7ac077079a9302a63a74c5b0e0",
        "29047525f5135a81ad5d9f5ce772700237267e0ab908290388ee060f19a6ee0e",
        "303db43e2056580294a4903ad98315c70243e22a96324e6088fa9c2346e85683",
        "2a5e0ce958c211bce2e2979b8b78155756ec8cc3142a7b9a83909b0050b7272a",
        "251568ac1942b1d3da24c87a6324d05e073b22692266b4d47aa24a76bd70b176",
        "0ad1ac8436349d96a39d916638b6426d8b9b43e126478383ac5e6e0732d7910f",
    ]
}

#[fixture]
fn valid_proof(valid_proof_data: ProofData) -> Proof {
    valid_proof_data.try_into().unwrap()
}

#[fixture]
fn valid_pubs() -> Public {
    u256!("2bb635ee7d9e1790de1d6ccec2d1e13dec5c4beffd75d71520107c791857c45e").into()
}

#[rstest]
fn produce_valid_challenges(valid_proof: Proof, valid_pubs: Public) {
    let challenges = Challenges::from((&valid_proof, &valid_pubs));
    assert_eq!(
        challenges.beta,
        fr!("1f9bb50de802b1eccba0e9e77a628456044aeb5b8e422b184a420b3e7aa68c9d")
    );
    assert_eq!(
        challenges.gamma,
        fr!("2687947f49e96d110364352b569ee93923705eca9ea3b65b3af06b50b4d66317")
    );
    assert_eq!(
        challenges.h0_w8[0],
        fr!("1462e903b9432434aa232a3dfbd3b4acdbef8a948265fa4db4cc6da8ff7276e8")
    );
    assert_eq!(
        challenges.h0_w8,
        [
            fr!("1462e903b9432434aa232a3dfbd3b4acdbef8a948265fa4db4cc6da8ff7276e8"),
            fr!("0209fa5962968cab1e25ab15fe51da6e52c0f03ee759455b5cbed98a8ee47095"),
            fr!("1f60b1c8106428b3b85579aae0a4d3151f0c2acda1637cabb29c84dff5377302"),
            fr!("1a7785c3dbeaf9f49d125f120568ed53ad6f900d63f5f24ca18dd3228f47f02a"),
            fr!("1c01656f27ee7bf50e2d1b7885ada3b04c445db3f75376438f1587eaf08d8919"),
            fr!("2e5a54197e9b137e9a2a9aa0832f7deed572f80992602b35e7231c09611b8f6c"),
            fr!("11039caad0cd7775fffacc0ba0dc85480927bd7ad855f3e5914570b3fac88cff"),
            fr!("15ecc8af0546a6351b3de6a47c186b097ac4583b15c37e44a254227160b80fd7"),
        ]
    );
    assert_eq!(
        challenges.h1_w4,
        [
            fr!("1f6b740eb7433d39b192af4874dbe87e0cf96f8742222b1fbf5f349381c4f8a1"),
            fr!("1f6a27d5f0cc22eadcbc6eb25263c4b5b7b36a6ccac048244a40ba71cca20380"),
            fr!("10f8da6429ee62f006bd966e0ca56fdf1b3a78c1379745718482c1006e3b0760"),
            fr!("10fa269cf0657d3edb93d7042f1d93a770807ddbaef9286cf9a13b22235dfc81"),
        ]
    );
    assert_eq!(
        challenges.h2_w3,
        [
            fr!("234cb9f6b948bb6638e85aa7053605b39be7d7ad42346b145e6ec3196381d639"),
            fr!("04b48dd3e6201e8320b3515874cc1f6f1a7b497adea68670bc8a7b60a7f239a2"),
            fr!("086306a841c8c6405eb499b7077f333a71d0c72058de7f0c28e8b719e48bf026"),
        ]
    );
    assert_eq!(
        challenges.h3_w3,
        [
            fr!("00297a316888778af472390c85e0e53d38219bc3568053b328647d346db17b50"),
            fr!("07e359bf77f1d9cd2bbb7394966044703fc6783460e2dd3af6ffa9d5b3eb0477"),
            fr!("28577a8200b74ed19822991565402eafb04bd450c2563fa3247dce89ce63803a"),
        ]
    );
    assert_eq!(
        challenges.xi,
        fr!("16d1c5505ea62592295f7bbfcf8c3dbf86d57b9b34c2797f4f6f90942924871e")
    );
    assert_eq!(
        challenges.zh,
        fr!("2013d106a3425f0e250ebec1378699ae88a47b14cb9da6bed5879e8bed92edc8")
    );
    assert_eq!(
        challenges.alpha,
        fr!("2137c2f4ce48363a2b597c65b08b974efbddc9ebfd74e2bd18bd04f9ca4b28ee")
    );
    assert_eq!(
        challenges.y,
        fr!("03d33b694aa0710e43e21f876d66ba35e3fd3c13549f5cc8d78568a6c3722ee0")
    );
}

#[rstest]
fn compute_valid_li_s0(valid_proof: Proof, valid_pubs: Public) {
    let li_s0 = Challenges::from((&valid_proof, &valid_pubs)).compute_li_s0();
    assert_eq!(
        li_s0,
        [
            fr!("10b8b1e3799c81cb63f0777ed58c1f338e52f589c956ce943227887f97b8b831"),
            fr!("29b2a1a0302535fb7b915c420c54cdbb91a8da39bf90749c8c88bceeacb6bbd1"),
            fr!("2c963022c4df39a6651f4a378af88133faed6097683ab2dd2798c32edf5a01d7"),
            fr!("0d7e6e2612c0a4b49c1cd53424d71fb3188dda67a0c1db9526d9ab9bf69383e2"),
            fr!("054d6cada58e265fc899fa383dbac7bd45f49306b84d1e00f5ef1add55fed5f7"),
            fr!("1cb7cb63d037125969495b2b887371926ad2969f3bcce889df6fdc023100d258"),
            fr!("19d43ce13b7d0eae7fbb6d3609cfbe1a018e10419322aa49445fd5c1fe5d8c52"),
            fr!("0887b06b0c6a0376906d9c82ee6fc73dbbb9ae28e0e21100013cf7c0f7240a46"),
        ]
    );
}

#[rstest]
fn compute_valid_li_s1(valid_proof: Proof, valid_pubs: Public) {
    let li_s1 = Challenges::from((&valid_proof, &valid_pubs)).compute_li_s1();
    assert_eq!(
        li_s1,
        [
            fr!("2c621184657bc456e7199c71c76e3ff74c452416a16498022c1e888f3219e601"),
            fr!("06cc540e58c020a9f5f9eb3ab4aac638fa6fabfad901ae061794833f48196e47"),
            fr!("0f054c370b4b2fe8677be22043b68bde4612887a1926ced9abcebeb334c1e114"),
            fr!("0436bb3a36d5336ba04b4da0d4f8ad3f6fb4184d67d048447c76ce6f2ec258cd"),
        ]
    );
}

#[rstest]
fn compute_valid_li_s2(valid_proof: Proof, valid_pubs: Public) {
    let li_s2 = Challenges::from((&valid_proof, &valid_pubs)).compute_li_s2();
    assert_eq!(
        li_s2,
        [
            fr!("2a1bf56528a44efb78919042dc1caeb3d2d9d38aa7c29c1983474e0b7ed12323"),
            fr!("0ffc564183dd963235238be620060c0e509e2e75fea722def2405cc8875e055a"),
            fr!("1db5dd5896223bb2a464fbc9461498732223d4b59a95277d27f17cc65910f5a7"),
            fr!("012cea9d2430d2a51ca09111e9016e251c88ce4241237d4574c352beec8a6898"),
            fr!("1618fba0e300faac6db323f58a019e0b46fc7daca479dc74a17f3ce9f5e2a702"),
            fr!("2aaf9fea073a81b5b77faab3a33290c54c57527863fb84ed585de5c1906c09a3"),
        ]
    );
}

#[rstest]
fn compute_valid_den_h1_base(valid_proof: Proof, valid_pubs: Public) {
    let den_h1_base = Challenges::from((&valid_proof, &valid_pubs)).compute_den_h1_base();
    assert_eq!(
        den_h1_base,
        fr!("0d9a086b232dd0d59d2d992f14e197d91665d051f060b1bfede2723917336365")
    );
}

#[rstest]
fn compute_valid_den_h2_base(valid_proof: Proof, valid_pubs: Public) {
    let den_h2_base = Challenges::from((&valid_proof, &valid_pubs)).compute_den_h2_base();
    assert_eq!(
        den_h2_base,
        fr!("019a210549e1551f8afd1543eb6d44e04b3357110e37d53df9a0435fb440f98a")
    );
}

#[rstest]
fn compute_valid_inverse_array(valid_proof: Proof, valid_pubs: Public) {
    let (inverse, l1) = Challenges::from((&valid_proof, &valid_pubs))
        .compute_inverse(valid_proof.inv)
        .unwrap();

    assert_eq!(
        l1,
        fr!("26ebd29ec995686c25154012397fe0b5fe15a4352c5fe9221d7866e8e6dc5634")
    );

    assert_eq!(
        inverse,
        Inverse {
            li_s0_inv: [
                fr!("16cc10e25ee1f11913077c2ec14f8064ea521534f454a9472d87312a66fd89fc"),
                fr!("0573194eec6d2e861de749bad852654072ce031f668a9d50d53cacde580bc019"),
                fr!("0b76febb221539bdf7c6bd197f94e1af9ba1670d0d2854aa4b993a423ac06cc6"),
                fr!("1fc950bd0c486c7d3109418e8ba51c836d01c1d33ad258dc9f7dc519f42b08dc"),
                fr!("17a7333d4fc3ed055b25515fa2fbed74bc5739f60cbb0a4e88524334aa839e26"),
                fr!("1b8b1ef7df1dddce0a4b4d44d5d31655c29c3dc817b3d0af33b05a5fdae129cd"),
                fr!("119a2c9c1615d2eec507ad88117df7b4630bff687a714f4fc816145f9336b6cc"),
                fr!("0106f6c7e72699e2454392ecb0bde68718b399c4a6643d03fe900ccf33140065"),
            ],
            li_s1_inv: [
                fr!("237e64e3278731d780abe4c2969a7d1f7896d2c6c3a3d5e4e9127a7c4af12c5e"),
                fr!("21f71ab5cc225db1fc8a221cfb407de160e066bfa5bc2e226d8073c1d5f8b6ac"),
                fr!("12eae0b27e783859b081ecf68957d927aca782616b25932d263daa35a8ef3faa"),
                fr!("23583839a590636975ba84d7b6cb15a5c02ccfe8b7e34ab000344d0697cd02a8"),
            ],
            li_s2_inv: [
                fr!("2131b303d3567a33db9ebf0856a5bd23af916bd90ea9021684f90b874d1776f0"),
                fr!("0287acbfd68ce07359d077c9bac205d39404ec3e943a3b1ec6bbca7fa7b66d1b"),
                fr!("303a2751dd878a600e21387d61c0109ef0836d3d12eb5a7bfca3188fb4a7fa12"),
                fr!("2f2f5ce8ae10add65acbcd9676eb2f3bf7c77c532c55dce795b2f8598bf66908"),
                fr!("1b90a778a11f5ecf08ed5c7468319d72603c0a38fe868b6d619755aa9cc5ac6a"),
                fr!("2b99be14a17d55eebe6349efbc166e54a2fe060b7f2ec01b4c1ddfead40c67b7"),
            ],

            den_h1: fr!("1aeffb9f554eeaf932d1ed40cefb3913f5e3bb3f98f600c1f540fa5281a6255a"),
            den_h2: fr!("08bc0fbff351c6f4846bcc700855ad248e4bb0ca78f3fddc7c384635e63e5b42"),
            zh_inv: fr!("1a766376b7787d79a45bf7e36997c9326fe58ba23eae60250134ae17b82d8ce4"),
        }
    )
}

#[rstest]
fn compute_valid_pi(valid_proof: Proof, valid_pubs: Public) {
    let challenges = Challenges::from((&valid_proof, &valid_pubs));
    let (_, l1) = challenges.compute_inverse(valid_proof.inv).unwrap();
    let pi = Proof::compute_pi(&valid_pubs, l1);

    assert_eq!(
        pi,
        fr!("0612f57d997d85dea8f53edccb95f52c4a9466adcf41d2a2ad13af40c3c7f01d")
    );
}

#[rstest]
fn compute_valid_r0(valid_proof: Proof, valid_pubs: Public) {
    let challenges = Challenges::from((&valid_proof, &valid_pubs));
    let (inverse, _) = challenges.compute_inverse(valid_proof.inv).unwrap();
    let r0 = valid_proof.compute_r0(&challenges, &inverse.li_s0_inv);

    assert_eq!(
        r0,
        fr!("01d2d22dcd5b137816b2c920c746e56c4003b8155c3dc4f81e9cc05279e545ed")
    );
}

#[rstest]
fn compute_valid_r1(valid_proof: Proof, valid_pubs: Public) {
    let challenges = Challenges::from((&valid_proof, &valid_pubs));
    let (inverse, l1) = challenges.compute_inverse(valid_proof.inv).unwrap();
    let pi = Proof::compute_pi(&valid_pubs, l1);
    let r1 = valid_proof.compute_r1(&challenges, pi, inverse.zh_inv, &inverse.li_s1_inv);

    assert_eq!(
        r1,
        fr!("028e84156c148b2621ebb9a63fe116e89ef118cd4a431fcfb9a15bb1a9bb8671")
    );
}

#[rstest]
fn compute_valid_r2(valid_proof: Proof, valid_pubs: Public) {
    let challenges = Challenges::from((&valid_proof, &valid_pubs));
    let (inverse, l1) = challenges.compute_inverse(valid_proof.inv).unwrap();
    let r2 = valid_proof.compute_r2(&challenges, l1, inverse.zh_inv, &inverse.li_s2_inv);

    assert_eq!(
        r2,
        fr!("2549bdba8b7e47b0660cac953719a704fa53eb585901340af232eae4f078a4e3")
    );
}

#[rstest]
fn compute_valid_fej(valid_proof: Proof, valid_pubs: Public) {
    let challenges = Challenges::from((&valid_proof, &valid_pubs));
    let (inverse, l1) = challenges.compute_inverse(valid_proof.inv).unwrap();
    let pi = Proof::compute_pi(&valid_pubs, l1);
    let r0 = valid_proof.compute_r0(&challenges, &inverse.li_s0_inv);
    let r1 = valid_proof.compute_r1(&challenges, pi, inverse.zh_inv, &inverse.li_s1_inv);
    let r2 = valid_proof.compute_r2(&challenges, l1, inverse.zh_inv, &inverse.li_s2_inv);

    let (f, e, j) =
        valid_proof.compute_fej(&challenges, r0, r1, r2, inverse.den_h1, inverse.den_h2);

    assert_eq!(
        f,
        g1!(
            "2da706cf32194b6e69694cbe55208043a63820cac926aeedfc33f61855b52dea",
            "0f6a323ecc014b0bc2065d818dd589a369fa201357fcf81232f19e7474c23eaf"
        ),
    );
    assert_eq!(
        e,
        g1!(
            "0efa28a915442813fb2cd536fb0a4a6772cb664011359090d6376cc263700ab8",
            "15cad0ba52fd3f99af5ab481f52355ffcb08537a706560f5877a985b7db654f0"
        ),
    );
    assert_eq!(
        j,
        g1!(
            "26b4d5d0b352928510c009474d99e073f328f56599c8e9509f5d3e8d8c6f2b3b",
            "0ccb5cc2e0e43014080183ee83e295e9ef1fc500d0a6380f11075e819af5ae6d"
        ),
    );
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
fn verify_valid_proof(valid_proof: Proof, valid_pubs: Public) {
    assert!(valid_proof.verify(valid_pubs).is_ok())
}

mod reject {
    use super::*;

    #[derive(Debug)]
    enum Changes {
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

    #[fixture]
    fn rng() -> impl Rng {
        rand::thread_rng()
    }

    impl Changes {
        fn perturbed(&self, mut proof: Proof, rng: &mut impl Rng) -> Proof {
            let random = Fr::random(rng);
            match self {
                Changes::C1 => {
                    proof.c1 = proof.c1 * random;
                }
                Changes::C2 => {
                    proof.c2 = proof.c2 * random;
                }
                Changes::W1 => {
                    proof.w1 = proof.w1 * random;
                }
                Changes::W2 => {
                    proof.w2 = proof.w2 * random;
                }
                Changes::Ql => {
                    proof.ql = random;
                }
                Changes::Qr => {
                    proof.qr = random;
                }
                Changes::Qm => {
                    proof.qm = random;
                }
                Changes::Qo => {
                    proof.qo = random;
                }
                Changes::Qc => {
                    proof.qc = random;
                }
                Changes::S1 => {
                    proof.s1 = random;
                }
                Changes::S2 => {
                    proof.s2 = random;
                }
                Changes::S3 => {
                    proof.s3 = random;
                }
                Changes::A => {
                    proof.a = random;
                }
                Changes::B => {
                    proof.b = random;
                }
                Changes::C => {
                    proof.c = random;
                }
                Changes::Z => {
                    proof.z = random;
                }
                Changes::Zw => {
                    proof.zw = random;
                }
                Changes::T1w => {
                    proof.t1w = random;
                }
                Changes::T2w => {
                    proof.t2w = random;
                }
                Changes::Inv => {
                    proof.inv = random;
                }
            }
            proof
        }
    }
    use Changes::*;

    #[rstest]
    fn an_invalid_proof(
        mut rng: impl Rng,
        valid_proof: Proof,
        valid_pubs: Public,
        #[values(
            C1, C2, W1, W2, Ql, Qr, Qm, Qo, Qc, S1, S2, S3, A, B, C, Z, Zw, T1w, T2w, Inv
        )]
        change: Changes,
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
