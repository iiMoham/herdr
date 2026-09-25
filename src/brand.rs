//! MoMo's user-facing identity. MoMo is a fork of herdr that installs next to
//! it: its own command, app directory, and name in the UI. Compatibility
//! identifiers (`HERDR_*` variables, hook sources, the API) keep herdr's names.

/// The command users type.
pub(crate) const CLI_NAME: &str = "momo";
/// The product name shown in prose.
pub(crate) const PRODUCT_NAME: &str = "MoMo";
/// The app directory name for release builds (`~/.config/momo`, state, sockets).
pub(crate) const APP_DIR: &str = "momo";
/// The app directory name for debug builds, so development never touches a
/// release install.
pub(crate) const DEV_APP_DIR: &str = "momo-dev";
/// The upstream app directory MoMo imports a config from on first start.
pub(crate) const UPSTREAM_APP_DIR: &str = "herdr";
/// MoMo's GitHub repository.
pub(crate) const REPOSITORY_URL: &str = "https://github.com/iiMoham/momo";
/// The update manifest attached to MoMo's latest GitHub Release. `momo update`
/// and remote SSH installs read it instead of herdr.dev.
pub(crate) const UPDATE_MANIFEST_URL: &str =
    "https://github.com/iiMoham/momo/releases/latest/download/latest.json";
/// Build channel MoMo release builds set through `HERDR_BUILD_CHANNEL`; the
/// release number goes in `HERDR_BUILD_ID`, so versions read `0.9.1-momo.3`.
pub(crate) const RELEASE_CHANNEL: &str = "momo";
/// MoMo has no preview builds. Upstream's preview code stays compiled behind
/// this flag so merges from herdr stay small.
pub(crate) const HAS_PREVIEW_CHANNEL: bool = false;
/// Why MoMo refuses the preview channel.
pub(crate) const NO_PREVIEW_CHANNEL: &str =
    "MoMo publishes stable releases only; the preview channel is not available (run `momo channel set stable`)";

/// This build's MoMo release number: `N` in `0.9.1-momo.N`, or 0 for local
/// and development builds.
pub(crate) fn release_number() -> u32 {
    release_number_for(crate::build_info::channel(), crate::build_info::build_id())
}

fn release_number_for(channel: &str, build_id: Option<&str>) -> u32 {
    if channel != RELEASE_CHANNEL {
        return 0;
    }
    build_id
        .and_then(|build_id| build_id.parse().ok())
        .unwrap_or(0)
}

/// Display label of a MoMo release: `0.9.1-momo.3`, or the base version for
/// release 0.
pub(crate) fn release_label(base_version: &str, release: u32) -> String {
    if release == 0 {
        base_version.to_owned()
    } else {
        format!("{base_version}-{RELEASE_CHANNEL}.{release}")
    }
}

/// Rewrite herdr's bundled agent skill for MoMo: the command, the product
/// name, and the skill name. `HERDR_*` variables stay, because MoMo panes
/// export the same variables.
pub(crate) fn rebrand_skill(text: &str) -> String {
    use std::sync::OnceLock;
    static RULES: OnceLock<Vec<(regex::Regex, &'static str)>> = OnceLock::new();
    let rules = RULES.get_or_init(|| {
        // Fixed patterns; the tests below prove every one compiles and applies.
        [
            (r"(?m)^name: herdr$", "name: momo"),
            (r"(^|[^\w:/.$-])herdr(\s+[a-z<\[-])", "${1}momo${2}"),
            (r"`herdr`", "`momo`"),
            (r"(^|[^\w])Herdr([^\w]|$)", "${1}MoMo${2}"),
        ]
        .into_iter()
        .filter_map(|(pattern, replacement)| {
            regex::Regex::new(pattern)
                .ok()
                .map(|pattern| (pattern, replacement))
        })
        .collect()
    });
    let mut text = text.to_owned();
    for (pattern, replacement) in rules {
        // Repeat so adjacent matches that share a boundary character are caught.
        loop {
            let next = pattern.replace_all(&text, *replacement).into_owned();
            if next == text {
                break;
            }
            text = next;
        }
    }
    text
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn release_numbers_come_only_from_momo_release_builds() {
        assert_eq!(release_number_for("momo", Some("3")), 3);
        assert_eq!(release_number_for("momo", Some("x")), 0);
        assert_eq!(release_number_for("momo", None), 0);
        assert_eq!(release_number_for("stable", Some("3")), 0);
        assert_eq!(release_number_for("preview", Some("3")), 0);
        assert_eq!(release_label("0.9.1", 3), "0.9.1-momo.3");
        assert_eq!(release_label("0.9.1", 0), "0.9.1");
    }

    #[test]
    fn skill_rebrand_renames_commands_and_product_but_keeps_env_vars() {
        let skill =
            "---\nname: herdr\ndescription: \"Control Herdr. Requires HERDR_ENV=1.\"\n---\n\
            # Herdr\n\nUse the `herdr` CLI inside a Herdr-managed pane:\n\n```bash\n\
            herdr pane list\nherdr agent read \"$HERDR_PANE_ID\" --lines 20\n```\n\
            Hook sources look like herdr:claude and files like herdr-agent-state.sh.\n";
        let rebranded = rebrand_skill(skill);
        assert!(
            !rebranded.contains("name: herdr"),
            "all four rules compiled"
        );
        assert_eq!(
            rebranded,
            "---\nname: momo\ndescription: \"Control MoMo. Requires HERDR_ENV=1.\"\n---\n\
            # MoMo\n\nUse the `momo` CLI inside a MoMo-managed pane:\n\n```bash\n\
            momo pane list\nmomo agent read \"$HERDR_PANE_ID\" --lines 20\n```\n\
            Hook sources look like herdr:claude and files like herdr-agent-state.sh.\n"
        );
    }

    #[test]
    fn bundled_skill_mentions_no_herdr_commands_after_rebrand() {
        let rebranded = rebrand_skill(include_str!("../skills/herdr/SKILL.md"));
        let leftover = regex::Regex::new(r"(^|[^\w:/.$-])herdr\s+[a-z]").unwrap();
        assert!(
            !leftover.is_match(&rebranded),
            "unrebranded command: {:?}",
            leftover.find(&rebranded).map(|found| found.as_str())
        );
        assert!(!rebranded.contains("Herdr "));
    }
}
