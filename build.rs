use std::path::Path;

fn main() 
{
    // give linker correct path
    println!("cargo:rustc-link-search=.\\lib");
}