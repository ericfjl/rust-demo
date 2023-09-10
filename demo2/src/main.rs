mod traffic_light;
fn bubble_sort<T: PartialOrd>(mut arr: Vec<T>) -> Vec<T> {
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

fn test1(){
    
    let arr = vec![34, 7, 23, 32, 5, 62, 30];
    let sorted_arr = bubble_sort(arr);
    println!("{:?}", sorted_arr); // Output: [5, 7, 23, 30, 32, 34, 62]

    let arr_str = vec!["apple", "orange", "banana", "grape"];
    let sorted_arr_str = bubble_sort(arr_str);
    println!("{:?}", sorted_arr_str); // Output: ["apple", "banana", "grape", "orange"]
}
fn main() {

    traffic_light::start();
}