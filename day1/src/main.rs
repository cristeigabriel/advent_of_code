// use core::slice::SlicePattern;
use std::io::{Error, ErrorKind};

fn part_one(vec: &Vec<i32>) -> usize {
    let mut previous: Option<i32> = None;

    vec.iter()
        .map(|x| {
            let mut state = false;

            if let Some(y) = previous {
                state = *x > y;
            }

            previous = Some(*x);

            state
        })
        .filter(|&x| x)
        .count()
}

fn part_two(vec: &Vec<i32>) -> Result<usize, Error> {
    if vec.len() < 3 {
        return Err(Error::new(
            ErrorKind::InvalidData,
            "Data not large enough to process",
        ));
    }

    Ok(part_one(
        &(0..vec.len() - 2)
            .into_iter()
            .map(|x| vec[x] + vec[x + 1] + vec[x + 2])
            .collect(),
    ))
}

fn main() -> Result<(), Error> {
    let input = include_str!("input.txt")
        .lines()
        .map(|x| x.parse().unwrap())
        .collect::<Vec<i32>>();

    println!("Part One: {}", part_one(&input));
    println!("Part Two: {}", part_two(&input)?);

    Ok(())
}
