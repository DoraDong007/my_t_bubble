fn bubble_sort<T: PartialOrd>(arr: &mut [T]) {
    let mut swapped;
    let mut n = arr.len();
    while n != 0 {
        swapped = 0;
        for i in 1..n {
            if arr[i - 1] > arr[i] {
                arr.swap(i - 1, i);
                swapped = i;
            }
        }
        n = swapped;
    }
}

fn main() {
    let mut numbers = [5, 3, 2, 4, 1];
    bubble_sort(&mut numbers);
    println!("{:?}", numbers);

    let mut float_numbers = [5.5, 3.3, 2.2, 4.4, 1.1];
    bubble_sort(&mut float_numbers);
    println!("{:?}", float_numbers);
}
