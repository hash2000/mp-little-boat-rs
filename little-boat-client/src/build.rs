fn main() {
    #[cfg(feature = "slint-ui")]
    slint_build::compile("src/ui/Application.slint").expect("Slint build failed");
}
