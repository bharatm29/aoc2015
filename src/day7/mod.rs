use std::collections::HashMap;

pub fn run() {
    let mut map = HashMap::<&str, u16>::new();

    include_str!("./input.txt").lines().for_each(|st| {
        let left_right = st.split(" -> ").collect::<Vec<&str>>();

        let left = left_right[0].split_whitespace().collect::<Vec<&str>>();
        let right = left_right[1].split_whitespace().collect::<Vec<&str>>();

        // println!("{:?}, {:?}", left, right);

        if left.len() == 1 {
            let first = left.first().unwrap();
            if first.chars().next().unwrap().is_numeric() {
                map.insert(&right.first().unwrap(), first.parse::<u16>().unwrap());
            } 
            else {
                if map.contains_key(first) {
                    map.insert(&right.first().unwrap(), *map.get(first).unwrap());
                }
            }
        } else {
            let res = evaluate_result(&left, &map);
            println!("Res: {}", res);
            if res != 0 {
                map.insert(&right.first().unwrap(), res);
            }
        }
        // println!("{:?}", map);
    });

    println!("{:?}", map);
    println!("Answer is: {}", map.get("a").unwrap());
}

fn evaluate_result(input: &Vec<&str>, map: &HashMap<&str, u16>) -> u16 {
    if input.contains(&"NOT") {
        return match map.get(input[1]) {
            Some(x) => !x,
            None => 0,
        }
    }
    else if input.contains(&"AND") {
        return map.get(input[0]).unwrap_or(&0) & map.get(input[2]).unwrap_or(&0);
    } else if input.contains(&"OR") {
        return map.get(input[0]).unwrap_or(&0) | map.get(input[2]).unwrap_or(&0);
    } else if input.contains(&"LSHIFT") {
        return map.get(input[0]).unwrap_or(&0) << input[2].parse::<u16>().unwrap();
    } else {
        return map.get(input[0]).unwrap_or(&0) >> input[2].parse::<u16>().unwrap();
    }
}
