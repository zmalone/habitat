#[cfg(test)]
mod support;

#[cfg(test)]
pub mod redis_tests {
    use support::setup;

    #[test]
    fn upload_a_package_and_then_install_it() {
        setup::origin_setup();
    }
}
