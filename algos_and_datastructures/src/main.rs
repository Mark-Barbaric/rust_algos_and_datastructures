use algos_and_datastructures::algos::binary_search1;

fn main() {
    let nums1 = vec![1, 5, 7, 22, 35];
    let target = 7;
    let pos = binary_search1::bs(7, nums1);

    if pos == -1 {
        println!("Couldn't find {}", target);
    } else {
        println!("Found target at index {}", pos);
    }
}