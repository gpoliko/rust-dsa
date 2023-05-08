pub fn selection_sort<T: Ord>(arr: &mut [T]) {
    let len = arr.len();

    for i in 0..len {
        let mut min_index = i;
        for j in i+1..len {
            if arr[j] < arr[min_index] {
                min_index = j;
            }
        }
        if min_index != i {
            arr.swap(i, min_index);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_empty_array() {
        let mut arr: [i32; 0] = [];
        selection_sort(&mut arr);
        assert!(arr.is_empty());
    }

    #[test]
    fn test_single_element_array() {
        let mut arr = [1];
        selection_sort(&mut arr);
        assert_eq!(arr, [1]);
    }

    #[test]
    fn test_sorted_array() {
        let mut arr = [1, 2, 3, 4, 5];
        selection_sort(&mut arr);
        assert_eq!(arr, [1, 2, 3, 4, 5]);
    }

    #[test]
    fn test_reverse_sorted_array() {
        let mut arr = [5, 4, 3, 2, 1];
        selection_sort(&mut arr);
        assert_eq!(arr, [1, 2, 3, 4, 5]);
    }

    #[test]
    fn test_unsorted_array() {
        let mut arr = [3, 1, 4, 2, 5];
        selection_sort(&mut arr);
        assert_eq!(arr, [1, 2, 3, 4, 5]);
    }
}
