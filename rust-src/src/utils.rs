pub fn remove_file_extension(path: &mut String) {
    if let Some(index) = path.rfind(".") {
        path.replace_range(index.., "");
    }
}

#[macro_export]
macro_rules! callable_method {
    ($object:expr, $method_name:expr) => {
        Callable::from_object_method($object, $method_name)
    };
}