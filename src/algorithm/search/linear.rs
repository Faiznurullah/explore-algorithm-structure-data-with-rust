
pub fn linear_search(arr: Vec<i32>, target: i32) -> i32 {
     for i in 0..arr.len(){
        if arr[i] == target {
            return i as i32;
        }
     }

     return -1;
}