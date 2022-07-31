pub trait Algorithm {
    fn name() -> String;
    fn sort(source: &mut Vec<f32>);
}
