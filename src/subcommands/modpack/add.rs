use super::check_output_directory;
use crate::{file_picker::pick_folder, TICK};
use anyhow::{Context as _, Result};
use colored::Colorize as _;
use inquire::Confirm;
use libium::{
    config::structs::{Config, Modpack, ModpackIdentifier},
    get_minecraft_dir,
    iter_ext::IterExt as _,
    modpack::add,
};
use std::path::PathBuf;

pub async fn curseforge(
    config: &mut Config,
    project_id: i32,
    output_dir: Option<PathBuf>,
    install_overrides: Option<bool>,
) -> Result<()> {
    eprint!("Checking modpack... ");
    let project = add::curseforge(config, project_id).await?;
    println!("{} ({})", *TICK, project.name);
    println!("Where should the modpack be installed to?");
    let output_dir = match output_dir {
        Some(some) => some,
        None => pick_folder(
            get_minecraft_dir(),
            "Pick an output directory",
            "Output Directory",
        )?
        .context("Please pick an output directory")?,
    };
    check_output_directory(&output_dir)?;
    let install_overrides = install_overrides.map_or_else(
        || {
            Confirm::new("Should overrides be installed?")
                .with_default(true)
                .prompt()
                .unwrap_or_default()
        },
        |some| some,
    );
    if install_overrides {
        println!(
            "{}",
            "WARNING: Files in your output directory may be overwritten by modpack overrides"
                .yellow()
                .bold()
        );
    }
    config.modpacks.push(Modpack {
        name: project.name,
        identifier: ModpackIdentifier::CurseForgeModpack(project.id),
        output_dir,
        install_overrides,
    });
    // Make added modpack active
    config.active_modpack = config.modpacks.len() - 1;
    Ok(())
}

pub async fn modrinth(
    config: &mut Config,
    project_id: &str,
    output_dir: Option<PathBuf>,
    install_overrides: Option<bool>,
) -> Result<()> {
    eprint!("Checking modpack... ");
    let project = add::modrinth(config, project_id).await?;
    println!("{} ({})", *TICK, project.title);
    println!("Where should the modpack be installed to?");
    let output_dir = match output_dir {
        Some(some) => some,
        None => pick_folder(
            get_minecraft_dir(),
            "Pick an output directory",
            "Output Directory",
        )?
        .context("Please pick an output directory")?,
    };
    check_output_directory(&output_dir)?;
    let install_overrides = install_overrides.map_or_else(
        || {
            Confirm::new("Should overrides be installed?")
                .with_default(true)
                .prompt()
                .unwrap_or_default()
        },
        |some| some,
    );
    if install_overrides {
        println!(
            "{}",
            "WARNING: Configs in your output directory may be overwritten by modpack overrides"
                .yellow()
                .bold()
        );
    }
    if !project.donation_urls.is_empty() {
        println!(
            "Consider supporting the mod creator on {}",
            project
                .donation_urls
                .iter()
                .map(|this| format!(
                    "{} ({})",
                    this.platform.bold(),
                    this.url.to_string().blue().underline()
                ))
                .display(", ")
        );
    }
    config.modpacks.push(Modpack {
        name: project.title,
        identifier: ModpackIdentifier::ModrinthModpack(project.id),
        output_dir,
        install_overrides,
    });
    // Make added modpack active
    config.active_modpack = config.modpacks.len() - 1;
    Ok(())
}
