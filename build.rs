fn main() {
    println!("cargo:rustc-link-search=native=/opt/local/lib");
    println!("cargo:rustc-link-lib=static=ncursesw");
    //println!("cargo:rustc-link-lib=static=ncurses");
    //println!("cargo:libdir=/opt/local/lib");
}
