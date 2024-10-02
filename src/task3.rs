#[test]

/*
fn main() {
    let arr: [i32; 5] = [1, 2, 3, 4, 5];
    // Fill the blanks to make the code work
    let slice: __ = __;
    assert_eq!(slice, &[2, 3, 4]);

    println!("Success!");
}
*/


fn main() {
    let arr: [i32; 5] = [1, 2, 3, 4, 5];
    // Fill the blanks to make the code work
    let slice: &[i32] = &arr[1..4];
    assert_eq!(slice, &[2, 3, 4]);

    println!("Success!");
}

/*
Чтобы код работал правильно, нужно создать срез массива, который включает элементы с индексами от 1 до 3 (включительно).
*/