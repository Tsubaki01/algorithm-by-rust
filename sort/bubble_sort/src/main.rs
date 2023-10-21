use std::time::Instant;

fn main() {
    // ソートする配列
    let mut vec1: Vec<i32> = (1..=10_000).rev().collect();
    let mut vec2: Vec<i32> = (1..=10_000).rev().collect();

    // バブルソート
    let start = Instant::now();
    bubble_sort(&mut vec1);
    let duration = start.elapsed();
    println!("Time elapsed in Arugorizumu is: {:?}", duration);

    // ChatGPTのバブルソート
    let start = Instant::now();
    bubble_sort_by_chatgpt(&mut vec2);
    let duration = start.elapsed();
    println!("Time elapsed in Arugorizumu is: {:?}", duration);
}

/**
 * 自分で作ったバブルソート
 */
fn bubble_sort(arr: &mut Vec<i32>) {
    let mut count = 0;
    for _ in 0..arr.len() {
        for j in (0..arr.len()).rev() {
            if count == j {
                break;
            }
            if arr[j] < arr[j - 1] {
                (arr[j - 1], arr[j]) = (arr[j], arr[j - 1]);
            }
        }
        count += 1;
    }
}

fn bubble_sort_by_chatgpt(arr: &mut Vec<i32>) {
    let len = arr.len();
    let mut swapped;

    for i in 0..len {
        swapped = false;

        for j in 0..(len - i - 1) {
            if arr[j] > arr[j + 1] {
                arr.swap(j, j + 1);
                swapped = true;
            }
        }

        if !swapped {
            break;
        }
    }
}
