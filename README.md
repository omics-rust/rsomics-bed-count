# rsomics-bed-count

Count BED records (non-header, non-blank lines). Prints one integer to stdout.

## Usage

```sh
rsomics-bed-count [INPUT]
rsomics-bed-count intervals.bed
cat intervals.bed | rsomics-bed-count
```

## Origin

Independent Rust implementation. Equivalent to:

```sh
grep -cv '^#' intervals.bed
```

License: MIT OR Apache-2.0.
