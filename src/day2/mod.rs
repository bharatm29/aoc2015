pub fn run(){
    let part1 = include_str!("./input.txt")
        .lines()
        .fold(0, |total, s| {
            let mut v = s.split('x').into_iter();
            let l = v.next().unwrap().parse::<i32>().unwrap();
            let w = v.next().unwrap().parse::<i32>().unwrap();
            let h = v.next().unwrap().parse::<i32>().unwrap();

            let side1 = w * l * 2;
            let side2 = w * h * 2;
            let side3 = h * l * 2;

            total + (side1 + side2 + side3) + (side1.min(side2.min(side3)) / 2)
        });

    let part2 = include_str!("./input.txt")
        .lines()
        .fold(0, |total, s| {
            let mut v = s.split('x').into_iter();
            let l = v.next().unwrap().parse::<i32>().unwrap();
            let w = v.next().unwrap().parse::<i32>().unwrap();
            let h = v.next().unwrap().parse::<i32>().unwrap();

            let mut v = vec![l, w, h];
            v.sort();

            total + (l * w * h) + (v[0] + v[0] + v[1] + v[1])
        });

    println!("Part 1: {}, Part 2: {}", part1, part2);
}
