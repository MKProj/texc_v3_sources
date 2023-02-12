use std::io::{Result, Write};
use std::fs::File;
use std::collections::HashMap;

macro_rules! map{
    ($($key: expr => $value: expr), *) => ({
            let mut map = std::collections::HashMap::new();
            
            $(
                let key = $key.to_string();
                let value = $value.to_string();
                map.insert(key, value);
            )*
            map
});
}

// A hashmap that contains the name and description
fn sources() -> HashMap<String, String>{
    map!(
        "texcgen" => "A Template Generator for TexCreate.", 
        "mkproj_texcgen" =>  "A Template Generator customized for the MKProject first party templates.", 
        "texcreate_repo" => "Provides the `Repo` type for TexCreate, provides a way to manage template releases.", 
        "texcore" => "Create LaTeX using native Rust types (provides TexCreate `Template` type)."
    )
}

// Creates the gh link
fn gh(name: &str) -> String{
    format!("https://github.com/MKProj/{name}")
}

// Writes the readme
fn readme(sources: HashMap<String, String>) -> Result<()>{
    let mut markdown = Vec::new();
    markdown.push("# TexCreate v3.0 Repositories".to_string());
    for (name, desc) in sources{
        markdown.push(format!("- [{}: {}]({})", &name, desc, gh(&name)))
    }
    let s: String = markdown.join("\n");
    let mut file = File::create("README.md")?;
    file.write_all(s.as_bytes())?;
    Ok(())
}

fn main() -> Result<()>{
    let sources = sources();
    readme(sources)?;
    Ok(())
}