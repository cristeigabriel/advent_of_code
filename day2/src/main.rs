#[derive(PartialEq, Eq)]
enum Axis {
    X,
    Z,
    AIM,
    XZ,
}

struct Submarine {
    x: i32,
    z: i32,
    aim: i32,
}

impl Submarine {
    fn new(x: i32, z: i32, aim: i32) -> Self {
        Self { x, z, aim }
    }

    fn sum(&self) -> i32 {
        self.x * self.z
    }

    fn part_one(&mut self, input: &Vec<(&str, &str)>) -> i32 {
        input
            .iter()
            .map(|&x| {
                let axis = if x.0.contains("ward") {
                    Axis::Z
                } else {
                    Axis::X
                };
                let positive = x.0.contains("for") || x.0.contains("down");
                let value = x.1.parse::<i32>().unwrap() * if positive { 1 } else { -1 };

                (axis, value)
            })
            .for_each(|x| {
                if x.0 == Axis::X {
                    self.x += x.1
                } else {
                    self.z += x.1
                }
            });

        self.sum()
    }

    fn part_two(&mut self, input: &Vec<(&str, &str)>) -> i32 {
        input
            .iter()
            .map(|&x| {
                let axis = if !x.0.contains("ward") {
                    Axis::AIM
                } else {
                    Axis::XZ
                };
                let positive = x.0.contains("for") || x.0.contains("down");
                let value = x.1.parse::<i32>().unwrap() * if positive { 1 } else { -1 };

                (axis, value)
            })
            .for_each(|x| {
                if x.0 == Axis::XZ {
                    self.x += x.1;
                    self.z += self.aim * x.1;
                } else if x.0 == Axis::AIM {
                    self.aim += x.1;
                } else {
                    unreachable!();
                }
            });

        self.sum()
    }
}

fn main() {
    let mut submarine = Submarine::new(0, 0, 0);
    let mut aim_submarine = Submarine::new(0, 0, 0);

    let input = include_str!("input.txt")
        .lines()
        .map(|x| x.split_whitespace())
        .map(|x| x.collect::<Vec<&str>>())
        .filter(|x| x.len() == 2)
        .map(|x| (*x.first().unwrap(), *x.last().unwrap()))
        .collect::<Vec<(&str, &str)>>();

    println!("Part One: {}", submarine.part_one(&input));
    println!("Part Two: {}", aim_submarine.part_two(&input));
}
