cd C:\Users\varsh\OneDrive\Documents\3Atlas\qa\cli
$bin = "..\..\target\release/atlas.exe"
$summary = "summary2.tsv"
"" | Set-Content $summary
function JCheck($s){ $t=$s.Trim(); if($t.Length -eq 0){return "EMPTY"}; try{$t|ConvertFrom-Json>$null;return "JSON_OK"}catch{return "JSON_BAD"} }
function Run($id,$cmd){ $so="$id.so";$se="$id.se"; & $bin @cmd 1>$so 2>$se; $ec=$LASTEXITCODE; $stdout=(Get-Content $so -Raw -EA SilentlyContinue); if($stdout-eq$null){$stdout=""}; $stderr=(Get-Content $se -Raw -EA SilentlyContinue); if($stderr-eq$null){$stderr=""}; $jj=if($cmd -contains "--json"){JCheck $stdout}else{"n/a"}; $panic=$stderr -match "panicked at"; Add-Content $summary "$id`tEC=$ec`t$jj`tPANIC=$panic`t$($cmd -join ' ')"; return @{ec=$ec;jj=$jj;panic=$panic;se=$stderr.Trim()} }

# Category 3: compile variations
Run "F01_multi"      @("compile","atlas.md","riverpod.md","--out","multi.atlas","--json")
Run "F02_mixed"      @("compile","atlas.md","packages\decisions\cloudflare_workers.yaml","--out","mixed.atlas","--json")
Run "F03_dupsrc"     @("compile","atlas.md","atlas.md","--out","dup.atlas","--json")
Run "F04_out_nodir"  @("compile","atlas.md","--out","nonexist_sub\out.atlas","--json")
Run "F05_out_isdir"  @("compile","atlas.md","--out","decisions","--json")
Run "F06_relpath"    @("compile","..\packages\atlas.md","--out","rel.atlas","--json")
Run "F07_abspath"    @("compile",(Resolve-Path atlas.md),"--out",(Join-Path $PWD abs.atlas),"--json")
Run "F08_spacepath"  @("compile",'atlas.md','--out',"out with spaces.atlas","--json")
# compile same file twice -> hash compare
& $bin compile atlas.md --out first.atlas 2>$null
& $bin compile atlas.md --out second.atlas 2>$null
$f1=(Get-FileHash first.atlas).Hash; $f2=(Get-FileHash second.atlas).Hash
Add-Content $summary "F09_idempotent`tEC=n/a`t$(if($f1-eq$f2){'SAME'}else{'DIFF'})`tn/a`tcompile atlas.md twice -> $f1 vs $f2"

# Category 8: determinism - solve 10x
$h0=$null; $det="DETERMINISTIC"
for($i=0;$i -lt 10;$i++){ $h=(Get-FileHash (cmd /c "$bin solve ""binary format"" --bundle baseline.atlas --json" 2>`$null | Out-String) 2>$null); }
# simpler: capture stdout text 10x
$outs=@()
for($i=0;$i -lt 10;$i++){ $o=& $bin solve "binary format" --bundle baseline.atlas --json 2>$null; $outs+=$o }
$uniq=$outs | Sort-Object -Unique
Add-Content $summary "G01_solve_det`tEC=n/a`t$(if($uniq.Count -eq 1){'SAME'}else{'DIFF('+$uniq.Count+')'})`tn/a`solve x10 identical query"

# decide determinism
$douts=@()
for($i=0;$i -lt 5;$i++){ $o=& $bin decide "choose edge storage" --bundle baseline.atlas --json 2>$null; $douts+=$o }
$duniq=$douts|Sort-Object -Unique
Add-Content $summary "G02_decide_det`tEC=n/a`t$(if($duniq.Count -eq 1){'SAME'}else{'DIFF('+$duniq.Count+')'})`tn/a`tdecide x5"

# JSON validity on success paths
Run "G03_solve_ok"   @("solve","binary format","--bundle","baseline.atlas","--json")
Run "G04_decide_ok"  @("decide","choose edge storage","--bundle","baseline.atlas","--json")
Run "G05_dump_ok"    @("dump","--bundle","baseline.atlas","--json")
Run "G06_install_ok" @("install","baseline.atlas","--name","qa_baseline","--json")

# panics with backtrace (cat 9)
$env:RUST_BACKTRACE="1"
$p = Run "H01_panic_bt" @("solve","x","--bundle","trunc.atlas","--json")
$env:RUST_BACKTRACE=""

# init --json produces no JSON? capture both
Run "I01_init_json"  @("init","jsonpkg","--json")
$ij=(Get-Content I01_init_json.so -Raw -EA SilentlyContinue); if($ij-eq$null){$ij=""}
Write-Host "I01 init --json STDOUT bytes: $($ij.Length)"
Write-Host "I01 init --json STDERR first line: $((Get-Content I01_init_json.se -Raw -EA SilentlyContinue).Split([Environment]::NewLine)[0])"

# unknown template fallback check
Run "I02_init_unktmpl" @("init","unktmpl","-t","nope","--json")

Write-Host "PHASE2 DONE"
Get-Content $summary