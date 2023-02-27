use std::io;

pub fn task8_2() {
    println!("\n\nВведите строку(на ингреше):");
    let mut latin = String::new();
    io::stdin()
    .read_line(&mut latin)
    .expect("Траблы с башкой...");
    println!("{latin}");
    println!("{}", pig_latin(&mut latin));
}

fn pig_latin(latin: &mut String) -> String
{
    let mut i = 0;
    let mut prev_i = 0;
    let mut pig_latin = String::new();
    for b in latin.chars()
    {
        if b==' ' || b =='\r'
        {
            let str_start = &latin.clone()[prev_i..i];
            let mut str_fin = String::from(str_start);
            //let str_in_start = &latin[prev_i..i];
            prev_i=i+1;
            for ch in str_fin.chars() 
            {
                if ch == 'a' || ch == 'e' || ch=='y' || ch=='u' || ch == 'i' || ch == 'o'
            {
                str_fin.push_str("-hay")
            }else
            {
                str_fin.remove(0);
                str_fin.push('-');
                str_fin.push(ch);
                str_fin.push_str("ay");
            }
            //let bruh = &latin.replace(str_start, &str_fin);
            pig_latin.push_str(&str_fin);
            pig_latin.push(' ');
            break;
            }
        }
        i+=1;
    }
    return pig_latin;
}