use snafu::Snafu;
use substrate_bn::{arith::U256, AffineG1, FieldError, Fq, Fr, GroupError, G1};

use crate::{Proof, ProofFields};

pub type ProofData = [U256; 24];
pub type ProofRawData = [u8; 32 * 24];

#[derive(Snafu, Debug)]
pub enum ProofDataError {
    #[snafu(display("Invalid field proof data '{field}': {error:?}"))]
    InvalidField {
        field: &'static str,
        error: FieldError,
    },
    #[snafu(display("Invalid point proof data '{field}': {error:?}"))]
    InvalidGroup {
        field: &'static str,
        error: GroupError,
    },
}

impl TryFrom<&ProofData> for Proof {
    type Error = ProofDataError;

    fn try_from(data: &ProofData) -> Result<Self, Self::Error> {
        use ProofFields::*;
        Ok(Self {
            c1: read_g1(C1, &data[..2])?,
            c2: read_g1(C2, &data[2..4])?,
            w1: read_g1(W1, &data[4..6])?,
            w2: read_g1(W2, &data[6..8])?,
            ql: read_fr(Ql, data[8])?,
            qr: read_fr(Qr, data[9])?,
            qm: read_fr(Qm, data[10])?,
            qo: read_fr(Qo, data[11])?,
            qc: read_fr(Qc, data[12])?,
            s1: read_fr(S1, data[13])?,
            s2: read_fr(S2, data[14])?,
            s3: read_fr(S3, data[15])?,
            a: read_fr(A, data[16])?,
            b: read_fr(B, data[17])?,
            c: read_fr(C, data[18])?,
            z: read_fr(Z, data[19])?,
            zw: read_fr(Zw, data[20])?,
            t1w: read_fr(T1w, data[21])?,
            t2w: read_fr(T2w, data[22])?,
            inv: read_fr(Inv, data[23])?,
        })
    }
}

impl TryFrom<ProofData> for Proof {
    type Error = ProofDataError;

    fn try_from(data: ProofData) -> Result<Self, Self::Error> {
        (&data).try_into()
    }
}

impl TryFrom<&ProofRawData> for Proof {
    type Error = ProofDataError;

    fn try_from(data: &ProofRawData) -> Result<Self, Self::Error> {
        [
            U256::from_slice(&data[0x000..0x020]).expect("Cannot fail to read 32 bytes"),
            U256::from_slice(&data[0x020..0x040]).expect("Cannot fail to read 32 bytes"),
            U256::from_slice(&data[0x040..0x060]).expect("Cannot fail to read 32 bytes"),
            U256::from_slice(&data[0x060..0x080]).expect("Cannot fail to read 32 bytes"),
            U256::from_slice(&data[0x080..0x0A0]).expect("Cannot fail to read 32 bytes"),
            U256::from_slice(&data[0x0A0..0x0C0]).expect("Cannot fail to read 32 bytes"),
            U256::from_slice(&data[0x0C0..0x0E0]).expect("Cannot fail to read 32 bytes"),
            U256::from_slice(&data[0x0E0..0x100]).expect("Cannot fail to read 32 bytes"),
            U256::from_slice(&data[0x100..0x120]).expect("Cannot fail to read 32 bytes"),
            U256::from_slice(&data[0x120..0x140]).expect("Cannot fail to read 32 bytes"),
            U256::from_slice(&data[0x140..0x160]).expect("Cannot fail to read 32 bytes"),
            U256::from_slice(&data[0x160..0x180]).expect("Cannot fail to read 32 bytes"),
            U256::from_slice(&data[0x180..0x1A0]).expect("Cannot fail to read 32 bytes"),
            U256::from_slice(&data[0x1A0..0x1C0]).expect("Cannot fail to read 32 bytes"),
            U256::from_slice(&data[0x1C0..0x1E0]).expect("Cannot fail to read 32 bytes"),
            U256::from_slice(&data[0x1E0..0x200]).expect("Cannot fail to read 32 bytes"),
            U256::from_slice(&data[0x200..0x220]).expect("Cannot fail to read 32 bytes"),
            U256::from_slice(&data[0x220..0x240]).expect("Cannot fail to read 32 bytes"),
            U256::from_slice(&data[0x240..0x260]).expect("Cannot fail to read 32 bytes"),
            U256::from_slice(&data[0x260..0x280]).expect("Cannot fail to read 32 bytes"),
            U256::from_slice(&data[0x280..0x2A0]).expect("Cannot fail to read 32 bytes"),
            U256::from_slice(&data[0x2A0..0x2C0]).expect("Cannot fail to read 32 bytes"),
            U256::from_slice(&data[0x2C0..0x2E0]).expect("Cannot fail to read 32 bytes"),
            U256::from_slice(&data[0x2E0..0x300]).expect("Cannot fail to read 32 bytes"),
        ]
        .try_into()
    }
}

impl ProofFields {
    fn str(&self) -> &'static str {
        match self {
            ProofFields::C1 => "c1",
            ProofFields::C2 => "c2",
            ProofFields::W1 => "w1",
            ProofFields::W2 => "w2",
            ProofFields::Ql => "ql",
            ProofFields::Qr => "qr",
            ProofFields::Qm => "qm",
            ProofFields::Qo => "qo",
            ProofFields::Qc => "qc",
            ProofFields::S1 => "s1",
            ProofFields::S2 => "s2",
            ProofFields::S3 => "s3",
            ProofFields::A => "a",
            ProofFields::B => "b",
            ProofFields::C => "c",
            ProofFields::Z => "z",
            ProofFields::Zw => "zw",
            ProofFields::T1w => "t1w",
            ProofFields::T2w => "t2w",
            ProofFields::Inv => "inv",
        }
    }

    fn x_str(&self) -> &'static str {
        match self {
            ProofFields::C1 => "c1.x",
            ProofFields::C2 => "c2.x",
            ProofFields::W1 => "w1.x",
            ProofFields::W2 => "w2.x",
            _ => "__undefined__x__",
        }
    }

    fn y_str(&self) -> &'static str {
        match self {
            ProofFields::C1 => "c1.y",
            ProofFields::C2 => "c2.y",
            ProofFields::W1 => "w1.y",
            ProofFields::W2 => "w2.y",
            _ => "__undefined__y__",
        }
    }
}

fn read_g1(field: ProofFields, data: &[U256]) -> Result<G1, ProofDataError> {
    let x = read_fq(&field.x_str(), data[0])?;
    let y = read_fq(&field.y_str(), data[1])?;
    AffineG1::new(x, y)
        .map_err(|e| ProofDataError::InvalidGroup {
            field: field.str(),
            error: e,
        })
        .map(Into::into)
}

fn read_fq(addr: &'static str, data: U256) -> Result<Fq, ProofDataError> {
    Fq::from_u256(data).map_err(|e| ProofDataError::InvalidField {
        field: addr,
        error: e,
    })
}

fn read_fr(field: ProofFields, data: U256) -> Result<Fr, ProofDataError> {
    Fr::new(data).ok_or_else(|| ProofDataError::InvalidField {
        field: field.str(),
        error: FieldError::NotMember,
    })
}

#[cfg(test)]
mod tests {
    use rstest::rstest;

    use crate::macros::{u256, u256s};

    use super::*;

    const PROOF_DATA: ProofData = u256s![
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
    ];

    const PROOF_RAW_DATA: ProofRawData = hex_literal::hex!(
        r#"
        1592b15070ee070dcec2a5e1e20a020976a21df73165ae5e462aa8d8b41c5323
        2cb15a95b1fe542bfb52320223dbbd9c9637404752880456f97ae718baf60f93
        0dcf1e0d545fb4684facd1009e5bbdd63a0220a2bb57bfdabb66199b1081c11a
        016314a75659deb23bf5252d4a6eea02120c8cf951116fb88d5d8524fe1da94b
        1f924a36446ef0bee0529d5f16cebedfe4eff9c004dac713546c8b817a227e06
        181c39853bb47989464a289ec5150230b6dea91ba8f34ab05b8363a6bd3f1795
        0c676f4dac5690b05695d332b6287cb4e4c39cc40ca24545218f960528dc0823
        0ed29ae6c74ac6e7fa7ecbb16cd796b4cd4d65ce2cca0b434e97af0cc442ff96
        090c4ff97cb0d01ee2b1a6396149e407c115920bf3956009b2a3cd6832b66dfa
        2fca50be168b4ce837e93ea419b521e8cbf0c3b5b72e9d85c072e82b5f619f91
        065fa71d25e993865bf9883fcf0c91f54411b1c97ada45c3e25c0caae389c647
        0188d5441830ba5e88f1f40e9beaa8127bda5ecea61eab1f3a3830c705bfab96
        1fb344c4278f0189db466b91fd12ce63d5e9152fe1bab6d26be79bb2bc64d880
        05dc42697d566da181418c12f6355ad050c5d4f4237fbb3d9d23643c27e12ba3
        236c777c03450f05e48de21b65b2ca0df1e368609bdb94e17b88e21f13e34faf
        01c8afe9bf70777b0ff54bef8b45ab992ef240e9f267529731803e2b467359e4
        12f892b9ece35a8756ebee70d80788dfe63b0a08cd554b2a003289a477a4cf12
        23b237b76cc10a9ee7c720628e544ba46d86bdb698c25f5b59c4bd14e66e2908
        0e5d909645d14fe84ae66ac8dfdfa6e1bbb83b7ac077079a9302a63a74c5b0e0
        29047525f5135a81ad5d9f5ce772700237267e0ab908290388ee060f19a6ee0e
        303db43e2056580294a4903ad98315c70243e22a96324e6088fa9c2346e85683
        2a5e0ce958c211bce2e2979b8b78155756ec8cc3142a7b9a83909b0050b7272a
        251568ac1942b1d3da24c87a6324d05e073b22692266b4d47aa24a76bd70b176
        0ad1ac8436349d96a39d916638b6426d8b9b43e126478383ac5e6e0732d7910f
        "#
    );

    #[test]
    fn read_a_valid_proof() {
        Proof::try_from(&PROOF_DATA).unwrap();
    }

    #[test]
    fn read_a_valid_raw_proof() {
        Proof::try_from(&PROOF_RAW_DATA).unwrap();
    }

    #[rstest]
    #[should_panic(expected = r#""c1", error: NotOnCurve"#)]
    #[case::invalid_curve(0, U256::from(0))]
    #[should_panic(expected = r#""w2.y", error: NotMember"#)]
    #[case::invalid_field_curve(7, u256!("fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff0"))]
    #[should_panic(expected = r#""ql", error: NotMember"#)]
    #[case::invalid_field_edge_case(8, u256!("30644e72e131a029b85045b68181585d2833e84879b9709143e1f593f0000001"))]
    #[case::valid_field_edge_case(8, u256!("30644e72e131a029b85045b68181585d2833e84879b9709143e1f593f0000000"))]
    fn reject_invalid_proof_data(#[case] id: usize, #[case] value: U256) {
        let mut proof_data = PROOF_DATA.clone();
        proof_data[id] = value;
        Proof::try_from(&proof_data).unwrap();
    }
}
