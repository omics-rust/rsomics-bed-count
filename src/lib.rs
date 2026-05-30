use rsomics_common::{Result, RsomicsError};
use std::io::BufRead;

/// Count non-header, non-blank BED records.
pub fn count<R: BufRead>(reader: R) -> Result<u64> {
    let mut n: u64 = 0;
    for line in reader.lines() {
        let line = line.map_err(RsomicsError::Io)?;
        if !line.starts_with('#') && !line.is_empty() {
            n += 1;
        }
    }
    Ok(n)
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Cursor;

    #[test]
    fn basic() {
        let input = "chr1\t0\t100\nchr2\t0\t200\n";
        assert_eq!(count(Cursor::new(input)).unwrap(), 2);
    }

    #[test]
    fn skips_header_and_blank() {
        let input = "# comment\nchr1\t0\t100\n\nchr2\t0\t200\n";
        assert_eq!(count(Cursor::new(input)).unwrap(), 2);
    }

    #[test]
    fn empty_file() {
        assert_eq!(count(Cursor::new("")).unwrap(), 0);
    }
}
