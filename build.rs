use std::path::Path;

fn main() 
{
    // give linker correct path
    println!("cargo:rustc-link-search=.\\lib");

    let profile = std::env::var("PROFILE").unwrap();

    #[allow(unused_assignments)]
    let mut target_dir = "";

    if profile == "release"
    {
        target_dir = ".\\target\\release";
    }
    else
    {
        target_dir = ".\\target\\debug";
    }

    if !Path::new(&(target_dir.to_owned() + "\\plugins")).exists()
        || !Path::new(&(target_dir.to_owned() + "\\libvlc.dll")).exists()
        || !Path::new(&(target_dir.to_owned() + "\\libvlccore.dll")).exists()
    {
        print!("Warning! Libvlc does not exist in ouput directory.");
        panic!("Aborting");
    }
}