use std::collections::HashMap;

use proconio::input;

struct Dice {
    total: usize,
    faces: HashMap<usize, usize>,
}

impl Dice {
    fn build(faces: Vec<usize>) -> Self {
        let mut face_counts = HashMap::new();
        let total = faces.len();
        for face in faces {
            *face_counts.entry(face).or_insert(0) += 1;
        }
        Dice {
            total,
            faces: face_counts,
        }
    }

    fn face_prob(&self, face: usize) -> f64 {
        if let Some(&count) = self.faces.get(&face) {
            count as f64 / self.total as f64
        } else {
            0.0
        }
    }

    fn same_face_prob(&self, other: &Self) -> f64 {
        let mut prob = 0f64;

        for &face in self.faces.keys() {
            prob += self.face_prob(face) * other.face_prob(face);
        }

        prob
    }
}

fn main() {
    input! {
        n: usize,
        dice: [[usize]; n],
    }

    let dice = dice.into_iter().map(Dice::build).collect::<Vec<_>>();

    let mut max_prob = 0f64;

    for x in 0..n {
        for y in 0..n {
            if x == y {
                continue;
            }

            max_prob = max_prob.max(dice[x].same_face_prob(&dice[y]));
        }
    }

    println!("{:.10}", max_prob);
}
