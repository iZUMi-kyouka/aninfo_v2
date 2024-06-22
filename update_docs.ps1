# Define source and destination directories
$sourceDir = ".\dist"
$destDir = ".\docs"

# Remove all files in the destination directory
Get-ChildItem -Path $destDir -File | Remove-Item -Force

# Copy all files from the source directory to the destination directory
Copy-Item -Path "$sourceDir\*" -Destination $destDir -Recurse -Force
