//! `linuxmcpd`: Linux host for the NativeMCP governed server.
//!
//! At repository-setup stage this binary reports its identity and exits. The
//! systemd integration, transport wiring and tool surface are named
//! gaps tracked as issues, not stubs: nothing here pretends to do work it
//! does not do.
//!
//! The governance invariants in the `core` repository's `docs/GOVERNANCE.md`
//! are normative for everything added to this binary.

/// Semantic version, from the crate manifest.
const VERSION: &str = env!("CARGO_PKG_VERSION");

/// Component identity as it appears in audit records.
const COMPONENT: &str = "linuxmcpd";

/// Target platform this build serves.
const PLATFORM: &str = "Linux";

fn main() {
    println!("{COMPONENT} {VERSION} ({PLATFORM})");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn identity_constants_are_populated() {
        assert!(!VERSION.is_empty());
        assert_eq!(COMPONENT, "linuxmcpd");
        assert_eq!(PLATFORM, "Linux");
    }

    #[test]
    fn version_is_semver_shaped() {
        let parts: Vec<&str> = VERSION.split('.').collect();
        assert_eq!(parts.len(), 3, "expected MAJOR.MINOR.PATCH, got {VERSION}");
        assert!(
            parts
                .iter()
                .all(|p| p.chars().next().is_some_and(|c| c.is_ascii_digit()))
        );
    }
}
