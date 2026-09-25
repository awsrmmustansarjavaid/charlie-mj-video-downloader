# ============================================================
# Charlie MJ Video Downloader - Native Host Build
# ============================================================
# Purpose:
# - Install PyInstaller
# - Convert host.py into a standalone Windows executable
# - Create host.exe for the native messaging host
#
# Requirements:
# - Python must be installed and available in PATH
# - pip must be available
# - host.py must exist in the current working directory
# ============================================================


# ------------------------------------------------------------
# Install / Upgrade PyInstaller
# ------------------------------------------------------------
# PyInstaller packages the Python script and its dependencies
# into a standalone executable.
#
# --upgrade ensures the latest available PyInstaller version
# is installed.
# ------------------------------------------------------------
python -m pip install --upgrade pyinstaller


# ------------------------------------------------------------
# Build the Native Host Executable
# ------------------------------------------------------------
# --onefile:
#   Creates a single executable file instead of a directory
#   containing multiple generated files.
#
# --name host:
#   Sets the output executable name to host.exe.
#
# host.py:
#   The Python native messaging host source file.
# ------------------------------------------------------------
pyinstaller --onefile --name host host.py


# ------------------------------------------------------------
# Display Build Output Location
# ------------------------------------------------------------
# After PyInstaller completes successfully, the generated
# executable should be located at:
#
# native-host/dist/host.exe
# ------------------------------------------------------------
Write-Host "Host executable: native-host/dist/host.exe"