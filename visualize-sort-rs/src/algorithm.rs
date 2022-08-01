use crate::emit::{EmitResult, EmitVec};

pub trait Algorithm: Send + Sync {
    fn name(&self) -> String;
    fn sort(&self, source: &mut EmitVec) -> EmitResult<()>;
}
