pub fn insertion_sort(arr: Vec<i32>) -> Vec<i32> {

    let mut arr = arr;
    let length: usize = arr.len();

    for i in 1..length {
        let key = arr[i];
        let mut j = i as i32 - 1;

        while j >= 0 && arr[j as usize] > key {
            arr[(j + 1) as usize] = arr[j as usize];
            j -= 1;
        }
        arr[(j + 1) as usize] = key;
    }

    return arr;

}