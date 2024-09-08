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
    time_lock::check_since,
    type_id::validate_type_id,
    utils::{check_owner_lock_32_bytes, check_owner_type_32_bytes, has_cell, ScriptLocation},
};

pub fn main() -> Result<(), Error> {
    let script = load_script()?;
    if script.args().raw_data().len() < 64 {
        return Err(Error::Encoding);
    }

    let args = script.args().raw_data();

    let receiver_script_hash = &args[0..32];
    let intent_data_hash = &args[32..64];
    if has_cell(0, Source::GroupOutput) {
        check_output(intent_data_hash)?;
    }

    if has_cell(0, Source::GroupInput) {
        check_input(receiver_script_hash, intent_data_hash)?;
    }

    Ok(())
}

pub fn check_input(
    receiver_script_hash: &[u8],
    expected_intent_data_hash: &[u8],
) -> Result<(), Error> {
    // load intent data
    let witness_args = load_witness_args(0, Source::GroupInput)?;
    let data: Vec<u8> = match witness_args.input_type().to_opt() {
        Some(data) => data.unpack(),
        None => return Err(Error::IntentDataMissing),
    };

    // check intent_data hash
    let intent_data_hash = ckb_hash::blake2b_256(&data);
    if intent_data_hash != expected_intent_data_hash {
        return Err(Error::IntentDataUnmatched);
    }

    // parse intent_data
    let intent_data = generated::intent::IntentData::from_slice(&data)?;
    let receiver_location: ScriptLocation =
        Into::<u8>::into(intent_data.receiver_location()).try_into()?;

    // check receiver existence
    let is_receiver_exist = match receiver_location {
        ScriptLocation::InputLock => check_owner_lock_32_bytes(receiver_script_hash),
        ScriptLocation::InputType => check_owner_type_32_bytes(receiver_script_hash, Source::Input),
        ScriptLocation::OutputType => {
            check_owner_type_32_bytes(receiver_script_hash, Source::Output)
        }
    };
    if is_receiver_exist {
        return Ok(());
    }

    // check owner claim expired intent
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

pub fn check_output(expected_intent_data_hash: &[u8]) -> Result<(), Error> {
    // load intent data
    let witness_args = load_witness_args(0, Source::GroupInput)?;
    let data: Vec<u8> = match witness_args.input_type().to_opt() {
        Some(data) => data.unpack(),
        None => return Err(Error::IntentDataMissing),
    };

    // check intent_data hash
    let intent_data_hash = ckb_hash::blake2b_256(&data);
    if intent_data_hash != expected_intent_data_hash {
        return Err(Error::IntentDataUnmatched);
    }

    // parse intent_data
    let intent_data = generated::intent::IntentData::from_slice(&data)?;

    // validate type_id
    validate_type_id(intent_data.type_id().as_slice().try_into().unwrap())?;

    // check authorizers existence
    if intent_data.authorizers().into_iter().all(|authorizer| {
        let authorizer_script_hash: packed::Byte32 = authorizer.script_hash();
        let authorizer_location: ScriptLocation =
            match Into::<u8>::into(authorizer.location()).try_into() {
                Ok(loc) => loc,
                Err(_) => return false,
            };

        let is_authorizer_exist = match authorizer_location {
            ScriptLocation::InputLock => {
                check_owner_lock_32_bytes(authorizer_script_hash.as_slice())
            }
            ScriptLocation::InputType => {
                check_owner_type_32_bytes(authorizer_script_hash.as_slice(), Source::Input)
            }
            ScriptLocation::OutputType => {
                check_owner_type_32_bytes(authorizer_script_hash.as_slice(), Source::Output)
            }
        };

        is_authorizer_exist
    }) {
        return Ok(());
    }

    Err(Error::CheckFailed)
}
