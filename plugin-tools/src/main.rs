use anyhow::{bail, Context, Result};
use clap::{App, Arg, ArgMatches};
use pkg_version::*;

mod sanitize_au_version;

fn main() -> Result<()> {
    let sanitize_au_version_cmd = "sanitize-au-version";

    let version_string = format!(
        "{}.{}.{}",
        pkg_version_major!(),
        pkg_version_minor!(),
        pkg_version_patch!()
    );

    let matches = App::new("Plugin Tools")
        .version(version_string.as_str())
        .author("Ruurd Adema <ruurd@owllab.nl>")
        .about("Helps with audio plugins")
        .subcommand(
            App::new(sanitize_au_version_cmd).arg(
                Arg::new("INPUT")
                    .about("Sets the input file to use")
                    .required(true)
                    .index(1),
            ),
        )
        .get_matches();

    if let Some(ref matches) = matches.subcommand_matches(sanitize_au_version_cmd) {
        return sanitize_au_version::sanitize_au_version(matches);
    }

    Ok(())
}
