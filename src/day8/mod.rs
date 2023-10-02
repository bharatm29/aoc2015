pub fn run(){
    let mut code_count = 0;
    let mut mem_count = 0;

    include_str!("./input.txt")
        .lines()
        .for_each(|line| {
            code_count += line.len() as u32;
            mem_count += parse_and_count(line);
        });

    println!("({}, {}) => Answer is: {}", code_count, mem_count, code_count - mem_count);
}

fn parse_and_count(st: &str) -> u32{
    let mut count = 0;
    let st = st.to_string();

    if !st.contains('\\') {
        return st.len() as u32 - 2;
    }

    let mut i = 0;
    while i < st.len() {
        if st.char_at(i) == '\\' {
            if st.char_at(i + 1) == '\\' || st.char_at(i + 1) == '"' {
                i += 2;
            }
            else {
                i += 4;
            }
            count += 1;
            continue;
        }

        count += 1;
        i += 1;
    }

    return count - 2;
}

trait StringUtil {
    fn char_at(&self, index: usize) -> char;
}

impl StringUtil for String {
    fn char_at(&self, index: usize) -> char {
        return self.chars().nth(index).unwrap();
    }
}
