pub fn run(input: &str) -> String {
    part1(input)
}

fn part1(input: &str) -> String {
    let nums = input
        .split("\n")
        .map(|x| x.parse::<i32>().unwrap())
        .collect::<Vec<i32>>();

    let mut inc_cnt = 0;
    let mut prev: i32 = *nums.first().unwrap();
    for n in nums {
        if n > prev {
            inc_cnt += 1;
        }
        prev = n;
    }

    format!("depth changed {} times", inc_cnt)
}
