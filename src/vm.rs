
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
                CompiledOp::Address(dest, reg, offset) => {

                }, 
                CompiledOp::ProcAddress(dest, proc) => {

                },
                CompiledOp::Mov(num, dest, src) => {

                },
                CompiledOp::Jump(ip) => {

                },
                CompiledOp::Bnz(ip, src) => {

                },
                CompiledOp::Bz(ip, src) => {

                },
                CompiledOp::AllocateData { dest, size, align } => {

                },
                CompiledOp::Call(proc, args) => {

                },
                CompiledOp::DynCall(src, args) => {

                },
                CompiledOp::SysCall(sys_call, args) => {

                },
                CompiledOp::Add(num, dest, src, src) => {

                },
                CompiledOp::Sub(num, dest, src, src) => {

                },
                CompiledOp::Mul(num, dest, src, src) => {

                },
                CompiledOp::Div(num, dest, src, src) => {

                },
                CompiledOp::Exp(num, dest, src, src) => {

                },
                CompiledOp::Mod(num, dest, src, src) => {

                },
                CompiledOp::Neg(num, dest, src) => { 

                },
                CompiledOp::Eq(num, dest, src, src) => {

                },
                CompiledOp::Gt(num, dest, src, src) => {

                },
                CompiledOp::Lt(num, dest, src, src) => {

                },
                CompiledOp::And(num, dest, src, src) => {

                },
                CompiledOp::Or(num, dest, src, src) => {

                },
                CompiledOp::Xor(num, dest, src, src) => {

                },
                CompiledOp::Not(num, dest, src) => {

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

