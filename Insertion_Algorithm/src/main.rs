use rand::Rng;
use std::fs::File;
use std::io::{self, Write};
use std::path::Path;
mod insertion_sorting;  // Import the sorting module

fn main() -> io::Result<()> {
    // Paths for two files
    let unsorted_path = Path::new("Random_number.txt");
    let sorted_path = Path::new("Sorted_number.txt");

    // Create the unsorted file (if doesn't exist)
    let mut unsorted_file = if !unsorted_path.exists() {
        File::create(unsorted_path)?
    } else {
        File::open(unsorted_path)?
    };

    // Create the sorted file (if doesn't exist)
    let mut sorted_file = if !sorted_path.exists() {
        File::create(sorted_path)?
    } else {
        File::open(sorted_path)?
    };

    // Create a random number generator
    let mut rng = rand::rng();

    // Generate 100,000 random numbers
    let mut random_numbers: Vec<i64> = Vec::with_capacity(100000);
    for _ in 0..100000 {
        let generated_number: i64 = rng.random_range(0..=1000000);
        random_numbers.push(generated_number);
    }

    // Write the unsorted numbers to the file
    for number in &random_numbers {
        writeln!(unsorted_file, "{}", number)?;
    }

    // Sort the numbers using insertion sort
    insertion_sorting::insertion_sorting(&mut random_numbers);

    // Write the sorted numbers to the second file
    for number in random_numbers {
        writeln!(sorted_file, "{}", number)?;
    }

    println!("Random numbers saved in Random_number.txt and sorted numbers saved in Sorted_number.txt!");
    Ok(())
}
