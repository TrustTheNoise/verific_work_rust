#![allow(unused_assignments)]

use rand::Rng;
use std::collections::HashMap;
use std::io;

pub fn task8_1() {
    print!("\n");
    let mut vect = Vec::new();
    println!("Введите размер массива");
    let mut size = String::new();

    io::stdin()
    .read_line(&mut size)
    .expect("Траблы с башкой");

    let size = size.trim().parse().expect("Вводите целые числа");

    for _i in 1..size
    {
        vect.push(rand::thread_rng().gen_range(1..10));
    }

    println!("{:?}", vect);

    let (aver_num, median, mod_of_spis) = calculation(&mut vect);

    println!("Среднее значение: {aver_num}\nМедиана: {median}\nМода списка: {mod_of_spis}");
}

fn calculation(vect: &mut Vec<i32>) -> (f32, f32, i32){

    // Среднее значение 
    let mut aver_num = 0;
    
    for num in &mut *vect
    {
        aver_num = aver_num + *num;
    }

    let aver_num: f32 = aver_num as f32 / vect.len() as f32; 
    //

    // медиана
    vect.sort();
    println!("Отсортированный вектор: {:?}", vect);
    
    let mut median: f32 = 0.;

    if (vect.len()) % 2 == 0
    {
        median = (vect[vect.len()/2]+vect[vect.len()/2-1]) as f32/2.;
    }else{
        median = vect[vect.len()/2] as f32;
    }
    //

    // мода списка
    let mut h_map = HashMap::new();



    for num_from_vect in &mut vect.iter() {
        let count = h_map.entry(num_from_vect).or_insert(0);
        *count += 1;
    }

    let mut max_elem: &i32 = &0;
    let mut max_now = 0;
    for elem_h_map in h_map
    {
        if elem_h_map.1 > max_now
        {
            max_now = elem_h_map.1;
            max_elem = elem_h_map.0;
        }
    }
    
    //

    (aver_num, median, *max_elem) 
    
}