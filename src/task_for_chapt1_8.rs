
#![allow(unused_assignments)]
use std::io;
use colored::*;



fn is_prime(i: i32, arr: &Vec<i32>) -> bool
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
    let mut num_string = String::new();
    let mut num: i32 = 0;
    loop
    {
        io::stdin()
        .read_line(&mut num_string)
        .expect("Траблы с башкой...");
        num = match num_string.trim().parse()
        {
            Ok(num) => num,
            Err(_) => {println!("Вводите целое число!!"); continue;},
        };
        break;
    }
    let mut arr: Vec<i32> = Vec::new();

    if num < 2
    {
        print!("Простых чисел нет!!");
        return;
    }
    
    let mut i =2;
    while i<num
    {
        if i<3
        {
            print!("{i}, ");
            i+=1;
            continue;
        }
        if is_prime(i, &arr)
        {
            if is_prime(i+2, &arr)
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