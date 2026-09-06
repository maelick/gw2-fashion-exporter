use std::{io, path::PathBuf};

use serde::Serialize;

#[derive(Debug, Copy, Clone, PartialEq)]
pub enum Format {
    /// CSV
    Csv,
    /// JSON
    Json,
}

#[derive(Debug, Clone, Serialize)]
pub enum OneOrMany<'a, T>
where
    T: Serialize,
{
    One(&'a T),
    Many(&'a [T]),
}

impl<'a, T: Serialize> OneOrMany<'a, T> {
    pub fn print(&self, format: Format, pretty: bool) -> anyhow::Result<()> {
        self.output(format, io::stdout(), pretty)
    }

    pub fn output<W: io::Write>(
        &self,
        format: Format,
        dest: W,
        pretty: bool,
    ) -> anyhow::Result<()> {
        match format {
            Format::Csv => output_csv(self, dest),
            Format::Json => output_json(self, dest, pretty),
        }
    }
}

pub fn detect_format(dest: Option<&PathBuf>) -> Format {
    match dest {
        Some(path) => match path.extension() {
            Some(ext) if ext == "json" => Format::Json,
            _ => Format::Csv,
        },
        None => Format::Csv,
    }
}

fn output_json<T: Serialize, W: io::Write>(data: &T, dest: W, pretty: bool) -> anyhow::Result<()> {
    if pretty {
        serde_json::to_writer_pretty(dest, data)?;
    } else {
        serde_json::to_writer(dest, data)?;
    }
    Ok(())
}

fn output_csv<T: Serialize, W: io::Write>(data: &T, dest: W) -> anyhow::Result<()> {
    let mut writer = csv::Writer::from_writer(dest);
    writer.serialize(data)?;
    Ok(())
}
