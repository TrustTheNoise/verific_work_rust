use rand::Rng;
use std::collections::HashMap;

pub fn task8_1() {
    print!("\n\n");
    let mut vect = Vec::new();
    for _i in 1..9
    {
        vect.push(rand::thread_rng().gen_range(1..10));
    }

    println!("{:?}", vect);

    let (aver_num, median, mod_of_spis) = func_of_task(&mut vect);

    println!("Среднее значение: {aver_num}\nМедиана: {median}\nМода списка: {mod_of_spis}");
}

fn func_of_task(vect: &mut Vec<i32>) -> (f32, &i32, &i32){

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
    
    let median = match vect.get(vect.len()/2)
    {
        Some(numer) => {numer},
        None => {&0},
    };
    //

    // мода списка
    let mut h_map = HashMap::new();



    for num_from_vect in &mut vect.iter() {
        let count = h_map.entry(num_from_vect).or_insert(0);
        *count += 1;
    }

    let mut _max = h_map.get(&vect[0]).copied().unwrap_or(0);
    let mut max_i=0;
    for i in 1..vect.len()
    {
        if vect[i]==vect[i-1] {
            continue;
        }
        let max1 = h_map.get(&vect[i]).copied().unwrap_or(0);
        if _max<max1
        {
            max_i=i;
        }
    }
    //

    (aver_num, median, &vect[max_i]) // возвращение кортежа
    
}