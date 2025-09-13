Set-StrictMode -Version 'Latest'
$ErrorActionPreference = 'Stop'

$FILE_NAME = "groonga-latest-x64-vs2022.zip"
curl.exe -fsSLO --output-dir "${env:TEMP}" "https://packages.groonga.org/windows/groonga/${FILE_NAME}"

$GROONGA_FILES = Expand-Archive -Path "${env:TEMP}\${FILE_NAME}" -DestinationPath "${env:TEMP}" -PassThru
$GROONGA_DIR = ($GROONGA_FILES |? {$_ -is [System.IO.DirectoryInfo] } | Select-Object -First 1).Parent

Write-Output "GROONGA_HOME=${GROONGA_DIR}" >> "${env:GITHUB_ENV}"
Write-Output "${GROONGA_DIR}\bin" >> "${env:GITHUB_PATH}"
