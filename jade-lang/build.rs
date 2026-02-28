// Embed Jade icon into jade.exe on Windows (like Python embeds its icon).
fn main() {
    if cfg!(target_os = "windows") {
        let ico_path = "installers/windows/icon/jade.ico";
        if std::path::Path::new(ico_path).exists() {
            let mut res = winres::WindowsResource::new();
            res.set_icon(ico_path);
            res.set("ProductName", "Jade Programming Language");
            res.set("FileDescription", "Jade 1.0.0 - Interpreter / JIT / AOT Compiler");
            res.set("LegalCopyright", "Copyright (C) Jade Language Team");
            if let Err(e) = res.compile() {
                eprintln!("cargo:warning=winres: {} (icon not embedded)", e);
            }
        } else {
            eprintln!("cargo:warning=Icon not found at {}, exe will use default icon", ico_path);
        }
    }
}
