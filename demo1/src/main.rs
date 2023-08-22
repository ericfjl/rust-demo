fn bubble_sort(mut arr: Vec<i32>) -> Vec<i32> {
    let n = arr.len();
    for i in 0..n {
        for j in 0..n - i - 1 {
            if arr[j] > arr[j + 1] {
                arr.swap(j, j + 1);
            }
        }
    }
    arr
}

fn main() {
    let arr = vec![34, 7, 23, 32, 5, 62, 30];
    let sorted_arr = bubble_sort(arr);
    println!("{:?}", sorted_arr); // Output: [5, 7, 23, 30, 32, 34, 62]
}