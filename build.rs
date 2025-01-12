fn main() {
    slint_build::compile("ui/file_form.slint").unwrap();
    slint_build::compile("ui/main.slint").unwrap();
}