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
        let profile = profile();
        assert_eq!(profile.expect("default profile is set"), "default");
    }

    #[test]
    fn test_profile_with_env() {
        env::set_var("BARDO_DEFAULT_PROFILE", "foobar");
        let profile = profile();
        assert_eq!(profile.expect("default profile is set"), "foobar");
        env::remove_var("BARDO_DEFAULT_PROFILE");
    }
}
