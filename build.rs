fn main() {
    println!("cargo:rustc-link-lib=formulas");
    println!("cargo:rustc-link-search=native=.");
}
