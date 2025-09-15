pub fn selection_sort(arr: Vec<i32>) -> Vec<i32> {

    let length: usize = arr.len();
    let mut arr = arr;

    for i in 0..length-1 {
        let mut min_index = i;
        for j in i+1..length{
            if arr[j] < arr[min_index] {
                min_index = j;
            }
        }
        arr.swap(i, min_index);
    }

    return arr;

}