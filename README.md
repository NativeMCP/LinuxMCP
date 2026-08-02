# NativeMCP / LinuxMCP

Native governed MCP server for Linux. systemd service, journald audit sink, POSIX job execution.

[![CI](https://github.com/NativeMCP/LinuxMCP/actions/workflows/ci.yml/badge.svg)](https://github.com/NativeMCP/LinuxMCP/actions/workflows/ci.yml)
[![License](https://img.shields.io/badge/license-Apache--2.0-blue.svg)](LICENSE)

`LinuxMCP` is the Linux member of the NativeMCP server family. It owns only
what is genuinely Linux-specific. Protocol handling, policy evaluation,
contract schema, audit and dispatch come from
[`NativeMCP/core`](https://github.com/NativeMCP/core) and are not reimplemented here.

## Platform surface

| Concern | Binding |
|---|---|
| Service lifecycle | systemd, `Type=notify`, socket activation |
| Audit sink | journald, structured fields |
| Secret custody | kernel keyring, `systemd-creds` |
| Path authority | canonicalized root resolution, `O_NOFOLLOW` traversal |
| Containment | cgroup v2 limits, `rlimit`, seccomp filter, namespace isolation |
| Distribution | `.deb` and `.rpm`, XDG-conformant configuration paths |

## Governance

The invariants in [`NativeMCP/core` `docs/GOVERNANCE.md`](https://github.com/NativeMCP/core/blob/main/docs/GOVERNANCE.md)
are normative for this repository. INV-1 (No Destructive Primitive) and INV-3
(Immutable Audit) in particular constrain every Linux API this server is
permitted to call.

## Status

Repository setup stage. The workspace is wired to `core`, builds clean and is
CI-green on `ubuntu-latest`. The daemon, packaging and user
surface are named gaps tracked as issues, not stubs.

## Build

```bash
rustup show
cargo fmt --all -- --check
cargo clippy --workspace --all-targets --all-features -- -D warnings
cargo test --workspace --all-features
```

## License

Apache-2.0. See [LICENSE](LICENSE).
