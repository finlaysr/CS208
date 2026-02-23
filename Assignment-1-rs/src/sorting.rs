use std::fmt::Debug;

pub fn selection_sort<T: Ord>(array: &mut [T]) {
    let len = array.len();
    for i in 0..len {
        let mut min_index = i;
        for j in i + 1..len {
            if array[j] < array[min_index] {
                min_index = j;
            }
        }

        if min_index != i {
            array.swap(i, min_index);
        }
    }
}

pub fn merge_sort<T: Ord + Copy + Debug>(array: &mut [T]) {
    let right = array.len() - 1;
    merge_sort_args(array, 0, right);
}

fn merge_sort_args<T: Ord + Copy + Debug>(array: &mut [T], left: usize, right: usize) {
    if left < right {
        let mid: usize = (left + right) / 2;

        merge_sort_args(array, left, mid);
        merge_sort_args(array, mid + 1, right);

        merge_array(array, left, mid, right);
    }
}

fn merge_array<T: Ord + Copy + Debug>(array: &mut [T], left: usize, mid: usize, right: usize) {
    let mut tmp_arr: Vec<Option<T>> = vec![];
    tmp_arr.resize(right - left + 1, None);

    let mut left_index = left;
    let mut right_index = mid + 1;
    let mut tmp_index = 0;

    while left_index <= mid && right_index <= right {
        if array[left_index] < array[right_index] {
            tmp_arr[tmp_index] = Some(array[left_index]);
            left_index += 1;
        } else {
            tmp_arr[tmp_index] = Some(array[right_index]);
            right_index += 1;
        }
        tmp_index += 1;
    }

    while left_index <= mid {
        tmp_arr[tmp_index] = Some(array[left_index]);
        tmp_index += 1;
        left_index += 1;
    }

    while right_index <= right {
        tmp_arr[tmp_index] = Some(array[right_index]);
        tmp_index += 1;
        right_index += 1;
    }

    let tmp_arr: Vec<T> = tmp_arr
        .iter()
        .map(|x| x.expect("Value unitialized!"))
        .collect();

    for i in 0..tmp_arr.len() {
        array[left + i] = tmp_arr[i];
    }
}
