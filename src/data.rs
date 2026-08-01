

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
    Ptr,
}

#[derive(Debug)]
pub enum Align {
    A8,
    A64,
    APtr,
}

#[derive(Debug)]
pub enum Arg {
    // TODO need to be able to put arrays of constaints someplace but arg seems wrong
    ConstByte(u8),
    ConstU64(u64),
    ConstI64(i64),
    ConstF64(u64),
    Reg(Reg),
    RegDeref(Reg, isize),
}

#[derive(Debug)]
pub enum Op { 
    // TODO need a way to do "sys" calls

    Lea(Arg), 
    LeaProc(Rc<str>),
    Mov(NumType, Arg, Arg, Arg),

    Label(Rc<str>), 
    Jump(Rc<str>),
    Bnz(Rc<str>, Arg),
    Bz(Rc<str>, Arg),

    // Note:  Dest for address, Src for size, Alignment
    AllocateData(Arg, Arg, Align),

    Call(Rc<str>, Vec<Arg>),
    DynCall(Arg, Vec<Arg>),

    Add(NumType, Arg, Arg, Arg)
    Sub(NumType, Arg, Arg, Arg)
    Mul(NumType, Arg, Arg, Arg)
    Div(NumType, Arg, Arg, Arg)
    Exp(NumType, Arg, Arg, Arg)
    Mod(NumType, Arg, Arg, Arg),
    Neg(NumType, Arg, Arg)

    Eq(NumType, Arg, Arg, Arg),
    Gt(NumType, Arg, Arg, Arg),
    Lt(NumType, Arg, Arg, Arg),

    And(NumType, Arg, Arg, Arg),
    Or(NumType, Arg, Arg, Arg),
    Xor(NumType, Arg, Arg, Arg),
    Not(NumType, Arg, Arg),
}

#[derive(Debug)]
pub struct Proc { 
    pub name : Rc<str>,
    pub instrs : Vec<Op>,
}

#[derive(Debug)]
pub (crate) enum CompiledOp { 
    // TODO need a way to do "sys" calls

    Lea(Arg), 
    LeaProc(usize),
    Mov(NumType, Arg, Arg, Arg),

    Jump(usize),
    Bnz(usize, Arg),
    Bz(usize, Arg),

    // Note:  Dest for address, Src for size, Alignment
    AllocateData(Arg, Arg, Align),

    Call(usize, Vec<Arg>),
    DynCall(Arg, Vec<Arg>),

    Add(NumType, Arg, Arg, Arg)
    Sub(NumType, Arg, Arg, Arg)
    Mul(NumType, Arg, Arg, Arg)
    Div(NumType, Arg, Arg, Arg)
    Exp(NumType, Arg, Arg, Arg)
    Mod(NumType, Arg, Arg, Arg),
    Neg(NumType, Arg, Arg)

    Eq(NumType, Arg, Arg, Arg),
    Gt(NumType, Arg, Arg, Arg),
    Lt(NumType, Arg, Arg, Arg),

    And(NumType, Arg, Arg, Arg),
    Or(NumType, Arg, Arg, Arg),
    Xor(NumType, Arg, Arg, Arg),
    Not(NumType, Arg, Arg),
}

#[derive(Debug)]
pub struct CompiledProc { 
    pub name : Rc<str>,
    pub (crate) instrs : Vec<CompiledOp>,
    pub (crate) frame_align : Align,
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


