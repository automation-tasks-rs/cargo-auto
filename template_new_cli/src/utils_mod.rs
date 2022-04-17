// bestia_dev_cargo_auto_new_cli/src/utils_mod.rs

//! Just an example how to create and call a module in a separate file.
//!
//! This doc-comments will be compiled into the `docs`.

/// return uppercase
pub fn make_uppercase(my_name: &str) -> String {
    // return
    my_name.to_uppercase()
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    pub fn test_make_uppercase() {
        assert_eq!(make_uppercase("abcd"), "ABCD");
        assert_eq!(make_uppercase("1234abcd"), "1234ABCD");
        assert_eq!(make_uppercase("čšž"), "ČŠŽ");
    }
}
