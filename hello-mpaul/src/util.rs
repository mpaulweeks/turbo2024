use crate::*;

pub type Rands = Vec<u32>;
pub fn shuffle<T: Clone>(arr: Vec<T>, rands: &mut Rands) -> Vec<T> {
    let mut copy = arr.clone();
    let mut out: Vec<T> = Vec::new();
    while copy.len() > 0 {
        let random_num = if let Some(seeded) = rands.pop() {
            seeded
        } else {
            rand()
        };
        let index = (random_num % copy.len() as u32) as usize;
        out.push(copy.remove(index));
    }
    return out;
}
