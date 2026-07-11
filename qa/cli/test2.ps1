cd C:\Users\varsh\OneDrive\Documents\3Atlas\qa\cli
$bin = "..\..\target\release/atlas.exe"
$summary = "summary.txt"
"" | Set-Content $summary

function Run($id, $cmd) {
    $so = "$id.out"; $se = "$id.err"
    & $bin @cmd 1>$so 2>$se
    $ec = $LASTEXITCODE
    $stdout = Get-Content $so -Raw -ErrorAction SilentlyContinue
    if ($stdout -eq $null) { $stdout = "" }
    $usesJson = $cmd -contains "--json"
    $validJson = "n/a"
    if ($usesJson) {
        try { $stdout | ConvertFrom-Json > $null; $validJson = "JSON_OK" } catch { $validJson = "JSON_BAD" }
    }
    $panic = (Get-Content $se -Raw -ErrorAction SilentlyContinue) -match "panicked at"
    Add-Content $summary "$id`tEC=$ec`t$validJson`tPANIC=$panic`t$($cmd -join ' ')"
}

# --- malformed compile validity (stdout only) ---
Run "M01_empty"    @("compile","empty.md","--out","o1.atlas","--json")
Run "M02_nofront"  @("compile","nofront.md","--out","o2.atlas","--json")
Run "M03_brokenfm" @("compile","brokenfm.md","--out","o3.atlas","--json")
Run "M04_dup"      @("compile","dupnodes.md","--out","o4.atlas","--json")
Run "M05_utf"      @("compile","brokenutf.md","--out","o5.atlas","--json")
Run "M06_huge"     @("compile","huge.md","--out","o6.atlas","--json")
Run "M07_nested"   @("compile","nested.md","--out","o7.atlas","--json")
Run "M08_onlyfm"   @("compile","onlyfm.md","--out","o8.atlas","--json")
Run "M09_badedge"  @("compile","badedge.md","--out","o9.atlas","--json")
Run "M10_garb"     @("compile","garbage.md","--out","o10.atlas","--json")

Write-Host "MALFORMED DONE"
Get-Content $summary
