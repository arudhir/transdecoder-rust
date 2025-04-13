use crate::types::{ORF, ORFType, Strand};
use std::collections::HashMap;  // I can't find the codon table in rust-bio??

pub fn make_codon_table() -> HashMap<&'static str, char> {
        [        
        ("TTT", 'F'), ("TTC", 'F'),  
        ("TTA", 'L'), ("TTG", 'L'), ("CTT", 'L'), ("CTC", 'L'), ("CTA", 'L'), ("CTG", 'L'),  
        ("ATT", 'I'), ("ATC", 'I'), ("ATA", 'I'),  
        ("ATG", 'M'),  
        ("GTT", 'V'), ("GTC", 'V'), ("GTA", 'V'), ("GTG", 'V'),  
        ("TCT", 'S'), ("TCC", 'S'), ("TCA", 'S'), ("TCG", 'S'),  
        ("AGT", 'S'), ("AGC", 'S'),
        ("CCT", 'P'), ("CCC", 'P'), ("CCA", 'P'), ("CCG", 'P'),  
        ("ACT", 'T'), ("ACC", 'T'), ("ACA", 'T'), ("ACG", 'T'),  
        ("GCT", 'A'), ("GCC", 'A'), ("GCA", 'A'), ("GCG", 'A'),  
        ("TAT", 'Y'), ("TAC", 'Y'),  
        ("CAT", 'H'), ("CAC", 'H'),  
        ("CAA", 'Q'), ("CAG", 'Q'),  
        ("AAT", 'N'), ("AAC", 'N'),  
        ("AAA", 'K'), ("AAG", 'K'),  
        ("GAT", 'D'), ("GAC", 'D'),  
        ("GAA", 'E'), ("GAG", 'E'),  
        ("TGT", 'C'), ("TGC", 'C'),  
        ("TGG", 'W'),  
        ("CGT", 'R'), ("CGC", 'R'), ("CGA", 'R'), ("CGG", 'R'),  
        ("AGA", 'R'), ("AGG", 'R'),
        ("GGT", 'G'), ("GGC", 'G'), ("GGA", 'G'), ("GGG", 'G'),  
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
    let rev_seq = seq.chars().rev().collect::<String>();
    let rev_comp: String = rev_seq.chars().map(|c| match c {  // Similar-ish to a lambda function
                                                              // in Python
        'A' => 'T',  // Single quotes makes it a char literal, double quotes would make it a string
        'T' => 'A',
        'C' => 'G',
        'G' => 'C',
        _ => 'N'    // match requires all cases to be accounted for
    }).collect();
    println!("{} --> {}", seq, rev_comp);
    return rev_comp;
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

    #[test]
    fn test_reverse_complement() {
        let seq = "ATGAAATGA".into();
        let rev_comp = reverse_complement(seq);
        assert_eq!(rev_comp, "TCATTTCAT");
    }
}
