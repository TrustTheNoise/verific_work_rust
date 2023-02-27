use std::io;
use colored::*;

fn isPrime(i: i32, arr: &Vec<i32>) -> bool
{
    for n in arr
    {
        let sqrt = (i as f32).sqrt();
        if (*n as f32) > sqrt
        {
            return true;
        }
        if i % *n == 0{
            return false;
        }
    }
    return true
}

pub fn task_for_chapt1_8() {
    
    println!("Напишите число до которого хотите просмотреть простые числа:");
    let mut num = String::new();
    io::stdin()
    .read_line(&mut num)
    .expect("Траблы с башкой...");
    let num: i32 = num.trim().parse().expect("Вводи целое число!!");
    let mut _f:i32=0;
    let mut arr: Vec<i32> = Vec::new();

    if num < 1
    {
        print!("Простых чисел нет!!");
        return;
    }
    
    print!("1, 2, ");
    let mut i =3;
    while i<num
    {
        if isPrime(i, &arr)
        {
            if isPrime(i+2, &arr)
            {
                arr.push(i);
                arr.push(i+2);
                print!("{}, {}, ", i.to_string().blue(), (i+2).to_string().blue());
                i+=2;
            }else
            {
                arr.push(i);
                print!("{i}, ")
            }
        }
        i+=2;
    }
}