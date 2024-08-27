use alloc::vec::Vec;
use ckb_std::{
    ckb_constants::Source,
    ckb_types::{
        packed,
        prelude::{Entity, Unpack},
    },
    high_level::{load_script, load_witness_args},
};

use crate::{
    error::Error,
    generated,
    intent::{verify_intent_data, ScriptLocation},
    time_lock::check_since,
    utils::{
        check_owner_lock_20_bytes, check_owner_lock_32_bytes, check_owner_type_20_bytes,
        check_owner_type_32_bytes, has_cell,
    },
};

pub fn main() -> Result<(), Error> {
    let script = load_script()?;
    if script.args().raw_data().len() < 40 {
        return Err(Error::Encoding);
    }

    let mut script_hash = [0u8; 20];
    script_hash.copy_from_slice(&script.args().raw_data()[0..20]);
    let mut intent_data_hash = [0u8; 20];
    intent_data_hash.copy_from_slice(&script.args().raw_data()[20..40]);

    if has_cell(0, Source::GroupInput) {
        check_input(script_hash, intent_data_hash)?;
    }

    if has_cell(0, Source::GroupOutput) {
        check_output(intent_data_hash)?;
    }

    Ok(())
}

pub fn check_input(
    expected_script_hash: [u8; 20],
    expected_intent_data_hash: [u8; 20],
) -> Result<(), Error> {
    let witness_args = load_witness_args(0, Source::GroupInput)?;
    let data: Vec<u8> = match witness_args.input_type().to_opt() {
        Some(data) => data.unpack(),
        None => todo!(),
    };

    let intent_data_hash = ckb_hash::blake2b_256(&data);
    if intent_data_hash[0..20] != expected_intent_data_hash {
        return Err(Error::IntentDataUnmatched);
    }

    let intent_data = generated::intent::IntentData::from_slice(&data)?;
    let script_location: ScriptLocation = Into::<u8>::into(intent_data.location()).try_into()?;

    let is_script_exist = match script_location {
        ScriptLocation::InputLock => check_owner_lock_20_bytes(&expected_script_hash),
        ScriptLocation::InputType => {
            check_owner_type_20_bytes(&expected_script_hash, Source::Input)
        }
        ScriptLocation::OutputType => {
            check_owner_type_20_bytes(&expected_script_hash, Source::Output)
        }
    };

    if is_script_exist {
        return Ok(());
    }

    let expire_since = intent_data.expire_since().unpack();

    if check_since(expire_since) {
        let owner_script_hash: packed::Byte32 = intent_data.owner().script_hash();
        let owner_script_location: ScriptLocation =
            Into::<u8>::into(intent_data.owner().location()).try_into()?;

        let is_owner_exist = match owner_script_location {
            ScriptLocation::InputLock => check_owner_lock_32_bytes(owner_script_hash.as_slice()),
            ScriptLocation::InputType => {
                check_owner_type_32_bytes(owner_script_hash.as_slice(), Source::Input)
            }
            ScriptLocation::OutputType => {
                check_owner_type_32_bytes(owner_script_hash.as_slice(), Source::Output)
            }
        };

        if is_owner_exist {
            return Ok(());
        }
    }

    Err(Error::CheckFailed)
}

pub fn check_output(expected_intent_data_hash: [u8; 20]) -> Result<(), Error> {
    let witness_args = load_witness_args(0, Source::GroupInput)?;
    let data: Vec<u8> = match witness_args.input_type().to_opt() {
        Some(data) => data.unpack(),
        None => todo!(),
    };

    let intent_data_hash = ckb_hash::blake2b_256(&data);
    if intent_data_hash[0..20] != expected_intent_data_hash {
        return Err(Error::IntentDataUnmatched);
    }

    let intent_data = generated::intent::IntentData::from_slice(&data)?;
    verify_intent_data(intent_data.as_reader())?;

    if intent_data.signers().into_iter().all(|signer| {
        let signer_script_hash: packed::Byte32 = signer.script_hash();
        let signer_script_location: ScriptLocation =
            match Into::<u8>::into(signer.location()).try_into() {
                Ok(loc) => loc,
                Err(_) => return false,
            };

        let is_signer_exist = match signer_script_location {
            ScriptLocation::InputLock => check_owner_lock_32_bytes(signer_script_hash.as_slice()),
            ScriptLocation::InputType => {
                check_owner_type_32_bytes(signer_script_hash.as_slice(), Source::Input)
            }
            ScriptLocation::OutputType => {
                check_owner_type_32_bytes(signer_script_hash.as_slice(), Source::Output)
            }
        };

        is_signer_exist
    }) {
        return Ok(());
    }

    Err(Error::CheckFailed)
}
