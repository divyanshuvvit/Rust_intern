fn find_median(arr: &[i32]) -> f64 {
    let len = arr.len();
    if len % 2 == 0 {
        let mid = len / 2;
        (arr[mid - 1] + arr[mid]) as f64 / 2.0
    } else {
        arr[len / 2] as f64
    }
}

fn main() {
    let arr_even = vec![1, 2, 3, 4, 5, 6];
    let arr_odd = vec![1, 2, 3, 4, 5];
    println!("Median of even-length array: {}", find_median(&arr_even));
    println!("Median of odd-length array: {}", find_median(&arr_odd));
}
