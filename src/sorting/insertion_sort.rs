pub fn insertion_sort<T: Ord>(arr: &mut [T]) {
    for i in 1..arr.len() {
        let mut j = i;
        while j > 0 && arr[j - 1] > arr[j] {
            arr.swap(j - 1, j);
            j -= 1;
        }
    }
}

mod tests {
    use super::*;

    #[test]
    fn test_insertion_sort_empty() {
        let mut arr: [i32; 0] = [];
        insertion_sort(&mut arr);
        assert_eq!(arr, []);
    }

    #[test]
    fn test_insertion_sort_single() {
        let mut arr = [1];
        insertion_sort(&mut arr);
        assert_eq!(arr, [1]);
    }

    #[test]
    fn test_insertion_sort_sorted() {
        let mut arr = [1, 2, 3, 4];
        insertion_sort(&mut arr);
        assert_eq!(arr, [1, 2, 3, 4]);
    }

    #[test]
    fn test_insertion_sort_reversed() {
        let mut arr = [4, 3, 2, 1];
        insertion_sort(&mut arr);
        assert_eq!(arr, [1, 2, 3, 4]);
    }

    #[test]
    fn test_insertion_sort_unsorted() {
        let mut arr = [3, 1, 4, 2];
        insertion_sort(&mut arr);
        assert_eq!(arr, [1, 2, 3, 4]);
    }
}
