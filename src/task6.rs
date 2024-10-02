#[test]

/*
// Fix errors
fn main() {
    let mut s = String::from("hello world");

    // Here, &s is `&String` type, but `first_letter` needs a `&str` type.
    // It works because `&String` can be implicitly converted to `&str. If you want to know more, this is called `Deref coercion`.
    let letter = first_letter(&s);

    s.clear(); // error!

    println!("the first letter is: {}", letter);
}
fn first_letter(s: &str) -> &str {
    &s[..1]
}
*/


fn main() {
    let mut s = String::from("hello world");

    // Вызываем функцию `first_letter`, которая теперь возвращает `char`.
    let letter = first_letter(&s);

    s.clear(); // Теперь это безопасно!

    println!("the first letter is: {}", letter);
}

fn first_letter(s: &str) -> char {
    s.chars().next().unwrap() // Возвращает первый символ или вызывает панику, если `s` пустой.
}

/*
Код имеет логическую ошибку, связанную с заимствованием и временем жизни переменных.
 Ошибка возникает потому, что попытка вернуть ссылку на часть строки, которая становится недействительной после вызова s.clear().
 Это приводит к висячей ссылке, так как letter ссылается на часть s, которая больше не существует после её очистки.

Вернуть собственный тип (например, String или char) вместо ссылки.
Сохранить результат в отдельной переменной, чтобы избежать проблем с заимствованием.
*/

