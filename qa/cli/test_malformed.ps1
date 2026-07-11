cd C:\Users\varsh\OneDrive\Documents\3Atlas\qa\cli
$bin = "..\..\target\release/atlas.exe"
$log = "log_malformed.txt"
"" | Set-Content $log
function Run($label, $cmd) {
    $out = & $bin @cmd 2>&1
    $ec = $LASTEXITCODE
    $txt = ($out | Out-String)
    $panicked = $txt -match "panicked at"
    $validJson = $false
    if ($cmd -contains "--json") {
        try { $txt | ConvertFrom-Json > $null; $validJson = $true } catch { $validJson = $false }
    }
    Add-Content $log "===== $label ====="
    Add-Content $log "CMD: atlas.exe $($cmd -join ' ')"
    Add-Content $log "EXIT: $ec  PANIC: $panicked  VALIDJSON: $validJson"
    Add-Content $log $txt
    Add-Content $log ""
}

# empty
Run "empty.md"        @("compile","empty.md","--out","o_empty.atlas","--json")
Run "nofront.md"      @("compile","nofront.md","--out","o_nofront.atlas","--json")
Run "onlyfm.md"       @("compile","onlyfm.md","--out","o_onlyfm.atlas","--json")
Run "brokenfm.md"     @("compile","brokenfm.md","--out","o_brokenfm.atlas","--json")
Run "dupnodes.md"     @("compile","dupnodes.md","--out","o_dup.atlas","--json")
Run "badedge.md"      @("compile","badedge.md","--out","o_badedge.atlas","--json")
Run "huge.md"         @("compile","huge.md","--out","o_huge.atlas","--json")
Run "brokenutf.md"    @("compile","brokenutf.md","--out","o_butf.atlas","--json")
Run "garbage.md"      @("compile","garbage.md","--out","o_garb.atlas","--json")
Run "nested.md"       @("compile","nested.md","--out","o_nest.atlas","--json")
# no json variant for one panic check
Run "empty.md-nohuman" @("compile","empty.md","--out","o_empty2.atlas")

Write-Host "DONE"
