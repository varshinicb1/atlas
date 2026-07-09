from dataclasses import dataclass, field
from typing import Optional


@dataclass
class Node:
    id: str
    name: str
    kind: str
    description: Optional[str] = None
    version: Optional[str] = None


@dataclass
class SolveResult:
    query: str
    bundle: str
    confidence: float
    total_matches: int
    nodes: list[Node] = field(default_factory=list)


@dataclass
class Recommendation:
    node_id: str
    confidence: float


@dataclass
class DecideResult:
    tree_id: str
    path: list[str]
    rationale: str
    recommendations: list[Recommendation] = field(default_factory=list)
    agent_instructions: Optional[str] = None


@dataclass
class VerificationCheck:
    name: str
    passed: bool
    message: str
    severity: str


@dataclass
class VerificationReport:
    passed: bool
    checks: list[VerificationCheck] = field(default_factory=list)
