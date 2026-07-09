# Package Manager

The Atlas Package Manager (Rust) resolves, fetches, caches, and verifies versioned
`.atlas` bundles. It follows the same patterns as `cargo`, `npm`, `uv`, and
`pnpm`.

## 1. Command surface

```bash
atlas install flutter@3.24          # exact version
atlas install "flutter@^3.24"       # semver range → resolves to latest compatible
atlas install flutter supabase      # multiple
atlas update                         # update all installed to latest compatible
atlas update flutter                 # update one
atlas uninstall flutter
atlas list                           # show installed
```

## 2. Registry

The Atlas Package Registry is a content-addressable object store (S3/R2/GCS)
fronted by an index:

```
atlas-registry.example.com/
  index.json                         # { "flutter": { "versions": ["3.24", ...] } }
  packages/flutter/3.24/manifest.json
  packages/flutter/3.24/bundle.atlas
  packages/flutter/3.24/bundle.atlas.sig
  packages/flutter/3.24/changelog.md
```

The index is signed with the registry's key. Each bundle is signed with the
publisher's key (or the registry key for official bundles).

## 3. Resolution

1. If `@<version>` is exact → fetch directly.
2. If semver range → resolve against `index.json`, pick latest semver-matching.
3. If no version → latest stable.
4. Resolution is deterministic per `registry` + `index_hash` (reproducible lock).

## 4. Caching

```
~/.atlas/
  bin/                          # atlas binaries (self-update)
  bundles/
    flutter/
      3.24/
        bundle.atlas
        manifest.json
        signature
  cache/
    <content-hash>              # deduplicated mmap'd pages
  index/
    cache.json                  # cached registry index
  lock.json                     # resolved dependency tree (like package-lock.json)
```

## 5. Verification on install

1. Download `bundle.atlas`.
2. Verify `blake3` checksum = manifest.
3. Verify ed25519 signature against:
   a. Publisher key (from registry index, HTTPS-fetched).
   b. Or local `~/.atlas/trusted-keys/` for private registries.
4. Extract provenance hashes; verify each referenced source is either a
   trusted registry entry or signed by a trusted key.
5. On failure → reject; print which check failed.

## 6. Private registries

- `~/.atlasrc` configures custom registries: `atlas config set registry https://internal.corp/atlas`.
- Private registry bundles are signed with the private registry's key.
- No telemetry; no mandatory outbound internet.

## 7. Publishing

```bash
atlas publish ./flutter@3.24.atlas     # requires registry credentials
atlas publish --tag latest --sign key.pub
```

Publishing is restricted to verified publishers. The registry rejects bundles
whose provenance checksums don't match known source artifacts (official docs,
GitHub releases, etc.).

## 8. Bundle naming

- Package names are lowercase, hyphen-separated, matching their canonical source
  (e.g., `flutter`, `supabase-flutter`, `riverpod`, `react`).
- Version is semver.
- The canonical filename is `{name}@{version}.atlas`.
