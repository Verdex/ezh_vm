

use std::rc::Rc;

// locals and globals are all references
// dest, src, [src]

pub const F64_SIZE : usize = std::mem::size_of::<f64>();
pub const I64_SIZE : usize = std::mem::size_of::<i64>();
pub const PTR_SIZE : usize = std::mem::size_of::<usize>();
pub const OFFSET_SIZE : usize = std::mem::size_of::<isize>();
pub const BOOL_SIZE : usize = std::mem::size_of::<bool>();

#[derive(Debug)]
pub struct Data(pub (crate) Vec<u8>);

#[derive(Debug)]
pub enum Reg {
    Gen64,
    Gen32,
    Gen16,
    Gen8,
    FGen64,
    FGen32,
    Ip,
    Stack,
    Base,
}

#[derive(Debug)]
pub enum Op<ID> { // TODO: ID might not make sense anymore?
    // TODO need a way to do "sys" calls
    // TODO label?
    //
    // TODO lea

    Jump(ID),
    BranchTrue(ID, ID), // TODO Branch not zero and branch zero ?

    AllocateData(ID, usize),
    EndOfMemory(ID),
    DropMemory(ID),

    Coroutine(ID, ID, Vec<ID>), 
    Resume(ID),
    Yield(ID),
    Finish(ID, ID),

    Call(ID, Vec<ID>),
    DynCall(ID, Vec<ID>),

    // Dest pointer, source pointer, source offset

    FAdd(ID, ID, ID),
    FSub(ID, ID, ID),
    FMul(ID, ID, ID),
    FDiv(ID, ID, ID),
    FExp(ID, ID, ID),
    FNeg(ID, ID),

    FEq(ID, ID, ID),
    FGt(ID, ID, ID),
    FLt(ID, ID, ID),

    IAdd(ID, ID, ID),
    ISub(ID, ID, ID),
    IMul(ID, ID, ID),
    IDiv(ID, ID, ID),
    IMod(ID, ID, ID),
    INeg(ID, ID),

    IEq(ID, ID, ID),
    IGt(ID, ID, ID),
    ILt(ID, ID, ID),

    // TODO ? (also binary not, and, or, xor ?)
    LNot(ID, ID),
    LAnd(ID, ID, ID),
    LOr(ID, ID, ID),
    LXor(ID, ID, ID),
    LEq(ID, ID, ID),
}

pub fn int64(x: i64) -> Data {
    Data(i64::to_ne_bytes(x).to_vec())
}

pub fn float64(x: f64) -> Data {
    Data(f64::to_ne_bytes(x).to_vec())
}

pub fn bool(x: bool) -> Data {
    Data(vec![x as u8])
}

pub fn offset(x: isize) -> Data {
    Data(isize::to_ne_bytes(x).to_vec())
}

#[derive(Debug)]
pub struct Proc { 
    pub name : Rc<str>,
    pub instrs : Vec<Op<Rc<str>>>,
}

#[derive(Debug)]
pub struct CompiledProc { 
    pub name : Rc<str>,
    pub (crate) instrs : Vec<Op<usize>>,
    pub (crate) slot_names : Vec<Rc<str>>,
    pub (crate) frame_size : usize,
}


pub type StackTrace = Vec<(Rc<str>, usize)>;

#[derive(Debug)]
pub enum VmError {
    UnknownProcId(usize, StackTrace),
    InstrPointerOutOfRange(usize, StackTrace),
    MemoryAccessOutOfRange(usize, StackTrace),
    SetMemoryOutOfRange(usize, usize, StackTrace),
    BinMathOp(StackTrace),
    ReturnDoesNotExist(StackTrace),
    MemoryDroppedOutOfRange { available: usize, attempt: usize, st: StackTrace },
}

impl std::fmt::Display for VmError {
    fn fmt(&self, f : &mut std::fmt::Formatter) -> std::fmt::Result {
        fn d(x : &StackTrace) -> String {
            x.into_iter().map(|(n, i)| format!("    {} at index {}\n", n, i)).collect()
        }
        match self { 
            VmError::UnknownProcId(id, st) => write!(f, "encountered unknown proc id: {}\n{}", id, d(st)),
            VmError::InstrPointerOutOfRange(ip, st) => write!(f, "encountered instruction pointer past proc length: {}\n{}", ip, d(st)),
            VmError::MemoryAccessOutOfRange(addr, st) => write!(f, "memory access out of range: {}\n{}", addr, d(st)),
            VmError::SetMemoryOutOfRange(addr, len, st) => write!(f, "set memory out of range: {} of length: {}\n{}", addr, len, d(st)),
            VmError::BinMathOp(st) => write!(f, "error with binary operator\n{}", d(st)),
            VmError::ReturnDoesNotExist(st) => write!(f, "Return does not exist\n{}", d(st)),
            VmError::MemoryDroppedOutOfRange { available, attempt, st} => write!(f, "Memory 'dropped' out of range:  Available: {}, attempted: {}\n{}", available, attempt, d(st)),
        }
    }
}

impl std::error::Error for VmError { }


