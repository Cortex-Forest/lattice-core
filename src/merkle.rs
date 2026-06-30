use crate::crypto::hash;

pub fn merkle_root(data: &[String]) -> String {
    if data.is_empty() {
        return "0".to_string();
    }

    let mut level = data.to_vec();

    while level.len() > 1 {
        let mut next = vec![];

        for i in (0..level.len()).step_by(2) {
            let left = &level[i];
            let right = if i + 1 < level.len() { &level[i + 1] } else { left };

            next.push(hash(&(left.clone() + right)));
        }

        level = next;
    }

    level[0].clone()
}