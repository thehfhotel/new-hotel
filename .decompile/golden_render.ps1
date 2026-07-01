$ErrorActionPreference = 'Stop'
Add-Type -AssemblyName System.Drawing

$out = 'C:\Users\nutok\new-hotel\.decompile\assets'
$templatePath = Join-Path $out 'thai_id_template.png'

# --- sample chip data (as the KP program would hold it after a card read) ---
$cid      = '1234567890123'
$thPrefix = 'นาย'; $thFirst = 'สมชาย'; $thLast = 'ใจดีมีสุขเกษม'
$enPrefix = 'Mr.'; $enFirst = 'Somchai'; $enLast = 'Jaideemesukkasem'
$birthday = '09/05/2533'   # dd/MM/yyyy, Buddhist year (BE 2533 = 1990)
$issue    = '15/03/2560'   # BE 2560 = 2017
$expire   = '14/03/2568'   # BE 2568 = 2025
$address  = '50 หมู่ 5 ตำบลบางแก้ว อำเภอบางพลี จังหวัดสมุทรปราการ'

$thMonths = @{ 1='ม.ค.';2='ก.พ.';3='มี.ค.';4='เม.ย.';5='พ.ค.';6='มิ.ย.';7='ก.ค.';8='ส.ค.';9='ก.ย.';10='ต.ค.';11='พ.ย.';12='ธ.ค.' }
$enMonths = @{ 1='Jan.';2='Feb.';3='Mar.';4='Apr.';5='May.';6='Jun.';7='Jul.';8='Aug.';9='Sep.';10='Oct.';11='Nov.';12='Dec.' }

function Format-CID($c) {
  return ($c.Substring(0,1) + ' ' + $c.Substring(1,4) + ' ' + $c.Substring(5,5) + ' ' + $c.Substring(10,2) + ' ' + $c.Substring(12,1))
}
# Thai date row: "DD <thai-abbrev> <BE-year>"
function Thai-Date($d) {
  $day=$d.Substring(0,2); $mo=[int]$d.Substring(3,2); $yr=$d.Substring(6,4)
  return ($day + ' ' + $thMonths[$mo] + ' ' + $yr)
}
# English date row: "DD <eng-abbrev> <AD-year = BE-543>"
function Eng-Date($d) {
  $day=$d.Substring(0,2); $mo=[int]$d.Substring(3,2); $yr=[int]$d.Substring(6,4) - 543
  return ($day + ' ' + $enMonths[$mo] + ' ' + $yr)
}
# Address transform, exactly as uuu does it
function Split-Address($a) {
  $s = $a.Replace('ตำบล','ต.').Replace('อำเภอ',"`nอ.").Replace('จังหวัด','จ.').Replace('แขวง',"`nแขวง.")
  $nl = $s.LastIndexOf("`n")
  if ($nl -lt 0) { return @($s, '') }
  $line2 = $s.Substring($nl+1)
  $line1 = $s.Substring(0,$nl).Replace("`n",'')
  return @($line1, $line2)
}

$tpl = [System.Drawing.Image]::FromFile($templatePath)
$bmp = New-Object System.Drawing.Bitmap(446,273)
$bmp.SetResolution(96,96)
$g = [System.Drawing.Graphics]::FromImage($bmp)
$g.SmoothingMode = [System.Drawing.Drawing2D.SmoothingMode]::HighQuality
$g.InterpolationMode = [System.Drawing.Drawing2D.InterpolationMode]::HighQualityBicubic
$g.TextRenderingHint = [System.Drawing.Text.TextRenderingHint]::AntiAlias
# StretchImage: template stretched into the 446x273 client area
$g.DrawImage($tpl, (New-Object System.Drawing.Rectangle(0,0,446,273)))

$fam = 'DilleniaUPC'
$f22 = New-Object System.Drawing.Font($fam, 22, [System.Drawing.FontStyle]::Bold)
$f18 = New-Object System.Drawing.Font($fam, 18, [System.Drawing.FontStyle]::Bold)
$f14 = New-Object System.Drawing.Font($fam, 14, [System.Drawing.FontStyle]::Bold)
$black = [System.Drawing.Brushes]::Black
$navy  = New-Object System.Drawing.SolidBrush([System.Drawing.Color]::FromArgb(255,0,0,128))

$addr = Split-Address $address

$g.DrawString((Format-CID $cid),                 $f22, $black, (New-Object System.Drawing.PointF(200,25)))
$g.DrawString(("$thPrefix $thFirst $thLast"),     $f22, $black, (New-Object System.Drawing.PointF(110,52)))
$g.DrawString(("$enPrefix $enFirst"),             $f18, $navy,  (New-Object System.Drawing.PointF(180,78)))
$g.DrawString($enLast,                            $f18, $navy,  (New-Object System.Drawing.PointF(205,96)))
$g.DrawString((Thai-Date $birthday),              $f18, $black, (New-Object System.Drawing.PointF(190,115)))
$g.DrawString((Eng-Date  $birthday),              $f18, $navy,  (New-Object System.Drawing.PointF(220,135)))
$g.DrawString($addr[0],                           $f18, $black, (New-Object System.Drawing.PointF(71,175)))
$g.DrawString($addr[1],                           $f18, $black, (New-Object System.Drawing.PointF(44,195)))
$g.DrawString((Thai-Date $issue),                 $f14, $black, (New-Object System.Drawing.PointF(44,215)))
$g.DrawString((Eng-Date  $issue),                 $f14, $navy,  (New-Object System.Drawing.PointF(44,237)))
$g.DrawString((Thai-Date $expire),                $f14, $black, (New-Object System.Drawing.PointF(244,215)))
$g.DrawString((Eng-Date  $expire),                $f14, $navy,  (New-Object System.Drawing.PointF(244,237)))
# photo placeholder
$g.FillRectangle((New-Object System.Drawing.SolidBrush([System.Drawing.Color]::FromArgb(255,210,210,210))), 332,129,103,119)
$g.DrawRectangle([System.Drawing.Pens]::Gray, 332,129,103,119)

$g.Dispose()
$dest = Join-Path $out 'golden_card_446x273.png'
$bmp.Save($dest, [System.Drawing.Imaging.ImageFormat]::Png)
Write-Output ("GOLDEN saved: {0}" -f $dest)

# also the 703x996 padded canvas (iHOTEL native), card at (121,91)
$canvas = New-Object System.Drawing.Bitmap(703,996)
$cg = [System.Drawing.Graphics]::FromImage($canvas)
$cg.Clear([System.Drawing.Color]::White)
$cg.DrawImage($bmp, 121, 91, 446, 273)
$cg.Dispose()
$dest2 = Join-Path $out 'golden_card_703x996.png'
$canvas.Save($dest2, [System.Drawing.Imaging.ImageFormat]::Png)
Write-Output ("GOLDEN padded saved: {0}" -f $dest2)
Write-Output ("Address line1: [{0}]" -f $addr[0])
Write-Output ("Address line2: [{0}]" -f $addr[1])
Write-Output ("DOB thai: [{0}]  eng: [{1}]" -f (Thai-Date $birthday), (Eng-Date $birthday))
