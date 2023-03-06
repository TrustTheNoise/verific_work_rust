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
    let mut pig_latin = String::new();
    let arr_of_let = ['a', 'e', 'y', 'u', 'i', 'o', 'A', 'E', 'Y', 'U', 'I', 'O'];

    for word in latin.split_whitespace()
    {
        let ch = match word.to_string().chars().nth(0)  {
            Some(char) => char,
            None => {println!("Слов нет!");
            break;},
        };
        if arr_of_let.contains(&ch)
        {
            pig_latin = pig_latin + word + "-hay ";
        }else{
            let mut word_string = word.to_string();
            word_string.remove(0);
            word_string.push('-');
            word_string.push(ch);
            word_string.push_str("ay ");
            pig_latin = pig_latin + &word_string[0..];
        }

    }
    return pig_latin;
}