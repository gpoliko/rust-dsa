use std::clone::Clone;

pub struct Array<T: Clone> {
    data: Vec<T>,
}

impl<T: Clone> Array<T> {
    pub fn new(size: usize, default: T) -> Self {
        let data = vec![default; size];
        Array { data }
    }

    pub fn len(&self) -> usize {
        self.data.len()
    }

    pub fn get(&self, index: usize) -> Option<&T> {
        self.data.get(index)
    }

    pub fn push(&mut self, value: T) {
        self.data.push(value);
    }

    pub fn pop(&mut self) -> Option<T> {
        if self.data.is_empty() {
            None
        } else {
            Some(self.data.remove(self.data.len() - 1))
        }
    }

    pub fn is_empty(&self) -> bool {
        self.data.is_empty()
    }

    pub fn set(&mut self, index: usize, value: T) -> Option<()> {
        if index < self.data.len() {
            self.data[index] = value;
            Some(())
        } else {
            None
        }
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new_array() {
        let arr: Array<i32> = Array::new(5, 0);
        assert_eq!(arr.len(), 5);
        for i in 0..5 {
            assert_eq!(arr.get(i), Some(&0));
        }
    }

    #[test]
    fn test_array_len() {
        let arr: Array<i32> = Array::new(5, 0);
        assert_eq!(arr.len(), 5);
    }


    #[test]
    fn test_get_and_set() {
        let mut arr: Array<i32> = Array::new(3, 0);
        assert_eq!(arr.get(0), Some(&0));
        assert_eq!(arr.set(0, 1), Some(()));
        assert_eq!(arr.get(0), Some(&1));
    }


    #[test]
    fn test_push_and_pop() {
        let mut arr: Array<i32> = Array::new(1, 1);
        arr.push(1);
        arr.push(2);
        arr.push(3);
        assert_eq!(arr.pop(), Some(3));
        assert_eq!(arr.pop(), Some(2));
        assert_eq!(arr.pop(), Some(1));
        arr.push(4);
        arr.push(5);
        assert_eq!(arr.pop(), Some(5));
        assert_eq!(arr.pop(), Some(4));
    }
}
