//The following will match any path after the base
pub const REGEX_STARTSWITH_SUFFIX: &str = r#"\b[[/]{1}[-a-zA-Z0-9@:%_+.~#?&/=]{1,}[/]{0,1}]{0,}\b$"#;
//Public url path prefix
pub const REGEX_PUBLIC_PREFIX: &str = r#"^/public{1}"#;