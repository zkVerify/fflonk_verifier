use substrate_bn::arith::U256;

pub(crate) trait Hasher {
    fn hash(&self) -> [u8; 32];
}

impl Hasher for &[u8] {
    fn hash(&self) -> [u8; 32] {
        use digest::Digest;
        sha3::Keccak256::digest(*self).into()
    }
}

impl Hasher for &[U256] {
    fn hash(&self) -> [u8; 32] {
        let mut buffer = Vec::<u8>::with_capacity(self.len() * 32);
        for d in *self {
            let mut inner = [0; 32];
            d.to_big_endian(&mut inner)
                .expect("BUG: should never fails!");
            buffer.extend_from_slice(&inner);
        }
        buffer.as_slice().hash()
    }
}

impl Hasher for [U256] {
    fn hash(&self) -> [u8; 32] {
        (&self).hash()
    }
}

impl Hasher for Vec<U256> {
    fn hash(&self) -> [u8; 32] {
        self.as_slice().hash()
    }
}

#[cfg(test)]
mod tests {
    use crate::{macros::u256, utils::FrModule};
    use rstest::rstest;

    use super::*;

    #[rstest]
    #[case::zero(u256!("290decd9548b62a8d60345a988386fc84ba6bc95484008f6362f93160ef3e563"), vec![U256::zero()])]
    #[case::zero_of_zero(u256!("20aa000426f73d95c72abaf47f289e50874dd894230eee8e3e67ccc2a42d61d8"), vec![u256!("290decd9548b62a8d60345a988386fc84ba6bc95484008f6362f93160ef3e563")])]
    #[case::zero_zero(u256!("1c053d5dd362f3501993d420ba93e87eb29b2bb845ddeefe74b26929c7ba5fb2"), vec![U256::zero(), U256::zero()])]
    #[case::zero_zero_zero(u256!("160bbcda5f7abc0bf6dbdd2720f72234c32292be4f6b386a4707aac730c08c20"), vec![U256::zero(), U256::zero(), U256::zero()])]
    #[case::some_u256(u256!("07d87f7eed9223d1a55da14bb15eb643a549958a8e4006dba9367247b039b571"), 
    vec![u256!("290decd9548b62a8d60345a988386fc84ba6bc95484008f6362f93160ef3e563"), U256::zero()])]
    #[case::some_u256(u256!("189b3f9023ec42435ff11d489e03af64b7632d6c8e6e413a504ae617e1282d97"), 
    vec![u256!("290decd9548b62a8d60345a988386fc84ba6bc95484008f6362f93160ef3e563"), U256::zero(), u256!("20aa000426f73d95c72abaf47f289e50874dd894230eee8e3e67ccc2a42d61d8")])]
    fn generate_valid_hash_against_the_one_used_in_the_solidity_impl(
        #[case] expected: U256,
        #[case] input: impl Hasher,
    ) {
        // All challenges are corrected to be Fr element (computed a module)
        let u256 = U256::from_slice(&input.hash()).unwrap().fr_module();
        assert_eq!(expected, u256)
    }
}
