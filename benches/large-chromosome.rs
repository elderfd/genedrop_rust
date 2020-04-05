#![feature(test)]
extern crate test;

#[cfg(test)]
mod tests {
    use super::*;
    use genedrop_core::chromosome;
    use rand::distributions::{Distribution, Uniform};
    use test::Bencher;

    #[bench]
    /// Attempts to recombine two very large chromosomes
    fn bench_10_mil_loci_recombine(b: &mut Bencher) {
        let n_loci: usize = 10000000;
        let gen_distance_dist = Uniform::new_inclusive(0.0, 0.5);
        let locus_value_dist = Uniform::new(0, 5);
        let mut rng = rand::thread_rng();

        let mut map: chromosome::ChromosomeMap = vec![];

        for _ in 0..(n_loci - 1) {
            map.push(gen_distance_dist.sample(&mut rng));
        }

        let mut father = chromosome::Chromosome::new(&map);
        let mut father_loci = vec![];
        for _ in 0..n_loci {
            father_loci.push(locus_value_dist.sample(&mut rng));
        }
        father.set_loci(father_loci);

        let mut mother = chromosome::Chromosome::new(&map);
        let mut mother_loci = vec![];
        for _ in 0..n_loci {
            mother_loci.push(locus_value_dist.sample(&mut rng));
        }
        mother.set_loci(mother_loci);

        b.iter(|| chromosome::recombine(&father, &mother))
    }
}
