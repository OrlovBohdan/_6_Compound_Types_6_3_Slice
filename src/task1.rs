#[test]

/*

// Fix the errors, DON'T add new lines!
fn main() {
    let arr = [1, 2, 3];
    let s1: [i32] = arr[0..2];

    let s2: str = "hello, world" as str;

    println!("Success!");
}
*/


// Fix the errors, DON'T add new lines!
fn main() {
    let arr = [1, 2, 3];
    let _s1: &[i32] = &arr[0..2];

    let _s2: &str = "hello, world";

    println!("Success!");
}

/*
Для создания среза массива требуется использовать тип среза &[i32], а не просто [i32].
Для работы со строковыми срезами нужно использовать тип &str, а не str.
*/