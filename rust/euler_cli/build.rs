use std::env;
use std::fs;
use std::io::Write;
use std::path::Path;

fn main() -> std::io::Result<()> {
    let out_dir = env::var("OUT_DIR").expect("OUT_DIR environment variable not set");
    let dest_path = Path::new(&out_dir).join("problems.rs");
    let mut f = fs::File::create(&dest_path)?;

    let problems_dir = Path::new("src/problems");

    let mut problems = fs::read_dir(problems_dir)?
        .filter_map(|entry| {
            let entry = entry.ok()?;
            let path = entry.path();
            if !path.is_file() {
                return None;
            }

            let file_name = path.file_stem()?.to_str()?;
            if !file_name.starts_with('p') || file_name == "mod" {
                return None;
            }
            
            let num = file_name[1..].parse::<u32>().ok()?;
            Some((num, file_name.to_string()))
        })
        .collect::<Vec<(u32, String)>>();

    problems.sort();

    let manifest_dir = env::var("CARGO_MANIFEST_DIR").unwrap();

    for (_, name) in &problems {
        let path = Path::new(&manifest_dir).join("src").join("problems").join(format!("{}.rs", name));
        writeln!(f, "#[path = r\"{}\"]", path.display())?;
        writeln!(f, "pub mod {};", name)?;
    }
    
    writeln!(f, r#"
pub fn solve_problem(problem: u32) -> anyhow::Result<Option<String>> {{
    match problem {{"#)?;

    for (num, name) in &problems {
        writeln!(f, "        {} => Ok(Some({}::solve()?)),", num, name)?;
    }

    writeln!(f, r#"        _ => Ok(None),
    }}
}}
"#)?;

    Ok(())
}
