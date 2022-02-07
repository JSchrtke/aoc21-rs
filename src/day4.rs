use std::cmp::Ordering::Equal;
use std::cmp::Ordering::Greater;
use std::cmp::Ordering::Less;

const BOARD_SIDE_LEN: usize = 5;
const BOARD_SIZE: usize = BOARD_SIDE_LEN * BOARD_SIDE_LEN;

pub fn run(input: &str) -> String {
    let (draw_numbers, fields) = parse_input(input);

    let score = final_score(draw_numbers, fields);

    format!("winning score is {}", score)
}

fn parse_input(input: &str) -> (Vec<usize>, Vec<usize>) {
    let mut lines = input.lines();

    let draw_numbers: Vec<usize> = lines
        .next()
        .expect("there are no lines in the input!")
        .split(',')
        // TODO This match should probably be extracted to a function
        .map(to_int_or_panic)
        .collect();

    let fields: Vec<usize> = lines
        .map(|l| l.to_owned() + " ")
        .collect::<String>()
        .split(' ')
        .filter(|c| !c.is_empty())
        .map(to_int_or_panic)
        .collect();

    return (draw_numbers, fields);

    fn to_int_or_panic(s: &str) -> usize {
        match s.parse() {
            Ok(num) => num,
            Err(e) => panic!(
                "expected input to only consist of numbers, but got {}\n{}",
                s, e
            ),
        }
    }
}

fn final_score(draw_numbers: Vec<usize>, fields: Vec<usize>) -> usize {
    valid_fields_or_panic(&fields);

    let mut marks: Vec<usize> = Vec::new();
    for n in draw_numbers {
        for (j, m) in fields.iter().enumerate() {
            if n == *m {
                marks.push(j);
            }
        }
        // can not possibly win before at least 5 numbers have been drawn
        if marks.len() < BOARD_SIDE_LEN {
            continue;
        }

        marks.sort_unstable();

        match winning_board_score(&marks, &fields) {
            Some(score) => return n * score,
            None => continue,
        };
    }

    panic!("Could not find a winning board!");
}

fn winning_board_score(marks: &[usize], fields: &[usize]) -> Option<usize> {
    if marks.len() < BOARD_SIDE_LEN {
        return None;
    }

    let winning_indices = match winning_column_indices(marks) {
        Some(indices) => indices,
        None => match winning_row_indices(marks) {
            Some(indices) => indices,
            None => return None,
        },
    };

    let winning_board_index = match winning_indices.first() {
        Some(index) => index / BOARD_SIZE,
        None => panic!("got empty winning indices!"),
    };

    let start = winning_board_index * BOARD_SIZE;
    let end = winning_board_index * BOARD_SIZE + BOARD_SIZE;
    let winning_board = fields.get(start..end).unwrap();

    let score: usize = winning_board
        .iter()
        .enumerate()
        .map(|(i, &x)| {
            if marks.contains(&(i + winning_board_index * BOARD_SIZE)) {
                0
            } else {
                x
            }
        })
        .sum();

    Some(score)
}

fn valid_fields_or_panic(fields: &[usize]) {
    if fields.len() % BOARD_SIZE != 0 {
        panic!(
            "field length must be a multiple of {}, but was {}",
            BOARD_SIZE,
            fields.len()
        );
    }

    let boards = fields.chunks(BOARD_SIZE).map(|chunk| chunk.to_vec());

    for mut board in boards {
        board.sort_unstable();
        let mut prev = board.first().unwrap();
        for f in board.get(1..).unwrap() {
            if f == prev {
                panic!("boards must not have duplicate values");
            }
            prev = f;
        }
    }
}

fn winning_column_indices(marks: &[usize]) -> Option<Vec<usize>> {
    // In order to have a winning column, there have to exist 5 (board_size) marks such that the
    // difference between one mark and the next is 5.
    for (i, m) in marks.iter().enumerate() {
        // The first of the winning marks has to be in the first row of the board, since it is the
        // start of a column. This means that it has to be smaller than the width of the board.
        if m < &BOARD_SIDE_LEN {
            // TODO: this if can be inverted
            if i > marks.len() - BOARD_SIDE_LEN {
                // Since there are less than 5 marks left, including the current one, there
                // can't be a full winning column, so we can stop looking for it here.
                break;
            }

            let mut prev = m;
            // NOTE the i + 1 has to always exist at this point, otherwise we would have broken
            // out of the loop by now.
            let rest = marks.get(i + 1..).unwrap();
            let mut winning_column_marks = Vec::new();
            winning_column_marks.push(*m);
            for curr in rest {
                if curr - prev == BOARD_SIDE_LEN {
                    winning_column_marks.push(*curr);
                    prev = curr;
                }
                if winning_column_marks.len() == BOARD_SIDE_LEN {
                    return Some(winning_column_marks);
                }
            }
        }
    }

    None
}

fn winning_row_indices(marks: &[usize]) -> Option<Vec<usize>> {
    match marks.len().cmp(&BOARD_SIDE_LEN) {
        Less => None,
        Equal => {
            if marks.first().unwrap() % BOARD_SIDE_LEN != 0 {
                return None;
            }
            let consecutive_count = count_consecutives(marks);
            if consecutive_count == 4 {
                Some(marks.to_vec())
            } else {
                None
            }
        }
        Greater => {
            for (mark_inc, mark) in marks
                .get(..marks.len() - BOARD_SIDE_LEN)
                .unwrap()
                .iter()
                .enumerate()
            {
                if mark % BOARD_SIDE_LEN == 0 {
                    let current_row_indices =
                        marks.get(mark_inc..mark_inc + BOARD_SIDE_LEN).unwrap();

                    let consecutive_count = count_consecutives(current_row_indices);

                    if consecutive_count == 4 {
                        return Some(current_row_indices.to_vec());
                    }
                }
            }
            None
        }
    }
}

fn count_consecutives(values: &[usize]) -> usize {
    let mut consecutive_count = 0;
    let mut prev = values.get(0).unwrap();
    for m in values.get(1..).unwrap() {
        if m == &(prev + 1) {
            consecutive_count += 1;
        } else {
            consecutive_count = 0;
        }
        prev = m;
    }

    consecutive_count
}

#[cfg(test)]
mod tests {
    use super::final_score;
    use super::parse_input;
    use super::valid_fields_or_panic;
    use super::winning_board_score;

    #[test]
    fn input_is_correctly_parsed() {
        let expected_draw_numbers = vec![
            7, 4, 9, 5, 11, 17, 23, 2, 0, 14, 21, 24, 10, 16, 13, 6, 15, 25, 12, 22, 18, 20, 8, 19,
            3, 26, 1,
        ];
        let expected_fields = vec![
            22, 13, 17, 11, 0, 8, 2, 23, 4, 24, 21, 9, 14, 16, 7, 6, 10, 3, 18, 5, 1, 12, 20, 15,
            19, 3, 15, 0, 2, 22, 9, 18, 13, 17, 5, 19, 8, 7, 25, 23, 20, 11, 10, 24, 4, 14, 21, 16,
            12, 6, 14, 21, 17, 24, 4, 10, 16, 15, 9, 19, 18, 8, 23, 26, 20, 22, 11, 13, 6, 5, 2, 0,
            12, 3, 7,
        ];

        let (draw_numbers, fields) = parse_input(crate::input::DAY_4_SAMPLE);

        assert_eq!(expected_draw_numbers, draw_numbers);
        assert_eq!(expected_fields, fields);
    }

    #[test]
    fn with_sample_input() {
        let (draw_numbers, fields) = parse_input(crate::input::DAY_4_SAMPLE);
        let winning_score = final_score(draw_numbers, fields);
        assert_eq!(4512, winning_score);
    }

    #[test]
    fn can_find_winning_score_for_winning_column() {
        let draw_numbers = vec![11, 21, 31, 41, 55, 51];
        /*
        The board:

            11 12 13 14 15
            21 22 23 24 25
            31 32 33 34 35
            41 42 43 44 45
            51 52 53 54 55

        board with marks:

            X 12 13 14 15
            X 22 23 24 25
            X 32 33 34 35
            X 42 43 44 45
            X 52 53 54 X

        The winning boards' score is the sum of all unmarked fields. In this case, that is 615
        */
        let fields = vec![
            11, 12, 13, 14, 15, 21, 22, 23, 24, 25, 31, 32, 33, 34, 35, 41, 42, 43, 44, 45, 51, 52,
            53, 54, 55,
        ];

        // The winnnig score is the score of the winning board, multiplied by the last drawn
        // number. In this case, that is 615 * 51 = 31365
        let expected_score = 615 * 51;

        let score = final_score(draw_numbers, fields);

        assert_eq!(expected_score, score);
    }

    #[test]
    #[should_panic(expected = "boards must not have duplicate values")]
    fn validating_fields_with_duplicates_should_panic() {
        let fields_with_duplicates = vec![
            1, 1, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22, 23, 24,
            25, 1, 1, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22, 23,
            24, 25,
        ];
        valid_fields_or_panic(&fields_with_duplicates);
    }

    #[test]
    #[should_panic(expected = "field length must be a multiple of 25, but was 49")]
    fn validating_fields_with_too_few_fields_panics() {
        // To contain a valid number of bingo boards, the array of all fields must be a multiple of 25
        // exactly.
        let too_few_fields = vec![
            1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22, 23, 24,
            25, 26, 27, 28, 29, 30, 31, 32, 33, 34, 35, 36, 37, 38, 39, 40, 41, 42, 43, 44, 45, 46,
            47, 48, 49,
        ];
        valid_fields_or_panic(&too_few_fields);
    }

    #[test]
    #[should_panic(expected = "field length must be a multiple of 25, but was 51")]
    fn validating_fields_with_too_many_fields_panics() {
        let too_many_fields = vec![
            1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22, 23, 24,
            25, 26, 27, 28, 29, 30, 31, 32, 33, 34, 35, 36, 37, 38, 39, 40, 41, 42, 43, 44, 45, 46,
            47, 48, 49, 50, 51,
        ];
        valid_fields_or_panic(&too_many_fields)
    }

    #[test]
    fn validating_fields() {
        // NOTE these are 2 distinct bingo boards. Each one has 25 fields, since it is 5x5 fields.
        // within those boards, no duplicates are allowed. However, the array of all fields on all
        // boards can have duplicate values, as long as they are not on the same board.
        let fields_with_duplicates_on_different_boards = vec![
            1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22, 23, 24,
            25, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22, 23,
            24, 25,
        ];
        valid_fields_or_panic(&fields_with_duplicates_on_different_boards);

        let duplicate_at_board_transition = vec![
            1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22, 23, 24,
            25, 25, 26, 27, 28, 29, 30, 31, 32, 33, 34, 35, 36, 37, 38, 39, 40, 41, 42, 43, 44, 45,
            46, 47, 48, 49,
        ];
        valid_fields_or_panic(&duplicate_at_board_transition);
    }

    #[test]
    fn finding_winning_score() {
        let too_few_marks = [1, 2, 3];
        let fields = [
            1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22, 23, 24,
            25,
        ];
        assert_eq!(None, winning_board_score(&too_few_marks, &fields));
    }

    #[test]
    fn winning_board_score_for_row() {
        /*
            1  2  3  4  5
            6  7  8  9  10
            11 12 13 14 15
            16 17 18 19 20
            21 22 23 24 25
        */
        let marks = vec![5, 6, 7, 8, 9];
        let fields = vec![
            1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22, 23, 24,
            25,
        ];
        let expected_score: usize = fields
            .iter()
            .enumerate()
            .map(|(i, &x)| if marks.contains(&i) { 0 } else { x })
            .sum();
        println!("expected score is: {}", expected_score);

        let actual_score =
            winning_board_score(&marks, &fields).expect("expected winning score, but got none!");

        assert_eq!(expected_score, actual_score);
    }
}
