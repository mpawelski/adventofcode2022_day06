use std::{collections::HashSet, io::Read};

use color_eyre::eyre::{eyre, Error, Result};

pub fn run_part1_v1(input: &str) -> Result<usize> {
    let result = input
        .as_bytes()
        .windows(4)
        .enumerate()
        .filter_map(|(idx, window)| {
            let hs: HashSet<_> = window.iter().collect();
            if hs.len() == 4 {
                Some(idx)
            } else {
                None
            }
        })
        .next()
        .ok_or(eyre!("nothing found in input"))?;

    Ok(result + 4)
}

pub fn run_part2_v1_hash_set(input: &str) -> Result<usize> {
    let window_size = 14;

    let result = input
        .as_bytes()
        .windows(window_size)
        .enumerate()
        .filter_map(|(idx, window)| {
            let hs: HashSet<_> = window.iter().collect();
            if hs.len() == window_size {
                Some(idx)
            } else {
                None
            }
        })
        .next()
        .ok_or(eyre!("nothing found in input"))?;

    Ok(result + window_size)
}

//same as version one but reuse hashset that is cleared
pub fn run_part2_v2_hash_set_reused(input: &str) -> Result<usize> {
    let window_size = 14;

    let mut hs: HashSet<u8> = HashSet::with_capacity(window_size);

    let result = input
        .as_bytes()
        .windows(window_size)
        .enumerate()
        .filter_map(|(idx, window)| {
            hs.clear();
            hs.extend(window.iter());
            if hs.len() == window_size {
                Some(idx)
            } else {
                None
            }
        })
        .next()
        .ok_or(eyre!("nothing found in input"))?;

    Ok(result + window_size)
}

// Version that is not using hashset to find if window is unique.
// Based on https://www.mattkeeter.com/blog/2022-12-10-xor/ and is much faster than hashset version.
pub fn run_part2_v3_no_hashset(input: &str) -> Result<usize> {
    let window_size = 14;

    let result = input
        .as_bytes()
        .windows(window_size)
        .enumerate()
        .filter_map(|(idx, window)| {
            let mut unique = true;
            for i in 0..window_size {
                for j in (i + 1)..window_size {
                    if window[i] == window[j] {
                        unique = false;
                    }
                }
            }
            if unique {
                Some(idx)
            } else {
                None
            }
        })
        .next()
        .ok_or(eyre!("nothing found in input"))?;

    Ok(result + window_size)
}

// Same as v3 but without indexing slice. Use iteration over slice and enumerate() method to avoid bound checking in rust.
pub fn run_part2_v4_no_hashset_no_bound_check(input: &str) -> Result<usize> {
    let window_size = 14;

    let result = input
        .as_bytes()
        .windows(window_size)
        .enumerate()
        .filter_map(|(idx, window)| {
            let mut unique = true;
            for (i, char_i) in (&window[0..window_size]).iter().enumerate() {
                for char_j in &window[i + 1..] {
                    if char_i == char_j {
                        unique = false;
                    }
                }
            }
            if unique {
                Some(idx)
            } else {
                None
            }
        })
        .next()
        .ok_or(eyre!("nothing found in input"))?;

    Ok(result + window_size)
}

// Using arrays with counter of how many characters is in window
// and separate counter for unique characters. When moving to next window
// change this counters and determine if this window has unique characters.
pub fn run_part2_v5_array_counters(input: &str) -> Result<usize> {
    let window_size = 14;

    let input = input.as_bytes();

    let mut unique_counter = 0;
    let mut char_counters = [0; ('z' as usize - 'a' as usize) + 1];

    let mut windows_with_index = input.windows(window_size).enumerate();

    // warning of unused code, but when I comment it out I get an error. A rustc bug?
    let Some((_, first_window)) = windows_with_index.next() else { return panic!("wrong input") };

    for &c in first_window {
        let char_counter = &mut char_counters['z' as usize - c as usize];
        if *char_counter == 0 {
            unique_counter += 1;
        }
        *char_counter += 1;
    }

    let mut first_char_in_previous_window = first_window[0];

    for (idx, window) in windows_with_index {
        let char_to_remove_counter =
            &mut char_counters['z' as usize - first_char_in_previous_window as usize];
        *char_to_remove_counter -= 1;
        if *char_to_remove_counter == 0 {
            unique_counter -= 1;
        }

        let new_char = *window.last().unwrap();

        let char_counter = &mut char_counters['z' as usize - new_char as usize];
        if *char_counter == 0 {
            unique_counter += 1;
        }
        *char_counter += 1;

        if unique_counter == window_size {
            return Ok(idx + window_size);
        }

        first_char_in_previous_window = *window.first().unwrap(); //for next loop iteration
    }
    Err(eyre!("nothing found"))
}

// Bit trick from https://www.mattkeeter.com/blog/2022-12-10-xor/
// Not yet the final XOR trick.
pub fn run_part2_v6_bit_tricks(input: &str) -> Result<usize> {
    let window_size = 14;

    let result = input
        .as_bytes()
        .windows(window_size)
        .enumerate()
        .filter_map(|(idx, window)| {
            let mut set = 0u32;
            for &c in window {
                set |= 1 << (c as u32 - 'a' as u32);
            }

            if set.count_ones() as usize == window_size {
                Some(idx)
            } else {
                None
            }
        })
        .next()
        .ok_or(eyre!("nothing found in input"))?;

    Ok(result + window_size)
}

// Final XOR trick from https://www.mattkeeter.com/blog/2022-12-10-xor/
// Very similar to v5 in idea but this one uses XOR black magic.
pub fn run_part2_v7_bit_tricks_xor(input: &str) -> Result<usize> {
    let window_size = 14;

    let input = input.as_bytes();

    let mut windows_with_index = input.windows(window_size).enumerate();

    let mut set = 0u32;

    let Some((_, first_window)) = windows_with_index.next() else { return panic!("wrong input") };

    //set bits for first window
    for &c in first_window {
        set ^= 1 << (c as u32 - 'a' as u32);
    }

    let mut first_char_in_previous_window = first_window[0];

    //iterate over next windows
    for (idx, window) in windows_with_index {
        let new_char = *window.last().unwrap();

        //turn on bit for new charater in window
        set ^= 1 << (new_char as u32 - 'a' as u32);

        //turn off leaving character from window
        set ^= 1 << (first_char_in_previous_window as u32 - 'a' as u32);

        // check the current window and see if we're done
        if set.count_ones() as usize == window_size {
            return Ok(idx + window_size);
        }

        first_char_in_previous_window = *window.first().unwrap(); //for next loop iteration
    }
    Err(eyre!("nothing found"))
}

#[cfg(test)]
mod tests {
    use color_eyre::eyre::Result;
    use std::sync::Once;

    use super::*;

    static INPUT_1_EXAMPLES_AND_RESULTS: [(&str, usize); 5] = [
        ("mjqjpqmgbljsphdztnvjfqwrcgsmlb", 7),
        ("bvwbjplbgvbhsrlpgdmjqwftvncz", 5),
        ("nppdvjthqldpwncqszvftbrmjlhg", 6),
        ("nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg", 10),
        ("zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw", 11),
    ];

    static INPUT_2_EXAMPLES_AND_RESULTS: [(&str, usize); 5] = [
        ("mjqjpqmgbljsphdztnvjfqwrcgsmlb", 19),
        ("bvwbjplbgvbhsrlpgdmjqwftvncz", 23),
        ("nppdvjthqldpwncqszvftbrmjlhg", 23),
        ("nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg", 29),
        ("zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw", 26),
    ];

    static INIT: Once = Once::new();

    pub fn initialize() {
        INIT.call_once(|| {
            color_eyre::install().unwrap(); // I like colors
        });
    }

    #[test]
    fn test_part1_v1() -> Result<()> {
        initialize();

        for (input, expected_result) in INPUT_1_EXAMPLES_AND_RESULTS {
            let result = run_part1_v1(input)?;
            assert_eq!(result, expected_result);
        }

        Ok(())
    }

    #[test]
    fn test_part2() -> Result<()> {
        initialize();

        for (input, expected_result) in INPUT_2_EXAMPLES_AND_RESULTS {
            let result = run_part2_v1_hash_set(input)?;
            assert_eq!(result, expected_result);
        }

        for (input, expected_result) in INPUT_2_EXAMPLES_AND_RESULTS {
            let result = run_part2_v2_hash_set_reused(input)?;
            assert_eq!(result, expected_result);
        }

        for (input, expected_result) in INPUT_2_EXAMPLES_AND_RESULTS {
            let result = run_part2_v3_no_hashset(input)?;
            assert_eq!(result, expected_result);
        }

        for (input, expected_result) in INPUT_2_EXAMPLES_AND_RESULTS {
            let result = run_part2_v4_no_hashset_no_bound_check(input)?;
            assert_eq!(result, expected_result);
        }

        for (input, expected_result) in INPUT_2_EXAMPLES_AND_RESULTS {
            let result = run_part2_v5_array_counters(input)?;
            assert_eq!(result, expected_result);
        }

        for (input, expected_result) in INPUT_2_EXAMPLES_AND_RESULTS {
            let result = run_part2_v6_bit_tricks(input)?;
            assert_eq!(result, expected_result);
        }

        for (input, expected_result) in INPUT_2_EXAMPLES_AND_RESULTS {
            let result = run_part2_v7_bit_tricks_xor(input)?;
            assert_eq!(result, expected_result);
        }

        Ok(())
    }
}
