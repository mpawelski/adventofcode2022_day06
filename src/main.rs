use color_eyre::eyre::Result;
use day06::*;

fn main() -> Result<()> {
    color_eyre::install()?;
    let input = std::fs::read_to_string("./input")?;

    let result_part1_v1 = run_part1_v1(&input)?;
    println!("result_part1_v1: {result_part1_v1}");

    let result_part2_v1 = run_part2_v1_hash_set(&input)?;
    println!("result_part2_v1: {result_part2_v1}");
    Ok(())
}
