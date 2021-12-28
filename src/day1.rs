pub fn run(input: &str) -> String {
    let depths = input.split("\n").map(|s| s.parse().unwrap()).collect();

    let d = count_increments(&depths);
    assert_eq!(1681, d);

    let a = average_depth_increases(&depths);
    assert_eq!(1704, a);

    format!(
        "Depth changed {} individual times and {} times on average",
        d, a,
    )
}

fn average_depth_increases(numbers: &Vec<i32>) -> i32 {
    let mut averages = Vec::new();
    for i in 0..numbers.len() - 2 {
        let mut average = 0;
        for j in 0..3 {
            average += numbers[i + j];
        }
        averages.push(average);
    }

    count_increments(&averages)
}

fn count_increments(numbers: &Vec<i32>) -> i32 {
    let mut increment_count = 0;
    let mut previous_n = numbers.first().unwrap();
    for n in numbers {
        if n > previous_n {
            increment_count += 1;
        }
        previous_n = n;
    }

    return increment_count;
}
