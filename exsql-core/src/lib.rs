const CORE_LIBRARY_VERSION: &'static str = "0.1.0";

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sanity_check_advertise_core_library_version() {
        assert_eq!(CORE_LIBRARY_VERSION, "0.1.0");
    }
}
