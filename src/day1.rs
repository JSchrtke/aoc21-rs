pub fn run(input: &str) -> String {
    part1(input)
}

fn part1(input: &str) -> String {
    let str_nums = input.split("\n");
    let mut inc_cnt = 0;
    let mut prev = 0;

    for (i, s) in str_nums.enumerate() {
        let n = s.parse::<i32>().unwrap();
        if i == 0 {
            prev = n;
            continue;
        }
        if n > prev {
            inc_cnt += 1;
        }
        prev = n;
    }

    String::from("depth changed ") + &inc_cnt.to_string() + " times"
}
