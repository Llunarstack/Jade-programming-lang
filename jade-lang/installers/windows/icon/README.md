# Jade icon (Windows)

This folder is the **canonical location** for the Jade icon on Windows:

- **`jade.ico`** — Used by the installer and file association so all `.jdl` files on the PC show the Jade logo in Explorer.
- **`jade.png`** — Same icon in PNG (e.g. for docs or IDE themes).

To make every `.jdl` file on your machine use this icon, run from repo root or from this folder:

```powershell
.\jade-lang\installers\windows\register-jade-icon.ps1
```

If you get "Access denied", run PowerShell as Administrator and run the script again.
