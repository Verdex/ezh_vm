

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
    Zero,
    Ip,
    Stack,
    Base,
}

#[derive(Debug)]
pub enum NumType {
    Byte,
    U64, 
    I64,
    F64,
}

#[derive(Debug)]
pub enum Align {
    A8,
    A64,
}

#[derive(Debug)]
pub enum Arg {
    Const(i64),
    Reg(Reg),
    RegDeref(Reg, isize),
}

#[derive(Debug)]
pub enum Op<Id> { 
    // TODO need a way to do "sys" calls

    Lea(Arg), 
    Leaf(Id),
    Mov(NumType, Arg, Arg, Arg),

    Label(Id), // Note: This shouldn't be allowed to appear in a compiled proc
    Jump(Id),
    Bnz(Id, Arg),
    Bz(Id, Arg),

    AllocateData(Arg, Align),

    Call(Id, Vec<Arg>),
    DynCall(Arg, Vec<Arg>),

    // Dest pointer, source pointer, source offset

    Add(NumType, Arg, Arg, Arg)
    Sub(NumType, Arg, Arg, Arg)
    Mul(NumType, Arg, Arg, Arg)
    Div(NumType, Arg, Arg, Arg)
    Exp(NumType, Arg, Arg, Arg)
    Mod(NumType, Arg, Arg, Arg),
    Neg(NumType, Arg, Arg)

    Eq(NumType, Arg, Arg, Id),
    Gt(NumType, Id, Id, Id),
    Lt(NumType, Id, Id, Id),


    // TODO ? (also binary not, and, or, xor ?)
    LNot(L, L),
    LAnd(L, L, L),
    LOr(L, L, L),
    LXor(L, L, L),
    LEq(L, L, L),
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


