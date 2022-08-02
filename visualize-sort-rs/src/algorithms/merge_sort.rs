use crate::{
    algorithm::Algorithm,
    emit::{self, EmitResult},
};

pub struct MergeSort;

impl Algorithm for MergeSort {
    fn name(&self) -> String {
        String::from("MergeSort")
    }

    fn sort(&self, source: &mut emit::EmitVec) -> EmitResult<()> {
        Ok(())
    }
}
