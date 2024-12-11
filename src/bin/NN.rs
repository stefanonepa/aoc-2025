use anyhow::*;
use std::fs::File;
use std::io::{BufRead, BufReader};
use code_timing_macros::time_snippet;
use const_format::concatcp;
use adv_code_2024::*;

const DAY: &str = "01"; // TODO: Fill the day
const INPUT_FILE: &str = concatcp!("input/", DAY, ".txt");

const TEST: &str = "\
3   4
4   3
2   5
1   3
3   9
3   3
"; // TODO: Add the test input

fn main() -> Result<()> {
    start_day(DAY);

    //region Part 1
    println!("=== Part 1 ===");

    fn part1<R: BufRead>(reader: R) -> Result<i32> {
        // TODO: Solve Part 1 of the puzzle
        let mut first_numbers = Vec::new();
        let mut second_numbers = Vec::new();

        for line in reader.lines().flatten() {
            // Split the line into whitespace-delimited parts and parse them as integers
            let mut numbers = line.split_whitespace().filter_map(|n| n.parse::<i32>().ok());
            if let (Some(first), Some(second)) = (numbers.next(), numbers.next()) {
                first_numbers.push(first);
                second_numbers.push(second);
            }
        }

        first_numbers.sort();
        second_numbers.sort();

        // Print the results
        println!("First numbers: {:?}", first_numbers);
        println!("Second numbers: {:?}", second_numbers);

        let difference= first_numbers
            .iter()
            .zip(second_numbers.iter()) // Zip the two vectors together
            .map(|(first, second)| (first - second).abs())
            .reduce(|acc, e| acc + e)// Calculate the difference
            .unwrap();

        println!("Differences: {:?}", difference);

        Ok(difference)
    }

    // TODO: Set the expected answer for the test input
    assert_eq!(11, part1(BufReader::new(TEST.as_bytes()))?);

    let input_file = BufReader::new(File::open(INPUT_FILE)?);
    let result = time_snippet!(part1(input_file)?);
    println!("Result = {}", result);
    //endregion

    //region Part 2
    // println!("\n=== Part 2 ===");
    //
    // fn part2<R: BufRead>(reader: R) -> Result<usize> {
    //     Ok(0)
    // }
    //
    // assert_eq!(0, part2(BufReader::new(TEST.as_bytes()))?);
    //
    // let input_file = BufReader::new(File::open(INPUT_FILE)?);
    // let result = time_snippet!(part2(input_file)?);
    // println!("Result = {}", result);
    //endregion

    Ok(())
}
