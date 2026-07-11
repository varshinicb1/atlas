cd C:\Users\varsh\OneDrive\Documents\3Atlas\qa\cli
$bin = "..\..\target\release/atlas.exe"
$summary = "summary.tsv"
"" | Set-Content $summary

function JCheck($s) {
    $t = $s.Trim()
    if ($t.Length -eq 0) { return "EMPTY" }
    try { $t | ConvertFrom-Json > $null; return "JSON_OK" } catch { return "JSON_BAD" }
}

function Run($id, $cmd) {
    $so = "$id.so"; $se = "$id.se"
    & $bin @cmd 1>$so 2>$se
    $ec = $LASTEXITCODE
    $stdout = (Get-Content $so -Raw -ErrorAction SilentlyContinue)
    if ($stdout -eq $null) { $stdout = "" }
    $stderr = (Get-Content $se -Raw -ErrorAction SilentlyContinue)
    if ($stderr -eq $null) { $stderr = "" }
    $usesJson = $cmd -contains "--json"
    $jj = if ($usesJson) { JCheck $stdout } else { "n/a" }
    $panic = $stderr -match "panicked at"
    Add-Content $summary "$id`tEC=$ec`t$jj`tPANIC=$panic`t$($cmd -join ' ')"
    return @{ec=$ec; jj=$jj; panic=$panic; se=$stderr.Trim()}
}

# build corrupt atlas by truncating baseline
$copy = [System.IO.File]::ReadAllBytes("$PWD\baseline.atlas")
$half = $copy[0..([Math]::Floor($copy.Length/2))]
[System.IO.File]::WriteAllBytes("$PWD\trunc.atlas", [byte[]]$half)

# ---- Category 2: arg edge cases ----
Run "A01_solve_noarg"   @("solve","--json")
Run "A02_decide_noarg"  @("decide","--json")
Run "A03_reason_noarg"  @("reason","--json")
Run "A04_search_noarg"  @("search","--json")
Run "A05_init_noarg"    @("init","--json")
Run "A06_publish_noarg" @("publish","--json")
Run "A07_compile_noarg" @("compile","--json")
Run "A08_unknown_flag"  @("solve","x","--bogus","--json")
Run "A09_bundle_missing"@("solve","anything","--bundle","doesnotexist.atlas","--json")
Run "A10_dir_as_file"   @("compile",".","--out","o.atlas","--json")
Run "A11_longquery"     @("solve",("q"*5000),"--bundle","baseline.atlas","--json")
Run "A12_emptyquery"    @("solve","","--bundle","baseline.atlas","--json")
Run "A13_query_inject"  @("solve",'"; DROP TABLE -- `n$(ls)',"--bundle","baseline.atlas","--json")

# ---- Category 4: solve/decide/reason ----
Run "B01_solve_nobundle" @("solve","x","--bundle","nope.atlas","--json")
Run "B02_solve_corrupt"  @("solve","x","--bundle","trunc.atlas","--json")
Run "B03_decide_empty"   @("decide","","--bundle","baseline.atlas","--json")
Run "B04_decide_badctx1" @("decide","x","--bundle","baseline.atlas","-c","novalue","--json")
Run "B05_decide_badctx2" @("decide","x","--bundle","baseline.atlas","-c","=value","--json")
Run "B06_decide_badctx3" @("decide","x","--bundle","baseline.atlas","-c","k1=v1","-c","k1=v2","--json")
Run "B07_decide_notree"  @("decide","some random thing","--bundle","baseline.atlas","--json")
Run "B08_reason_corrupt"  @("reason","x","--bundle","trunc.atlas","--json")
Run "B09_reason_normal"   @("reason","atlas architecture","--bundle","baseline.atlas","--json")

# ---- Category 5: verify ----
Run "C01_verify_corrupt" @("verify","--bundle","trunc.atlas","--json")
Run "C02_verify_nobundle"@("verify","--bundle","nope.atlas","--json")
Run "C03_verify_badpolicy"@("verify","--bundle","baseline.atlas","--policy","totally_unknown_policy_xyz","--json")
Run "C04_verify_empty_policy"@("verify","--bundle","baseline.atlas","--policy","","--json")
Run "C05_verify_artifact"@("verify","--bundle","baseline.atlas","--artifact","nonexistent_artifact","--json")
Run "C06_verify_ok"      @("verify","--bundle","baseline.atlas","--json")

# ---- Category 6: init ----
Run "D01_init_space"   @("init","my project","--json")
Run "D02_init_slash"   @("init","a/b/c","--json")
Run "D03_init_traversal"@("init","../evilpkg","--json")
Run "D04_init_unicode" @("init","пакет_тест","--json")
Run "D05_init_existing"@("init","atlas","--json")   # atlas.md / atlas.atlas present -> subdir atlas
Run "D06_init_tmpl_flutter" @("init","p_flutter","-t","flutter","--json")
Run "D07_init_tmpl_rust"    @("init","p_rust","-t","rust","--json")
Run "D08_init_tmpl_ts"      @("init","p_ts","-t","typescript","--json")
Run "D09_init_tmpl_py"      @("init","p_py","-t","python","--json")
Run "D10_init_tmpl_ts7"     @("init","p_ts7","-t","typescript_7","--json")
Run "D11_init_tmpl_unknown" @("init","p_unk","-t","does_not_exist","--json")
Run "D12_init_dir_traversal"@("init","x","-d","..\out_evil","--json")

# ---- Category 7: install/search/publish ----
Run "E01_install_unsupported" @("install","garbage.md","--name","g","--json")
Run "E02_search_empty"    @("search","","--json")
Run "E03_search_long"     @("search",("s"*4000),"--json")
Run "E04_search_special"  @("search","!@#$%^&*()","--json")
Run "E05_publish_nopath"  @("publish","nonexistent_dir_xyz","--registry","http://localhost:9999","--json")
Run "E06_publish_md"      @("publish","atlas.md","--registry","http://localhost:9999","--json")

Write-Host "PHASE1 DONE"
Get-Content $summary
