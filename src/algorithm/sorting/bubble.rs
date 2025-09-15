pub fn bubble_sort(arr : Vec<i32>) -> Vec<i32> { 

    let mut arr = arr;
    let n = arr.len();
    for i in 0..n-1 {
        for j in 0..n-i-2 {
            if arr[j] > arr[j+1] {
                arr.swap(j, j+1);
            }
        }
    }
    arr

}