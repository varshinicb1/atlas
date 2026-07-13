"""
Registry-backed Atlas Agent.

An ``Agent`` is a self-contained knowledge consumer: it discovers relevant
packages from the live Atlas registry, compiles them into a local bundle, and
answers questions deterministically through the ``atlas`` CLI — no LLM needed
for retrieval.

Example
-------
    import atlas_sdk as atlas
    agent = atlas.Agent("frontend")
    result = agent.solve("How do I validate a TypeScript form safely?")
    for node in result.nodes:
        print(node.name, node.confidence)
"""

from __future__ import annotations

import json
import shutil
import tempfile
import urllib.parse
import urllib.request
from dataclasses import dataclass, field
from pathlib import Path
from typing import Optional

from .client import compile, decide as _decide, solve as _solve, verify as _verify
from .models import (
    DecideResult,
    Node,
    SolveResult,
    VerificationReport,
)

DEFAULT_REGISTRY = "https://atlas-hub-registry.cbvarshini1.workers.dev"


class RegistryClient:
    """Thin HTTP client for the Atlas registry read API."""

    def __init__(self, registry: str = DEFAULT_REGISTRY, api_key: Optional[str] = None):
        self.registry = registry.rstrip("/")
        self.api_key = api_key

    def _get(self, path: str) -> dict:
        url = f"{self.registry}{path}"
        req = urllib.request.Request(
            url,
            headers={
                "Accept": "application/json",
                # Cloudflare bot mitigation rejects urllib's default UA with 403.
                "User-Agent": "AtlasAgent/0.1.0 (+https://atlas-sh.pages.dev)",
            },
        )
        if self.api_key:
            req.add_header("X-API-Key", self.api_key)
        with urllib.request.urlopen(req, timeout=20) as resp:
            return json.loads(resp.read().decode("utf-8"))

    def search(self, query: str, limit: int = 10) -> list[dict]:
        q = urllib.parse.quote(query)
        data = self._get(f"/api/search?q={q}&limit={limit}")
        return data.get("results", [])

    def get_package(self, name: str) -> Optional[dict]:
        try:
            return self._get(f"/api/v1/packages/{urllib.parse.quote(name)}")
        except urllib.error.HTTPError as e:
            if e.code == 404:
                return None
            raise


@dataclass
class Agent:
    """
    A registry-backed knowledge agent.

    The agent maintains a local workspace where discovered packages are fetched
    and compiled into a single ``.atlas`` bundle, which is then queried via the
    deterministic ``solve`` / ``decide`` / ``verify`` engines.
    """

    name: str = "atlas-agent"
    registry: str = DEFAULT_REGISTRY
    api_key: Optional[str] = None
    discover_limit: int = 5
    # Optional fixed set of package names; when set, discover() is skipped.
    packages: list[str] = field(default_factory=list)
    # Optional local source files (paths); when set, the bundle is built from
    # these directly instead of being fetched from the registry. Useful for
    # decision-tree YAMLs and offline agents.
    sources: list[str] = field(default_factory=list)
    workspace: Optional[str] = None

    def __post_init__(self):
        self._client = RegistryClient(self.registry, self.api_key)
        if self.workspace:
            self._ws = Path(self.workspace)
            self._ws.mkdir(parents=True, exist_ok=True)
            self._own_ws = False
        else:
            self._ws = Path(tempfile.mkdtemp(prefix=f"atlas-agent-{self.name}-"))
            self._own_ws = True
        self._bundle: Optional[Path] = None

    # -- discovery ---------------------------------------------------------

    def discover(self, query: str) -> list[str]:
        """Return package names relevant to ``query`` from the registry."""
        if self.packages:
            return list(self.packages)
        results = self._client.search(query, limit=self.discover_limit)
        return [r["name"] for r in results]

    # -- bundle management -------------------------------------------------

    def _fetch_sources(self, names: list[str]) -> list[Path]:
        sources: list[Path] = []
        for n in names:
            pkg = self._client.get_package(n)
            if not pkg:
                continue
            files = pkg.get("files", {})
            if not files:
                continue
            for fname, content in files.items():
                if not isinstance(content, str):
                    continue
                dest = self._ws / fname
                dest.write_text(content, encoding="utf-8")
                sources.append(dest)
        return sources

    def build_bundle(self, query: str) -> Path:
        """Discover, fetch, and compile a bundle for ``query``."""
        if self.sources:
            local = [Path(s) for s in self.sources if Path(s).exists()]
            if not local:
                raise RuntimeError(f"Agent '{self.name}': none of sources={self.sources} exist")
            out = self._ws / "bundle.atlas"
            compile(local, out=out)
            self._bundle = out
            return out
        names = self.discover(query)
        if not names:
            raise RuntimeError(f"Agent '{self.name}': no packages found for query '{query}'")
        sources = self._fetch_sources(names)
        if not sources:
            raise RuntimeError(f"Agent '{self.name}': no source files fetched for {names}")
        out = self._ws / "bundle.atlas"
        compile(sources, out=out)
        self._bundle = out
        return out

    @property
    def bundle(self) -> Path:
        if self._bundle is None:
            raise RuntimeError("Bundle not built. Call build_bundle() or solve()/decide() first.")
        return self._bundle

    # -- reasoning ---------------------------------------------------------

    def solve(self, query: str, discover_limit: Optional[int] = None) -> SolveResult:
        """Search the knowledge graph for nodes matching ``query``."""
        if discover_limit is not None:
            self.discover_limit = discover_limit
        if self._bundle is None:
            self.build_bundle(query)
        else:
            # Refresh bundle if the query yields a different package set.
            names = self.discover(query)
            if names:
                self._fetch_sources(names)
                out = self._ws / "bundle.atlas"
                compile([self._ws / f for f in self._ws.glob("*.md")], out=out)
                self._bundle = out
        return _solve(self.bundle, query)

    def decide(self, query: str, context: Optional[dict[str, str]] = None) -> Optional[DecideResult]:
        """Walk a decision tree in the bundle to a recommendation."""
        if self._bundle is None:
            self.build_bundle(query)
        return _decide(self.bundle, query, context)

    def verify(self, query: str) -> VerificationReport:
        """Verify the discovered bundle for ``query``."""
        if self._bundle is None:
            self.build_bundle(query)
        return _verify(self.bundle)

    def cleanup(self) -> None:
        if self._own_ws and self._ws.exists():
            shutil.rmtree(self._ws, ignore_errors=True)
            self._bundle = None
