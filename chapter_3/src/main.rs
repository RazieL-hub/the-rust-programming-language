fn main() {
    let mut x = 5;
    println!("x is {}", x);
    x = 6;
    println!("x is {}", x);

    const MAX_POINT: u32 = 100_000;

    let x = 5;
    let x = x + 1;
    let x = x * 2;
    println!("Значение x равно {}", x);

    // сложение
    let sum = 5 + 10;
    // вычитание
    let difference = 95.5 - 4.3;
    // умножение
    let product = 4 * 30;
    // деление
    let quotient = 56.7 / 32.2;
    // остаток
    let remainder = 43 % 5;

    let t = true;
    let f: bool = false; // с явной аннотацией типа

    let c = 'z';
    let z = 'ƶ';
    let heart_eyed_cat = '😻';
    let tup: (i32, f64, u8) = (500, 6.4, 1);
    let (x, y, z) = tup;
    println!("Значение y равно {}", x);
    println!("Значение y равно {}", y);
    println!("Значение y равно {}", z);

    let five_hundred = tup.0;
    let six_point_four = tup.1;
    let one = tup.2;
    let months = ["Январь", "Февраль", "Март", "Апрель", "Май", "Июнь", "Июль",
"Август", "Сентябрь", "Октябрь", "Ноябрь", " Декабрь"];
    let a: [i32; 5] = [1, 2, 3, 4, 5];
    let a = [3; 5]; // [3, 3, 3, 3, 3]
    another_function();
    another_function_with_args(5, 6.0);
    println!("Result function five is {}", five());
    println!("Result funtction plus on for func_five is {}", plus_one(five()));
    if five() < 5 {
        println!("Value function five is less 5")
    }
    else {
        println!("Value function five is equal or more 5")
    }

    if five() < 5 {
        println!("Value function five is less 5")
    }
    else if five() > 5 {
        println!("Value function five is more 5")
    }
    else {
        println!("Value function five is equal 5")
    }

    let condition: bool = true;
    let number: i32 = if condition {
        55
    } else {
        66
    };
    println!("NUmber is {}", number);

    let mut counter : i32 = 0;

    let result: i32 = loop {
        counter += 1;
        if counter == 10 {
            break counter * 2
        }
        else {
            println!("Counter is {}", counter)
        }
    };
    println!("REsult is after counter = {}", result);

    let mut while_counter = 3;
    while while_counter != 0 {
        println!("While_COunter - {}", while_counter);
        while_counter -= 1
    }
    let a = [10, 20, 30, 40, 50];
    let mut index = 0;
    while index < 5 {
    println!("Значение равно {}", a[index]);
    index = index + 1;
    }
    for element in a.iter() {
        println!("Element a-array is {}", element);
    }
    for num in (1..4).rev() {
        println!("Num is {}", num)
    }

}

fn another_function() {
    println!("Еще одна функция.");
}

fn another_function_with_args(x: i32, y: f64) {
    println!("Значение x = {}, значение y = {}", x, y)
}

fn five() -> i32 {
    5
}

fn plus_one(x: i32) -> i32 {
    x + 1
}