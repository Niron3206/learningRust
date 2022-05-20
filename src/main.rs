use std::io;

fn main() {    
    println!("Выберите ваш вариант\nЦельсия в Фаренгейт - 0\nФаренгейт в Цельсия - 1");
    let mut choice = String::new();
    io::stdin()
        .read_line(&mut choice)
        .unwrap();

    let choice: u8 = choice.trim().parse().expect("Либо вы ввели не число, либо вы ввели не 0 или не 1.");

    println!("Введите градусы");
    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .unwrap();

    let mut result = String::new();
    match choice {
        0 => result = cel_to_fah(&input),
        1 => result = fah_to_cel(&input),
        _ => println!("Вы ничего не выбрали."),
    }
    println!("Ответ: {}", result);
}

fn cel_to_fah(input: &String) -> String {
    let mut input_float: f32 = input.trim().parse().expect("Вы ввели не число.");
    input_float = (input_float*9.0/5.0)+32.0;
    let output: String = input_float.to_string();
    output
}

fn fah_to_cel(input: &String) -> String {
    let mut input_float: f32 = input.trim().parse().expect("Вы ввели не число.");
    input_float = (input_float - 32.0)*5.0/9.0;
    let output: String = input_float.to_string();
    output
}