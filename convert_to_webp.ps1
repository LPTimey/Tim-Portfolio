# ===============================
# PDF/JPG/PNG → PNG/WebP Conversion Script
# ===============================

# Ordner für PNGs und WebPs
$pngDir = Join-Path -Path "." -ChildPath "png"
$webpDir = Join-Path -Path "." -ChildPath "webp"

# Ordner erstellen, falls nicht vorhanden
foreach ($dir in @($pngDir, $webpDir)) {
    if (!(Test-Path -Path $dir)) {
        New-Item -ItemType Directory -Path $dir | Out-Null
    }
}

# PNG-Dateien im Hauptverzeichnis finden (außerhalb des png-Ordners)
$additionalPngs = Get-ChildItem -Path "." -Filter "*.png" | Where-Object { $_.DirectoryName -ne (Resolve-Path $pngDir).Path }

foreach ($png in $additionalPngs) {
    $destPath = Join-Path $pngDir $png.Name
    Write-Host "found $($png)"

    # Wenn Ziel nicht existiert oder Quelle neuer ist, dann kopieren
    if (!(Test-Path -Path $destPath) -or ((Get-Item $png.FullName).LastWriteTime -gt (Get-Item $destPath).LastWriteTime)) {
        Copy-Item -Path $png.FullName -Destination $destPath -Force
        Write-Host "🖼️  PNG aktualisiert/kopiert: $($png.Name) -> png/"
    }
}

# JPG/JPEG-Dateien im Hauptverzeichnis finden
$jpgFiles = Get-ChildItem -File | Where-Object { $_.Extension -match '\.jpe?g$' }
Write-host "huh $($jpgFiles)"

foreach ($jpg in $jpgFiles) {
    Write-Host "found $($jpg)"
    $baseName = [System.IO.Path]::GetFileNameWithoutExtension($jpg.Name)
    $pngOutput = Join-Path $pngDir "$baseName.png"

    # Nur konvertieren, wenn PNG nicht existiert oder das JPG neuer ist
    if (!(Test-Path $pngOutput) -or ((Get-Item $jpg.FullName).LastWriteTime -gt (Get-Item $pngOutput).LastWriteTime)) {
        ffmpeg -y -i "$($jpg.FullName)" "$pngOutput"
        Write-Host "🖼️  JPG konvertiert: $($jpg.Name) → $([System.IO.Path]::GetFileName($pngOutput))"
    }
}

# Alle PDFs im aktuellen Ordner verarbeiten
$pdfFiles = Get-ChildItem -Path "." -Filter "*.pdf"

foreach ($pdf in $pdfFiles) {
    $baseName = [System.IO.Path]::GetFileNameWithoutExtension($pdf.Name)
    $pngPrefix = Join-Path $pngDir "$baseName-page"

    Write-Host "📄 Verarbeite PDF: $($pdf.Name)"

    # PDF -> PNG-Dateien im PNG-Ordner speichern
    pdftoppm -png -rx 150 -ry 150 "$($pdf.FullName)" "$pngPrefix"
}

# Alle PNGs im png-Ordner verarbeiten → WebP
$pngFiles = Get-ChildItem -Path $pngDir -Filter "*.png"

foreach ($png in $pngFiles) {
    $baseName = [System.IO.Path]::GetFileNameWithoutExtension($png.Name)
    $webpPath = Join-Path $webpDir "$baseName.webp"

    # PNG -> WebP mit ffmpeg
    ffmpeg -y -i "$($png.FullName)" "$webpPath"
    Write-Host "➡️  WebP erstellt: $($png.Name) → $([System.IO.Path]::GetFileName($webpPath))"
}

Write-Host "`n🔍 Vergleiche WebP-Größen und erstelle ggf. kleinere Varianten ..."

# Alle WebP-Dateien durchgehen
$webpFiles = Get-ChildItem -Path $webpDir -Filter "*.webp" -File | Where-Object {
    $_.Name -notlike '*_small.webp' -and
    $_.Name -notlike '*_very_small.webp' -and
    $_.Name -notlike '*_tiny.webp' #-and
}

foreach ($webp in $webpFiles) {
    $smallPath = $webp.FullName -replace '\.webp$', '_small.webp'
    $verySmallPath = $webp.FullName -replace '\.webp$', '_very_small.webp'
    $tinyPath = $webp.FullName -replace '\.webp$', '_tiny.webp'

    # Skaliere auf max. 1080px Höhe (Seitenverhältnis beibehalten)
    ffmpeg -y -i "$($webp.FullName)" -vf "scale=-1:1080" -compression_level 6 "$smallPath"
    ffmpeg -y -i "$($webp.FullName)" -vf "scale=-1:720" -compression_level 6 "$verySmallPath"
    ffmpeg -y -i "$($webp.FullName)" -vf "scale=-1:480" -compression_level 6 "$tinyPath"

    # Dateigrößen vergleichen
    $originalSize = (Get-Item $webp.FullName).Length
    $smallSize = (Get-Item $smallPath).Length
    $verySmallSize = (Get-Item $verySmallPath).Length
    $tinySize = (Get-Item $tinyPath).Length

    if ($smallSize -lt $originalSize) {
        Write-Host "✅ Kleinere Version erstellt für: $($webp.Name) → $([Math]::Round($smallSize / 1KB, 1)) KB"
    } else {
        Remove-Item -Path $smallPath
        Write-Host "ℹ️  Keine kleinere Version nötig für: $($webp.Name)"
    }
    if ($verySmallSize -lt $originalSize) {
        Write-Host "✅ Kleinere Version erstellt für: $($webp.Name) → $([Math]::Round($verySmallSize / 1KB, 1)) KB"
    } else {
        Remove-Item -Path $verySmallPath
        Write-Host "ℹ️  Keine kleinere Version nötig für: $($webp.Name)"
    }
    if ($tinySize -lt $originalSize) {
        Write-Host "✅ Kleinere Version erstellt für: $($webp.Name) → $([Math]::Round($tinySize / 1KB, 1)) KB"
    } else {
        Remove-Item -Path $tinyPath
        Write-Host "ℹ️  Keine kleinere Version nötig für: $($webp.Name)"
    }
}

Write-Host "`n🏁 Fertig! PNGs in ./png, WebPs in ./webp"
