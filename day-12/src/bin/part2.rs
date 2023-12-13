use std::fs;
use std::collections::HashMap;

fn main() {
    let content = fs::read_to_string("in.txt")
        .expect("Failed to read file");

    let mut result: u64 = 0;
    for line in content.lines() {
        let mut iter = line.split_whitespace();
        let sequence = format!("{}?", iter.next().unwrap()).repeat(5);
        let sequence = &sequence[..sequence.len() - 1].as_bytes();
        let groups = format!("{},", iter.next().unwrap()).repeat(5);
        let groups: &Vec<u8> = &groups[..groups.len() - 1].split(",")
            .map(|x| x.parse::<u8>().unwrap()).collect();

        result += follow(&mut HashMap::with_capacity(4095), sequence, 0, groups, 0, 0);
    }

    println!("Result: {}", result);
}

fn follow(results: &mut HashMap<u32, u64>, seq: &[u8], seq_idx: usize, grps: &Vec<u8>, grp_idx: usize, grp_len: usize) -> u64 {
    let groups = grps.len();
    if grp_idx > groups || (grp_idx < groups && grp_len > grps[grp_idx] as usize) {
        return 0;
    }
    
    if seq_idx == seq.len() {
        if (grp_idx == groups && grp_len == 0) || (grp_idx == groups - 1 && grp_len == grps[grp_idx] as usize) {
            return 1;
        } else {
            return 0;
        }
    }

    let key: u32 = (seq_idx as u32) << 16 | (grp_idx as u32) << 8 | (grp_len as u32);
    if let Some(&val) = results.get(&key) {
        return val;
    }

    let mut result: u64 = 0;
    let curr = seq[seq_idx];
    if curr == b'.' || curr  == b'?' {
        if grp_idx == groups || grp_len == 0 || grp_len == grps[grp_idx] as usize {
            result += follow(results, seq, seq_idx + 1, grps, if grp_len > 0 { grp_idx + 1 } else { grp_idx }, 0)
        }
    }

    if curr == b'#' || curr == b'?' {
        result += follow(results, seq, seq_idx + 1, grps, grp_idx, grp_len + 1)
    }
    
    results.insert(key, result);
    return result;
}