use std::{fs, vec};

fn main() {
    let content = fs::read_to_string("in.txt")
        .expect("Failed to read file");

    let mut boxes : Vec<Vec<(String, u8)>> = vec![vec![]; 256];
    for s in content.split(",") {
        if s.ends_with('-') {
            let label = &s[..s.len() - 1];
            let index = hash(label) as usize;
            let slot = boxes[index]
                .iter()
                .position(|b| b.0 == label);

            if let Some(i) = slot {
                boxes[index].remove(i);
            }
        } else {
            let label = &s[..s.len() - 2];
            let index = hash(label) as usize;
            let focal: u8 = s[s.len() - 1..]
                .parse()
                .unwrap_or(0);
            
            let tuple = boxes[index]
                .iter_mut()
                .find(|b| b.0 == label);

            if let Some(t) = tuple {
                t.1 = focal;
            } else {
                boxes[index].push((label.to_string(), focal));
            }
        }
    }

    // One plus the box number of the lens in question.
    // The slot number of the lens within the box: 1 for the first lens, 2 for the second lens, and so on.
    // The focal length of the lens.
    let mut sum = 0;
    for (i, _box) in boxes.iter().enumerate() {
        for (j, tuple) in boxes[i].iter().enumerate() {
            sum += (i + 1) * (j + 1) * tuple.1 as usize;
        }
    }

    println!("Sum: {}", sum);
}


// Determine the ASCII code for the current character of the string.
// Increase the current value by the ASCII code you just determined.
// Set the current value to itself multiplied by 17.
// Set the current value to the remainder of dividing itself by 256.
fn hash(s: &str) -> u32 {
    let mut value = 0;
    for b in s.bytes() {
        value += b as u32;
        value *= 17;
        value %= 256;
    }
    value
}