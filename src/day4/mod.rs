use md5;

pub fn run(){
    let mut cnt = 0;
    let input = "iwrupvqb";
    while true {
        let st = format!("{}{}", input, cnt);
        let digest = md5::compute(&st);

        if format!("{:x}", digest).starts_with("00000"){
            println!("Result: {st}");
            break;
        }

        cnt += 1;
    }
}
