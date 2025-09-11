use std::{
    fs::{self, File},
    io::Write,
};

/// Initializes a new C/C++ project in the current directory.
pub fn cv_init() -> Result<(), Box<dyn std::error::Error>> {
    let cwd = std::env::current_dir()?;
    // Create src directory
    let src_dir = cwd.join("src");
    if !src_dir.exists() {
        fs::create_dir_all(&src_dir)?;
    }
    // .gitignore
    let gitignore_path = cwd.join(".gitignore");
    if !gitignore_path.exists() {
        let mut f = File::create(&gitignore_path)?;
        writeln!(
            f,
            "/target\n*.o\n*.obj\n*.exe\n*.out\n*.a\n*.so\n*.dll\n*.dylib\n.DS_Store\n"
        )?;
    }
    // README.md
    let readme_path = cwd.join("README.md");
    if !readme_path.exists() {
        let mut f = File::create(&readme_path)?;
        writeln!(
            f,
            "# {}\n\nProject initialized by cv.\n",
            cwd.file_name().unwrap().to_string_lossy()
        )?;
    }
    // src/main.c
    let main_c_path = src_dir.join("main.c");
    if !main_c_path.exists() {
        let mut f = File::create(&main_c_path)?;
        writeln!(
            f,
            "#include <stdio.h>\n\nint main() {{\n    printf(\"Hello, world!\\n\");\n    return 0;\n}}\n"
        )?;
    }
    // cvproject.toml
    let cvproject_path = cwd.join("cvproject.toml");
    if !cvproject_path.exists() {
        let mut f = File::create(&cvproject_path)?;
        writeln!(
            f,
            "[project]\nname = \"{}\"\nversion = \"0.1.0\"\n\n[dependencies]\n",
            cwd.file_name().unwrap().to_string_lossy()
        )?;
    }
    println!("Initialized new C/C++ project in {}", cwd.display());
    Ok(())
}
