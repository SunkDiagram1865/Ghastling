pub const PRODUCT_NAME: &str = "Axolotl Launcher";
pub const SHORT_PRODUCT_NAME: &str = "Axolotl";
pub const ORGANIZATION_NAME: &str = "Garbage Human Studio";
pub const ORGANIZATION_SHORT_NAME: &str = "GHS";
pub const DEVELOPER_NAME: &str = "Mystic Stars";
pub const WEBSITE: &str = "https://www.ghs.red";
pub const BUNDLE_IDENTIFIER: &str = "red.ghs.axolotl";
pub const DEEP_LINK_SCHEME: &str = "axolotl";

pub fn user_agent(version: &str, os: &str) -> String {
    format!("garbage-human-studio/axolotl/{version} ({os})")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn user_agent_is_unique_and_contains_no_contact_information() {
        let user_agent = user_agent("1.2.3", "windows");

        assert_eq!(user_agent, "garbage-human-studio/axolotl/1.2.3 (windows)");
        assert!(!user_agent.contains("ghs.red"));
        assert!(!user_agent.contains("http"));
        assert!(!user_agent.contains('@'));
    }
}
