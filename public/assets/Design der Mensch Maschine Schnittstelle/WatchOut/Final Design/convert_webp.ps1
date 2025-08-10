# Ordner für PNGs und WebPs
$pngDir = Join-Path -Path "." -ChildPath "png"
$webpDir = Join-Path -Path "." -ChildPath "webp"

# Ordner erstellen, falls nicht vorhanden
foreach ($dir in @($pngDir, $webpDir)) {
    if (!(Test-Path -Path $dir)) {
        New-Item -ItemType Directory -Path $dir | Out-Null
    }
}

# Alle PDFs im aktuellen Ordner verarbeiten
$pdfFiles = Get-ChildItem -Path "." -Filter "*.pdf"

foreach ($pdf in $pdfFiles) {
    $baseName = [System.IO.Path]::GetFileNameWithoutExtension($pdf.Name)
    $pngPrefix = Join-Path $pngDir "$baseName-page"

    Write-Host "📄 Verarbeite: $($pdf.Name)"

    # PDF -> PNG-Dateien im PNG-Ordner speichern
    pdftoppm -png -rx 150 -ry 150 "$($pdf.FullName)" "$pngPrefix"

    # Alle erzeugten PNG-Dateien laden
    $pngFiles = Get-ChildItem -Path $pngDir -Filter "$baseName-page*.png"

    foreach ($png in $pngFiles) {
        $index = $png.BaseName -replace "$baseName-page[-]?", ""
        $webpPath = Join-Path $webpDir "$baseName-$index.webp"

        # PNG -> WebP mit ffmpeg
        ffmpeg -y -i "$($png.FullName)" "$webpPath"
    }
}

Write-Host "`n✅ Fertig! PNGs in ./png, WebPs in ./webp"
