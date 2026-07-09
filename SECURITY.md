# Security

Atlas is a supply-chain for engineering knowledge. Security is not optional.

## 1. Principles

- **No untrusted execution.** The `.atlas` binary is data, not code. No embedded
  scripts, no eval, no runtime code generation from the binary.
- **Provenance is mandatory.** Every node traces to a source hash. No source =
  `UNVERIFIED` status, excluded from authoritative answers.
- **Signing is mandatory for distribution.** Every published bundle carries an
  ed25519 signature verified by the Package Manager.

## 2. Supply chain

```
Publisher ─signs─▶ bundle.atlas ─verify─▶ Package Registry
                                                │
                                         User install
                                                │
                                          verify signature
                                          verify checksum
                                          verify provenance
```

| Step | Mechanism |
| --- | --- |
| Bundle signing | ed25519 over the `content_checksum` (blake3). |
| Registry authenticity | Registry index signed with registry key; fetched over HTTPS. |
| Publisher identity | Publisher key pinned in registry index or in `~/.atlas/trusted-keys/`. |
| Provenance check | Each source referenced in IR must have a hash matching a known source file or trusted registry entry. |
| On install | All three checks (sig, checksum, provenance) pass or the bundle is rejected. |

## 3. Verification rules (at runtime)

| Rule | What it prevents |
| --- | --- |
| API existence check | Prevents hallucinated package APIs. |
| Version match | Prevents suggesting APIs that don't exist in the user's pinned version. |
| Provenance anchor | Every fact tied to a hash of the original source. |
| Confidence threshold | Nodes below `min_confidence` are excluded unless user requests low-confidence. |
| Manifest hash | Prevents tampered bundles post-install. |

## 4. Private data

- Atlas processes user data only when the user compiles a private repo. The
  resulting `.atlas` is owned by the user; no telemetry.
- The CLI and Runtime make no outbound calls unless the user runs `atlas install`
  or `atlas update`.
- The Studio runs fully offline after initial load.

## 5. Signed binaries

The Atlas CLI binary itself is code-signed. Auto-updates verify the binary
signature before applying.

## 6. Vulnerability reporting

See `SECURITY.md` in the repository root for contact instructions (TBD).
