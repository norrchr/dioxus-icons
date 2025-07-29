use crate::config::{IconConfig, IconResource};
use crate::utils::{icon_resource_path, icon_out_dir};
use crate::parser::process_svg_file;
use std::{collections::HashMap, io::Write, path::PathBuf, process::exit};
use walkdir::WalkDir;

#[derive(clap::Parser)]
pub(crate) struct BuildArgs {
    #[clap(short, long, value_delimiter = ',', num_args = 1..)]
    name: Option<Vec<String>>,
}

pub (crate) fn build_command(config: IconConfig, args: BuildArgs) {
    let icon_resource_path = icon_resource_path(config.resource_path.clone());

    let args_name = args.name.unwrap_or_default();
    let mut resources: Vec<IconResource> = Vec::new();

    if args_name.is_empty() {
        resources = config.resources.clone();
    } else {
        for name in args_name {
            let resource = config.resources.iter().find(|r| r.name == name);
            if let Some(resource) = resource {
                let dst = icon_resource_path.join(PathBuf::from(resource.name.clone()));
                if !dst.exists() {
                    println!("Icon resource for '{}' not found at '{}'", resource.name, dst.display());
                    exit(1)
                }

                resources.push(resource.clone())
            } else {
                println!("Resource '{}' not found", name);
                exit(1)
            }
        }
    }

    if resources.is_empty() {
        println!("No resources found, have you configured the resources in the config file and run the update command?");
        exit(1);
    }

    let icon_out_dir = icon_out_dir();

    // create icon out directory for resource
    std::fs::create_dir_all(&icon_out_dir).expect("Failed to create icon_resource out directory");

    // create the mod.rs file for the icon_out directory -- icons/mod.rs
    // this is passed down to the handlers to register their own modules with any required feature gating etc
    let mut mod_file = std::fs::File::create(icon_out_dir.join(PathBuf::from("mod.rs"))).expect("Failed to create icon_resource mod file");

    println!("Building resources for: {}", resources.iter().map(|r| r.name.as_str()).collect::<Vec<_>>().join(", "));

    for resource in resources {
        println!("Generating icon components for resource '{}'", resource.name);

        if resource.name == "heroicons" {
            process_heroicons(resource, config.clone(), &mut mod_file);
        } else if resource.name == "materialicons" {
            process_material_icons(resource, config.clone(), &mut mod_file);
        } else {
            println!("Resource '{}' has no handler -- skipping", resource.name);
        }
    }

    // flush the mod file
    mod_file.flush().expect("Failed to flus icon_resource mod file");
}

fn starts_with_digit(s: &str) -> bool {
    s.chars().next().map(|c| c.is_ascii_digit()).unwrap_or(false)
}

/// Process icon files for heroicons
/// Expected structure is: dioxus_icons::heroicons::__name__ {}
fn process_heroicons(resource: IconResource, config: IconConfig, icon_out_mod_file: &mut std::fs::File) {
    let icon_resource_path = icon_resource_path(config.resource_path);
    let icon_out_dir = icon_out_dir();
    let src = icon_resource_path.join(PathBuf::from(resource.name.clone()));

    let icon_out_path = icon_out_dir.join(PathBuf::from(resource.name.clone()));
    std::fs::create_dir_all(&icon_out_path).expect("Failed to create icon out directory for resource");

    let mut fd = std::fs::File::create(&icon_out_path.join(PathBuf::from("mod.rs"))).expect("Failed to create icon file for resource");
    fd.write(b"use dioxus::prelude::*;\nuse crate::IconProps;\n").expect("Failed to write to icon file for resource");

    for entry in WalkDir::new(src)
        .into_iter()
        .filter_map(Result::ok)
        .filter(|e| e.file_type().is_file())
        .filter(|e| e.file_name().to_string_lossy().ends_with(".svg"))
    {
        let path = entry.path();
        let file_name = path.file_name().unwrap().to_string_lossy();
        let file_name = file_name.strip_suffix(".svg").unwrap();
        let ancestors = path.ancestors().collect::<Vec<_>>();

        let typ = ancestors[1].file_name().unwrap();
        let sz = ancestors[2].file_name().unwrap();
        let icon_name = heck::AsSnakeCase(format!("{}_{}_{}", file_name.replace("-", "_"), typ.display(), sz.display()));

        let svg = process_svg_file(path.to_path_buf(), icon_name.to_string()).expect("Failed to parse svg file");
        fd.write_all(svg.as_bytes()).expect("Failed to write to icon file for resource");
        fd.write("\n\n".as_bytes()).expect("Failed to write to icon file for resource");
    }

    fd.flush().expect("Failed to flush icon file for resource");

    // register the module with the top level icon mod file
    icon_out_mod_file.write(format!("#[cfg(feature = \"heroicons\")]\npub mod {};\n", resource.name).as_bytes()).expect("Failed to write features to mod file");

}

/// Process icon files for material icons
/// These will be modularized into a module per icon category
/// Expected structure is: dioxus_icons::materialicons::__category__::__name__size__?type?__ {}
fn process_material_icons(resource: IconResource, config: IconConfig, mod_file_clone: &mut std::fs::File) {
    let icon_resource_path = icon_resource_path(config.resource_path);
    let icon_out_dir = icon_out_dir();
    let src = icon_resource_path.join(PathBuf::from(resource.name.clone()));

    let mut groups: HashMap<String, HashMap<String, Vec<(String, PathBuf)>>> = HashMap::new();

    // Group app the files by category and type
    for entry in WalkDir::new(src)
        .into_iter()
        .filter_map(Result::ok)
        .filter(|e| e.file_type().is_file())
        .filter(|e| e.file_name().to_string_lossy().ends_with(".svg"))
    {
        let path = entry.path();
        let file_name = match path.file_name() {
            Some(name) => name.to_string_lossy().into_owned().strip_suffix("px.svg").unwrap().to_string(),
            None => {
                println!("Failed to get file name for path '{}'", path.display());
                exit(1);
            }
        };
        let mut typ = path.parent().unwrap().file_name().unwrap().to_string_lossy().to_string();
        let name = path.parent().unwrap().parent().unwrap().file_name().unwrap().to_string_lossy().to_string();
        let category = path.parent().unwrap().parent().unwrap().parent().unwrap().file_name().unwrap().to_string_lossy().to_string();

        typ = if typ == "materialicons" {
            "icons".to_string()
        } else {
            typ.strip_prefix("materialicons").unwrap().to_string()
        };

        let icon_name = heck::AsSnakeCase(format!("{}_{}_{}px", name.replace("-", "_"), if typ == "icons" { "" } else { "icons" }, file_name)).to_string();

        groups.entry(category).or_insert_with(HashMap::new).entry(typ).or_insert_with(Vec::new).push((icon_name, path.to_path_buf()));
    }

    // Process each group, read svg files and generate components, and output to modularized files
    // icons/materialicons/__category__.rs
    // icons/materialicons/mod.rs

    std::fs::create_dir_all(icon_out_dir.join(PathBuf::from(resource.name.clone()))).expect("Failed to create icon resource out directory");
    let mut content = String::new();

    // icons/materialicons/mod.rs
    let mut resource_mod_file = std::fs::File::create(icon_out_dir.join(PathBuf::from(resource.name.clone())).join(PathBuf::from("mod.rs"))).expect("Failed to create icon type mod file");
    let mut resource_features = std::collections::HashSet::new();

    for (category, typ) in groups {

        //println!("Generating icon components for resource '{}' in category '{}'", resource.name, category);

        // create dir: icons/materialicons/action
        let category_dir = icon_out_dir.join(PathBuf::from(resource.name.clone())).join(PathBuf::from(category.clone()));
        std::fs::create_dir_all(&category_dir).expect("Failed to create icon category directory");

        // create mod file at: icons/materialicons/action/mod.rs
        let mut category_mod_file = std::fs::File::create(category_dir.join(PathBuf::from("mod.rs"))).expect("Failed to create icon mod file");

        for (typ, list) in typ {

            let feature_name = if typ == "icons" {
                "material-icons".to_string()
            } else {
                format!("material-icons-{}", typ.replace("_", "-"))
            };

            resource_features.insert(feature_name.clone());

            println!("Generating icon components for resource '{}' in category '{}' and type '{}'", resource.name, category, typ);
            let mut fd = std::fs::File::create(category_dir.join(PathBuf::from(format!("{}.rs", typ)))).expect("Failed to create icon type file");
            fd.write(format!("use dioxus::prelude::*;\nuse crate::IconProps;\n").as_bytes()).expect("Failed to write imports");

            for (mut icon_name, icon_path) in list {

                if starts_with_digit(&icon_name) {
                    icon_name = format!("_{}", icon_name);
                }

                let svg = process_svg_file(icon_path, icon_name.to_string()).expect("Failed to parse svg file");
                fd.write_all(svg.as_bytes()).expect("Failed to write to file");
                fd.write("\n\n".as_bytes()).expect("Failed to write to file");

                content.clear();
            }

            fd.flush().expect("Failed to flush file");

            // icons/materialicons/mod.rs -> mod typ;
            category_mod_file.write(format!("#[cfg(feature = \"{}\")]\npub mod {};\n", feature_name, typ).as_bytes()).expect("Failed to write to mod file");
        }

        resource_mod_file.write(format!("pub mod {};\n", category).as_bytes()).expect("Failed to write to mod file");

        // flush the category mod file
        category_mod_file.flush().expect("Failed to flush mod file");
    
    }

    // flush the resource mod file
    resource_mod_file.flush().expect("Failed to flush mod file");

    // #[cfg(any(feature = "material-icons", feature = "material-icons-outlined", feature = "material-icons-round", feature = "material-icons-sharp", feature = "material-icons-twotone"))]
    mod_file_clone.write(format!("#[cfg(any({}))]\npub mod {};\n", resource_features.iter().map(|f| format!("feature = \"{}\"", f)).collect::<Vec<_>>().join(", "), resource.name).as_bytes()).expect("Failed to write features to mod file");
}