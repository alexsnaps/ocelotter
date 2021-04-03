use std::fmt;

use crate::constant_pool::CpAttr;
use crate::constant_pool::ACC_STATIC;
use crate::JvmValue;

#[derive(Debug, Clone)]
pub struct OtField {
    // We store the klass_name rather than the klass's id because when
    // the OtField is created, it is too early - the klass doesn't have
    // an id yet
    offset: u16,
    klass_name: String,
    flags: u16,
    name_idx: u16,
    desc_idx: u16,
    name: String,
    desc: String,
    attrs: Vec<CpAttr>,
}

impl OtField {
    pub fn of(
        offset: u16,
        klass_name: String,
        field_name: String,
        field_desc: String,
        field_flags: u16,
        name: u16,
        desc: u16,
    ) -> OtField {
        OtField {
            offset: offset,
            klass_name: klass_name.to_string(),
            // FIXME
            flags: field_flags,
            name_idx: name,
            desc_idx: desc,
            name: field_name,
            desc: field_desc,
            attrs: Vec::new(),
        }
    }

    pub fn get_offset(&self) -> u16 {
        self.offset
    }

    pub fn set_attr(&self, _index: u16, _attr: CpAttr) -> () {}

    pub fn get_name(&self) -> String {
        self.name.clone()
    }

    pub fn is_static(&self) -> bool {
        self.flags & ACC_STATIC == ACC_STATIC
    }

    pub fn get_klass_name(&self) -> String {
        self.klass_name.clone()
    }

    pub fn get_fq_name_desc(&self) -> String {
        self.klass_name.clone() + "." + &self.name + ":" + &self.desc
    }

    pub fn get_default(&self) -> JvmValue {
        match self.desc.as_str() {
            "Z" => JvmValue::Boolean { val: false },
            "B" => JvmValue::Byte { val: 0 },
            "S" => JvmValue::Short { val: 0 },
            "C" => JvmValue::Char { val: '\0' },
            "I" => JvmValue::Int { val: 0i32 },
            "J" => JvmValue::Long { val: 0i64 },
            "F" => JvmValue::Float { val: 0.0 },
            "D" => JvmValue::Double { val: 0.0 },
            _ => JvmValue::ObjRef { val: 0 },
        }
    }
}

impl fmt::Display for OtField {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}.{}:{} at {} with {:?}", self.klass_name, self.name, self.desc, self.offset, self.attrs)
    }
}
