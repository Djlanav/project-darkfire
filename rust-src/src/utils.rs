pub fn remove_file_extension(path: &mut String) {
    if let Some(index) = path.rfind(".") {
        path.replace_range(index.., "");
    }
}