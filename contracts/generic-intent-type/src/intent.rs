use crate::error::Error;
use crate::generated;

use alloc::vec::Vec;
use molecule::prelude::Reader;

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

pub fn verify_intent_data(value: generated::intent::IntentDataReader) -> Result<(), Error> {
    let _: ScriptLocation = Into::<u8>::into(value.location()).try_into()?;
    let _: ScriptLocation = Into::<u8>::into(value.owner().location()).try_into()?;

    value
        .signers()
        .iter()
        .map(|signer| {
            let _: ScriptLocation = Into::<u8>::into(signer.location()).try_into()?;
            Ok(())
        })
        .collect::<Result<Vec<_>, Error>>()?;

    value
        .targets()
        .iter()
        .map(|target| match target.to_enum() {
            generated::intent::IntentTargetUnionReader::Script(_script) => Ok(()),
            generated::intent::IntentTargetUnionReader::AnotherIntent(another_intent) => {
                let intent_data = generated::intent::IntentDataReader::from_slice(
                    another_intent.intent_data().raw_data(),
                )?;
                verify_intent_data(intent_data)?;

                Ok(())
            }
        })
        .collect::<Result<Vec<_>, Error>>()?;

    Ok(())
}
