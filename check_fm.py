import glob
for f in sorted(glob.glob("packages/*.md")):
    c = open(f, encoding="utf-8").read()
    parts = c.split("---", 2)
    has_close = c[3:].find("---") >= 0
    closing = "YES" if has_close else "MISSING"
    print(f"{f:45s} sections={len(parts)}  closing={closing}")
