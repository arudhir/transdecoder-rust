#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Strand {
    Plus,  // <- public by default because Strand is pub
    Minus,
    Unknown
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum ORFType {
    Complete,
    FivePrimePartial,
    ThreePrimePartial,
    Internal
}

#[derive(Debug, Clone, PartialEq)]
pub struct ORF {
    pub transcript_id: String,
    pub start: u32,
    pub end: u32,
    pub strand: Strand,
    pub frame: u8,  // 0, 1, 2
    pub length: u64,
    pub sequence: String,
    pub protein: String,
    pub has_start: bool,
    pub has_stop: bool,
    pub orf_type: ORFType,
    pub score: f64,  // Markov model score
}

#[cfg(test)]
// Rust convention seems to enjoy tests in the appropriate file defined as a "submodule"
mod tests {
    use super::*;  // Since this is a module within the file, we want to import the namespace for
                   // the stuff inside this file
    #[test]
    fn test_strand_plus() {
        let s = Strand::Plus;
        assert_eq!(s, Strand::Plus);
    }
    
    #[test]
    fn test_strand_minus() {
        let s = Strand::Minus;
        assert_eq!(s, Strand::Minus);
    }

    #[test]
    fn test_strand_unknown() {
        let s = Strand::Unknown;
        assert_eq!(s, Strand::Unknown);
    }

    #[test]
    fn test_orftype_complete() {
        let o = ORFType::Complete;
        assert_eq!(o, ORFType::Complete);
    }

    #[test]
    fn test_orftype_5prime_partial() {
        let o = ORFType::FivePrimePartial;
        assert_eq!(o, ORFType::FivePrimePartial);
    }

    #[test]
    fn test_orftype_3prime_partial() {
        let o = ORFType::ThreePrimePartial;
        assert_eq!(o, ORFType::ThreePrimePartial);
    }

    #[test]
    fn test_orftype_internal() {
        let o = ORFType::Internal;
        assert_eq!(o, ORFType::Internal);
    }

    #[test]
    fn test_orf_construction() {
        let orf = ORF {
            transcript_id: "transcript1".into(),  // We're using a string literal, &'static str,
                                                   // and we want to coerce(?) it into a String
            start: 1,
            end: 300,
            strand: Strand::Plus,
            frame: 0,
            length: 300,
            sequence: "ATGAGCAAAGGCGAAGAACTGTTTACCGGCGTGGTGCCGATTCTGGTGGAACTGGATGGCGATGTGAACGGCCATAAATTTAGCGTGAGCGGCGAAGGCGAAGGCGATGCGACCTATGGCAAACTGACCCTGAAATTTATTTGCACCACCGGCAAACTGCCGGTGCCGTGGCCGACCCTGGTGACCACCTTTAGCTATGGCGTGCAGTGCTTTAGCCGCTATCCGGATCATATGAAACAGCATGATTTTTTTAAAAGCGCGATGCCGGAAGGCTATGTGCAGGAACGCACCATTTTTTTTAAAGATGATGGCAACTATAAAACCCGCGCGGAAGTGAAATTTGAAGGCGATACCCTGGTGAACCGCATTGAACTGAAAGGCATTGATTTTAAAGAAGATGGCAACATTCTGGGCCATAAACTGGAATATAACTATAACAGCCATAACGTGTATATTATGGCGGATAAACAGAAAAACGGCATTAAAGTGAACTTTAAAATTCGCCATAACATTGAAGATGGCAGCGTGCAGCTGGCGGATCATTATCAGCAGAACACCCCGATTGGCGATGGCCCGGTGCTGCTGCCGGATAACCATTATCTGAGCACCCAGAGCGCGCTGAGCAAAGATCCGAACGAAAAACGCGATCATATGGTGCTGCTGGAATTTGTGACCGCGGCGGGCATTACCCATGGCATGGATGAACTGTATAAA".into(),
            protein: "MSKGEELFTGVVPILVELDGDVNGHKFSVSGEGEGDATYGKLTLKFICTTGKLPVPWPTLVTTFSYGVQCFSRYPDHMKQHDFFKSAMPEGYVQERTIFFKDDGNYKTRAEVKFEGDTLVNRIELKGIDFKEDGNILGHKLEYNYNSHNVYIMADKQKNGIKVNFKIRHNIEDGSVQLADHYQQNTPIGDGPVLLPDNHYLSTQSALSKDPNEKRDHMVLLEFVTAAGITHGMDELYK".into(),
            has_start: true,
            has_stop: true,
            orf_type: ORFType::Complete,
            score: 0.0
        };
        assert_eq!(orf.frame, 0);
        assert_eq!(orf.orf_type, ORFType::Complete);
    }
}
