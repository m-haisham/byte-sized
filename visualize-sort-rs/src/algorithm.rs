use crate::emit::{EmitResult, EmitVec};

pub trait Algorithm {
    fn name(&self) -> String;
    fn sort(&self, source: &mut EmitVec) -> EmitResult<()>;
}
