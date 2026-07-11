cd C:\Users\varsh\OneDrive\Documents\3Atlas\qa\cli
$bin = "..\..\target\release/atlas.exe"
# bundle WITH decision trees
& $bin compile atlas.md ..\..\packages\decisions\cloudflare_workers.yaml --out cftree.atlas --json 2>$null
Write-Host "compile cftree EC=$LASTEXITCODE"
# decide with a query that should match the cloudflare decision tree
& $bin decide "I need to store config flags at the edge" --bundle cftree.atlas --json 2>$null | Out-File -Encoding utf8 dec_out.txt
Write-Host "=== decide matching output (first 600 chars) ==="
(Get-Content dec_out.txt -Raw).Substring(0,[Math]::Min(600,(Get-Content dec_out.txt -Raw).Length))
# verify total_matches vs nodes inconsistency for solve
& $bin solve "binary format" --bundle baseline.atlas --json 2>$null | Out-File -Encoding utf8 sol_out.txt
Write-Host "=== solve output key fields ==="
python -c "import json; d=json.loads(open('sol_out.txt',encoding='utf-16').read()); print('total_matches=',d.get('total_matches'),'len(nodes)=',len(d.get('nodes',[])))"
# verify with valid policy
& $bin verify --bundle baseline.atlas --policy eu-ai-act --json 2>$null | Out-File -Encoding utf8 ver_out.txt
Write-Host "=== verify eu-ai-act passed field ==="
python -c "import json; d=json.loads(open('ver_out.txt',encoding='utf-16').read()); print('passed=',d.get('passed'))"
# panic backtrace
$env:RUST_BACKTRACE="1"
& $bin solve "x" --bundle trunc.atlas --json 2>panic_bt.txt 1>$null
$env:RUST_BACKTRACE=""
Write-Host "=== PANIC BACKTRACE ==="
Get-Content panic_bt.txt | Select-Object -First 12
# encoding proof
Write-Host "=== encoding proof (first bytes of G03 solve) ==="
python -c "b=open('G03_solve_ok.so','rb').read(4); print('hex=',b.hex(),'is_utf16le_bom=', b[:2]==b'\xff\xfe')"
# traversal artifacts
Write-Host "=== traversal: files written OUTSIDE cwd (../evilpkg) ==="
Get-ChildItem ..\evilpkg -Recurse | Select-Object FullName | ForEach-Object { $_.FullName }
