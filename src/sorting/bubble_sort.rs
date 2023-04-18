pub fn bubble_sort<T: Ord>(arr: &mut [T]) {
    let len = arr.len();
    let mut swapped = true;

    while swapped {
        swapped = false;
        for i in 1..len {
            if arr[i - 1] > arr[i] {
                arr.swap(i - 1, i);
                swapped = true;
            }
        }
        len -= 1;
    }
}
