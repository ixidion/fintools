Set-Variable $env:FINAPP__CACHE_FILE = 'N:\\weekly_symbols\\cache\\symbol_cache.json'
Set-Variable $env:FINAPP__OUTPUT_PATH = 'N:\\weekly_symbols\\new'
Set-Variable $env:FINAPP__OUTPUT_PATH_DIFF = 'N:\\weekly_symbols\\diffs'
Start-Process fintools.exe