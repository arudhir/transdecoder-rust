use crate::types::{ORF, ORFType, Strand};
use std::collections::HashMap;  // I can't find the codon table in rust-bio??

pub fn make_codon_table() -> HashMap<&'static str, char> {
        [
        // Phenylalanine
        ("TTT", 'F'), ("TTC", 'F'),
        // Leucine
        ("TTA", 'L'), ("TTG", 'L'), ("CTT", 'L'), ("CTC", 'L'), ("CTA", 'L'), ("CTG", 'L'),
        // Isoleucine
        ("ATT", 'I'), ("ATC", 'I'), ("ATA", 'I'),
        // Methionine (start)
        ("ATG", 'M'),
        // Valine
        ("GTT", 'V'), ("GTC", 'V'), ("GTA", 'V'), ("GTG", 'V'),
        // Serine
        ("TCT", 'S'), ("TCC", 'S'), ("TCA", 'S'), ("TCG", 'S'),
        ("AGT", 'S'), ("AGC", 'S'),
        // Proline
        ("CCT", 'P'), ("CCC", 'P'), ("CCA", 'P'), ("CCG", 'P'),
        // Threonine
        ("ACT", 'T'), ("ACC", 'T'), ("ACA", 'T'), ("ACG", 'T'),
        // Alanine
        ("GCT", 'A'), ("GCC", 'A'), ("GCA", 'A'), ("GCG", 'A'),
        // Tyrosine
        ("TAT", 'Y'), ("TAC", 'Y'),
        // Histidine
        ("CAT", 'H'), ("CAC", 'H'),
        // Glutamine
        ("CAA", 'Q'), ("CAG", 'Q'),
        // Asparagine
        ("AAT", 'N'), ("AAC", 'N'),
        // Lysine
        ("AAA", 'K'), ("AAG", 'K'),
        // Aspartic Acid
        ("GAT", 'D'), ("GAC", 'D'),
        // Glutamic Acid
        ("GAA", 'E'), ("GAG", 'E'),
        // Cysteine
        ("TGT", 'C'), ("TGC", 'C'),
        // Tryptophan
        ("TGG", 'W'),
        // Arginine
        ("CGT", 'R'), ("CGC", 'R'), ("CGA", 'R'), ("CGG", 'R'),
        ("AGA", 'R'), ("AGG", 'R'),
        // Glycine
        ("GGT", 'G'), ("GGC", 'G'), ("GGA", 'G'), ("GGG", 'G'),
        // Stop codons
        ("TAA", '*'), ("TAG", '*'), ("TGA", '*'),
    ]
    .into_iter()
    .collect()  // No ; means that we're returning this
}
/*
- Publicly discoverable function
- Takes in a borrowed string reference
    - There are three types of values
        - Owned: When out of scope, Rust frees it from memory; String, Vec<T>
        - Borrowed reference: The data is borrowed immutably, read-only; no clone/move; &T
        - Mutable reference: The data is borrowed mutably, but you can only have one mutable reference at a time
            - i.e., let's say we wanted to add a stop codon manually
- Returns a vector of ORFs (list of ORFs)
*/

pub fn find_orfs(seq: &str, strand: Strand) -> Vec<ORF> {
    let mut orfs = Vec::new();  // A dynamic vector of ORFs as an accumulator, like orfs = [] in
                               // Python
    // Figure out which strand to scan

    // For each frame:
        // If the strand is Strand::Minus, reverse-complement the string
        // Frame-shift
    // Scan through the frame codon-by-codon
        // For each codon:
            // If it is a start codon, remember its index
            // If it's an end codon AND we have a start codon,
                // Calculate the start/end positions (in nucleotides)
                // Extract the ORF
                // Translate to protein
                // Classify the ORF as complete, partial, internal (helper function classify_orf)
                // Create an ORF Struct and add it to the orfs vector
    // Return the list of ORFs
    return orfs;
}

fn is_start(seq: &str) -> bool {
    return false
}

fn is_stop(seq: &str) -> bool {
    return false
}

fn translate(seq: &str) -> String {
    let chars: Vec<char> = seq.chars().collect();  // seq.chars() turns it into an iter of
                                                   // characters. .collect() consumes the iter and
                                                   // builds something new
    let mut protein = String::new();
    let codon_table = make_codon_table();
    for codon in chars.chunks(3) {
        if codon.len() < 3 {
            continue;
        }
        let codon_str = codon.iter().collect::<String>();
        let aa = codon_table[&codon_str.as_str()];
        protein.push(aa);
    }
    return protein;
}

fn classify_orf(orf: &str) -> ORFType {
    //
    return ORFType::Complete;
}

fn reverse_complement(seq: &str) -> String {
    return String::new();
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_translate_basic() {
        let seq = "ATG".into();
        let aa_str = translate(seq);
        assert_eq!(aa_str, "M");
    }

    #[test]
    fn test_translate_intermediate() {
        let seq = "ATGAAATGA".into();
        let aa_str = translate(seq);
        assert_eq!(aa_str, "MK*");
    }

    #[test]
    fn test_translate_non_multiple_3() {
        let seq = "ATGAAATGAA".into();  // 10 nts, should drop the last one
        let aa_str = translate(seq);
        assert_eq!(aa_str, "MK*");
    }
}
