use crate::bip32_path::Bip32Path;
use alloc::vec::Vec;
use ledger_device_sdk::nbgl::NbglHomeAndSettings;

// sign context
pub struct Context {
    pub raw_tx: Vec<u8>,
    pub path: Bip32Path,
    pub review_finished: bool,
    pub home: NbglHomeAndSettings,
}

// Implement constructor for TxInfo with default values
impl Context {
    // Constructor
    pub fn new() -> Context {
        Context {
            raw_tx: Vec::new(),
            path: Default::default(),
            review_finished: false,
            home: Default::default(),
        }
    }
    // Get review status
    #[allow(dead_code)]
    pub fn finished(&self) -> bool {
        self.review_finished
    }
    // Implement reset for TxInfo
    pub fn reset(&mut self) {
        self.raw_tx.clear();
        self.path = Default::default();
        self.review_finished = false;
    }
}
