pub const REGEX_PREFIX: &str = r#"^"#;

//The following will match any path after the base
pub const REGEX_STARTSWITH_SUFFIX: &str =
    r#"{1}\b[[/]{1}[-a-zA-Z0-9@:%_+.~#?&/=]{1,}[/]{0,1}]{0,}\b$"#;

pub const REGEX_EXACT_SUFFIX: &str = r#"$"#;

//Public url path prefix
pub const REGEX_PUBLIC_PREFIX: &str = r#"^/public{1}"#;

pub const EXACT: &str = "EXACT";
pub const STARTSWITH: &str = "STARTSWITH";

pub const WILDCARD: &str = "*";

pub const CACHE_CONTROL_DEFAULT: &str = "max-age=0, no-store, must-revalidate";
