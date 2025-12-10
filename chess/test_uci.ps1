# Test UCI engine communication
$engine = Start-Process -FilePath "target\release\chess.exe" -RedirectStandardInput "test_input.txt" -RedirectStandardOutput "test_output.txt" -RedirectStandardError "test_error.txt" -NoNewWindow -PassThru

Start-Sleep -Seconds 1

# Send UCI commands
"uci" | Out-File -FilePath "test_input.txt" -Encoding ASCII -NoNewline
Start-Sleep -Milliseconds 100
"isready" | Out-File -FilePath "test_input.txt" -Append -Encoding ASCII -NoNewline
Start-Sleep -Milliseconds 100
"position startpos" | Out-File -FilePath "test_input.txt" -Append -Encoding ASCII -NoNewline
Start-Sleep -Milliseconds 100
"go depth 1" | Out-File -FilePath "test_input.txt" -Append -Encoding ASCII -NoNewline
Start-Sleep -Seconds 2
"quit" | Out-File -FilePath "test_input.txt" -Append -Encoding ASCII -NoNewline

Start-Sleep -Seconds 1

# Read output
Get-Content "test_output.txt"
Write-Host "--- Errors ---"
Get-Content "test_error.txt"

$engine.Kill()

