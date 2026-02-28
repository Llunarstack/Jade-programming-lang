"use strict";
const vscode = require("vscode");
const { spawn } = require("child_process");

const AUTOSAVE_DEBOUNCE_MS = 300;
let autosaveTimeouts = new Map();

function activate(context) {
  // Run from buffer: pipe current document to `jade -` (no save needed)
  context.subscriptions.push(
    vscode.commands.registerCommand("jade.runFromBuffer", async () => {
      const editor = vscode.window.activeTextEditor;
      if (!editor || editor.document.languageId !== "jade") {
        vscode.window.showWarningMessage("Open a .jdl file first.");
        return;
      }
      const source = editor.document.getText();
      const cwd = editor.document.uri.scheme === "file"
        ? vscode.workspace.getWorkspaceFolder(editor.document.uri)?.uri.fsPath || require("path").dirname(editor.document.uri.fsPath)
        : undefined;

      const channel = vscode.window.createOutputChannel("Jade");
      channel.clear();
      channel.show(true);

      const jade = spawn("jade", ["-"], {
        cwd: cwd || process.cwd(),
        stdio: ["pipe", "pipe", "pipe"],
        shell: process.platform === "win32",
      });

      let out = "";
      let err = "";
      jade.stdout.on("data", (d) => { out += d; });
      jade.stderr.on("data", (d) => { err += d; });
      jade.on("close", (code) => {
        if (out) channel.append(out);
        if (err) channel.append(err);
        if (code !== 0 && !err) channel.append(`Exit code: ${code}\n`);
      });

      jade.stdin.write(source, "utf8", () => {
        jade.stdin.end();
      });
    })
  );

  // Debounced autosave for .jdl: save 300ms after user stops typing
  context.subscriptions.push(
    vscode.workspace.onDidChangeTextDocument((e) => {
      if (e.document.languageId !== "jade" || e.document.uri.scheme !== "file") return;
      if (e.document.isUntitled) return;

      const key = e.document.uri.toString();
      if (autosaveTimeouts.has(key)) clearTimeout(autosaveTimeouts.get(key));

      const timeout = setTimeout(() => {
        autosaveTimeouts.delete(key);
        if (e.document.isDirty) {
          e.document.save().then(
            () => {},
            () => {}
          );
        }
      }, AUTOSAVE_DEBOUNCE_MS);
      autosaveTimeouts.set(key, timeout);
    })
  );
}

function deactivate() {
  for (const t of autosaveTimeouts.values()) clearTimeout(t);
  autosaveTimeouts.clear();
}

module.exports = { activate, deactivate };
