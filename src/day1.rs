pub fn run(input: &str) -> String {
    format!(
        "Depth changed {} individual times and {} times on average",
        part1(input),
        part2(input)
    )
}

fn part1(input: &str) -> i32 {
    let nums = nums_from_input(input);

    count_increments(nums)
}

fn nums_from_input(input: &str) -> Vec<i32> {
    input
        .split("\n")
        .map(|x| x.parse::<i32>().unwrap())
        .collect::<Vec<i32>>()
}

fn count_increments(nums: Vec<i32>) -> i32 {
    let mut inc_cnt = 0;
    let mut prev: i32 = *nums.first().unwrap();
    for n in nums {
        if n > prev {
            inc_cnt += 1;
        }
        prev = n;
    }

    inc_cnt
}

fn part2(input: &str) -> i32 {
    let nums = nums_from_input(input);
    let averages = sliding_avg(nums, 3);

    count_increments(averages)
}

fn sliding_avg(values: Vec<i32>, win_size: usize) -> Vec<i32> {
    if win_size >= values.len() {
        return values;
    }

    let mut avgs = Vec::new();
    for i in 0..values.len() - 2 {
        let mut a: i32 = 0;
        for j in 0..win_size {
            a += values[i + j];
        }
        avgs.push(a as i32);
    }

    avgs
}
