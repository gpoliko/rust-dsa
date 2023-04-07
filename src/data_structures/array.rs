pub struct Array<T, const N: usize> {
    data: [T; N],
}

impl<T, const N: usize> Array<T, N> {
    pub fn new(value: T) -> Self where T: Copy {
        Array { data: [value; N] }
    }

    pub fn len(&self) -> usize {
        N
    }

    pub fn get(&self, index: usize) -> Option<&T> {
        if index < N {
            Some(&self.data[index])
        } else {
            None
        }
    }

    pub fn set(&mut self, index: usize, value: T) -> Option<()> {
        if index < N {
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
        let arr: Array<i32, 5> = Array::new(0);
        assert_eq!(arr.len(), 5);
        for i in 0..5 {
            assert_eq!(arr.get(i), Some(&0));
        }
    }

    #[test]
    fn test_get_and_set() {
        let mut arr: Array<i32, 3> = Array::new(0);
        assert_eq!(arr.get(0), Some(&0));
        assert_eq!(arr.set(0, 1), Some(()));
        assert_eq!(arr.get(0), Some(&1));
    }
}
