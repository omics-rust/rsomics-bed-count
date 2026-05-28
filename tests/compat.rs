use rsomics_bed_count::count;
use std::io::Cursor;

#[test]
fn count_basic() {
    let input = "chr1\t0\t100\nchr2\t0\t200\nchrX\t100\t200\n";
    assert_eq!(count(Cursor::new(input)).unwrap(), 3);
}

#[test]
fn count_skips_header() {
    let input = "# track name=test\nchr1\t0\t100\n";
    assert_eq!(count(Cursor::new(input)).unwrap(), 1);
}

#[test]
fn count_skips_blank() {
    let input = "chr1\t0\t100\n\nchr2\t0\t200\n";
    assert_eq!(count(Cursor::new(input)).unwrap(), 2);
}

#[test]
fn count_empty() {
    assert_eq!(count(Cursor::new("")).unwrap(), 0);
}

#[test]
fn wc_equiv() {
    // wc -l equivalent on non-header BED
    let lines = 10;
    let input: String = (0..lines)
        .map(|i| format!("chr1\t{}\t{}\n", i * 100, i * 100 + 100))
        .collect();
    assert_eq!(count(Cursor::new(input.as_str())).unwrap(), lines as u64);
}
