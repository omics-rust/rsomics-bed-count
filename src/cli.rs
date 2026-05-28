use clap::Parser;
use rsomics_bed_count::count;
use rsomics_common::{CommonFlags, Result, RsomicsError, Tool, ToolMeta};
use rsomics_help::{Example, FlagSpec, HelpSpec, Section};
use std::fs::File;
use std::io::{self, BufReader};
use std::path::PathBuf;

pub const META: ToolMeta = ToolMeta {
    name: env!("CARGO_PKG_NAME"),
    version: env!("CARGO_PKG_VERSION"),
};

pub const HELP: HelpSpec = HelpSpec {
    name: META.name,
    version: META.version,
    tagline: "Count BED records (non-header, non-blank lines). Prints one integer.",
    origin: None,
    usage_lines: &["[INPUT]"],
    sections: &[Section {
        title: "OPTIONS",
        flags: &[FlagSpec {
            short: Some('h'),
            long: "help",
            aliases: &[],
            value: None,
            type_hint: Some("bool"),
            required: false,
            default: None,
            description: "Show this help",
            why_default: None,
        }],
    }],
    examples: &[
        Example {
            description: "Count records from file",
            command: "rsomics-bed-count intervals.bed",
        },
        Example {
            description: "Pipe from stdin",
            command: "cat intervals.bed | rsomics-bed-count",
        },
    ],
    json_result_schema_doc: None,
};

#[derive(Parser, Debug)]
#[command(name = "rsomics-bed-count", disable_help_flag = true)]
pub struct Cli {
    /// Input BED file (default: stdin)
    pub input: Option<PathBuf>,

    #[command(flatten)]
    pub common: CommonFlags,
}

impl Tool for Cli {
    fn meta() -> ToolMeta {
        META
    }
    fn common(&self) -> &CommonFlags {
        &self.common
    }

    fn execute(self) -> Result<()> {
        let n = match &self.input {
            Some(p) => {
                let reader = BufReader::new(File::open(p).map_err(RsomicsError::Io)?);
                count(reader)?
            }
            None => {
                let stdin = io::stdin();
                count(stdin.lock())?
            }
        };
        println!("{n}");
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use clap::CommandFactory;

    #[test]
    fn cli_definition_is_valid() {
        super::Cli::command().debug_assert();
    }
}
