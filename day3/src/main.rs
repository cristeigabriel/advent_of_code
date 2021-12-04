const CONST_WIDTH: usize = 12;

fn input_to_bytes(input: &Vec<i32>) -> Vec<[bool; CONST_WIDTH]> {
    input
        .iter()
        .map(|x| {
            [0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11]
                .map(|y| ((*x >> ((CONST_WIDTH - 1) - y)) & 1) != 0)
        })
        .collect()
}

fn most_common_bits_in_vector(input: &Vec<[bool; CONST_WIDTH]>) -> Vec<bool> {
    let mut counter = [0; CONST_WIDTH];

    for x in input {
        for y in 0..CONST_WIDTH {
            if x[y] {
                counter[y] += 1;
            }
        }
    }

    counter.iter().map(|&x| x >= input.len() / 2).collect()
}

/// Spec UB: if in a bit column the inputs are 500-500, then good luck
fn part_one(input: &Vec<i32>) -> i32 {
    let input_to_bytes = input_to_bytes(&input);

    let mut gamma: i32 = 0;
    let mut epsilon: i32 = 0;
    most_common_bits_in_vector(&input_to_bytes)
        .iter()
        .enumerate()
        .for_each(|x| {
            gamma <<= 1;
            epsilon <<= 1;

            if *x.1 {
                gamma |= 1;
            } else {
                epsilon |= 1;
            }
        });

    gamma * epsilon
}

fn compute_most_common_bit_of_vector(base: &[bool]) -> bool {
    base.iter().filter(|&&x| x).count() >= base.len()
}

fn part_two(input: &Vec<i32>) -> i32 {
    let input_to_bytes = input_to_bytes(&input);

    let mut oxygen_rating = 0;
    input_to_bytes
        .iter()
        .filter(|&&x| {
            ![false; CONST_WIDTH]
                .iter()
                .enumerate()
                .map(|y| x[y.0] == compute_most_common_bit_of_vector(&x[y.0..CONST_WIDTH]))
                .any(|x| !x)
        })
        .collect::<Vec<&[bool; CONST_WIDTH]>>()
        .first()
        .unwrap()
        .iter()
        .for_each(|&x| {
            oxygen_rating <<= 1;

            if x {
                oxygen_rating |= 1;
            }
        });

    let mut co2_rating = 0;
    input_to_bytes
        .iter()
        .filter(|&&x| {
            [false; CONST_WIDTH]
                .iter()
                .enumerate()
                .map(|y| x[y.0] == compute_most_common_bit_of_vector(&x[y.0..CONST_WIDTH]))
                .any(|x| x)
        })
        .collect::<Vec<&[bool; CONST_WIDTH]>>()
        .first()
        .unwrap()
        .iter()
        .for_each(|&x| {
            co2_rating <<= 1;

            if x {
                co2_rating |= 1;
            }
        });

    oxygen_rating * co2_rating
}

fn main() {
    let input = include_str!("input.txt")
        .lines()
        .map(|x| {
            assert!(x.len() >= CONST_WIDTH, "String too small");
            i32::from_str_radix(x, 2).unwrap()
        })
        .collect::<Vec<i32>>();

    println!("Part One: {}", part_one(&input));
    println!("Part Two: {}", part_two(&input));
}
