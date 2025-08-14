# ===============================
# PDF/PNG → PNG/WebP Conversion Script
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

    # Nur kopieren, wenn die Datei im Zielordner noch nicht existiert
    if (!(Test-Path -Path $destPath)) {
        Copy-Item -Path $png.FullName -Destination $destPath
        Write-Host "🖼️  PNG kopiert: $($png.Name) -> png/"
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
$webpFiles = Get-ChildItem -Path $webpDir -Filter "*.webp"

foreach ($webp in $webpFiles) {
    $smallPath = $webp.FullName -replace '\.webp$', '_small.webp'

    # Skaliere auf max. 1080px Höhe (Seitenverhältnis beibehalten)
    ffmpeg -y -i "$($webp.FullName)" -vf "scale=-1:1080" -compression_level 6 "$smallPath"

    # Dateigrößen vergleichen
    $originalSize = (Get-Item $webp.FullName).Length
    $smallSize = (Get-Item $smallPath).Length

    if ($smallSize -lt $originalSize) {
        Write-Host "✅ Kleinere Version erstellt für: $($webp.Name) → $([Math]::Round($smallSize / 1KB, 1)) KB"
    } else {
        # Falls nicht kleiner: löschen
        Remove-Item -Path $smallPath
        Write-Host "ℹ️  Keine kleinere Version nötig für: $($webp.Name)"
    }
}

Write-Host "`n🏁 Fertig! PNGs in ./png, WebPs in ./webp"
