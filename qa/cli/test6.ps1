cd C:\Users\varsh\OneDrive\Documents\3Atlas\qa\cli
$bin = "..\..\target\release/atlas.exe"
function Run($id,$cmd){ $so="$id.so";$se="$id.se"; & $bin @cmd 1>$so 2>$se; $ec=$LASTEXITCODE; $se2=(Get-Content $se -Raw -EA SilentlyContinue); if($se2-eq$null){$se2=""}; $panic=$se2 -match "panicked at"; Write-Host "$id EC=$ec PANIC=$panic : $($cmd -join ' ')"; if($panic){ Get-Content $se | Select-Object -First 4 | ForEach-Object { Write-Host "    $_" } } }

# zero-byte atlas
"" | Set-Content -NoNewline zero.atlas
Run "Z01_dump_zero"   @("dump","--bundle","zero.atlas","--json")
Run "Z02_solve_zero"  @("solve","x","--bundle","zero.atlas","--json")
Run "Z03_verify_zero" @("verify","--bundle","zero.atlas","--json")
# reason with bogus model
Run "Z04_reason_model"@("reason","x","--bundle","baseline.atlas","--model","totally_bogus_model_xyz","--json")
Run "Z05_reason_ollama"@("reason","x","--bundle","baseline.atlas","--model","ollama","--json")
# dump corrupt (truncated)
Run "Z06_dump_corrupt"@("dump","--bundle","trunc.atlas","--json")
# compile emit-ir flag
Run "Z07_emitir"      @("compile","atlas.md","--out","e.atlas","--emit-ir","--json")
# search against unreachable registry (timeout/err handling)
Run "Z08_search_badreg"@("search","x","--registry","http://localhost:9999","--json")
# install a .atlas that does not exist
Run "Z09_install_missing"@("install","ghost.atlas","--name","g","--json")
# solve with bundle being a directory
Run "Z10_solve_dir"   @("solve","x","--bundle",".","--json")
# decide no tree bundle but valid query
Run "Z11_decide_notree"@("decide","anything","--bundle","baseline.atlas","--json")
