use std::collections::HashSet;

pub fn run(){
    let mut set: HashSet<(i32, i32)> = HashSet::new();
    let mut x1 = 0;
    let mut x2 = 0;
    let mut y1 = 0;
    let mut y2 = 0;
    let mut alt = 1;

    set.insert((x1, y1));

    include_str!("./input.txt")
        .chars().for_each(|c| {
            match c {
                '^' => { if alt == 1 {y1 += 1;} else {y2 += 1} },
                'v' => { if alt == 1 {y1 -= 1;} else {y2 -= 1} },
                '>' => { if alt == 1 {x1 += 1;} else {x2 += 1} },
                '<' => { if alt == 1 {x1 -= 1;} else {x2 -= 1} },
                _ => {}
            }

            alt *= -1;

            set.insert((x1, y1));
            set.insert((x2, y2));
        });

    println!("{}", set.len() as i32);
}
