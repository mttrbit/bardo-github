use std::env;

pub fn profile() -> Option<String> {
    env::var_os("BARDO_DEFAULT_PROFILE")
        .map(|s| std::ffi::OsString::into_string(s).unwrap())
        .or_else(|| Some("default".to_string()))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_profile() {
        let p = profile();
        assert_eq!(p.expect("default profile is set"), "default");
        env::set_var("BARDO_DEFAULT_PROFILE", "foobar");
        let p = profile();
        assert_eq!(p.expect("default profile is set"), "foobar");
        env::remove_var("BARDO_DEFAULT_PROFILE");
    }
}
