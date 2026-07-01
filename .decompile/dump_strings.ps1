$ErrorActionPreference = 'Stop'
$path = 'C:\Users\nutok\Downloads\Hotel-2018- V.1.45\KPThaiNationalIDCard.exe'
$bytes = [System.IO.File]::ReadAllBytes($path)
$asm = [System.Reflection.Assembly]::Load($bytes)
$t = $asm.GetType('q3uifC6dT931xg0UY6.KYJngMCOCJNSf8T7gH')
if ($null -eq $t) { Write-Output "TYPE NOT FOUND"; $asm.GetTypes() | ForEach-Object { $_.FullName }; exit 1 }
$m = $t.GetMethod('Ushn1vyok', ([System.Reflection.BindingFlags]::NonPublic -bor [System.Reflection.BindingFlags]::Static))
if ($null -eq $m) { Write-Output "METHOD NOT FOUND"; exit 1 }
$ids = @(0,24,48,142,188,194,204,216,228,238,250,264,282,318,468,482,664,730,756,782,818,852,858,914,936,962,968,976,986,994,1006,1022,1028,1040,1052,1066,1080,1092,1106,1118,1130,1142,1154,1166,1178,1190,1202,1214,1226,1238,1250,1262,1274,1286,1298,1310,1644)
foreach ($id in $ids) {
  try {
    $s = $m.Invoke($null, @([int]$id))
    $hex = ($s.ToCharArray() | ForEach-Object { '{0:X4}' -f [int][char]$_ }) -join ' '
    Write-Output ("{0}`t[{1}]`tU+{2}" -f $id, $s, $hex)
  } catch {
    $msg = $_.Exception.Message
    if ($_.Exception.InnerException) { $msg = $_.Exception.InnerException.Message }
    Write-Output ("{0}`tERROR: {1}" -f $id, $msg)
  }
}
