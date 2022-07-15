use web3::types::H256;

pub struct AppData {
    pub private_key: H256,
    pub private_key2: H256,
}

impl AppData {
    pub fn new(private_key: H256, private_key2: H256) -> Self {
        Self {
            private_key,
            private_key2,
        }
    }
}
