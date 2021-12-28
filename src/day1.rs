pub fn run(input: &str) -> String {
    format!(
        "Depth changed {} individual times and {} times on average",
        part1(input),
        part2(input)
    )
}

fn part1(input: &str) -> i32 {
    let numbers = numbers_from_input(input);

    count_increments(numbers)
}

fn numbers_from_input(input: &str) -> Vec<i32> {
    input
        .split("\n")
        .map(|x| x.parse::<i32>().unwrap())
        .collect::<Vec<i32>>()
}

fn count_increments(numbers: Vec<i32>) -> i32 {
    let mut increment_count = 0;
    let mut previous_n: i32 = *numbers.first().unwrap();
    for n in numbers {
        if n > previous_n {
            increment_count += 1;
        }
        previous_n = n;
    }

    increment_count
}

fn part2(input: &str) -> i32 {
    let numbers = numbers_from_input(input);
    let averages = sliding_average(numbers, 3);

    count_increments(averages)
}

fn sliding_average(values: Vec<i32>, window_size: usize) -> Vec<i32> {
    if window_size >= values.len() {
        return values;
    }

    let mut averages = Vec::new();
    for i in 0..values.len() - 2 {
        let mut average: i32 = 0;
        for j in 0..window_size {
            average += values[i + j];
        }
        averages.push(average as i32);
    }

    averages
}
