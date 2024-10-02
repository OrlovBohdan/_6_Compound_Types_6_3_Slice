#[test]

/*

fn main() {
    let arr: [char; 3] = ['中', '国', '人'];

    let slice = &arr[..2];

    // Modify '8' to make it work
    // TIPS: slice( reference ) IS NOT an array, if it is an array, then `assert!` will be passed: Each of the two chars '中' and '国'  occupies 4 bytes, 2 * 4 = 8
    assert!(std::mem::size_of_val(&slice) == 8);

    println!("Success!");
}
*/


fn main() {
    let arr: [char; 3] = ['中', '国', '人'];

    let slice = &arr[..2];

    // Modify '8' to make it work
    // TIPS: slice( reference ) IS NOT an array, if it is an array, then `assert!` will be passed: Each of the two chars '中' and '国'  occupies 4 bytes, 2 * 4 = 8
    assert!(std::mem::size_of_val(&slice) == 16);

    println!("Success!");
}

/*
Ошибка в вашем коде возникает из-за неверного ожидания размера среза.
Срез не является массивом, и его размер не равен сумме размеров его элементов.
Вместо этого срез состоит из указателя на данные и длины, и его размер обычно составляет 16 байт на 64-битных системах
(8 байт для указателя и 8 байт для длины). Следовательно, правильное значение для условия должно быть 16.
*/