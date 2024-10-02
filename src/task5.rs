#[test]

/*

fn main() {
    let s = "你好，世界";
    // Modify this line to make the code work
    let slice = &s[0..2];

    assert!(slice == "你");

    println!("Success!");
}
*/


fn main() {
    let s = "你好，世界";
    // Modify this line to make the code work
    let slice = &s[0..3];

    assert!(slice == "你");

    println!("Success!");
}

/*
Чтобы получить правильный срез для одного символа, необходимо использовать диапазон байт,
 который соответствует первому символу. Для символа "你" это байтовый диапазон 0..3, так как он занимает 3 байта.
*/