use std::io;
use colored::*;

pub fn task_for_chapt1_8() {
    let mut num = String::new();
    io::stdin()
    .read_line(&mut num)
    .expect("Траблы с башкой...");
    let num = num.trim().parse().expect("Вводи целое число!!");
    view_all_simpl_num(&num);
    println!("");
}

fn view_all_simpl_num(num: &i32)
{
    let mut _f:i32=0;
    let mut j=1;
    while j<*num
    {
        _f=0;
        let mut i = 2;
        while i<j
        {
            if j%i == 0
            {
                _f = 1;
                break;
            }
            i+=1;
        }
        if _f==0
        {
            if j!=1
            {
                let mut i = 2;
                while i<(j+2)
                {
                    if (j+2)%i == 0
                    {
                        _f = 1;
                        break;
                    }
                    i = i + 1;
                }
                if _f==0
                {
                    print!("{}, {}, ", j.to_string().blue(), (j+2).to_string().blue());
                    j = j+3;
                    continue;
                }
            }
            print!("{j}, ");
        }
        j = j+1;
    }
}