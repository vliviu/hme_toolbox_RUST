use hme_toolbox::data::fasta;

#[test]
fn test_read_fasta() {
    let result = fasta::read_fasta("tests/sample.fasta");
    assert!(result.is_ok());
    let sequences = result.unwrap();
    assert!(sequences.len() > 0);
    assert_eq!(sequences[0].0, "sequence1");
}


