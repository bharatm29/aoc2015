pub fn run(){
    // part1();
    part2();
}

fn part2(){
    let mut v = vec![vec![0;1000];1000];

    include_str!("./input.txt")
        .lines()
        .for_each(|st| {
            let inst: Vec<&str> = st.split_whitespace().collect();
            let mut cords1: Vec<&str> = vec![];
            let mut cords2: Vec<&str> = vec![];

            if inst[0] == "turn" {
                 cords1 = inst[2].split(',').collect::<Vec<&str>>();
                 cords2 = inst[4].split(',').collect::<Vec<&str>>();
            }
            else{
                 cords1 = inst[1].split(',').collect::<Vec<&str>>();
                 cords2 = inst[3].split(',').collect::<Vec<&str>>();
            }
            let x1 = cords1[0].parse::<i32>().unwrap();
            let y1 = cords1[1].parse::<i32>().unwrap();

            let x2 = cords2[0].parse::<i32>().unwrap();
            let y2 = cords2[1].parse::<i32>().unwrap();

            // println!("Current cords: ({}, {}); ({}, {})", x1, y1, x2, y2);

            if inst[0] == "turn" {
                if inst[1] == "off" {
                    for i in x1..=x2 {
                        for j in y1..=y2 {
                            if v[i as usize][j as usize] > 0 {
                                v[i as usize][j as usize] -= 1;
                            }
                        }
                    }
                }
                else{
                    for i in x1..=x2 {
                        for j in y1..=y2 {
                            v[i as usize][j as usize] += 1;
                        }
                    }
                }
            }
            else {
                for i in x1..=x2 {
                    for j in y1..=y2 {
                        v[i as usize][j as usize] += 2;
                    }
                }
            }

            // v.iter().for_each(|vv| println!("{:?}", vv));
        });

    let mut ans = 0;
    for i in 0..1000 {
        for j in 0..1000 {
            ans += v[i as usize][j as usize];
        }
    }

    println!("Answer is: {}", ans);
}

fn part1(){
    let mut v = vec![vec![false;1000];1000];

    include_str!("./input.txt")
        .lines()
        .for_each(|st| {
            let inst: Vec<&str> = st.split_whitespace().collect();
            let mut cords1: Vec<&str> = vec![];
            let mut cords2: Vec<&str> = vec![];

            if inst[0] == "turn" {
                 cords1 = inst[2].split(',').collect::<Vec<&str>>();
                 cords2 = inst[4].split(',').collect::<Vec<&str>>();
            }
            else{
                 cords1 = inst[1].split(',').collect::<Vec<&str>>();
                 cords2 = inst[3].split(',').collect::<Vec<&str>>();
            }
            let x1 = cords1[0].parse::<i32>().unwrap();
            let y1 = cords1[1].parse::<i32>().unwrap();

            let x2 = cords2[0].parse::<i32>().unwrap();
            let y2 = cords2[1].parse::<i32>().unwrap();

            // println!("Current cords: ({}, {}); ({}, {})", x1, y1, x2, y2);

            if inst[0] == "turn" {
                if inst[1] == "off" {
                    for i in x1..=x2 {
                        for j in y1..=y2 {
                            v[i as usize][j as usize] =  false;
                        }
                    }
                }
                else{
                    for i in x1..=x2 {
                        for j in y1..=y2 {
                            v[i as usize][j as usize] = true;
                        }
                    }
                }
            }
            else {
                for i in x1..=x2 {
                    for j in y1..=y2 {
                        v[i as usize][j as usize] = !v[i as usize][j as usize];
                    }
                }
            }

            // v.iter().for_each(|vv| println!("{:?}", vv));
        });

    let mut ans = 0;
    for i in 0..1000 {
        for j in 0..1000 {
            if v[i as usize][j as usize] {
                ans += 1;
            }
        }
    }

    println!("Answer is: {}", ans);
}
