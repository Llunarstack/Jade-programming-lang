# Jade documentation site

Static documentation site for the Jade programming language. All content is derived from the interpreter (lexer, parser, eval, builtins). Code examples go from **basic to advanced**.

## Structure

- **index.html** — Homepage with Jade logo, hero gradient, features, and code showcase (basic → advanced)
- **assets/jade.png** — Jade logo (copy from `installers/windows/icon/jade.png` if missing)
- **docs/** — Documentation pages
  - **getting-started.html** — Install, run, REPL, check, build, first program
  - **syntax.html** — Functions, variables, operators, literals, keywords
  - **control-flow.html** — if, match, cond, when, unless, either, switch, for, while, try/catch, defer, guard
  - **types.html** — Primitives, collections, ranges, type names, indexing
  - **functions.html** — Declarations, typed/untyped params, lambdas, decorators
  - **reference.html** — **Single-page language reference**: Values (every type + examples, escape sequences), Output (out, colors, tables, progress, gradient), Code examples (basic → advanced), then CLI, grammar, operators, keywords
  - **builtins.html** — Built-in reference (core, math, stats, string, io, algo, dsa, bits, uf, trie, random, crypto, regex)
  - **values.html**, **output.html**, **code-examples.html** — Redirect to reference.html#values, #output, #code-examples

## GitHub Pages

The repo uses the **Deploy to GitHub Pages** workflow (`.github/workflows/pages.yml`). To turn on the site:

1. Push the workflow and this folder to your repo.
2. In the repo: **Settings → Pages**.
3. Under **Build and deployment**, set **Source** to **GitHub Actions**.
4. After the next push to `main`, the site will be at `https://<org>.github.io/Jade-programming-lang/`.

Installer download links in the docs point to `../../../dist/installers/...` (local `dist`). On GitHub Pages there is no `dist`, so either use **Releases** for downloads or add a step in the workflow to attach built installers.

## Viewing locally

Open `index.html` in a browser, or serve the folder:

```bash
cd docs-site
python -m http.server 8000
```

Then visit http://localhost:8000

## Design

Premium dark theme: jade-green accent and gradients, DM Sans + JetBrains Mono, sticky header with logo, hero with gradient background and logo, tier labels (Basic / Intermediate / Advanced) for code, glass-style cards, consistent code blocks and sidebar nav across all docs.
