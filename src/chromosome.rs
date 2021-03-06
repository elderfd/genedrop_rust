extern crate readonly;

use crate::errors::*;
use rand::prelude::*;

type Locus = i32;
type GeneticDistance = f64;
type LociValues = std::vec::Vec<Locus>;

pub type ChromosomeMap = std::vec::Vec<GeneticDistance>;

#[readonly::make]
#[derive(Debug, Clone)]
pub struct Chromosome<'a> {
    #[readonly]
    pub loci: LociValues,
    map: &'a ChromosomeMap,
}

impl<'a> Chromosome<'a> {
    /// Constructs a new chromosome
    pub fn new(map: &'a ChromosomeMap) -> Self {
        Chromosome {
            loci: vec![Default::default(); map.len() + 1],
            map,
        }
    }

    /// Sets the loci values
    pub fn set_loci(&mut self, loci: LociValues) -> Option<String> {
        if self.map.len() != loci.len() - 1 {
            return Some(format!(
                "Cannot assign loci of size {}. Map expects {}.",
                loci.len(),
                self.map.len() + 1
            ));
        }

        self.loci = loci;

        None
    }

    /// Returns the number of loci
    pub fn len(&self) -> usize {
        self.map.len() + 1
    }

    pub fn is_empty(&self) -> bool {
        self.map.is_empty()
    }
}

impl std::ops::Index<usize> for Chromosome<'_> {
    type Output = Locus;

    fn index(&self, index: usize) -> &Self::Output {
        &self.loci[index]
    }
}

impl std::ops::IndexMut<usize> for Chromosome<'_> {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.loci[index]
    }
}

/// Recombines two chromosomes
///
/// # Arguments
///
/// * `father_chromosome` - The first chromosome to draw loci from
/// * `mother_chromosome` - The second chromosome to draw loci from
///
/// # Example
///
/// ```
/// use genedrop_core::chromosome;
///
/// let map = vec![0.5, 0.1, 0.05];
///
/// let mut father_chromosome = chromosome::Chromosome::new(&map);
/// father_chromosome.set_loci(vec![1, 1, 1, 0]);
/// let mut mother_chromosome = chromosome::Chromosome::new(&map);
/// mother_chromosome.set_loci(vec![1, 0, 1, 1]);
///
/// let child = chromosome::recombine(&father_chromosome, &mother_chromosome);
/// ```
pub fn recombine<'a>(
    father_chromosome: &Chromosome<'a>,
    mother_chromosome: &Chromosome<'a>,
) -> Result<Chromosome<'a>> {
    if !std::ptr::eq(father_chromosome.map, mother_chromosome.map) {
        bail!("Chromosomes with different maps cannot be recombined.");
    }

    // TODO: It's not actually mother and father chromosomes is it???

    // NB: It doesn't matter whether we use father or mother chromosome as must be the same
    let mut child_chromosome: Chromosome = Chromosome::new(father_chromosome.map);

    // Choose initial chromosome to choose from at random
    let mut choose_from: &Chromosome;
    let mut other_chromosome: &Chromosome;

    if random::<f64>() < 0.5 {
        choose_from = &father_chromosome;
        other_chromosome = &mother_chromosome;
    } else {
        choose_from = &mother_chromosome;
        other_chromosome = &father_chromosome;
    };

    for i in 0..child_chromosome.len() {
        child_chromosome[i] = choose_from[i];

        if i < child_chromosome.len() - 1 && random::<f64>() < child_chromosome.map[i] {
            std::mem::swap(&mut choose_from, &mut other_chromosome);
        }
    }

    Ok(child_chromosome)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[allow(unused_must_use)]
    #[test]
    /// Tests that attempting to recombine chromosomes of unequal size gives an error
    fn unequal_chromosome_error() {
        let map = vec![0.5, 0.1, 0.05];

        let mut father_chromosome = Chromosome::new(&map);
        let father_err = father_chromosome.set_loci(vec![1, 1, 1]);
        let mut mother_chromosome = Chromosome::new(&map);
        let mother_err = mother_chromosome.set_loci(vec![1, 0, 1, 1]);

        // TODO: This is actually testing chromosome, not recombination per se
        assert!(father_err.is_some());
        assert!(!mother_err.is_some());
    }

    #[test]
    // Tests that an error is returned if chromosomes with different maps are recombined
    fn different_maps_recombine() {
        let father_map = vec![0.5, 0.1, 0.05];
        let mother_map = vec![0.5, 0.1, 0.05];

        let mut father_chromosome = Chromosome::new(&father_map);
        let father_err = father_chromosome.set_loci(vec![1, 1, 1, 0]);
        let mut mother_chromosome = Chromosome::new(&mother_map);
        let mother_err = mother_chromosome.set_loci(vec![1, 0, 1, 1]);

        assert!(father_err.is_none());
        assert!(mother_err.is_none());

        let result = recombine(&father_chromosome, &mother_chromosome);

        assert!(result.is_err());
    }

    #[test]
    /// Tests that recombination doesn't throw an error
    fn can_work_at_all() {
        let map = vec![0.5, 0.1, 0.05];

        let mut father_chromosome = Chromosome::new(&map);
        let father_err = father_chromosome.set_loci(vec![1, 1, 1, 0]);
        let mut mother_chromosome = Chromosome::new(&map);
        let mother_err = mother_chromosome.set_loci(vec![1, 0, 1, 1]);

        assert!(father_err.is_none());
        assert!(mother_err.is_none());

        let result = recombine(&father_chromosome, &mother_chromosome);

        assert!(result.is_ok());
    }
}
