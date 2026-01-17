
$source = "..\nebula-vscode\icon.png"
$dest = "logo.bmp"

Write-Host "Converting $source to $dest..."

Add-Type -AssemblyName System.Drawing
$img = [System.Drawing.Image]::FromFile($source)
$img.Save($dest, [System.Drawing.Imaging.ImageFormat]::Bmp)
$img.Dispose()

Write-Host "Conversion complete."
