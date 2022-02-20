pub fn run(input: &str) -> String {
    let depths: Vec<i32> = input.split('\n').map(|s| s.parse().unwrap()).collect();

    let d = count_increments(&depths);

    let a = average_depth_increases(&depths);

    format!(
        "Depth changed {} individual times and {} times on average",
        d, a,
    )
}

fn average_depth_increases(numbers: &[i32]) -> i32 {
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

fn count_increments(numbers: &[i32]) -> i32 {
    let mut increment_count = 0;
    let mut previous_n = numbers.first().unwrap();
    for n in numbers {
        if n > previous_n {
            increment_count += 1;
        }
        previous_n = n;
    }

    increment_count
}

#[cfg(test)]
mod tests {
    use super::run;

    #[test]
    fn with_sample_input() {
        let expected = String::from("Depth changed 7 individual times and 5 times on average");

        let actual = run(TEST_INPUT);

        assert_eq!(expected, actual);
    }

    const TEST_INPUT: &str = "199
200
208
210
200
207
240
269
260
263";
}
