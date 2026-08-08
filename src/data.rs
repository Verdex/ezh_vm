

use std::rc::Rc;

// locals and globals are all references
// dest, src, [src]

pub const PTR_SIZE : usize = std::mem::size_of::<usize>();

#[derive(Debug)]
pub enum Reg {
    Zero,
    Ip,
    Stack,
    Base,
    Gen,
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
pub enum Dest {
    Reg(Reg),
    RegDeref(Reg, isize),
}

#[derive(Debug)]
pub enum Src {
    ConstByte(u8),
    ConstU64(u64),
    ConstI64(i64),
    ConstF64(u64),
    Reg(Reg),
    RegDeref(Reg, isize),
}

#[derive(Debug)]
pub enum Op { 
    Address(Dest, Reg, isize), 
    ProcAddress(Dest, Rc<str>),
    Mov(NumType, Dest, Src),

    Label(Rc<str>), 
    Jump(Rc<str>),
    Bnz(Rc<str>, Src),
    Bz(Rc<str>, Src),

    AllocateData { dest: Dest, size: Src, align: Src },

    Call(Rc<str>, Vec<Src>),
    DynCall(Src, Vec<Src>),
    SysCall(Rc<str>, Vec<Src>),

    Add(NumType, Dest, Src, Src)
    Sub(NumType, Dest, Src, Src)
    Mul(NumType, Dest, Src, Src)
    Div(NumType, Dest, Src, Src)
    Exp(NumType, Dest, Src, Src)
    Mod(NumType, Dest, Src, Src),
    Neg(NumType, Dest, Src)

    Eq(NumType, Dest, Src, Src),
    Gt(NumType, Dest, Src, Src),
    Lt(NumType, Dest, Src, Src),

    And(NumType, Dest, Src, Src),
    Or(NumType, Dest, Src, Src),
    Xor(NumType, Dest, Src, Src),
    Not(NumType, Dest, Src),
}

#[derive(Debug)]
pub struct Proc { 
    pub name : Rc<str>,
    pub instrs : Vec<Op>,
}

#[derive(Debug)]
pub (crate) enum CompiledOp { 
    Address(Dest, Reg, isize), 
    ProcAddress(Dest, usize),
    Mov(NumType, Dest, Src),

    Jump(usize),
    Bnz(usize, Src),
    Bz(usize, Src),

    AllocateData { dest: Dest, size: Src, align: Src },

    Call(usize, Vec<Src>),
    DynCall(Src, Vec<Src>),
    SysCall(usize, Vec<Src>),

    Add(NumType, Dest, Src, Src)
    Sub(NumType, Dest, Src, Src)
    Mul(NumType, Dest, Src, Src)
    Div(NumType, Dest, Src, Src)
    Exp(NumType, Dest, Src, Src)
    Mod(NumType, Dest, Src, Src),
    Neg(NumType, Dest, Src)

    Eq(NumType, Dest, Src, Src),
    Gt(NumType, Dest, Src, Src),
    Lt(NumType, Dest, Src, Src),

    And(NumType, Dest, Src, Src),
    Or(NumType, Dest, Src, Src),
    Xor(NumType, Dest, Src, Src),
    Not(NumType, Dest, Src),
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


