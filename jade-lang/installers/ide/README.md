# IDE recognition for Jade

Use these so **VS Code, Cursor, JetBrains, and other editors** recognize `.jdl` as the Jade programming language and use the **official icon** (`jade-lang/installers/windows/icon/jade.png`).

## Logo without extensions (all IDEs / file managers)

So **any** IDE or file manager shows the Jade logo for `.jdl` files without installing the VS Code extension:

- **Windows (installer):** When you check "Associate .jdl files with Jade", the installer writes the Jade icon into the registry (`HKCR\.jdl\DefaultIcon` and `Jade.SourceFile\DefaultIcon`). Explorer, VS Code (when it respects shell icons), and other apps then show the Jade logo for `.jdl` files.
- **Linux (install script):** The script registers the MIME type `text/x-jdl` and installs the Jade icon as the freedesktop MIME icon for that type (`~/.local/share/icons/hicolor/*/mimetypes/text-x-jdl.png`). File managers and IDEs that follow freedesktop (Nautilus, many GTK apps) show the Jade logo.
- **macOS:** The install script registers the `.jdl` UTI so the system recognizes the type; a custom icon for the UTI would require an app bundle, so the system may show a generic document icon until you install the Jade extension.

No IDE extension is required for the logo on Windows and Linux after install.

## VS Code / Cursor

### Run the current .jdl file

From the terminal, pass the **path** to the file. If the file is in a subfolder, include the folder:

```powershell
jade .zenflow\workflows\hi.jdl
```

Or use the full path: `jade C:\Users\...\your\folder\.zenflow\workflows\hi.jdl`.

To run whatever `.jdl` file is currently open: **Terminal → Run Task → Run current Jade file** (after adding the task below). In your project folder create `.vscode/tasks.json`:

```json
{
  "version": "2.0.0",
  "tasks": [
    {
      "label": "Run current Jade file",
      "type": "shell",
      "command": "jade",
      "args": ["${file}"],
      "group": { "kind": "test", "isDefault": true },
      "presentation": { "reveal": "always", "panel": "shared" }
    }
  ]
}
```

Then **Ctrl+Shift+B** (or Terminal → Run Task) runs the active file with the correct path.

### Run from buffer (no save) and debounced autosave

The **Jade Language** extension (same folder you install for syntax) adds:

- **Jade: Run from buffer (no save)** — Runs the current editor content through `jade -` without saving. **Ctrl+Shift+B** (Cmd+Shift+B on Mac) when a `.jdl` file is open. Output appears in the **Jade** output channel (View → Output → Jade).
- **Debounced autosave** — For open `.jdl` files, the extension saves the document **300 ms** after you stop typing. No need to hit Ctrl+S before running.

Make sure `jade` is on your PATH. You can also use the task "Run current Jade file" to run the file from disk (last saved version).

<details>
<summary>Earlier note: autosave and stdin</summary>

Autosave is an **editor** feature. The Jade repo also sets a short autosave delay (500 ms) in `.vscode/settings.json` so files save soon after you stop typing—no need to hit Ctrl+S before running. You can use the same `files.autoSave: "afterDelay"` and `files.autoSaveDelay: 500` in your own workspace or user settings.

To run code without saving at all, the **editor** must send the current buffer to Jade. The language supports that via **stdin**: run `jade -` and pipe source in. From the shell you can do `Get-Content file.jdl -Raw | jade -` to run the file from disk.

</details>

### Extension bundled with the language

The Jade **Windows installer** and **Linux/macOS install scripts** install the VS Code/Cursor extension automatically:

- **Windows:** During setup, leave **"Install Jade extension for VS Code and Cursor"** checked (default). The extension is copied into your `%USERPROFILE%\.vscode\extensions` and `%USERPROFILE%\.cursor\extensions` so both IDEs get syntax, run-from-buffer, and debounced autosave.
- **Linux / macOS:** Run `./install.sh` from `jade-lang/installers/linux` or `jade-lang/installers/macos`; the script copies the extension into `~/.vscode/extensions` and `~/.cursor/extensions`.

Restart VS Code or Cursor after installing Jade so they pick up the extension.

### Manual install (syntax highlighting)

If you didn’t use the installer or want to install the extension by hand:

1. Open **VS Code** or **Cursor**.
2. **Ctrl+Shift+P** (Cmd+Shift+P on Mac) → **Developer: Install Extension from Location...**
3. Select the **`jade-lang/installers/ide/vscode-snippet`** folder (or `C:\Program Files\Jade 1.0.0\ide\vscode` after a Windows install).
4. Click **Reload** when prompted.

After reload, every `.jdl` file (in any folder, e.g. `Desktop\test`) will use the Jade language: correct icon, syntax highlighting for `out`, strings, numbers, keywords, and language config (e.g. `//` and `#` comments, bracket matching).

**If you don’t have the repo open:** Install from the full path to that folder, e.g.  
`C:\Users\macfa\Desktop\Development\Jade\jade-lang\installers\ide\vscode-snippet`.

## JetBrains (IDEA, PyCharm, etc.)

1. **Settings → Editor → File Types**
2. Click **+** to add a new type, name it **Jade**
3. Add pattern: `*.jdl`
4. To use the Jade icon: in the new type, set the icon to **`jade-lang/installers/windows/icon/jade.png`**.
5. Optionally set syntax highlighting to a similar language (e.g. Kotlin or plain text) until a Jade plugin exists.

## MIME / OS

- **Linux:** Install the MIME type from `../linux/jade.xml` (see `../mime/README.md`).
- **Windows:** The Windows installer associates `.jdl` with `jade.exe`.
- **macOS:** The macOS install script registers a UTI; use “Open With” to run with `jade`.

All of this works with the **same binary**: interpreted (`jade file.jdl`), AOT (`jade build file.jdl`), and optional JIT when built with `--features jit`.
