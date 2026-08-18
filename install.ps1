#!/usr/bin/env pwsh

$ErrorActionPreference = "Stop"

$Url = "https://github.com/genkii/termixel/releases/download/v0.1.0/termixel.exe"
$InstallDir = "$env:LOCALAPPDATA\Termixel"
$Binary = "$InstallDir\termixel.exe"

Write-Host "Installing Termixel..."

New-Item -ItemType Directory -Force -Path $InstallDir | Out-Null

Invoke-WebRequest -Uri $Url -OutFile $Binary

$currentPath = [Environment]::GetEnvironmentVariable("Path", "User")

if ($currentPath -notlike "*$InstallDir*") {
    [Environment]::SetEnvironmentVariable(
        "Path",
        "$currentPath;$InstallDir",
        "User"
    )
}

Write-Host ""
Write-Host "Termixel has been installed to:"
Write-Host $Binary
Write-Host ""
Write-Host "Termixel has been added to your user PATH."
Write-Host "Restart your terminal for the changes to take effect."
