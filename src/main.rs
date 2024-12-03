use day1::day1;
use day2::day2;
use day3::day3;
mod day1;
mod day2;
mod day3;
mod helpers;
fn main() -> Result<(), Box<dyn std::error::Error>> {
    // day1();
    // day2();

    // day2()?;
    day3()?;

    

    Ok(())
}
