$ErrorActionPreference = 'Stop'
Add-Type -AssemblyName System.Drawing

# Measure, for GDI+ DrawString(text, font, brush, PointF(X0,Y0)):
#   - vertical offset from Y0 to the glyph baseline (bottom of a baseline-sitting glyph)
#   - vertical offset from Y0 to the cap-top (top ink of "E")
#   - horizontal offset from X0 to the first ink column (left side bearing / pad)
# for DilleniaUPC Bold at the three point sizes the card uses.

$fam = 'DilleniaUPC'
$X0 = 20; $Y0 = 20
foreach ($pt in @(22,18,14)) {
  $bmp = New-Object System.Drawing.Bitmap(300,140)
  $bmp.SetResolution(96,96)
  $g = [System.Drawing.Graphics]::FromImage($bmp)
  $g.Clear([System.Drawing.Color]::White)
  $g.TextRenderingHint = [System.Drawing.Text.TextRenderingHint]::AntiAlias
  $f = New-Object System.Drawing.Font($fam, $pt, [System.Drawing.FontStyle]::Bold)
  # "E" sits on the baseline and has a flat top; good for cap-top & baseline.
  $g.DrawString('E', $f, [System.Drawing.Brushes]::Black, (New-Object System.Drawing.PointF($X0,$Y0)))
  $g.Dispose()

  $minX=999; $minY=999; $maxY=-1
  for ($y=0; $y -lt $bmp.Height; $y++) {
    for ($x=0; $x -lt $bmp.Width; $x++) {
      $px = $bmp.GetPixel($x,$y)
      if ($px.R -lt 128) {
        if ($y -lt $minY) { $minY = $y }
        if ($y -gt $maxY) { $maxY = $y }
        if ($x -lt $minX) { $minX = $x }
      }
    }
  }
  $emPx = $pt * 96.0 / 72.0
  $topOff  = $minY - $Y0     # Y0 -> cap top
  $baseOff = $maxY - $Y0     # Y0 -> baseline (bottom of E)
  $leftOff = $minX - $X0     # X0 -> first ink column
  Write-Output ("pt={0}  emPx={1:N2}  capTopOff={2}  baselineOff={3}  leftPad={4}  baseline/emPx={5:N4}" -f $pt, $emPx, $topOff, $baseOff, $leftOff, ($baseOff/$emPx))
  $bmp.Dispose()
}
