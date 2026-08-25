
use std::rc::Rc;
use crate::data::{Op, CompiledProc, VmError, StackTrace};

pub struct Vm {
    stack: Vec<u8>,
    heap: Vec<u8>,
    procs: Vec<CompiledProc>,
    ip: usize,
    proc: usize,
    stack: usize,
    base: usize,
    gen: u64,
}

impl Vm {
    // TODO: compiled procs, system calls
    pub fn new(procs : Vec<CompiledProc>) -> Vm {
        todo!()
    }

    pub fn run(&mut self, entry : usize) -> Result<usize, VmError> {
        if entry >= self.procs.len() {
            return Err(VmError::UnknownProcId(entry, self.stack_trace()));
        }

        self.ip = entry;

        loop {
            if self.current.ip >= self.procs[self.current.id].instrs.len() {
                // TODO: with the right construction of compiled proc this might not have to be
                // something that is even checked (will need to reject procs without returns)
                return Err(VmError::InstrPointerOutOfRange(self.current.ip, self.stack_trace()));
            }

            match self.procs[self.current.id].instrs[self.current.ip] {
                CompiledOp::Address(Dest, Reg, isize) => {

                }, 
                CompiledOp::ProcAddress(Dest, usize) => {

                },
                CompiledOp::Mov(NumType, Dest, Src) => {

                },
                CompiledOp::Jump(usize) => {

                },
                CompiledOp::Bnz(usize, Src) => {

                },
                CompiledOp::Bz(usize, Src) => {

                },
                CompiledOp::AllocateData { dest: Dest, size: Src, align: Src } => {

                },
                CompiledOp::Call(usize, Vec<Src>) => {

                },
                CompiledOp::DynCall(Src, Vec<Src>) => {

                },
                CompiledOp::SysCall(usize, Vec<Src>) => {

                },
                CompiledOp::Add(NumType, Dest, Src, Src) => {

                },
                CompiledOp::Sub(NumType, Dest, Src, Src) => {

                },
                CompiledOp::Mul(NumType, Dest, Src, Src) => {

                },
                CompiledOp::Div(NumType, Dest, Src, Src) => {

                },
                CompiledOp::Exp(NumType, Dest, Src, Src) => {

                },
                CompiledOp::Mod(NumType, Dest, Src, Src) => {

                },
                CompiledOp::Neg(NumType, Dest, Src) => { 

                },
                CompiledOp::Eq(NumType, Dest, Src, Src) => {

                },
                CompiledOp::Gt(NumType, Dest, Src, Src) => {

                },
                CompiledOp::Lt(NumType, Dest, Src, Src) => {

                },
                CompiledOp::And(NumType, Dest, Src, Src) => {

                },
                CompiledOp::Or(NumType, Dest, Src, Src) => {

                },
                CompiledOp::Xor(NumType, Dest, Src, Src) => {

                },
                CompiledOp::Not(NumType, Dest, Src) => {

                },
            }
        }
    }

    fn stack_trace(&self) -> StackTrace {
        // Note:  Previous frames will have already incremented past the current call op
        self.frames.iter().map(|x| (x.id, x.ip - 1))
                          .chain(std::iter::once( (self.current.id, self.current.ip) ) )
                          .map(|(id, ip)| (Rc::clone(&self.procs[id].name), ip))
                          .collect()
    }
}


#[cfg(test)]
mod test { 
    use super::*;

}

