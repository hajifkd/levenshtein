use std::io;

#[derive(Clone, Debug, Copy)]
enum Operation {
    Add,
    Delete,
    Replace,
    Nop,
}

fn main() -> Result<(), Box<std::error::Error>> {
    let mut s1 = String::new();
    let mut s2 = String::new();

    println!("String 1?",);
    let _ = io::stdin().read_line(&mut s1)?;
    println!("String 2?",);
    let _ = io::stdin().read_line(&mut s2)?;

    let s1: Vec<_> = s1.trim().chars().collect();
    let s2: Vec<_> = s2.trim().chars().collect();

    let mut ops = vec![vec![vec![]; s2.len() + 1]; s1.len() + 1];
    let mut dists = vec![vec![0; s2.len() + 1]; s1.len() + 1];

    ops[0][0] = vec![Operation::Nop];

    for i in 1..=s1.len() {
        ops[i][0] = ops[i - 1][0].clone();
        ops[i][0].push(Operation::Add);
        dists[i][0] = dists[i - 1][0] + 1;
    }

    for i in 1..=s2.len() {
        ops[0][i] = ops[0][i - 1].clone();
        ops[0][i].push(Operation::Delete);
        dists[0][i] = dists[0][i - 1] + 1;
    }

    for i in 1..=s1.len() {
        for j in 1..=s2.len() {
            let eqs = s1[i - 1] == s2[j - 1];
            let lengths = [
                if eqs {
                    (0, (i - 1, j - 1), Operation::Nop)
                } else {
                    (1, (i - 1, j - 1), Operation::Replace)
                },
                (1, (i - 1, j), Operation::Add),
                (1, (i, j - 1), Operation::Delete),
            ];

            let (dist, (i1, j1), op) = lengths
                .into_iter()
                .map(|&(dd, (i, j), op)| (dd + dists[i][j], (i, j), op))
                .min_by(|a, b| a.0.cmp(&b.0))
                .unwrap();

            dists[i][j] = dist;
            ops[i][j] = ops[i1][j1].clone();
            ops[i][j].push(op);
        }
    }

    println!("dist = {}", dists[s1.len()][s2.len()]);
    println!("ops = {:?}", &ops[s1.len()][s2.len()][1..]);

    Ok(())
}
