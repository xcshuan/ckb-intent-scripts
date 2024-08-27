use std::mem::size_of;

use ckb_testtool::ckb_types::packed;

#[derive(Default, Debug)]
pub struct AnotherIntent {
    pub script_hash: [u8; 32],
    pub intent_data: IntentData,
}

#[derive(Debug)]
pub enum IntentTarget {
    Script(packed::Script),
    AnotherIntent(AnotherIntent),
}

impl Default for IntentTarget {
    fn default() -> Self {
        Self::Script(packed::Script::default())
    }
}

#[derive(Default, Debug)]
pub struct IntentData {
    pub owner: [u8; 32],
    pub singers: Vec<[u8; 32]>,
    pub targets: Vec<IntentTarget>,
    pub calldata: Vec<u8>,
}

#[test]
fn test_intent_data() {
    let a = IntentData {
        targets: vec![IntentTarget::AnotherIntent(AnotherIntent {
            script_hash: Default::default(),
            intent_data: Default::default(),
        })],
        ..Default::default()
    };

    println!("{:?}", a);

    println!("size: {}", size_of::<IntentData>())
}
