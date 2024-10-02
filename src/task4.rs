#[test]

/*
fn main() {
    let s = String::from("hello");

    let slice1 = &s[0..2];
    // Fill the blank to make the code work, DON'T USE 0..2 again
    let slice2 = &s[__];

    assert_eq!(slice1, slice2);

    println!("Success!");
}
*/


fn main() {
    let s = String::from("hello");

    let slice1 = &s[0..2];
    // Fill the blank to make the code work, DON'T USE 0..2 again
    let slice2 = &s[..2];

    assert_eq!(slice1, slice2);

    println!("Success!");
}

/*
Для того чтобы код работал и обе переменные slice1 и slice2 содержали один и тот же срез строки "he",
можно воспользоваться диапазоном ..2, что является кратким эквивалентом записи 0..2.
*/