use mnist::MnistBuilder;
use std::usize;

fn main() {
    let data = MnistBuilder::new().finalize();

    let mut counts: [usize; 10] = [0; 10];
    for label in data.trn_lbl {
        counts[usize::from(label)] += 1;
    }
    for index in 0..counts.len() {
        println!("{}: {}", index, counts[index])
    }
}
