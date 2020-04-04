extern crate uuid;

use crate::chromosome::recombine;
use crate::chromosome::Chromosome;
use crate::errors::*;
use std::vec::Vec;
use uuid::Uuid;

static new_uuid: fn() -> Uuid = Uuid::new_v4;

type Homology<'a> = Vec<Chromosome<'a>>;

pub struct Individual<'a> {
    pub id: String,
    pub homologies: Vec<Homology<'a>>,
}

impl<'a> Individual<'a> {
    pub fn new() -> Self {
        Individual {
            id: new_uuid().to_string(),
            homologies: Default::default(),
        }
    }

    pub fn ploidy(&self) -> Result<usize> {
        if self.homologies.len() == 0 || self.homologies[0].len() == 0 {
            bail!("Cannot determine ploidy, no genotype data.");
        }

        return Ok(self.homologies[0].len());
    }

    pub fn chromosome_number(&self) -> Result<usize> {
        if self.homologies.len() == 0 {
            bail!("Cannot determine number of chromosomes, no genotype data.");
        }

        // TODO: This info is also in the map - maybe better to retrieve from there?
        return Ok(self.homologies.len());
    }

    pub fn get_gamete(&self, homology_index: usize) -> Result<Chromosome> {
        let ploidy = self
            .ploidy()
            .chain_err(|| "Error determing ploidy for gamete generation")?;

        if ploidy != 2 {
            bail!("Gamete production only implemented for diploids.");
        }

        recombine(
            &self.homologies[homology_index][0],
            &self.homologies[homology_index][1],
        )
    }
}

pub fn breed<'a>(father: &'a Individual, mother: &'a Individual) -> Result<Individual<'a>> {
    let father_ploidy = father
        .ploidy()
        .chain_err(|| "Error determining father ploidy")?;

    let mother_ploidy = mother
        .ploidy()
        .chain_err(|| "Error determining mother ploidy")?;

    if father_ploidy != mother_ploidy {
        bail!(format!(
            "Cannot breed organisms of different ploidies. Father has ploidy {} and mother {}.",
            father_ploidy, mother_ploidy
        ));
    }

    // TODO: There is some complexity here as to how to choose which chromosomes to recombine if ploidy is greater than 2
    if father_ploidy > 2 {
        bail!("Breeding not implemented for ploidies above 2.");
    }

    // TODO: Need to check that the individuals are using the same maps

    // If all the above checks pass, then maybe we are ready to do some breeding!
    let mut child = Individual::new();

    for chromosome_number in 0..father.chromosome_number().unwrap() {
        let mut new_homology: Homology = vec![];

        let father_gamete = father.get_gamete(chromosome_number).chain_err(|| {
            format!(
                "Error generating father gamete for chromosome number {}",
                chromosome_number
            )
        })?;

        let mother_gamete = mother.get_gamete(chromosome_number).chain_err(|| {
            format!(
                "Error generating mother gamete for chromosome number {}",
                chromosome_number
            )
        })?;

        new_homology.push(father_gamete);
        new_homology.push(mother_gamete);

        child.homologies.push(new_homology);
    }

    return Ok(child);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    /// Tests that breeding doesn't throw an error
    fn can_work() {
        let map = vec![0.5];

        let mut mother = Individual::new();
        let mut mother_chrom = Chromosome::new(&map);
        mother_chrom.set_loci(vec![1, 1]);

        let mother_chrom_clone = mother_chrom.clone();

        mother
            .homologies
            .push(vec![mother_chrom, mother_chrom_clone]);

        let mut father = Individual::new();
        let mut father_chrom = Chromosome::new(&map);
        father_chrom.set_loci(vec![0, 0]);

        let father_chrom_clone = father_chrom.clone();

        father
            .homologies
            .push(vec![father_chrom, father_chrom_clone]);

        assert!(breed(&father, &mother).is_ok());
    }
}
