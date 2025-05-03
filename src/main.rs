use anyhow::Result;
use clap::{Parser, ValueEnum};
use std::{ffi::OsString, path::PathBuf, time::SystemTime};

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct CliArgs {
    /// Don't actually do anything.
    #[arg(long, short = 'n', default_value_t = false)]
    pub dry_run: bool,

    /// Print what happens.
    #[arg(long, short = 'v', default_value_t = false)]
    pub verbose: bool,

    /// Which timestamp to use.
    #[arg(value_enum, long, short = 'k', default_value_t = TimestampKind::Created)]
    pub kind: TimestampKind,

    #[arg()]
    pub file: Vec<PathBuf>,
}

#[derive(Debug, Clone, ValueEnum)]
pub enum TimestampKind {
    Created,
    Modified,

    /// created or modified, whichever is smaller
    Earliest,

    /// created or modified, whichever is larger
    Latest,
}

fn main() -> Result<()> {
    let args = CliArgs::parse();

    for path in &args.file {
        run(path, &args)?;
    }

    Ok(())
}

fn run(path: &PathBuf, args: &CliArgs) -> Result<()> {
    let meta = std::fs::metadata(path)?;
    let old_path = path; //.canonicalize()?;

    let timestamp = match args.kind {
        TimestampKind::Created => meta.created()?,
        TimestampKind::Modified => meta.modified()?,
        TimestampKind::Earliest => {
            let a = meta.created()?;
            let b = meta.modified()?;
            a.min(b)
        },
        TimestampKind::Latest => {
            let a = meta.created()?;
            let b = meta.modified()?;
            a.max(b)
        },
    };

    let prefix = get_prefix(timestamp)?;
    let base_name = old_path.file_name().unwrap();

    let mut base_name_prefixed = OsString::from(prefix);
    base_name_prefixed.push("_");
    base_name_prefixed.push(&base_name);

    let mut new_path = old_path.clone();
    new_path.set_file_name(base_name_prefixed);

    if args.dry_run || args.verbose {
        println!("mv {:?} {:?}", old_path, new_path);
    }

    if !args.dry_run {
        std::fs::rename(old_path, new_path)?;
    }

    Ok(())
}

fn get_prefix(system_time: SystemTime) -> anyhow::Result<String> {
    use time::macros::format_description;

    let ts = time::OffsetDateTime::from(system_time);
    let format_desc = format_description!(
        "[year repr:last_two][month][day]_[hour repr:24][minute][second]"
    );

    let prefix = ts.format(format_desc)?;
    Ok(prefix)
}
