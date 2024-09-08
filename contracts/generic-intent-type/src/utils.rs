use alloc::vec::Vec;
use ckb_std::{
    ckb_constants::Source,
    debug,
    error::SysError,
    high_level::{load_cell_lock_hash, load_cell_type_hash, QueryIter},
    syscalls::load_cell,
};

use crate::error::Error;
#[derive(Default, Debug)]
pub enum ScriptLocation {
    #[default]
    InputLock,
    InputType,
    OutputType,
}

impl TryFrom<u8> for ScriptLocation {
    type Error = Error;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(ScriptLocation::InputLock),
            1 => Ok(ScriptLocation::InputType),
            2 => Ok(ScriptLocation::OutputType),
            _ => Err(Error::Encoding),
        }
    }
}

pub fn has_cell(index: usize, source: Source) -> bool {
    let mut buf = Vec::new();
    match load_cell(&mut buf, 0, index, source) {
        Ok(_) => true,
        Err(e) => {
            // just confirm cell presence, no data needed
            if let SysError::LengthNotEnough(_) = e {
                return true;
            }
            debug!("load cell err: {:?}", e);
            false
        }
    }
}

pub fn check_owner_lock_32_bytes(owner_lock_hash: &[u8]) -> bool {
    QueryIter::new(load_cell_lock_hash, Source::Input)
        .any(|cell_lock_hash| owner_lock_hash[..] == cell_lock_hash[0..32])
}

pub fn check_owner_type_32_bytes(owner_input_type_hash: &[u8], source: Source) -> bool {
    QueryIter::new(load_cell_type_hash, source).any(|cell_type_hash| {
        if let Some(cell_type_hash) = cell_type_hash {
            owner_input_type_hash[..] == cell_type_hash[0..32]
        } else {
            false
        }
    })
}
