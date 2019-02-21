use std::collections::HashMap;
use std::fmt;
use std::sync::atomic::{AtomicUsize, Ordering};

pub mod constant_pool;
pub mod object;

use crate::runtime::constant_pool::CpAttr;
use crate::runtime::constant_pool::CpEntry;
use crate::runtime::object::OtObj;

//////////// RUNTIME KLASS AND RELATED HANDLING

#[derive(Clone, Debug)]
pub struct OtKlass {
    id: usize,
    name: String,
    super_name: String,
    flags: u16,
    cp_entries: Vec<CpEntry>,
    methods: Vec<OtMethod>,
    name_desc_lookup: HashMap<String, usize>,
}

impl OtKlass {
    pub fn of(
        klass_name: String,
        super_klass: String,
        flags: u16,
        cp_entries: &Vec<CpEntry>,
        methods: &Vec<OtMethod>,
    ) -> OtKlass {
        let mut lookup = HashMap::new();
        let mut i = 0;
        while i < methods.len() {
            let meth = match methods.get(i).clone() {
                Some(val) => val.clone(),
                None => panic!("Error: method {} not found on {}", i, klass_name),
            };
            lookup.insert(meth.get_fq_name_desc().clone(), i);
            i = i + 1;
        }
        dbg!(lookup.clone());
        OtKlass {
            id: 0, // This indicates that the class has not yet been loaded into a repo
            name: klass_name,
            super_name: super_klass,
            flags: flags,
            cp_entries: cp_entries.to_vec(),
            methods: methods.to_vec(),
            name_desc_lookup: lookup,
        }
    }

    pub fn set_id(&mut self, id: usize) -> () {
        self.id = id
    }

    pub fn get_id(&self) -> usize {
        self.id
    }

    // FIXME: Shouldn't this be OtField for consistency
    pub fn set_static_field(&self, _f: String, _vals: JvmValue) -> () {}

    pub fn get_name(&self) -> String {
        self.name.to_owned()
    }

    pub fn get_super_name(&self) -> String {
        self.super_name.to_owned()
    }

    pub fn get_methods(&self) -> Vec<OtMethod> {
        self.methods.clone()
    }

    // NOTE: This is fully-qualified
    pub fn get_method_by_name_and_desc(&self, name_desc: String) -> OtMethod {
        dbg!(&self.name_desc_lookup);
        let opt_idx = self.name_desc_lookup.get(&name_desc);
        let idx: usize = match opt_idx {
            Some(value) => value.clone(),
            None => panic!("Error: method {} not found on {}", name_desc, self.name),
        };
        let opt_meth = self.methods.get(idx).clone();
        match opt_meth {
            Some(val) => val.clone(),
            None => panic!("Error: method {} not found on {}", name_desc, self.name),
        }
    }

    pub fn lookup_cp(&self, cp_idx: u16) -> CpEntry {
        let idx = cp_idx as usize;
        // dbg!(&self.cp_entries);
        match self.cp_entries.get(idx).clone() {
            Some(val) => val.clone(),
            None => panic!(
                "Error: No entry found on {} at CP index {}",
                self.name, cp_idx
            ),
        }
    }

    pub fn cp_as_string(&self, i: u16) -> String {
        match self.lookup_cp(i) {
            CpEntry::utf8 { val: s } => s,
            CpEntry::class { idx: utf_idx } => self.cp_as_string(utf_idx),
            CpEntry::methodref { clz_idx, nt_idx } => {
                self.cp_as_string(clz_idx) + "." + &self.cp_as_string(nt_idx)
            }
            CpEntry::name_and_type {
                name_idx: nidx,
                type_idx: tidx,
            } => self.cp_as_string(nidx) + ":" + &self.cp_as_string(tidx),
            _ => panic!(
                "Unimplemented stringify of CP entry found in {} at index {}",
                self.name, i
            ),
        }
    }
}

// flags: u16,
// cp_entries: Vec<CpEntry>,
// name_desc_lookup: HashMap<String, usize>,
impl fmt::Display for OtKlass {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{} ISA {} with methods ", self.name, self.super_name)
    }
}

#[derive(Clone, Debug)]
pub struct OtMethod {
    klass_name: String,
    flags: u16,
    name: String,
    name_desc: String,
    name_idx: u16,
    desc_idx: u16,
    code: Vec<u8>,
    attrs: Vec<CpAttr>,
}

impl OtMethod {
    pub fn of(
        klass_name: String,
        name: String,
        desc: String,
        flags: u16,
        name_idx: u16,
        desc_idx: u16,
    ) -> OtMethod {
        let name_and_desc = name.clone() + ":" + &desc.clone();
        OtMethod {
            klass_name: klass_name.to_string(),
            flags: flags,
            name: name.clone(),
            name_desc: name_and_desc,
            attrs: Vec::new(),
            code: Vec::new(),
            // FIXME
            name_idx: desc_idx,
            desc_idx: desc_idx,
        }
    }

    pub fn set_attr(&self, _index: u16, _attr: CpAttr) -> () {}

    pub fn set_code(&mut self, code: Vec<u8>) -> () {
        self.code = code;
    }

    pub fn get_code(&self) -> Vec<u8> {
        self.code.clone()
    }

    pub fn get_klass_name(&self) -> String {
        self.klass_name.clone()
    }

    pub fn get_desc(&self) -> String {
        self.name_desc.clone()
    }

    pub fn get_fq_name_desc(&self) -> String {
        self.klass_name.clone() + "." + &self.name_desc.clone()
    }

    pub fn get_flags(&self) -> u16 {
        self.flags
    }
}

impl fmt::Display for OtMethod {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}.{}", self.klass_name, self.name_desc)
    }
}

#[derive(Debug)]
pub struct OtField {
    class_name: String,
    flags: u16,
    name_idx: u16,
    desc_idx: u16,
    name: String,
    attrs: Vec<CpAttr>,
}

impl OtField {
    pub fn of(
        klass_name: String,
        field_name: String,
        field_flags: u16,
        name: u16,
        desc: u16,
    ) -> OtField {
        OtField {
            class_name: klass_name.to_string(),
            // FIXME
            flags: field_flags,
            name_idx: name,
            desc_idx: desc,
            name: field_name,
            attrs: Vec::new(),
        }
    }

    pub fn set_attr(&self, _index: u16, _attr: CpAttr) -> () {}

    pub fn get_name(&self) -> String {
        String::from("")
    }

    pub fn get_klass(&self) -> OtKlass {
        // FIXME DUMMY
        return OtKlass {
            id: 0,
            name: "DUMMY_CLASS".to_string(),
            super_name: "DUMMY_SUPER0".to_string(),
            flags: 0,
            cp_entries: Vec::new(),
            methods: Vec::new(),
            name_desc_lookup: HashMap::new(),
        };
    }
}

impl fmt::Display for OtField {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}.{}:{}", self.class_name, self.name, self.desc_idx)
    }
}

//////////// RUNTIME VALUES

#[derive(Clone)]
pub enum JvmValue {
    Boolean { val: bool },
    Byte { val: i8 },
    Short { val: i16 },
    Int { val: i32 },
    Long { val: i64 },
    Float { val: f32 },
    Double { val: f64 },
    Char { val: char },
    ObjRef { val: OtObj },
}

impl JvmValue {
    fn name(&self) -> char {
        match *self {
            JvmValue::Boolean { val: _ } => 'Z',
            JvmValue::Byte { val: _ } => 'B',
            JvmValue::Short { val: _ } => 'S',
            JvmValue::Int { val: _ } => 'I',
            JvmValue::Long { val: _ } => 'J',
            JvmValue::Float { val: _ } => 'F',
            JvmValue::Double { val: _ } => 'D',
            JvmValue::Char { val: _ } => 'C',
            JvmValue::ObjRef { val: _ } => 'A',
        }
    }
}

impl fmt::Display for JvmValue {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            JvmValue::Boolean { val: v } => write!(f, "{}", v),
            JvmValue::Byte { val: v } => write!(f, "{}", v),
            JvmValue::Short { val: v } => write!(f, "{}", v),
            JvmValue::Int { val: v } => write!(f, "{}", v),
            JvmValue::Long { val: v } => write!(f, "{}", v),
            JvmValue::Float { val: v } => write!(f, "{}", v),
            JvmValue::Double { val: v } => write!(f, "{}", v),
            JvmValue::Char { val: v } => write!(f, "{}", v),
            JvmValue::ObjRef { val: v } => write!(f, "{}", v.clone()),
        }
    }
}

impl Default for JvmValue {
    fn default() -> JvmValue {
        JvmValue::Int { val: 0i32 }
    }
}

//////////// RUNTIME STACKS AND LOCAL VARS

pub struct InterpEvalStack {
    stack: Vec<JvmValue>,
}

impl InterpEvalStack {
    pub fn of() -> InterpEvalStack {
        InterpEvalStack { stack: Vec::new() }
    }

    pub fn push(&mut self, val: JvmValue) -> () {
        let s = &mut self.stack;
        s.push(val);
    }

    pub fn pop(&mut self) -> JvmValue {
        let s = &mut self.stack;
        match s.pop() {
            Some(value) => value,
            None => panic!("pop() on empty stack"),
        }
    }

    pub fn aconst_null(&mut self) -> () {
        self.push(JvmValue::ObjRef {
            val: OtObj::get_null(),
        });
    }

    pub fn iconst(&mut self, v: i32) -> () {
        self.push(JvmValue::Int { val: v });
    }

    pub fn iadd(&mut self) -> () {
        // For a runtime checking interpreter - type checks would go here...
        let i1 = match self.pop() {
            JvmValue::Int { val: i } => i,
            _ => panic!("Unexpected, non-integer value encountered"),
        };
        let i2 = match self.pop() {
            JvmValue::Int { val: i } => i,
            _ => panic!("Unexpected, non-integer value encountered"),
        };

        self.push(JvmValue::Int { val: i1 + i2 });
    }

    pub fn isub(&mut self) -> () {
        // For a runtime checking interpreter - type checks would go here...
        let i1 = match self.pop() {
            JvmValue::Int { val: i } => i,
            _ => panic!("Unexpected, non-integer value encountered"),
        };
        let i2 = match self.pop() {
            JvmValue::Int { val: i } => i,
            _ => panic!("Unexpected, non-integer value encountered"),
        };

        self.push(JvmValue::Int { val: i1 - i2 });
    }
    pub fn imul(&mut self) -> () {
        // For a runtime checking interpreter - type checks would go here...
        let i1 = match self.pop() {
            JvmValue::Int { val: i } => i,
            _ => panic!("Unexpected, non-integer value encountered"),
        };
        let i2 = match self.pop() {
            JvmValue::Int { val: i } => i,
            _ => panic!("Unexpected, non-integer value encountered"),
        };

        self.push(JvmValue::Int { val: i1 * i2 });
    }

    pub fn irem(&mut self) -> () {
        // For a runtime checking interpreter - type checks would go here...
        let i1 = match self.pop() {
            JvmValue::Int { val: i } => i,
            _ => panic!("Unexpected, non-integer value encountered"),
        };
        let i2 = match self.pop() {
            JvmValue::Int { val: i } => i,
            _ => panic!("Unexpected, non-integer value encountered"),
        };

        self.push(JvmValue::Int { val: i2 % i1 });
    }
    pub fn ixor(&self) -> () {}
    pub fn idiv(&mut self) -> () {
        // For a runtime checking interpreter - type checks would go here...
        let i1 = match self.pop() {
            JvmValue::Int { val: i } => i,
            _ => panic!("Unexpected, non-integer value encountered"),
        };
        let i2 = match self.pop() {
            JvmValue::Int { val: i } => i,
            _ => panic!("Unexpected, non-integer value encountered"),
        };

        self.push(JvmValue::Int { val: i2 / i1 });
    }
    pub fn iand(&self) -> () {}
    pub fn ineg(&mut self) -> () {
        let i1 = match self.pop() {
            JvmValue::Int { val: i } => i,
            _ => panic!("Unexpected, non-integer value encountered"),
        };
        self.push(JvmValue::Int { val: -i1 });
    }
    pub fn ior(&self) -> () {}

    pub fn dadd(&mut self) -> () {
        // For a runtime checking interpreter - type checks would go here...
        let i1 = match self.pop() {
            JvmValue::Double { val: i } => i,
            _ => panic!("Unexpected, non-double value encountered"),
        };
        let i2 = match self.pop() {
            JvmValue::Double { val: i } => i,
            _ => panic!("Unexpected, non-double value encountered"),
        };

        self.push(JvmValue::Double { val: i1 + i2 });
    }
    pub fn dsub(&mut self) -> () {
        // For a runtime checking interpreter - type checks would go here...
        let i1 = match self.pop() {
            JvmValue::Double { val: i } => i,
            _ => panic!("Unexpected, non-double value encountered"),
        };
        let i2 = match self.pop() {
            JvmValue::Double { val: i } => i,
            _ => panic!("Unexpected, non-double value encountered"),
        };

        self.push(JvmValue::Double { val: i1 - i2 });
    }
    pub fn dmul(&mut self) -> () {
        // For a runtime checking interpreter - type checks would go here...
        let i1 = match self.pop() {
            JvmValue::Double { val: i } => i,
            _ => panic!("Unexpected, non-double value encountered"),
        };
        let i2 = match self.pop() {
            JvmValue::Double { val: i } => i,
            _ => panic!("Unexpected, non-double value encountered"),
        };

        self.push(JvmValue::Double { val: i1 * i2 });
    }

    pub fn dconst(&mut self, v: f64) -> () {
        self.push(JvmValue::Double { val: v });
    }

    pub fn i2d(&self) -> () {}
    pub fn dup(&mut self) -> () {
        let i1 = self.pop();
        self.push(i1.to_owned());
        self.push(i1.to_owned());
    }
    pub fn dupX1(&mut self) -> () {
        let i1 = self.pop();
        let i1c = i1.clone();
        let i2 = self.pop();
        self.push(i1);
        self.push(i2);
        self.push(i1c);
    }
}

pub struct InterpLocalVars {
    lvt: Vec<JvmValue>,
}

impl InterpLocalVars {
    pub fn of(var_count: u8) -> InterpLocalVars {
        let mut out = InterpLocalVars { lvt: Vec::new() };
        for i in 0..var_count {
            out.lvt.push(JvmValue::default());
        }

        out
    }

    pub fn load(&self, idx: u8) -> JvmValue {
        self.lvt[idx as usize].clone()
    }

    pub fn store(&mut self, idx: u8, val: JvmValue) -> () {
        self.lvt[idx as usize] = val
    }

    pub fn iinc(&mut self, idx: u8, incr: u8) -> () {
        match self.lvt[idx as usize] {
            JvmValue::Int { val: v } => {
                self.lvt[idx as usize] = JvmValue::Int { val: v + 1 };
            }
            _ => panic!("Non-integer value encountered in IINC of local var {}", idx),
        }
    }
}

//////////// SHARED RUNTIME STRUCTURES

pub struct VmContext<'a> {
    heap: SharedSimpleHeap,
    repo: &'a SharedKlassRepo<'a>,
}

impl<'a> VmContext<'a> {
    pub fn of() -> VmContext<'a> {
        VmContext {
            heap: SharedSimpleHeap {},
            repo: &SharedKlassRepo::of(),
        }
    }

    pub fn get_repo(&mut self) -> &SharedKlassRepo {
        self.repo
    }

    pub fn get_heap(&mut self) -> &mut SharedSimpleHeap {
        &mut self.heap
    }

    pub fn allocate_obj(&mut self, klass: &OtKlass) -> OtObj {
        self.heap.allocate_obj(klass)
    }

    pub fn allocate_int_arr(&mut self, size: i32) -> OtObj {
        self.heap.allocate_int_arr(size)
    }
}

#[derive(Debug)]
pub struct SharedKlassRepo<'a> {
    klass_count: AtomicUsize,
    klass_lookup: HashMap<String, &'a OtKlass>,
    id_lookup: HashMap<usize, &'a OtKlass>,
}

impl<'a> SharedKlassRepo<'a> {
    pub fn of() -> SharedKlassRepo<'a> {
        SharedKlassRepo {
            klass_lookup: HashMap::new(),
            id_lookup: HashMap::new(),
            klass_count: AtomicUsize::new(1),
        }
    }

    pub fn lookup_field(&self, _klass_name: String, _idx: u16) -> OtField {
        // FIXME DUMMY
        OtField::of(
            "DUMMY_KLASS".to_string(),
            "DUMMY_FIELD".to_string(),
            0,
            1,
            2,
        )
    }

    pub fn lookup_method_exact(&self, klass_name: &String, fq_name_desc: String) -> OtMethod {
        match self.klass_lookup.get(klass_name) {
            Some(k) => k.get_method_by_name_and_desc(fq_name_desc),
            None => panic!("No klass called {} found in repo", klass_name),
        }
    }

    pub fn lookup_method_virtual(&self, klass_name: &String, _idx: u16) -> OtMethod {
        let klass = self.klass_lookup.get(klass_name);
        // FIXME DUMMY
        OtMethod::of(
            "DUMMY_KLASS".to_string(),
            "DUMMY_METH".to_string(),
            "DUMMY_DESC".to_string(),
            0,
            1,
            2,
        )
    }

    // FIXME SIG
    pub fn lookup_klass(&self, klass_name: String) -> &OtKlass {
        match self.klass_lookup.get(&klass_name) {
            Some(value) => *value,
            None => panic!("Error looking up {} - no value returned", klass_name),
        }
    }

    pub fn add_klass(&mut self, mut k: OtKlass) -> () {
        k.set_id(self.klass_count.fetch_add(1, Ordering::SeqCst));
        let id = k.get_id();
        self.klass_lookup.insert(k.get_name().clone(), &k);
        self.id_lookup.insert(id, &k);
    }
}

pub struct SharedSimpleHeap {
    // Free list
// Alloc table
}

impl SharedSimpleHeap {
    pub fn allocate_obj(&self, klass: &OtKlass) -> OtObj {
        let klassid = klass.get_id();
        OtObj::of(klassid)
    }

    pub fn allocate_int_arr(&self, size: i32) -> OtObj {
        OtObj::int_arr_of(size)
    }
}
