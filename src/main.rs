#![allow(non_snake_case)]
use std::io;

mod task_for_chapt1_2;
mod task8_1;
mod task8_2;
mod task8_3;
mod task_for_chapt1_8;
mod task_for_chapt3_4;

fn main() {
    loop
    {
        print!("\n\nКакое задание:\n1.Угадайка(Глава 2)\n2.Калькулятор делителей напряжения(Глава 3-4)\n3.Задачи на базовые коллекции(Глава 8)\n4.Вычисление простых чисел (Главы 1-8)\n5.Выход\n"); 
        print!("Выбирайте:\n");
        let mut choise = String::new();

        io::stdin()
        .read_line(&mut choise)
        .expect("Траблы");

        let choise: i32 = choise.trim().parse().expect("Вводи целое число!!!");
        match choise 
        {
            1 => task_for_chapt1_2::task_for_chapt1_2(),
            2 => task_for_chapt3_4::task_for_chapt3_4(),
            3 => { 
                println!("Какое задание из Главы 8:\nВыбирайте");
                let mut choise3 = String::new();

                io::stdin()
                .read_line(&mut choise3)
                .expect("Траблы");

                let choise3: i32 = choise3.trim().parse().expect("Вводи целое число!!!");

                match choise3
                {
                    1 => task8_1::task8_1(),
                    2 => task8_2::task8_2(),
                    3 => task8_3::task8_3(),
                    _ => println!("Такого варианта нет!!!"),
                }
            }
            4 => task_for_chapt1_8::task_for_chapt1_8(),
            5 => break,
            _ => println!("Такого варианта нет!!!"),
        }
    }
}
