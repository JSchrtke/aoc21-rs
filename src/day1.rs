pub fn run(input: &str) -> String {
    format!(
        "[day 1] Depth changed {} individual times and {} times on average",
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
    let averages = sliding_average(nums, 3);

    count_increments(averages)
}

fn sliding_average(values: Vec<i32>, win_size: usize) -> Vec<i32> {
    if win_size >= values.len() {
        return values;
    }

    let mut avgs = Vec::new();
    for i in 0..values.len() - 2 {
        for _j in 0..win_size {
            let a = values[i] + values[i + 1] + values[i + 2];
            avgs.push(a as i32)
        }
    }

    avgs
}
