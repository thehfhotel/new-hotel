$ErrorActionPreference = 'Stop'
Add-Type -AssemblyName System.Drawing
$out = 'C:\Users\nutok\new-hotel\.decompile\assets'
New-Item -ItemType Directory -Force -Path $out | Out-Null

# 1) Extract the blank card template (Resources.Untitled_1__Copy_) from KP EXE
$path = 'C:\Users\nutok\Downloads\Hotel-2018- V.1.45\KPThaiNationalIDCard.exe'
$bytes = [System.IO.File]::ReadAllBytes($path)
$asm = [System.Reflection.Assembly]::Load($bytes)
$resType = $asm.GetType('KPNationalIDCard.Example.Properties.Resources')
if ($null -eq $resType) {
  Write-Output "Resources type not found; enumerating candidates:"
  $asm.GetTypes() | Where-Object { $_.Name -eq 'Resources' } | ForEach-Object { $_.FullName }
} else {
  $prop = $resType.GetProperty('Untitled_1__Copy_', ([System.Reflection.BindingFlags]::NonPublic -bor [System.Reflection.BindingFlags]::Static -bor [System.Reflection.BindingFlags]::Public))
  if ($null -eq $prop) {
    Write-Output "Property Untitled_1__Copy_ not found. Available image-ish members:"
    $resType.GetProperties(([System.Reflection.BindingFlags]::NonPublic -bor [System.Reflection.BindingFlags]::Static -bor [System.Reflection.BindingFlags]::Public)) | ForEach-Object { "  " + $_.Name + " : " + $_.PropertyType.Name }
  } else {
    $img = $prop.GetValue($null, $null)
    $bmp = [System.Drawing.Bitmap]$img
    $dest = Join-Path $out 'thai_id_template.png'
    $bmp.Save($dest, [System.Drawing.Imaging.ImageFormat]::Png)
    Write-Output ("TEMPLATE saved: {0}  ({1} x {2}, pixelformat={3})" -f $dest, $bmp.Width, $bmp.Height, $bmp.PixelFormat)
  }
}

# 2) Confirm DilleniaUPC Bold family name in upcdb.ttf
$pfc = New-Object System.Drawing.Text.PrivateFontCollection
$fontFile = 'C:\Windows\Fonts\upcdb.ttf'
$pfc.AddFontFile($fontFile)
foreach ($fam in $pfc.Families) {
  Write-Output ("FONT upcdb.ttf family: [{0}]  bold-available={1}" -f $fam.Name, $fam.IsStyleAvailable([System.Drawing.FontStyle]::Bold))
  # cell metrics for baseline math
  $em = $fam.GetEmHeight([System.Drawing.FontStyle]::Bold)
  $asc = $fam.GetCellAscent([System.Drawing.FontStyle]::Bold)
  $desc = $fam.GetCellDescent([System.Drawing.FontStyle]::Bold)
  $lg = $fam.GetLineSpacing([System.Drawing.FontStyle]::Bold)
  Write-Output ("     emHeight={0} cellAscent={1} cellDescent={2} lineSpacing={3}" -f $em, $asc, $desc, $lg)
}
