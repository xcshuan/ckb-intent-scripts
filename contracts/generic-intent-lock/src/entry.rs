use ckb_std::{ckb_constants::Source, high_level::load_script};

use crate::{
    error::Error,
    time_lock::check_since,
    utils::{check_owner_lock_32_bytes, check_owner_type_32_bytes, ScriptLocation},
};

pub fn main() -> Result<(), Error> {
    let script = load_script()?;
    let args = script.args().raw_data();
    if args.len() < 32 + 1 + 32 + 1 + 8 {
        return Err(Error::Encoding);
    }

    let receiver_script_hash = &args[0..32];
    let receiver_location: ScriptLocation = args[32].try_into()?;
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

    let owner_script_hash = &args[33..65];
    let owner_script_location: ScriptLocation = args[65].try_into()?;
    // check owner claim expired intent
    let expire_since = u64::from_le_bytes(args[66..74].try_into().unwrap());

    if check_since(expire_since) {
        let is_owner_exist = match owner_script_location {
            ScriptLocation::InputLock => check_owner_lock_32_bytes(owner_script_hash),
            ScriptLocation::InputType => {
                check_owner_type_32_bytes(owner_script_hash, Source::Input)
            }
            ScriptLocation::OutputType => {
                check_owner_type_32_bytes(owner_script_hash, Source::Output)
            }
        };

        if is_owner_exist {
            return Ok(());
        }
    }

    Err(Error::CheckFailed)
}
