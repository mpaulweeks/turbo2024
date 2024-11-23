use crate::*;

pub fn shuffle<T: Clone>(arr: Vec<T>) -> Vec<T> {
    let mut copy = arr.clone();
    let mut out: Vec<T> = Vec::new();
    while (copy.len() > 0) {
        let index = (rand() % copy.len() as u32) as usize;
        out.push(copy.remove(index));
    }
    return out;
}
