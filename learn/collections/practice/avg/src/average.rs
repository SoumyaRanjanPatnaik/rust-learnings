use std::collections::HashMap;

#[derive(Debug)]
pub enum Mode {
    One(i32, i32),
    Many(Vec<i32>, i32),
    None
}
pub fn mean(arr: &Vec<i32>) -> f64 {
    let mut sum = 0;
    for e in arr {
        sum+=e;
    }
    sum as f64 / arr.len() as f64
}

fn is_sorted(arr: &Vec<i32>) -> bool {
    let mut sorted = true;
    let mut prev = 0;
    for e in arr {
        if *e < prev{
            sorted = false;
        }
        prev = *e;
    }
    sorted
}
pub fn median(arr: &Vec<i32>) -> f64 {
    let len= arr.len();

    if len % 2 == 0 {
        return arr[(len + 1)/2] as f64
    }
    else if len > 1{
        (arr[len/2] + arr[(len+1)/2] ) as f64 / 2.0
    }
    else {
        f64::NAN
    }
}


pub fn mode(arr: &Vec<i32>) -> Mode {
    let mut mode = Mode::None;
    let mut count = HashMap::new();
    for e in arr {
        let curr_count = count.entry(*e).or_insert(0);
        *curr_count += 1;
    }
    let mut max_count = 0; 
    for tup in &count {
        if *tup.1 <=1 {
            continue;
        }
        if *tup.1 > max_count {
            max_count = *tup.1;
            mode = match mode {
                Mode::None => Mode::One(*tup.0, *tup.1),
                _ => Mode::One(*tup.0, *tup.1),
            };
        }
        else if *tup.1 == max_count{
            mode = match mode {
                Mode::One(n,c) => Mode::Many(vec![n, *tup.0], c),
                Mode::Many(mut v,c) => {
                    v.push(*tup.0);
                    Mode::Many(v,c)
                },
                _ => Mode::None,
            }
        }
    }
    mode
}
