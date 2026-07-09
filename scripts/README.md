# Scripts

| Script | Description |
|--------|-------------|
| `compile-all.py` | Compile all knowledge packages |
| `validate-all.py` | Run atlas verify on all packages |
| `ci.py` | Full CI pipeline (build -> test -> compile -> validate -> lint) |
| `quality-score.py` | Score packages on quality metrics |

### Quick start

```bash
# Full CI
python scripts/ci.py

# Just compile and validate packages
python scripts/compile-all.py
python scripts/validate-all.py

# Score packages
python scripts/quality-score.py --all --json
```
