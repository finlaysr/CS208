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

pub fn merge_sort<T: Ord + Copy>(array: &mut [T]) {
    if array.len() <= 1 {
        return;
    }

    let mid = array.len() / 2;
    merge_sort(&mut array[..mid]);
    merge_sort(&mut array[mid..]);

    merge_array(
        &mut array[..mid].to_vec(),
        &mut array[mid..].to_vec(),
        array,
    );
}

fn merge_array<T: Ord + Copy>(left: &mut [T], right: &mut [T], out: &mut [T]) {
    let (mut left_idx, mut right_idx, mut tmp_idx) = (0, 0, 0);

    while left_idx < left.len() && right_idx < right.len() {
        if left[left_idx] <= right[right_idx] {
            out[tmp_idx] = left[left_idx];
            left_idx += 1;
        } else {
            out[tmp_idx] = right[right_idx];
            right_idx += 1;
        }
        tmp_idx += 1;
    }

    while left_idx < left.len() {
        out[tmp_idx] = left[left_idx];
        left_idx += 1;
        tmp_idx += 1;
    }

    while right_idx < right.len() {
        out[tmp_idx] = right[right_idx];
        right_idx += 1;
        tmp_idx += 1;
    }
}
