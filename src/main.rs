fn main() {
    let father_chromosome: genedrop_core::Chromosome = vec![1, 1, 1, 0];
    let mother_chromosome: genedrop_core::Chromosome = vec![1, 0, 1, 0];

    let child_chromosome = genedrop_core::recombine(father_chromosome, mother_chromosome);

    println!("Hello, world!");
    println!("{:?}", child_chromosome);
}
