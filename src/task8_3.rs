#![allow(unused_assignments)]

use std::collections::HashMap;
use std::io;
pub fn task8_3()
{
    let mut departments: HashMap<String, Vec<String>> = HashMap::new();
    loop
    {
        let mut choise_string = String::new();
        let mut choise: i32 = 0;
        loop
        {
            print!("\n1.Добавить сотрудника\n2.Просмотреть сотрудников\n3.Выйти\nВыбирайте\n");
            io::stdin()
            .read_line(&mut choise_string)
            .expect("Траблы");
            choise = match choise_string.trim().parse()
            {
                Ok(res) => res,
                Err(_) => {println!("Вводите целое число!!!"); continue;}
            };
            break;
        }
        match choise
        {
            1i32 => {
                create_empl_info(&mut departments);
            }
            2i32 => {
                view_depart_info(&mut departments);
            }
            3i32 => break,
            _ => println!("Такого варианта нет!!1!!"),
        }
    }
}

fn create_empl_info(departments: &mut HashMap<String, Vec<String>>)
{
    println!("Введите имя сотрудника:");
                let mut employee = String::new();
                io::stdin()
                .read_line(&mut employee)
                .expect("Траблы");
                let employee = (employee.trim()).to_string();
                
                println!("Введите отдел:");
                let mut depart = String::new();
                io::stdin()
                .read_line(&mut depart)
                .expect("Траблы");
                let depart = (depart.trim()).to_string();
                
                let mut _try_vect:Vec<String> = Vec::new();

                let count = departments.entry(depart).or_insert(_try_vect);
                count.push(employee.clone());
}

fn view_depart_info(departments: &HashMap<String, Vec<String>>)
{
    let mut vec: Vec<String> = departments.clone().into_keys().collect();
    vec.sort_unstable();
    for dep in vec
    {
        println!("{}: {:?}", dep, departments[&dep]);
    }
}