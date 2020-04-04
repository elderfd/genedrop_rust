fn main() {
    let map = vec![0.5, 0.1, 0.05];

    let mut father_chromosome = genedrop_core::Chromosome::new(&map);
    father_chromosome.set_loci(vec![1, 1, 1, 0]);

    let mut mother_chromosome = genedrop_core::Chromosome::new(&map);
    mother_chromosome.set_loci(vec![1, 0, 1, 1]);

    let child_chromosome = genedrop_core::recombine(&father_chromosome, &mother_chromosome);

    println!("Hello, world!");
    println!("{:?}", child_chromosome);
}
