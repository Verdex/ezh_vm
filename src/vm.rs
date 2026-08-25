
use std::rc::Rc;
use crate::data::{CompiledOp, CompiledProc, VmError, StackTrace};

pub struct Vm {
    stack: Vec<u8>,
    heap: Vec<u8>,
    procs: Vec<CompiledProc>,
    ip: usize,
    proc: usize,
    sp: usize,
    bp: usize,
    gp: u64,
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

        self.proc = entry;
        self.ip = 0;

        loop {
            if self.ip >= self.procs[self.proc].instrs.len() {
                // TODO: with the right construction of compiled proc this might not have to be
                // something that is even checked (will need to reject procs without returns)
                return Err(VmError::InstrPointerOutOfRange(self.ip, self.stack_trace()));
            }

            match self.procs[self.proc].instrs[self.ip] {
                CompiledOp::Address(ref dest, ref reg, offset) => {

                }, 
                CompiledOp::ProcAddress(ref dest, proc) => {

                },
                CompiledOp::Mov(ref num, ref dest, ref src) => {

                },
                CompiledOp::Jump(ip) => {

                },
                CompiledOp::Bnz(ip, ref src) => {

                },
                CompiledOp::Bz(ip, ref src) => {

                },
                CompiledOp::AllocateData { ref dest, ref size, ref align } => {

                },
                CompiledOp::Call(proc, ref args) => {

                },
                CompiledOp::DynCall(ref src, ref args) => {

                },
                CompiledOp::SysCall(sys_call, ref args) => {

                },
                CompiledOp::Add(ref num, ref dest, ref a, ref b) => {

                },
                CompiledOp::Sub(ref num, ref dest, ref a, ref b) => {

                },
                CompiledOp::Mul(ref num, ref dest, ref a, ref b) => {

                },
                CompiledOp::Div(ref num, ref dest, ref a, ref b) => {

                },
                CompiledOp::Exp(ref num, ref dest, ref a, ref b) => {

                },
                CompiledOp::Mod(ref num, ref dest, ref a, ref b) => {

                },
                CompiledOp::Neg(ref num, ref dest, ref src) => { 

                },
                CompiledOp::Eq(ref num, ref dest, ref a, ref b) => {

                },
                CompiledOp::Gt(ref num, ref dest, ref a, ref b) => {

                },
                CompiledOp::Lt(ref num, ref dest, ref a, ref b) => {

                },
                CompiledOp::And(ref num, ref dest, ref a, ref b) => {

                },
                CompiledOp::Or(ref num, ref dest, ref a, ref b) => {

                },
                CompiledOp::Xor(ref num, ref dest, ref a, ref b) => {

                },
                CompiledOp::Not(ref num, ref dest, ref src) => {

                },
            }
        }
    }

    fn stack_trace(&self) -> StackTrace {
        // Note:  Previous frames will have already incremented past the current call op
        /*self.frames.iter().map(|x| (x.id, x.ip - 1))
                          .chain(std::iter::once( (self.current.id, self.current.ip) ) )
                          .map(|(id, ip)| (Rc::clone(&self.procs[id].name), ip))
                          .collect()
                          */
        todo!()
    }
}


#[cfg(test)]
mod test { 
    use super::*;

}

