use std::collections::HashSet;

pub fn find(sum: u32) -> HashSet<[u32; 3]> {
    let mut res = HashSet::new();
    for i in 1..sum {
        for j in 1..sum {
            if sum <= i+j {
                break;
            }
            let k = sum - i - j;
            let mut triplet : [u32; 3] = [i, j, k];
            triplet.sort();
            if triplet[0]*triplet[0] + triplet[1]*triplet[1] == triplet[2]*triplet[2] {
                res.insert(triplet);
            }
        }
    }
    res
    }
