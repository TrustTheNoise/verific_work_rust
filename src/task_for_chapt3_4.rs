#![allow(unused_assignments)]

use std::io;

pub fn task_for_chapt3_4() {
    println!("Введите входное напряжение: ");
    let mut u_in = String::new();
    let mut u_out = String::new();

    io::stdin()
    .read_line(&mut u_in)
    .expect("Беды с барашкой");
    let u_in: f32 = u_in.trim().parse().expect("Вводи числовые значения");

    println!("Введите выходное напряжение: ");
    io::stdin()
    .read_line(&mut u_out)
    .expect("Беды с барашкой");
    let u_out: f32 = u_out.trim().parse().expect("Вводи числовые значения");

    if (u_in / 2.) == u_out {
        println!("Подойдёт пара любых одинаковых значений сопротивления");
        return;
    }

    all_res_view(u_in, u_out);

}

fn all_res_bigger(u_in: f32, u_out: f32) -> (Vec<(u32, u32)>, (u32, u32), (u32, u32))
{
    let mut min = (0, 0);
    let mut max: (u32, u32) = (0,0);
    let mut min_u: f32 = 0.0;
    let mut max_u: f32 = 1000000.0;
    let mut vect:Vec<(u32, u32)> = Vec::new();
    
    let mut arr_of_res = Vec:: from(
        [100.0, 102.0, 105.0, 107.0, 110.0, 113.0,
         115.0, 118.0, 121.0, 124.0, 127.0, 130.0,
         133.0, 137.0, 140.0, 143.0, 147.0, 150.0,
         154.0, 158.0, 162.0, 165.0, 169.0, 174.0,
         178.0, 182.0, 187.0, 191.0, 196.0, 200.0,
         205.0, 210.0, 215.0, 221.0, 226.0, 232.0,
         237.0, 243.0, 249.0, 255.0, 261.0, 267.0,
         274.0, 280.0, 287.0, 294.0, 301.0, 309.0,
         316.0, 324.0, 332.0, 340.0, 348.0, 357.0,
         365.0, 374.0, 383.0, 392.0, 402.0, 412.0,
         422.0, 432.0, 442.0, 453.0, 464.0, 475.0,
         487.0, 499.0, 511.0, 523.0, 536.0, 549.0,
         562.0, 576.0, 590.0, 604.0, 619.0, 634.0,
         649.0, 665.0, 681.0, 698.0, 715.0, 732.0,
         750.0, 768.0, 787.0, 806.0, 825.0, 845.0,
         866.0, 887.0, 909.0, 931.0, 953.0, 976.0]);
    
    for i in arr_of_res.len()..(arr_of_res.len()*2) {
        arr_of_res.push(arr_of_res[i-96] * 10.0);
    }

    let mut low:usize = 0;
    while low < arr_of_res.len()
    {
        let mut high = arr_of_res.len()-1;
        let mut low1 = low;

        while low1<high
        {
            let mid = (low1+high)/2;

            let this_cycle = u_in * arr_of_res[mid] / (arr_of_res[low] + arr_of_res[mid]);

            if ((this_cycle-u_out).abs() ) < 0.00001
            {
                vect.push((arr_of_res[low] as u32, arr_of_res[mid] as u32));
                break;
            }

            if this_cycle < u_out
            {

                low1 = mid + 1;
                if this_cycle < u_out && this_cycle>min_u 
                {
                    min = (arr_of_res[low] as u32, arr_of_res[mid] as u32);
                    min_u = this_cycle
                }
            }
            if this_cycle > u_out
            {
                high = mid - 1;
                if this_cycle > u_out && this_cycle < max_u 
                {
                    max = (arr_of_res[low] as u32, arr_of_res[mid] as u32);
                    max_u = this_cycle
                }
            }
        }


        low += 1;
    }


    (vect, min, max)
}



fn all_res_less(u_in: f32, u_out: f32) -> (Vec<(u32, u32)>, (u32, u32), (u32, u32))
{
    let mut min = (0, 0);
    let mut max: (u32, u32) = (0,0);
    let mut min_u: f32 = 0.0;
    let mut max_u: f32 = 1000000.0;
    let mut vect:Vec<(u32, u32)> = Vec::new();
    let mut arr_of_res = Vec:: from(
       [100.0, 102.0, 105.0, 107.0, 110.0, 113.0,
        115.0, 118.0, 121.0, 124.0, 127.0, 130.0,
        133.0, 137.0, 140.0, 143.0, 147.0, 150.0,
        154.0, 158.0, 162.0, 165.0, 169.0, 174.0,
        178.0, 182.0, 187.0, 191.0, 196.0, 200.0,
        205.0, 210.0, 215.0, 221.0, 226.0, 232.0,
        237.0, 243.0, 249.0, 255.0, 261.0, 267.0,
        274.0, 280.0, 287.0, 294.0, 301.0, 309.0,
        316.0, 324.0, 332.0, 340.0, 348.0, 357.0,
        365.0, 374.0, 383.0, 392.0, 402.0, 412.0,
        422.0, 432.0, 442.0, 453.0, 464.0, 475.0,
        487.0, 499.0, 511.0, 523.0, 536.0, 549.0,
        562.0, 576.0, 590.0, 604.0, 619.0, 634.0,
        649.0, 665.0, 681.0, 698.0, 715.0, 732.0,
        750.0, 768.0, 787.0, 806.0, 825.0, 845.0,
        866.0, 887.0, 909.0, 931.0, 953.0, 976.0]);
    
    for i in arr_of_res.len()..(arr_of_res.len()*2) {
        arr_of_res.push(arr_of_res[i-96] * 10.0);
    }

    let mut low:usize = 0;
    while low < arr_of_res.len()
    {
        if (u_in * arr_of_res[low] / (arr_of_res[low] + arr_of_res[arr_of_res.len()-1])) > u_out 
        {
            break;
        }

        let mut high = arr_of_res.len()-1;
        let mut low1 = low;

        while low1<high
        {
            let mid = (low1+high)/2;

            let this_cycle = u_in * arr_of_res[low] / (arr_of_res[low] + arr_of_res[mid]);

            if ((this_cycle-u_out).abs() ) < 0.00001
            {
                vect.push((arr_of_res[low] as u32, arr_of_res[mid] as u32));
                break;
            }

            if this_cycle < u_out
            {
                high = mid - 1;
                if this_cycle < u_out && this_cycle>min_u 
                {
                    min = (arr_of_res[low] as u32, arr_of_res[mid] as u32);
                    min_u = this_cycle
                }
            }
            if this_cycle > u_out
            {
                low1 = mid +1;
                if this_cycle > u_out && this_cycle < max_u 
                {
                    max = (arr_of_res[low] as u32, arr_of_res[mid] as u32);
                    max_u = this_cycle
                }
            }
        }


        low += 1;
    }

    (vect, min, max)
}

fn all_res_view(u_in: f32, u_out: f32)
{
    let mut vect_par: Vec<(u32, u32)> =Vec::new();
    let mut min:(u32, u32) = (0,0);
    let mut max:(u32, u32) = (0,0);
    if u_out < (u_in/2.) 
    {
        (vect_par, min, max) = all_res_less(u_in, u_out);
        if vect_par.is_empty()
        {
            output_results_approx(u_in, u_out, min, max);
        }else{
            println!("Точные значения: {:?}", vect_par);
            output_results_approx(u_in, u_out, min, max);
        }
    }else{
        (vect_par, min, max) = all_res_bigger(u_in, u_out);
        if vect_par.is_empty()
        {
            output_results_approx(u_in, u_out, min, max);
        }else{
            println!("Точные значения: {:?}", vect_par);
            output_results_approx(u_in, u_out, min, max);
        }
    }

}

fn output_results_approx(u_in: f32, u_out: f32, min: (u32, u32), max: (u32, u32))
{
    let mut u_approx: f32 = 0.0;
    if min.0<min.1
    {
        u_approx = u_in * min.0 as f32 / (min.0 as f32 + min.1 as f32);
    }else
    {
        u_approx = u_in * min.1 as f32 / (min.0 as f32 + min.1 as f32);
    }
    let absolute_mist = u_approx-u_out;
    let relative_mist = (1.-u_out/u_approx)* 100.;

        println!("Приближенные значения:\n
    Большее соотношение:
    Входящее напряжение: {u_in}
    R1: {}       R2: {}
    ---------------------------------
    Выходное напряжение: {u_approx}
    Абсолютная разница: {absolute_mist} V
    Относительная разница: {relative_mist} %", min.0, min.1);

    if max.0<max.1
    {
        u_approx = u_in * max.0 as f32 / (max.0 as f32 + max.1 as f32);
    }else
    {
        u_approx = u_in * max.1 as f32 / (max.0 as f32 + max.1 as f32);
    }
    let absolute_mist = u_approx-u_out;
    let relative_mist = (1.-u_out/u_approx)* 100.;

        println!("Меньшее соотношение:
    Входящее напряжение: {u_in}
    R1: {}
    R2: {}
    ---------------------------------
    Выходное напряжение: {u_approx}
    Абсолютная разница: {absolute_mist} V
    Относительная разница: {relative_mist} %", max.0, max.1);
}