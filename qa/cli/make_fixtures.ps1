cd C:\Users\varsh\OneDrive\Documents\3Atlas\qa\cli
$ErrorActionPreference = 'Continue'
$bin = "..\..\target\release/atlas.exe"

# ---- Fixtures for malformed inputs ----
Set-Content -Path empty.md -Value "" -NoNewline
Set-Content -Path nofront.md -Value "# Just a heading`n`nSome text with no frontmatter."
Set-Content -Path onlyfm.md -Value "---`nkind: Package`nid: package:test`nname: T`n---`n"
Set-Content -Path brokenfm.md -Value "---`nkind: Package`nid: package:test`nname: T`nthis is not valid yaml: : :`n---`n# Body"
Set-Content -Path dupnodes.md -Value "---`nkind: Package`nid: package:dup`nname: Dup`nconcepts:`n  - name: A`n    id: concept:dup/x`n  - name: B`n    id: concept:dup/x`n---`n# Dup"
Set-Content -Path badedge.md -Value "---`nkind: Package`nid: package:be`nname: BE`nconcepts:`n  - name: A`n    id: concept:be/a`n---`n# BE`n`nSee [nonexistent](concept:be/zzz)."
# 5MB file
("x" * (5*1024*1024)) | Set-Content -Path huge.md
# non-UTF8 bytes
$b = [byte[]]@(0xFF,0xFE,0x00,0x01,0xCA,0xFE,0xBA,0xBE)
[System.IO.File]::WriteAllBytes("$PWD\brokenutf.md", $b)
# binary garbage renamed
[System.IO.File]::WriteAllBytes("$PWD\garbage.md", [System.Text.Encoding]::ASCII.GetBytes("ATLAS notreallybinary garbage here"))
# deeply nested yaml
$nested = "---`nkind: Package`nid: package:nest`nname: N`nx:"
for ($i=0; $i -lt 250; $i++) { $nested += "`n"+("  "*$i)+"level${i}:" }
Set-Content -Path nested.md -Value $nested

Write-Host "FIXTURES CREATED"
Get-ChildItem | Select-Object Name,Length | Format-Table -AutoSize
