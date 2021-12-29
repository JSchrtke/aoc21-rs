pub fn run(input: &str) -> String {
    let number_strings: Vec<&str> = input.split('\n').collect();

    let p = power_consumption(&number_strings);
    assert_eq!(4139586, p);

    let l = life_support_rating(&number_strings);
    assert_eq!(1800151, l);

    format!("power consumption is {}\n\tlife support rating is {}", p, l)
}

fn power_consumption(number_strings: &Vec<&str>) -> i32 {
    let number_count = number_strings.len();
    let bit_count = number_strings.first().unwrap().len();

    let mut high_counts = vec![0; bit_count];

    for s in number_strings {
        for (i, c) in s.chars().enumerate() {
            match c {
                '1' => high_counts[i] += 1,
                '0' => continue,
                _ => panic!("something"),
            }
        }
    }

    let mut gamma_rate: i32 = 0;
    for n in high_counts {
        gamma_rate = gamma_rate << 1;
        if n > number_count / 2 {
            gamma_rate += 1;
        } else if n < number_count / 2 {
            continue;
        } else {
            panic!("this should not happen");
        }
    }

    let epsilon_mask = 2i32.pow(bit_count as u32) - 1;
    let epsilon_rate = !gamma_rate & epsilon_mask;

    let power_consumption = gamma_rate * epsilon_rate;

    return power_consumption;
}

fn life_support_rating(number_strings: &Vec<&str>) -> u32 {
    o2_gen_rating(number_strings) * co2_scrubber_rating(number_strings)
}

fn o2_gen_rating(number_strings: &Vec<&str>) -> u32 {
    let bit_count = number_strings.first().unwrap().len();

    let mut numbers: Vec<u32> = number_strings
        .iter()
        .map(|s| u32::from_str_radix(s, 2).unwrap())
        .collect();

    let mut mask = 2u32.pow(bit_count as u32 - 1);

    while numbers.len() > 1 {
        let high_counts = numbers.iter().filter(|&n| n & mask == mask).count();
        let mut most_common_bit = 0;
        if high_counts * 2 >= numbers.len() {
            most_common_bit = 1;
        }

        let mut filter = 0;
        if most_common_bit == 1 {
            filter = mask
        }

        numbers = numbers
            .into_iter()
            .filter(|n| (n & mask) == filter)
            .collect();

        mask >>= 1;
    }

    let o2 = numbers.first().unwrap();

    return *o2;
}

fn co2_scrubber_rating(number_strings: &Vec<&str>) -> u32 {
    let mut numbers: Vec<u32> = number_strings
        .iter()
        .map(|s| u32::from_str_radix(s, 2).expect("not a valid binary number"))
        .collect();

    let bit_count = number_strings.first().unwrap().len() as u32;
    let mut mask = 2u32.pow(bit_count - 1);

    while numbers.len() > 1 {
        let high_counts = numbers.iter().filter(|&n| n & mask == mask).count();

        let mut filter = mask;
        if high_counts * 2 >= numbers.len() {
            filter = 0;
        }

        numbers = numbers
            .into_iter()
            .filter(|&n| n & mask == filter)
            .collect();

        mask >>= 1;
    }
    let co2 = *numbers.first().unwrap();

    co2
}
