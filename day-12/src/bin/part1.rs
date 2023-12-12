use std::fs;

fn main() {
    let content = fs::read_to_string("in.txt")
        .expect("Failed to read file");

    let mut result: u16 = 0;
    for line in content.lines() {
        let mut iter = line.split_whitespace();
        let sequence: &[u8] = iter.next().unwrap().as_bytes();
        let groups: Vec<u8> = iter.next().unwrap()
            .split(",").map(|x| x.parse::<u8>().unwrap())
            .collect();

        result += follow(sequence, 0, &groups, 0, 0);
    }

    println!("Result: {}", result);
}

fn follow(seq: &[u8], seq_idx: usize, grps: &Vec<u8>, grp_idx: usize, grp_len: usize) -> u16 {
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

    let mut result: u16 = 0;
    let curr = seq[seq_idx];
    if curr == b'.' || curr  == b'?' {
        if grp_idx == groups || grp_len == 0 || grp_len == grps[grp_idx] as usize {
            result += follow(seq, seq_idx + 1, grps, if grp_len > 0 { grp_idx + 1 } else { grp_idx }, 0)
        }
    }

    if curr == b'#' || curr == b'?' {
        result += follow(seq, seq_idx + 1, grps, grp_idx, grp_len + 1);
    }
    
    return result;
}