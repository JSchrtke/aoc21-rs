pub fn run(input: &str) -> String {
    let number_strings: Vec<&str> = input.split('\n').collect();

    let bit_index = number_strings.first().unwrap().len() as u32 - 1;

    let numbers: Vec<u32> = number_strings
        .iter()
        .map(|s| u32::from_str_radix(s, 2).unwrap())
        .collect();

    let p = power_consumption(&numbers, bit_index);
    assert_eq!(4139586, p);

    let l = life_support_rating(numbers, bit_index);
    assert_eq!(1800151, l);

    format!("power consumption is {}\n\tlife support rating is {}", p, l)
}

fn power_consumption(numbers: &[u32], bit_index: u32) -> u32 {
    let mut gamma_rate = 0;
    for i in (0..=bit_index).rev() {
        gamma_rate <<= 1;
        gamma_rate += most_common_bit(numbers, i);
    }

    let epsilon_mask = 2u32.pow(bit_index as u32) - 1;
    let epsilon_rate = !gamma_rate & epsilon_mask;

    gamma_rate * epsilon_rate
}

fn most_common_bit(numbers: &[u32], bit_index: u32) -> u32 {
    let mask = 2u32.pow(bit_index);

    let mut high_counts = 0;
    for n in numbers.iter() {
        if n & mask == mask {
            high_counts += 1;
        }
    }

    if high_counts * 2 >= numbers.len() {
        1
    } else {
        0
    }
}

fn life_support_rating(numbers: Vec<u32>, bit_index: u32) -> u32 {
    filter_by_most_common_bit(numbers.clone(), bit_index)
        * filter_by_least_common_bit(numbers, bit_index)
}

fn filter_by_most_common_bit(mut numbers: Vec<u32>, bit_index: u32) -> u32 {
    for i in (0..=bit_index).rev() {
        let most_common_bit = most_common_bit(&numbers, i);
        numbers = filter_by_bit(numbers, i, most_common_bit);

        if numbers.len() == 1 {
            break;
        }
    }

    *numbers.first().unwrap()
}

fn filter_by_bit(mut numbers: Vec<u32>, bit_index: u32, filter_by: u32) -> Vec<u32> {
    let mask = 2u32.pow(bit_index);

    let filter = mask * filter_by;

    numbers = numbers
        .into_iter()
        .filter(|&n| (n & mask) == filter)
        .collect();

    numbers
}

fn filter_by_least_common_bit(mut numbers: Vec<u32>, bit_index: u32) -> u32 {
    for i in (0..=bit_index).rev() {
        let mut least_common_bit = 0;
        if most_common_bit(&numbers, i) == 0 {
            least_common_bit = 1;
        }
        numbers = filter_by_bit(numbers, i, least_common_bit);

        if numbers.len() == 1 {
            break;
        }
    }
    *numbers.first().unwrap()
}
