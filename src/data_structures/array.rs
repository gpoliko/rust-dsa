pub struct Array<T> {
    data: Vec<T>,
}

impl<T: Default + Clone> Array<T> {
    pub fn new(size: usize) -> Self {
        let data = vec![T::default(); size];
        Self { data}
    }

    pub fn len(&self) -> usize {
        self.data.len()
    }

    pub fn get(&self, index: usize) -> Option<&T> {
        self.data.get(index)
    }

    pub fn get_mut(&mut self, index: usize) -> Option<&mut T> {
        self.data.get_mut(index)
    }

    pub fn set(&mut self, index: usize, value: T) -> bool {
        if index >= self.len() {
            return false;
        }
        self.data[index] = value;
        true
    }

    pub fn push(&mut self, value: T) {
        self.data.push(value);
    }

    pub fn pop(&mut self) -> Option<T> {
        self.data.pop()
    }
}

pub mod array {
    pub use super::Array;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new_array() {
        let array: Array<i32> = Array::new();
        assert_eq!(array.len(), 0);
    }

    #[test]
    fn test_push_array() {
        let mut array: Array<i32> = Array::new();
        array.push(1);
        assert_eq!(array.len(), 1);
        assert_eq!(array.get(0), Some(&1));
    }

    #[test]
    fn test_pop_array() {
        let mut array: Array<i32> = Array::new();
        array.push(1);
        assert_eq!(array.pop(), Some(1));
        assert_eq!(array.len(), 0);
    }
}

