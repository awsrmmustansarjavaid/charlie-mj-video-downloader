# Requires Python and PyInstaller.
python -m pip install --upgrade pyinstaller
pyinstaller --onefile --name host host.py
Write-Host "Host executable: native-host/dist/host.exe"
