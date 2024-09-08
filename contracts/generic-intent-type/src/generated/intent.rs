// Generated by Molecule 0.8.0

use ckb_std::ckb_types::packed::*;
use molecule::prelude::*;
#[derive(Clone)]
pub struct ScriptAttr(molecule::bytes::Bytes);
impl ::core::fmt::LowerHex for ScriptAttr {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        use molecule::hex_string;
        if f.alternate() {
            write!(f, "0x")?;
        }
        write!(f, "{}", hex_string(self.as_slice()))
    }
}
impl ::core::fmt::Debug for ScriptAttr {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        write!(f, "{}({:#x})", Self::NAME, self)
    }
}
impl ::core::fmt::Display for ScriptAttr {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        write!(f, "{} {{ ", Self::NAME)?;
        write!(f, "{}: {}", "location", self.location())?;
        write!(f, ", {}: {}", "script_hash", self.script_hash())?;
        write!(f, " }}")
    }
}
impl ::core::default::Default for ScriptAttr {
    fn default() -> Self {
        let v = molecule::bytes::Bytes::from_static(&Self::DEFAULT_VALUE);
        ScriptAttr::new_unchecked(v)
    }
}
impl ScriptAttr {
    const DEFAULT_VALUE: [u8; 33] = [
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        0, 0, 0,
    ];
    pub const TOTAL_SIZE: usize = 33;
    pub const FIELD_SIZES: [usize; 2] = [1, 32];
    pub const FIELD_COUNT: usize = 2;
    pub fn location(&self) -> Byte {
        Byte::new_unchecked(self.0.slice(0..1))
    }
    pub fn script_hash(&self) -> Byte32 {
        Byte32::new_unchecked(self.0.slice(1..33))
    }
    pub fn as_reader<'r>(&'r self) -> ScriptAttrReader<'r> {
        ScriptAttrReader::new_unchecked(self.as_slice())
    }
}
impl molecule::prelude::Entity for ScriptAttr {
    type Builder = ScriptAttrBuilder;
    const NAME: &'static str = "ScriptAttr";
    fn new_unchecked(data: molecule::bytes::Bytes) -> Self {
        ScriptAttr(data)
    }
    fn as_bytes(&self) -> molecule::bytes::Bytes {
        self.0.clone()
    }
    fn as_slice(&self) -> &[u8] {
        &self.0[..]
    }
    fn from_slice(slice: &[u8]) -> molecule::error::VerificationResult<Self> {
        ScriptAttrReader::from_slice(slice).map(|reader| reader.to_entity())
    }
    fn from_compatible_slice(slice: &[u8]) -> molecule::error::VerificationResult<Self> {
        ScriptAttrReader::from_compatible_slice(slice).map(|reader| reader.to_entity())
    }
    fn new_builder() -> Self::Builder {
        ::core::default::Default::default()
    }
    fn as_builder(self) -> Self::Builder {
        Self::new_builder()
            .location(self.location())
            .script_hash(self.script_hash())
    }
}
#[derive(Clone, Copy)]
pub struct ScriptAttrReader<'r>(&'r [u8]);
impl<'r> ::core::fmt::LowerHex for ScriptAttrReader<'r> {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        use molecule::hex_string;
        if f.alternate() {
            write!(f, "0x")?;
        }
        write!(f, "{}", hex_string(self.as_slice()))
    }
}
impl<'r> ::core::fmt::Debug for ScriptAttrReader<'r> {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        write!(f, "{}({:#x})", Self::NAME, self)
    }
}
impl<'r> ::core::fmt::Display for ScriptAttrReader<'r> {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        write!(f, "{} {{ ", Self::NAME)?;
        write!(f, "{}: {}", "location", self.location())?;
        write!(f, ", {}: {}", "script_hash", self.script_hash())?;
        write!(f, " }}")
    }
}
impl<'r> ScriptAttrReader<'r> {
    pub const TOTAL_SIZE: usize = 33;
    pub const FIELD_SIZES: [usize; 2] = [1, 32];
    pub const FIELD_COUNT: usize = 2;
    pub fn location(&self) -> ByteReader<'r> {
        ByteReader::new_unchecked(&self.as_slice()[0..1])
    }
    pub fn script_hash(&self) -> Byte32Reader<'r> {
        Byte32Reader::new_unchecked(&self.as_slice()[1..33])
    }
}
impl<'r> molecule::prelude::Reader<'r> for ScriptAttrReader<'r> {
    type Entity = ScriptAttr;
    const NAME: &'static str = "ScriptAttrReader";
    fn to_entity(&self) -> Self::Entity {
        Self::Entity::new_unchecked(self.as_slice().to_owned().into())
    }
    fn new_unchecked(slice: &'r [u8]) -> Self {
        ScriptAttrReader(slice)
    }
    fn as_slice(&self) -> &'r [u8] {
        self.0
    }
    fn verify(slice: &[u8], _compatible: bool) -> molecule::error::VerificationResult<()> {
        use molecule::verification_error as ve;
        let slice_len = slice.len();
        if slice_len != Self::TOTAL_SIZE {
            return ve!(Self, TotalSizeNotMatch, Self::TOTAL_SIZE, slice_len);
        }
        Ok(())
    }
}
#[derive(Clone, Debug, Default)]
pub struct ScriptAttrBuilder {
    pub(crate) location: Byte,
    pub(crate) script_hash: Byte32,
}
impl ScriptAttrBuilder {
    pub const TOTAL_SIZE: usize = 33;
    pub const FIELD_SIZES: [usize; 2] = [1, 32];
    pub const FIELD_COUNT: usize = 2;
    pub fn location(mut self, v: Byte) -> Self {
        self.location = v;
        self
    }
    pub fn script_hash(mut self, v: Byte32) -> Self {
        self.script_hash = v;
        self
    }
}
impl molecule::prelude::Builder for ScriptAttrBuilder {
    type Entity = ScriptAttr;
    const NAME: &'static str = "ScriptAttrBuilder";
    fn expected_length(&self) -> usize {
        Self::TOTAL_SIZE
    }
    fn write<W: molecule::io::Write>(&self, writer: &mut W) -> molecule::io::Result<()> {
        writer.write_all(self.location.as_slice())?;
        writer.write_all(self.script_hash.as_slice())?;
        Ok(())
    }
    fn build(&self) -> Self::Entity {
        let mut inner = Vec::with_capacity(self.expected_length());
        self.write(&mut inner)
            .unwrap_or_else(|_| panic!("{} build should be ok", Self::NAME));
        ScriptAttr::new_unchecked(inner.into())
    }
}
#[derive(Clone)]
pub struct ScriptAttrVec(molecule::bytes::Bytes);
impl ::core::fmt::LowerHex for ScriptAttrVec {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        use molecule::hex_string;
        if f.alternate() {
            write!(f, "0x")?;
        }
        write!(f, "{}", hex_string(self.as_slice()))
    }
}
impl ::core::fmt::Debug for ScriptAttrVec {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        write!(f, "{}({:#x})", Self::NAME, self)
    }
}
impl ::core::fmt::Display for ScriptAttrVec {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        write!(f, "{} [", Self::NAME)?;
        for i in 0..self.len() {
            if i == 0 {
                write!(f, "{}", self.get_unchecked(i))?;
            } else {
                write!(f, ", {}", self.get_unchecked(i))?;
            }
        }
        write!(f, "]")
    }
}
impl ::core::default::Default for ScriptAttrVec {
    fn default() -> Self {
        let v = molecule::bytes::Bytes::from_static(&Self::DEFAULT_VALUE);
        ScriptAttrVec::new_unchecked(v)
    }
}
impl ScriptAttrVec {
    const DEFAULT_VALUE: [u8; 4] = [0, 0, 0, 0];
    pub const ITEM_SIZE: usize = 33;
    pub fn total_size(&self) -> usize {
        molecule::NUMBER_SIZE + Self::ITEM_SIZE * self.item_count()
    }
    pub fn item_count(&self) -> usize {
        molecule::unpack_number(self.as_slice()) as usize
    }
    pub fn len(&self) -> usize {
        self.item_count()
    }
    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }
    pub fn get(&self, idx: usize) -> Option<ScriptAttr> {
        if idx >= self.len() {
            None
        } else {
            Some(self.get_unchecked(idx))
        }
    }
    pub fn get_unchecked(&self, idx: usize) -> ScriptAttr {
        let start = molecule::NUMBER_SIZE + Self::ITEM_SIZE * idx;
        let end = start + Self::ITEM_SIZE;
        ScriptAttr::new_unchecked(self.0.slice(start..end))
    }
    pub fn as_reader<'r>(&'r self) -> ScriptAttrVecReader<'r> {
        ScriptAttrVecReader::new_unchecked(self.as_slice())
    }
}
impl molecule::prelude::Entity for ScriptAttrVec {
    type Builder = ScriptAttrVecBuilder;
    const NAME: &'static str = "ScriptAttrVec";
    fn new_unchecked(data: molecule::bytes::Bytes) -> Self {
        ScriptAttrVec(data)
    }
    fn as_bytes(&self) -> molecule::bytes::Bytes {
        self.0.clone()
    }
    fn as_slice(&self) -> &[u8] {
        &self.0[..]
    }
    fn from_slice(slice: &[u8]) -> molecule::error::VerificationResult<Self> {
        ScriptAttrVecReader::from_slice(slice).map(|reader| reader.to_entity())
    }
    fn from_compatible_slice(slice: &[u8]) -> molecule::error::VerificationResult<Self> {
        ScriptAttrVecReader::from_compatible_slice(slice).map(|reader| reader.to_entity())
    }
    fn new_builder() -> Self::Builder {
        ::core::default::Default::default()
    }
    fn as_builder(self) -> Self::Builder {
        Self::new_builder().extend(self.into_iter())
    }
}
#[derive(Clone, Copy)]
pub struct ScriptAttrVecReader<'r>(&'r [u8]);
impl<'r> ::core::fmt::LowerHex for ScriptAttrVecReader<'r> {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        use molecule::hex_string;
        if f.alternate() {
            write!(f, "0x")?;
        }
        write!(f, "{}", hex_string(self.as_slice()))
    }
}
impl<'r> ::core::fmt::Debug for ScriptAttrVecReader<'r> {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        write!(f, "{}({:#x})", Self::NAME, self)
    }
}
impl<'r> ::core::fmt::Display for ScriptAttrVecReader<'r> {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        write!(f, "{} [", Self::NAME)?;
        for i in 0..self.len() {
            if i == 0 {
                write!(f, "{}", self.get_unchecked(i))?;
            } else {
                write!(f, ", {}", self.get_unchecked(i))?;
            }
        }
        write!(f, "]")
    }
}
impl<'r> ScriptAttrVecReader<'r> {
    pub const ITEM_SIZE: usize = 33;
    pub fn total_size(&self) -> usize {
        molecule::NUMBER_SIZE + Self::ITEM_SIZE * self.item_count()
    }
    pub fn item_count(&self) -> usize {
        molecule::unpack_number(self.as_slice()) as usize
    }
    pub fn len(&self) -> usize {
        self.item_count()
    }
    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }
    pub fn get(&self, idx: usize) -> Option<ScriptAttrReader<'r>> {
        if idx >= self.len() {
            None
        } else {
            Some(self.get_unchecked(idx))
        }
    }
    pub fn get_unchecked(&self, idx: usize) -> ScriptAttrReader<'r> {
        let start = molecule::NUMBER_SIZE + Self::ITEM_SIZE * idx;
        let end = start + Self::ITEM_SIZE;
        ScriptAttrReader::new_unchecked(&self.as_slice()[start..end])
    }
}
impl<'r> molecule::prelude::Reader<'r> for ScriptAttrVecReader<'r> {
    type Entity = ScriptAttrVec;
    const NAME: &'static str = "ScriptAttrVecReader";
    fn to_entity(&self) -> Self::Entity {
        Self::Entity::new_unchecked(self.as_slice().to_owned().into())
    }
    fn new_unchecked(slice: &'r [u8]) -> Self {
        ScriptAttrVecReader(slice)
    }
    fn as_slice(&self) -> &'r [u8] {
        self.0
    }
    fn verify(slice: &[u8], _compatible: bool) -> molecule::error::VerificationResult<()> {
        use molecule::verification_error as ve;
        let slice_len = slice.len();
        if slice_len < molecule::NUMBER_SIZE {
            return ve!(Self, HeaderIsBroken, molecule::NUMBER_SIZE, slice_len);
        }
        let item_count = molecule::unpack_number(slice) as usize;
        if item_count == 0 {
            if slice_len != molecule::NUMBER_SIZE {
                return ve!(Self, TotalSizeNotMatch, molecule::NUMBER_SIZE, slice_len);
            }
            return Ok(());
        }
        let total_size = molecule::NUMBER_SIZE + Self::ITEM_SIZE * item_count;
        if slice_len != total_size {
            return ve!(Self, TotalSizeNotMatch, total_size, slice_len);
        }
        Ok(())
    }
}
#[derive(Clone, Debug, Default)]
pub struct ScriptAttrVecBuilder(pub(crate) Vec<ScriptAttr>);
impl ScriptAttrVecBuilder {
    pub const ITEM_SIZE: usize = 33;
    pub fn set(mut self, v: Vec<ScriptAttr>) -> Self {
        self.0 = v;
        self
    }
    pub fn push(mut self, v: ScriptAttr) -> Self {
        self.0.push(v);
        self
    }
    pub fn extend<T: ::core::iter::IntoIterator<Item = ScriptAttr>>(mut self, iter: T) -> Self {
        for elem in iter {
            self.0.push(elem);
        }
        self
    }
    pub fn replace(&mut self, index: usize, v: ScriptAttr) -> Option<ScriptAttr> {
        self.0
            .get_mut(index)
            .map(|item| ::core::mem::replace(item, v))
    }
}
impl molecule::prelude::Builder for ScriptAttrVecBuilder {
    type Entity = ScriptAttrVec;
    const NAME: &'static str = "ScriptAttrVecBuilder";
    fn expected_length(&self) -> usize {
        molecule::NUMBER_SIZE + Self::ITEM_SIZE * self.0.len()
    }
    fn write<W: molecule::io::Write>(&self, writer: &mut W) -> molecule::io::Result<()> {
        writer.write_all(&molecule::pack_number(self.0.len() as molecule::Number))?;
        for inner in &self.0[..] {
            writer.write_all(inner.as_slice())?;
        }
        Ok(())
    }
    fn build(&self) -> Self::Entity {
        let mut inner = Vec::with_capacity(self.expected_length());
        self.write(&mut inner)
            .unwrap_or_else(|_| panic!("{} build should be ok", Self::NAME));
        ScriptAttrVec::new_unchecked(inner.into())
    }
}
pub struct ScriptAttrVecIterator(ScriptAttrVec, usize, usize);
impl ::core::iter::Iterator for ScriptAttrVecIterator {
    type Item = ScriptAttr;
    fn next(&mut self) -> Option<Self::Item> {
        if self.1 >= self.2 {
            None
        } else {
            let ret = self.0.get_unchecked(self.1);
            self.1 += 1;
            Some(ret)
        }
    }
}
impl ::core::iter::ExactSizeIterator for ScriptAttrVecIterator {
    fn len(&self) -> usize {
        self.2 - self.1
    }
}
impl ::core::iter::IntoIterator for ScriptAttrVec {
    type Item = ScriptAttr;
    type IntoIter = ScriptAttrVecIterator;
    fn into_iter(self) -> Self::IntoIter {
        let len = self.len();
        ScriptAttrVecIterator(self, 0, len)
    }
}
impl<'r> ScriptAttrVecReader<'r> {
    pub fn iter<'t>(&'t self) -> ScriptAttrVecReaderIterator<'t, 'r> {
        ScriptAttrVecReaderIterator(&self, 0, self.len())
    }
}
pub struct ScriptAttrVecReaderIterator<'t, 'r>(&'t ScriptAttrVecReader<'r>, usize, usize);
impl<'t: 'r, 'r> ::core::iter::Iterator for ScriptAttrVecReaderIterator<'t, 'r> {
    type Item = ScriptAttrReader<'t>;
    fn next(&mut self) -> Option<Self::Item> {
        if self.1 >= self.2 {
            None
        } else {
            let ret = self.0.get_unchecked(self.1);
            self.1 += 1;
            Some(ret)
        }
    }
}
impl<'t: 'r, 'r> ::core::iter::ExactSizeIterator for ScriptAttrVecReaderIterator<'t, 'r> {
    fn len(&self) -> usize {
        self.2 - self.1
    }
}
impl ::core::iter::FromIterator<ScriptAttr> for ScriptAttrVec {
    fn from_iter<T: IntoIterator<Item = ScriptAttr>>(iter: T) -> Self {
        Self::new_builder().extend(iter).build()
    }
}
#[derive(Clone)]
pub struct IntentData(molecule::bytes::Bytes);
impl ::core::fmt::LowerHex for IntentData {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        use molecule::hex_string;
        if f.alternate() {
            write!(f, "0x")?;
        }
        write!(f, "{}", hex_string(self.as_slice()))
    }
}
impl ::core::fmt::Debug for IntentData {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        write!(f, "{}({:#x})", Self::NAME, self)
    }
}
impl ::core::fmt::Display for IntentData {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        write!(f, "{} {{ ", Self::NAME)?;
        write!(f, "{}: {}", "type_id", self.type_id())?;
        write!(f, ", {}: {}", "receiver_location", self.receiver_location())?;
        write!(f, ", {}: {}", "owner", self.owner())?;
        write!(f, ", {}: {}", "expire_since", self.expire_since())?;
        write!(f, ", {}: {}", "authorizers", self.authorizers())?;
        write!(f, ", {}: {}", "targets", self.targets())?;
        write!(f, ", {}: {}", "input_data", self.input_data())?;
        let extra_count = self.count_extra_fields();
        if extra_count != 0 {
            write!(f, ", .. ({} fields)", extra_count)?;
        }
        write!(f, " }}")
    }
}
impl ::core::default::Default for IntentData {
    fn default() -> Self {
        let v = molecule::bytes::Bytes::from_static(&Self::DEFAULT_VALUE);
        IntentData::new_unchecked(v)
    }
}
impl IntentData {
    const DEFAULT_VALUE: [u8; 118] = [
        118, 0, 0, 0, 32, 0, 0, 0, 64, 0, 0, 0, 65, 0, 0, 0, 98, 0, 0, 0, 106, 0, 0, 0, 110, 0, 0,
        0, 114, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 4, 0, 0, 0, 0, 0, 0,
        0,
    ];
    pub const FIELD_COUNT: usize = 7;
    pub fn total_size(&self) -> usize {
        molecule::unpack_number(self.as_slice()) as usize
    }
    pub fn field_count(&self) -> usize {
        if self.total_size() == molecule::NUMBER_SIZE {
            0
        } else {
            (molecule::unpack_number(&self.as_slice()[molecule::NUMBER_SIZE..]) as usize / 4) - 1
        }
    }
    pub fn count_extra_fields(&self) -> usize {
        self.field_count() - Self::FIELD_COUNT
    }
    pub fn has_extra_fields(&self) -> bool {
        Self::FIELD_COUNT != self.field_count()
    }
    pub fn type_id(&self) -> Byte32 {
        let slice = self.as_slice();
        let start = molecule::unpack_number(&slice[4..]) as usize;
        let end = molecule::unpack_number(&slice[8..]) as usize;
        Byte32::new_unchecked(self.0.slice(start..end))
    }
    pub fn receiver_location(&self) -> Byte {
        let slice = self.as_slice();
        let start = molecule::unpack_number(&slice[8..]) as usize;
        let end = molecule::unpack_number(&slice[12..]) as usize;
        Byte::new_unchecked(self.0.slice(start..end))
    }
    pub fn owner(&self) -> ScriptAttr {
        let slice = self.as_slice();
        let start = molecule::unpack_number(&slice[12..]) as usize;
        let end = molecule::unpack_number(&slice[16..]) as usize;
        ScriptAttr::new_unchecked(self.0.slice(start..end))
    }
    pub fn expire_since(&self) -> Uint64 {
        let slice = self.as_slice();
        let start = molecule::unpack_number(&slice[16..]) as usize;
        let end = molecule::unpack_number(&slice[20..]) as usize;
        Uint64::new_unchecked(self.0.slice(start..end))
    }
    pub fn authorizers(&self) -> ScriptAttrVec {
        let slice = self.as_slice();
        let start = molecule::unpack_number(&slice[20..]) as usize;
        let end = molecule::unpack_number(&slice[24..]) as usize;
        ScriptAttrVec::new_unchecked(self.0.slice(start..end))
    }
    pub fn targets(&self) -> BytesVec {
        let slice = self.as_slice();
        let start = molecule::unpack_number(&slice[24..]) as usize;
        let end = molecule::unpack_number(&slice[28..]) as usize;
        BytesVec::new_unchecked(self.0.slice(start..end))
    }
    pub fn input_data(&self) -> Bytes {
        let slice = self.as_slice();
        let start = molecule::unpack_number(&slice[28..]) as usize;
        if self.has_extra_fields() {
            let end = molecule::unpack_number(&slice[32..]) as usize;
            Bytes::new_unchecked(self.0.slice(start..end))
        } else {
            Bytes::new_unchecked(self.0.slice(start..))
        }
    }
    pub fn as_reader<'r>(&'r self) -> IntentDataReader<'r> {
        IntentDataReader::new_unchecked(self.as_slice())
    }
}
impl molecule::prelude::Entity for IntentData {
    type Builder = IntentDataBuilder;
    const NAME: &'static str = "IntentData";
    fn new_unchecked(data: molecule::bytes::Bytes) -> Self {
        IntentData(data)
    }
    fn as_bytes(&self) -> molecule::bytes::Bytes {
        self.0.clone()
    }
    fn as_slice(&self) -> &[u8] {
        &self.0[..]
    }
    fn from_slice(slice: &[u8]) -> molecule::error::VerificationResult<Self> {
        IntentDataReader::from_slice(slice).map(|reader| reader.to_entity())
    }
    fn from_compatible_slice(slice: &[u8]) -> molecule::error::VerificationResult<Self> {
        IntentDataReader::from_compatible_slice(slice).map(|reader| reader.to_entity())
    }
    fn new_builder() -> Self::Builder {
        ::core::default::Default::default()
    }
    fn as_builder(self) -> Self::Builder {
        Self::new_builder()
            .type_id(self.type_id())
            .receiver_location(self.receiver_location())
            .owner(self.owner())
            .expire_since(self.expire_since())
            .authorizers(self.authorizers())
            .targets(self.targets())
            .input_data(self.input_data())
    }
}
#[derive(Clone, Copy)]
pub struct IntentDataReader<'r>(&'r [u8]);
impl<'r> ::core::fmt::LowerHex for IntentDataReader<'r> {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        use molecule::hex_string;
        if f.alternate() {
            write!(f, "0x")?;
        }
        write!(f, "{}", hex_string(self.as_slice()))
    }
}
impl<'r> ::core::fmt::Debug for IntentDataReader<'r> {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        write!(f, "{}({:#x})", Self::NAME, self)
    }
}
impl<'r> ::core::fmt::Display for IntentDataReader<'r> {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        write!(f, "{} {{ ", Self::NAME)?;
        write!(f, "{}: {}", "type_id", self.type_id())?;
        write!(f, ", {}: {}", "receiver_location", self.receiver_location())?;
        write!(f, ", {}: {}", "owner", self.owner())?;
        write!(f, ", {}: {}", "expire_since", self.expire_since())?;
        write!(f, ", {}: {}", "authorizers", self.authorizers())?;
        write!(f, ", {}: {}", "targets", self.targets())?;
        write!(f, ", {}: {}", "input_data", self.input_data())?;
        let extra_count = self.count_extra_fields();
        if extra_count != 0 {
            write!(f, ", .. ({} fields)", extra_count)?;
        }
        write!(f, " }}")
    }
}
impl<'r> IntentDataReader<'r> {
    pub const FIELD_COUNT: usize = 7;
    pub fn total_size(&self) -> usize {
        molecule::unpack_number(self.as_slice()) as usize
    }
    pub fn field_count(&self) -> usize {
        if self.total_size() == molecule::NUMBER_SIZE {
            0
        } else {
            (molecule::unpack_number(&self.as_slice()[molecule::NUMBER_SIZE..]) as usize / 4) - 1
        }
    }
    pub fn count_extra_fields(&self) -> usize {
        self.field_count() - Self::FIELD_COUNT
    }
    pub fn has_extra_fields(&self) -> bool {
        Self::FIELD_COUNT != self.field_count()
    }
    pub fn type_id(&self) -> Byte32Reader<'r> {
        let slice = self.as_slice();
        let start = molecule::unpack_number(&slice[4..]) as usize;
        let end = molecule::unpack_number(&slice[8..]) as usize;
        Byte32Reader::new_unchecked(&self.as_slice()[start..end])
    }
    pub fn receiver_location(&self) -> ByteReader<'r> {
        let slice = self.as_slice();
        let start = molecule::unpack_number(&slice[8..]) as usize;
        let end = molecule::unpack_number(&slice[12..]) as usize;
        ByteReader::new_unchecked(&self.as_slice()[start..end])
    }
    pub fn owner(&self) -> ScriptAttrReader<'r> {
        let slice = self.as_slice();
        let start = molecule::unpack_number(&slice[12..]) as usize;
        let end = molecule::unpack_number(&slice[16..]) as usize;
        ScriptAttrReader::new_unchecked(&self.as_slice()[start..end])
    }
    pub fn expire_since(&self) -> Uint64Reader<'r> {
        let slice = self.as_slice();
        let start = molecule::unpack_number(&slice[16..]) as usize;
        let end = molecule::unpack_number(&slice[20..]) as usize;
        Uint64Reader::new_unchecked(&self.as_slice()[start..end])
    }
    pub fn authorizers(&self) -> ScriptAttrVecReader<'r> {
        let slice = self.as_slice();
        let start = molecule::unpack_number(&slice[20..]) as usize;
        let end = molecule::unpack_number(&slice[24..]) as usize;
        ScriptAttrVecReader::new_unchecked(&self.as_slice()[start..end])
    }
    pub fn targets(&self) -> BytesVecReader<'r> {
        let slice = self.as_slice();
        let start = molecule::unpack_number(&slice[24..]) as usize;
        let end = molecule::unpack_number(&slice[28..]) as usize;
        BytesVecReader::new_unchecked(&self.as_slice()[start..end])
    }
    pub fn input_data(&self) -> BytesReader<'r> {
        let slice = self.as_slice();
        let start = molecule::unpack_number(&slice[28..]) as usize;
        if self.has_extra_fields() {
            let end = molecule::unpack_number(&slice[32..]) as usize;
            BytesReader::new_unchecked(&self.as_slice()[start..end])
        } else {
            BytesReader::new_unchecked(&self.as_slice()[start..])
        }
    }
}
impl<'r> molecule::prelude::Reader<'r> for IntentDataReader<'r> {
    type Entity = IntentData;
    const NAME: &'static str = "IntentDataReader";
    fn to_entity(&self) -> Self::Entity {
        Self::Entity::new_unchecked(self.as_slice().to_owned().into())
    }
    fn new_unchecked(slice: &'r [u8]) -> Self {
        IntentDataReader(slice)
    }
    fn as_slice(&self) -> &'r [u8] {
        self.0
    }
    fn verify(slice: &[u8], compatible: bool) -> molecule::error::VerificationResult<()> {
        use molecule::verification_error as ve;
        let slice_len = slice.len();
        if slice_len < molecule::NUMBER_SIZE {
            return ve!(Self, HeaderIsBroken, molecule::NUMBER_SIZE, slice_len);
        }
        let total_size = molecule::unpack_number(slice) as usize;
        if slice_len != total_size {
            return ve!(Self, TotalSizeNotMatch, total_size, slice_len);
        }
        if slice_len < molecule::NUMBER_SIZE * 2 {
            return ve!(Self, HeaderIsBroken, molecule::NUMBER_SIZE * 2, slice_len);
        }
        let offset_first = molecule::unpack_number(&slice[molecule::NUMBER_SIZE..]) as usize;
        if offset_first % molecule::NUMBER_SIZE != 0 || offset_first < molecule::NUMBER_SIZE * 2 {
            return ve!(Self, OffsetsNotMatch);
        }
        if slice_len < offset_first {
            return ve!(Self, HeaderIsBroken, offset_first, slice_len);
        }
        let field_count = offset_first / molecule::NUMBER_SIZE - 1;
        if field_count < Self::FIELD_COUNT {
            return ve!(Self, FieldCountNotMatch, Self::FIELD_COUNT, field_count);
        } else if !compatible && field_count > Self::FIELD_COUNT {
            return ve!(Self, FieldCountNotMatch, Self::FIELD_COUNT, field_count);
        };
        let mut offsets: Vec<usize> = slice[molecule::NUMBER_SIZE..offset_first]
            .chunks_exact(molecule::NUMBER_SIZE)
            .map(|x| molecule::unpack_number(x) as usize)
            .collect();
        offsets.push(total_size);
        if offsets.windows(2).any(|i| i[0] > i[1]) {
            return ve!(Self, OffsetsNotMatch);
        }
        Byte32Reader::verify(&slice[offsets[0]..offsets[1]], compatible)?;
        ByteReader::verify(&slice[offsets[1]..offsets[2]], compatible)?;
        ScriptAttrReader::verify(&slice[offsets[2]..offsets[3]], compatible)?;
        Uint64Reader::verify(&slice[offsets[3]..offsets[4]], compatible)?;
        ScriptAttrVecReader::verify(&slice[offsets[4]..offsets[5]], compatible)?;
        BytesVecReader::verify(&slice[offsets[5]..offsets[6]], compatible)?;
        BytesReader::verify(&slice[offsets[6]..offsets[7]], compatible)?;
        Ok(())
    }
}
#[derive(Clone, Debug, Default)]
pub struct IntentDataBuilder {
    pub(crate) type_id: Byte32,
    pub(crate) receiver_location: Byte,
    pub(crate) owner: ScriptAttr,
    pub(crate) expire_since: Uint64,
    pub(crate) authorizers: ScriptAttrVec,
    pub(crate) targets: BytesVec,
    pub(crate) input_data: Bytes,
}
impl IntentDataBuilder {
    pub const FIELD_COUNT: usize = 7;
    pub fn type_id(mut self, v: Byte32) -> Self {
        self.type_id = v;
        self
    }
    pub fn receiver_location(mut self, v: Byte) -> Self {
        self.receiver_location = v;
        self
    }
    pub fn owner(mut self, v: ScriptAttr) -> Self {
        self.owner = v;
        self
    }
    pub fn expire_since(mut self, v: Uint64) -> Self {
        self.expire_since = v;
        self
    }
    pub fn authorizers(mut self, v: ScriptAttrVec) -> Self {
        self.authorizers = v;
        self
    }
    pub fn targets(mut self, v: BytesVec) -> Self {
        self.targets = v;
        self
    }
    pub fn input_data(mut self, v: Bytes) -> Self {
        self.input_data = v;
        self
    }
}
impl molecule::prelude::Builder for IntentDataBuilder {
    type Entity = IntentData;
    const NAME: &'static str = "IntentDataBuilder";
    fn expected_length(&self) -> usize {
        molecule::NUMBER_SIZE * (Self::FIELD_COUNT + 1)
            + self.type_id.as_slice().len()
            + self.receiver_location.as_slice().len()
            + self.owner.as_slice().len()
            + self.expire_since.as_slice().len()
            + self.authorizers.as_slice().len()
            + self.targets.as_slice().len()
            + self.input_data.as_slice().len()
    }
    fn write<W: molecule::io::Write>(&self, writer: &mut W) -> molecule::io::Result<()> {
        let mut total_size = molecule::NUMBER_SIZE * (Self::FIELD_COUNT + 1);
        let mut offsets = Vec::with_capacity(Self::FIELD_COUNT);
        offsets.push(total_size);
        total_size += self.type_id.as_slice().len();
        offsets.push(total_size);
        total_size += self.receiver_location.as_slice().len();
        offsets.push(total_size);
        total_size += self.owner.as_slice().len();
        offsets.push(total_size);
        total_size += self.expire_since.as_slice().len();
        offsets.push(total_size);
        total_size += self.authorizers.as_slice().len();
        offsets.push(total_size);
        total_size += self.targets.as_slice().len();
        offsets.push(total_size);
        total_size += self.input_data.as_slice().len();
        writer.write_all(&molecule::pack_number(total_size as molecule::Number))?;
        for offset in offsets.into_iter() {
            writer.write_all(&molecule::pack_number(offset as molecule::Number))?;
        }
        writer.write_all(self.type_id.as_slice())?;
        writer.write_all(self.receiver_location.as_slice())?;
        writer.write_all(self.owner.as_slice())?;
        writer.write_all(self.expire_since.as_slice())?;
        writer.write_all(self.authorizers.as_slice())?;
        writer.write_all(self.targets.as_slice())?;
        writer.write_all(self.input_data.as_slice())?;
        Ok(())
    }
    fn build(&self) -> Self::Entity {
        let mut inner = Vec::with_capacity(self.expected_length());
        self.write(&mut inner)
            .unwrap_or_else(|_| panic!("{} build should be ok", Self::NAME));
        IntentData::new_unchecked(inner.into())
    }
}
