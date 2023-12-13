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
    let x = read_fq(field.x_str(), data[0])?;
    let y = read_fq(field.y_str(), data[1])?;
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

    const PROOF_RAW_DATA: ProofRawData = hex_literal::hex!(
        r#"
        17f87a599ca7d3a86ffd7de8cf12adcfd418136c14aec5ced91f4a49b2975c2c
        1287c8ed2b2009c6fe9031e272439442d8ccda251de9c8737c2e5af3689a1767
        1b74f0d660e9e88f0f8f87c6e32be65cb71204e4fd385c29fa93f3aa043c26ba
        2581eb0d9e2b5942ec8ffc9d61650e05d049c8d35b986f1224b6876d12b6194b
        1da25c0ab8021a9b52681e5510be5f2e38bc5daf6ade3d58a0d54711aa33c534
        1fed05884b416a93d551d27a6fdf683972568ff0d2c9a26c8425d0604c3b77a9
        2ca037535c6e9d94a8cf15511dd38a5a43377816242ce93846d8f882306f39a3
        17daf2a44ced35aa8ac02921c5f8c0557f30290d5940f52e2d1fa4608ea5b1db
        0954ff268194b6e09677a8e930a1cf8e38b5315807ed8b393954b626263896d9
        184f910581d502641cd8cff4512b1d4e382932b55dc8d816484b9de0c9c43630
        1a345a58b9a9f87ac671a3f7bb17032c41a75a537f9101a5aeb83009feeef401
        0c74addd2dbe0ee47fcfc2b1cf5cec3c5e86692ef48f1c0235fad1d7a01c668e
        03372f5c6df30567156e9a2788f8a404033b4cc12591084918018425b36c85e1
        20120c2975a7dfb730fdae333a771049473e4c13eb3ccd85911d8a6e1a8ec19f
        00e2b3945fa3224f8f395791ed78709d153044397bf0a48cc41a2007b5228086
        012b96cd44c4f4ea2fdc8beb2414e0bb5b3c9de9df1a938044e522e1c6fff631
        25008ebe0c16aac088bc38cbb5f487b5601673421aa31462869c8c992e4ca321
        181f1c35924e14d4b3aa39a55331f016e7a1bda6b0562f227493c38f2bcd94aa
        1ea83ce07e30d84945c0a665d1f9e0e93fd2db9f3a61fd9c05f33e753715dbec
        1deed29feb3a59387ea9b087fc0c6b36b2a69124da7ced65b852d1535a385b64
        1a950c68fe0cd92b6f4e83890b62a8e115f126ba0399084b6def365ed80fe360
        27887a2f0b8a87c873b171d74db622cd77e67291bee1c59a9fa7f00ca0b87e95
        09c6dfcc7db43ceee36998f660efa5e1c485a083a43c497b8e1061ab2b9bc0c2
        1948698c7b7f3b4c2b6f8ca07f6ca519c27dc72e87e67bbe4675a92a92371897
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
