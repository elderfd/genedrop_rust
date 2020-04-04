use rand::prelude::*;

type Locus = i32;
pub type Chromosome = std::vec::Vec<Locus>;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    /// Tests that attempting to recombine chromosomes of unequal size gives an error
    fn unequal_chromosome_error() {
        let father_chromosome = vec![1, 1, 1, 0];
        let mother_chromosome = vec![1, 0, 1];

        assert!(recombine(father_chromosome, mother_chromosome).is_err());
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
/// use genedrop_core::recombine;
///
/// let child = recombine(vec![0, 1, 2, 1], vec![0, 2, 2, 1]);
/// ```
pub fn recombine<'a>(
    father_chromosome: Chromosome,
    mother_chromosome: Chromosome,
) -> Result<Chromosome, &'a str> {
    if father_chromosome.len() != mother_chromosome.len() {
        return Result::Err("Chromosomes of unequal length cannot be recombined.");
    }

    let mut child_chromosome: Chromosome = vec![0; father_chromosome.len()];

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

        // TODO: Add proper genetic distance
        if random::<f64>() < 0.5 {
            std::mem::swap(&mut choose_from, &mut other_chromosome);
        }
    }

    return Result::Ok(child_chromosome);
}
