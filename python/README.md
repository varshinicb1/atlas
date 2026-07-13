# Atlas Python SDK

Python SDK for the [Atlas Knowledge Operating System](https://atlas-sh.pages.dev) — the Terraform for AI agent knowledge.

```bash
pip install atlas-sdk
```

## Quickstart

```python
import atlas_sdk as atlas

# Solve: search a knowledge bundle
result = atlas.solve("flutter_core.atlas", "stateful widget")
for node in result.nodes:
    print(node.name, node.kind)

# Decide: walk a decision tree
decision = atlas.decide("tech_stack.atlas", "build web app", context={"type": "saas"})
if decision:
    print(decision.rationale)
    for rec in decision.recommendations:
        print(f"  -> {rec.node_id} (confidence: {rec.confidence})")

# Use the Agent class for registry-backed agents
from atlas_sdk import Agent
agent = Agent("my-agent", packages=["flutter_core", "riverpod"])
result = agent.solve("How to use AsyncNotifier?")
```

## Documentation

See the [full API reference](https://atlas-sh.pages.dev/docs/api.html) and [quickstart guide](https://atlas-sh.pages.dev/docs/quickstart.html).

## License

Apache 2.0

