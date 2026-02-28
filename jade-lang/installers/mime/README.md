# MIME and IDE recognition for Jade

Use these so **IDEs, file managers, and the OS** recognize `.jdl` as the Jade programming language.

## MIME type

- **Linux / freedesktop:** Use `linux/jade.xml` (or the same content). Install to `~/.local/share/mime/packages/jade.xml` or `/usr/share/mime/packages/jade.xml`, then run `update-mime-database`.
- **macOS:** The macOS installer registers a UTI; MIME is less critical.
- **Windows:** File association is done by the Windows installer (`.jdl` → jade.exe).

Standard identifiers:

- **MIME:** `text/x-jade` (primary), `application/x-jade` (alias)
- **macOS UTI:** `org.jade-lang.source` (conforms to `public.plain-text`, `public.source-code`)

## IDEs

- **VS Code:** Use the Jade VS Code extension (language id `jade` or `jade-lang`). Ensure the extension declares `"extensions": [".jdl"]` and `"configuration": "./language-configuration.json"` so the editor treats `.jdl` as Jade.
- **JetBrains (IDEA, etc.):** Add a file type association: **Settings → Editor → File Types → Add** “Jade” with pattern `*.jdl`. Attach a text/script highlighter or a custom plugin if you have one.
- **Other editors:** Many accept TextMate-style grammars or a simple “extension → language” map; point `.jdl` to a Jade grammar or “plain text” until a grammar exists.

## Compiled / JIT / Interpreted

The **same binary** supports:

- **Interpreted:** `jade file.jdl` or `jade run file.jdl`
- **AOT compiled:** `jade build file.jdl -o myapp`
- **JIT:** Build with `--features jit` if your version exposes a JIT mode

Installers ship this single binary; no separate “compiler” or “interpreter” install needed.
