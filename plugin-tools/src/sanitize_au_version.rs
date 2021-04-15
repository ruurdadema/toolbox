use anyhow::{bail, Context, Result};
use clap::ArgMatches;

fn convert_version_string_to_number(version_string: &str) -> Result<i64> {
    let parts: Vec<&str> = version_string.split('.').collect();

    let version_major = parts.get(0).unwrap_or(&"0").parse::<i64>()?;
    let version_minor = parts.get(1).unwrap_or(&"0").parse::<i64>()?;
    let version_patch = parts.get(2).unwrap_or(&"0").parse::<i64>()?;

    if version_major > 0xFFF {
        bail!("Version major too high! (max = {})", 0xFFF);
    }

    if version_minor > 0xFF {
        bail!("Version minor too high! (max = {})", 0xFF);
    }

    if version_patch > 0xFF {
        bail!("Version patch too high! (max = {})", 0xFF);
    }

    let mut version_number = version_major << 16;
    version_number += version_minor << 8;
    version_number += version_patch;

    Ok(version_number)
}

pub fn sanitize_au_version(matches: &ArgMatches) -> Result<()> {
    let input = matches.value_of("INPUT").unwrap();

    println!("{}", input);

    if !input.ends_with(".plist") {
        bail!("Expecting a plist file with extension .plist");
    }

    let path = std::path::Path::new(input);

    if !path.is_file() {
        bail!("Expecting a file");
    }

    if let Ok(mut plist) = plist::Value::from_file(path.clone()) {
        let plist_dict = plist.as_dictionary_mut().context("expected a dictionary")?;

        let version_string = plist_dict
            .get("CFBundleVersion")
            .context("unable to find CFBundleVersion")?
            .as_string()
            .context("CFBundleVersion not a string")?
            .to_string();

        println!("Found version: {}", version_string);

        let audio_components = plist_dict
            .get_mut("AudioComponents")
            .context("unable to find AudioComponents array")?
            .as_array_mut()
            .context("AudioComponents is not an array")?;

        for component in audio_components.iter_mut() {
            let component = component
                .as_dictionary_mut()
                .context("AudioComponent not a disctionary")?;

            let component_version_number = component
                .get("version")
                .context("version number not found")?
                .as_signed_integer()
                .context("failed to get version as integer")?;

            let new_version_number = convert_version_string_to_number(&version_string)?;
            component.insert(
                "version".to_string(),
                plist::Value::Integer(new_version_number.into()),
            );

            println!(
                "Original component version number: {}",
                component_version_number
            );
            println!("Updated component version number: {}", new_version_number);
        }

        plist.to_file_xml(path)?;
    }

    Ok(())
}
