#[derive(Debug, PartialEq)]
pub struct CustomSet<T> {
    elements: Vec<T>
}

impl<T> CustomSet<T>
    where
        T: Clone + Ord,
{
    pub fn new(input: &[T]) -> Self {
        CustomSet {
            elements: input.to_vec()
        }
    }
}