import * as vscode from "vscode";
import * as path from "path";
import * as cp from "child_process";

function findCli(): string {
  const cfg = vscode.workspace.getConfiguration("atlas");
  const custom = cfg.get<string>("cliPath");
  if (custom) return custom;
  return "atlas"; // assume on PATH, or in workspace
}

function runAtlas(args: string[]): Promise<string> {
  return new Promise((resolve, reject) => {
    const cli = findCli();
    const cwd = vscode.workspace.workspaceFolders?.[0]?.uri?.fsPath || process.cwd();
    const child = cp.spawn(cli, args, { cwd, shell: false });
    let stdout = "";
    let stderr = "";
    child.stdout.on("data", (d) => (stdout += d.toString()));
    child.stderr.on("data", (d) => (stderr += d.toString()));
    child.on("close", (code) => {
      if (code === 0) resolve(stdout);
      else reject(new Error(stderr || `Exit code ${code}`));
    });
    child.on("error", reject);
  });
}

class AtlasExplorerPanel {
  public static currentPanel: AtlasExplorerPanel | undefined;
  private readonly _panel: vscode.WebviewPanel;
  private _disposables: vscode.Disposable[] = [];

  private constructor(panel: vscode.WebviewPanel, extensionUri: vscode.Uri) {
    this._panel = panel;
    this._panel.webview.html = this._getHtml();
    this._panel.onDidDispose(() => this.dispose(), null, this._disposables);
    this._panel.webview.onDidReceiveMessage(
      async (msg) => {
        switch (msg.command) {
          case "solve":
            try {
              const result = await runAtlas([
                "solve",
                "--bundle",
                msg.bundle || "knowledge.atlas",
                msg.query,
                "--json",
              ]);
              this._panel.webview.postMessage({
                command: "solveResult",
                data: JSON.parse(result),
              });
            } catch (e: any) {
              this._panel.webview.postMessage({
                command: "error",
                message: e.message,
              });
            }
            return;
          case "dump":
            try {
              const result = await runAtlas([
                "dump",
                "--bundle",
                msg.bundle || "knowledge.atlas",
              ]);
              this._panel.webview.postMessage({
                command: "dumpResult",
                data: JSON.parse(result),
              });
            } catch (e: any) {
              this._panel.webview.postMessage({
                command: "error",
                message: e.message,
              });
            }
            return;
        }
      },
      null,
      this._disposables
    );
  }

  public static createOrShow(extensionUri: vscode.Uri) {
    const column = vscode.window.activeTextEditor?.viewColumn || vscode.ViewColumn.One;
    if (AtlasExplorerPanel.currentPanel) {
      AtlasExplorerPanel.currentPanel._panel.reveal(column);
      return;
    }
    const panel = vscode.window.createWebviewPanel(
      "atlasExplorer",
      "Atlas Knowledge Graph",
      column,
      { enableScripts: true, retainContextWhenHidden: true }
    );
    AtlasExplorerPanel.currentPanel = new AtlasExplorerPanel(panel, extensionUri);
  }

  public dispose() {
    AtlasExplorerPanel.currentPanel = undefined;
    this._panel.dispose();
    while (this._disposables.length) {
      const d = this._disposables.pop();
      if (d) d.dispose();
    }
  }

  private _getHtml(): string {
    return `<!DOCTYPE html>
<html lang="en">
<head>
  <meta charset="UTF-8">
  <meta name="viewport" content="width=device-width, initial-scale=1.0">
  <title>Atlas Knowledge Graph</title>
  <style>
    * { margin: 0; padding: 0; box-sizing: border-box; }
    body { font-family: -apple-system, BlinkMacSystemFont, 'Segoe UI', Roboto, sans-serif; background: #0d1117; color: #c9d1d9; }
    .toolbar { display: flex; gap: 8px; padding: 12px 16px; background: #161b22; border-bottom: 1px solid #30363d; align-items: center; }
    .toolbar input { flex: 1; padding: 8px 12px; border: 1px solid #30363d; border-radius: 6px; background: #0d1117; color: #c9d1d9; font-size: 14px; }
    .toolbar input:focus { outline: none; border-color: #58a6ff; }
    .toolbar button { padding: 8px 16px; border: none; border-radius: 6px; cursor: pointer; font-size: 14px; font-weight: 500; }
    .btn-primary { background: #238636; color: #fff; }
    .btn-primary:hover { background: #2ea043; }
    .btn-secondary { background: #21262d; color: #c9d1d9; border: 1px solid #30363d; }
    .btn-secondary:hover { background: #30363d; }
    .content { padding: 16px; overflow-y: auto; height: calc(100vh - 56px); }
    .stats { display: flex; gap: 24px; margin-bottom: 16px; flex-wrap: wrap; }
    .stat-card { background: #161b22; border: 1px solid #30363d; border-radius: 8px; padding: 16px 24px; flex: 1; min-width: 140px; }
    .stat-card .value { font-size: 28px; font-weight: 600; color: #58a6ff; }
    .stat-card .label { font-size: 12px; color: #8b949e; margin-top: 4px; text-transform: uppercase; letter-spacing: 0.5px; }
    .graph-container { background: #161b22; border: 1px solid #30363d; border-radius: 8px; padding: 16px; min-height: 300px; }
    .node-list { display: grid; gap: 8px; margin-top: 16px; }
    .node-card { background: #161b22; border: 1px solid #30363d; border-radius: 6px; padding: 12px 16px; cursor: pointer; transition: border-color 0.15s; }
    .node-card:hover { border-color: #58a6ff; }
    .node-card .name { font-weight: 500; color: #c9d1d9; }
    .node-card .kind { font-size: 11px; color: #8b949e; text-transform: uppercase; letter-spacing: 0.5px; }
    .node-card .desc { font-size: 13px; color: #8b949e; margin-top: 4px; }
    .loading { text-align: center; padding: 40px; color: #8b949e; }
    .loading::after { content: "..."; animation: dots 1.5s infinite; }
    @keyframes dots { 0%,20% { content: "."; } 40% { content: ".."; } 60%,100% { content: "..."; } }
    .error { color: #f85149; padding: 12px; background: #2d1517; border: 1px solid #f85149; border-radius: 6px; margin: 8px 0; }
    kbd { background: #21262d; border: 1px solid #30363d; border-radius: 3px; padding: 1px 4px; font-size: 11px; }
  </style>
</head>
<body>
  <div class="toolbar">
    <input id="queryInput" type="text" placeholder='Ask Atlas: e.g., "stateful widget", "error handling rust"...' onkeydown="if(event.key==='Enter') solve()" />
    <button class="btn-primary" onclick="solve()">Solve</button>
    <button class="btn-secondary" onclick="dumpBundle()">Load Graph</button>
    <span style="font-size:12px;color:#8b949e"><kbd>Ctrl+Shift+P</kbd> Atlas commands</span>
  </div>
  <div class="content">
    <div id="stats" class="stats"></div>
    <div id="graph" class="graph-container">
      <div class="loading" id="loading">Loading Atlas bundle</div>
    </div>
    <div id="results" class="node-list"></div>
  </div>

  <script>
    const vscode = acquireVsCodeApi();
    let bundleData = null;

    window.addEventListener('message', event => {
      const msg = event.data;
      if (msg.command === 'solveResult') renderSolve(msg.data);
      else if (msg.command === 'dumpResult') renderDump(msg.data);
      else if (msg.command === 'error') renderError(msg.message);
    });

    function solve() {
      const q = document.getElementById('queryInput').value.trim();
      if (!q) return;
      document.getElementById('results').innerHTML = '<div class="loading">Thinking</div>';
      vscode.postMessage({ command: 'solve', query: q, bundle: bundleData?.bundle || '' });
    }

    function dumpBundle() {
      document.getElementById('graph').innerHTML = '<div class="loading">Loading knowledge graph</div>';
      vscode.postMessage({ command: 'dump', bundle: '' });
    }

    function renderSolve(data) {
      const results = document.getElementById('results');
      if (!data || !data.nodes || data.nodes.length === 0) {
        results.innerHTML = '<div class="node-card" style="color:#8b949e">No matching nodes found.</div>';
        return;
      }
      const kindColors = { Package: '#d29922', Concept: '#58a6ff', Api: '#3fb950', Workflow: '#bc8cff', Example: '#f0883e' };
      results.innerHTML = '<div style="margin-bottom:8px;color:#8b949e">Found ' + data.total_matches + ' matches (confidence: ' + (data.confidence * 100).toFixed(0) + '%)</div>' +
        data.nodes.map(n => '<div class="node-card" onclick="showDetails(\'' + n.id + '\')">' +
          '<div class="kind">' + (n.kind || '') + '</div>' +
          '<div class="name">' + n.name + '</div>' +
          (n.description ? '<div class="desc">' + n.description.slice(0, 120) + '</div>' : '') +
          (n.version ? '<div class="desc">v' + n.version + '</div>' : '') +
        '</div>').join('');
    }

    function renderDump(data) {
      bundleData = data;
      if (!data) return;
      document.getElementById('stats').innerHTML =
        '<div class="stat-card"><div class="value">' + (data.nodes?.length || 0) + '</div><div class="label">Nodes</div></div>' +
        '<div class="stat-card"><div class="value">' + (data.edges?.length || 0) + '</div><div class="label">Edges</div></div>' +
        '<div class="stat-card"><div class="value">' + (data.decision_trees?.length || 0) + '</div><div class="label">Decision Trees</div></div>';

      const kinds = {};
      for (const n of data.nodes || []) {
        const k = n.kind || 'Unknown';
        kinds[k] = (kinds[k] || 0) + 1;
      }
      document.getElementById('graph').innerHTML =
        '<div style="margin-bottom:12px;font-size:14px;font-weight:500">Knowledge Graph: ' + (data.name || 'bundle') + '</div>' +
        '<div style="display:flex;gap:8px;flex-wrap:wrap;margin-bottom:16px">' +
        Object.entries(kinds).map(([k, v]) =>
          '<span style="background:#21262d;padding:4px 10px;border-radius:12px;font-size:12px">' + k + ': ' + v + '</span>'
        ).join('') +
        '</div>' +
        (data.decision_trees?.length > 0 ? '<div style="margin-top:12px"><strong>Decision Trees:</strong><ul>' +
          data.decision_trees.map(t => '<li style="margin:4px 0;font-size:13px">' + t.id + ' (' + (t.trigger?.tags?.join(', ') || 'no tags') + ')</li>').join('') +
        '</ul></div>' : '') +
        '<div style="margin-top:12px;font-size:13px;color:#8b949e">' +
        'Nodes breakdown: ' + data.nodes?.slice(0, 30).map(n => '<span style="color:' + (kindColors[n.kind] || '#8b949e') + '">' + n.name + '</span>').join(', ') +
        (data.nodes?.length > 30 ? '...' : '') +
        '</div>';
    }

    function renderError(msg) {
      document.getElementById('results').innerHTML = '<div class="error">' + msg + '</div>';
    }

    function showDetails(id) {
      vscode.postMessage({ command: 'solve', query: id });
    }

    dumpBundle();
  </script>
</body>
</html>`;
  }
}

export function activate(context: vscode.ExtensionContext) {
  context.subscriptions.push(
    vscode.commands.registerCommand("atlas.explore", () => {
      AtlasExplorerPanel.createOrShow(context.extensionUri);
    })
  );

  context.subscriptions.push(
    vscode.commands.registerCommand("atlas.search", async () => {
      const query = await vscode.window.showInputBox({
        prompt: "Search knowledge packages",
        placeHolder: "e.g., flutter, rust, typescript",
      });
      if (!query) return;
      try {
        const result = await runAtlas(["search", query, "--json"]);
        const data = JSON.parse(result);
        const packages = data.results || data.packages || [];
        if (packages.length > 0) {
          const items = packages.map((p: any) => ({
            label: p.name,
            description: p.description,
            detail: `v${p.version} | ${p.download_count || 0} downloads`,
          }));
          vscode.window.showQuickPick(items, { placeHolder: "Search results" });
        } else {
          vscode.window.showInformationMessage("No packages found.");
        }
      } catch (e: any) {
        vscode.window.showErrorMessage(`Search failed: ${e.message}`);
      }
    })
  );

  context.subscriptions.push(
    vscode.commands.registerCommand("atlas.compile", async (uri?: vscode.Uri) => {
      const fileUri = uri || vscode.window.activeTextEditor?.document.uri;
      if (!fileUri) {
        vscode.window.showErrorMessage("Open a .md file to compile");
        return;
      }
      const filePath = fileUri.fsPath;
      const ext = path.extname(filePath);
      if (ext !== ".md" && ext !== ".yaml") {
        vscode.window.showErrorMessage("Atlas compiles .md and .yaml files");
        return;
      }
      try {
        const outPath = filePath.replace(ext, ".atlas");
        const result = await runAtlas([
          "compile",
          filePath,
          "--out", outPath,
          "--json",
        ]);
        const data = JSON.parse(result);
        vscode.window.showInformationMessage(
          `Compiled ${data.nodes} nodes, ${data.edges} edges → ${data.output}`
        );
      } catch (e: any) {
        vscode.window.showErrorMessage(`Compile failed: ${e.message}`);
      }
    })
  );

  context.subscriptions.push(
    vscode.commands.registerCommand("atlas.solve", async () => {
      const query = await vscode.window.showInputBox({
        prompt: "Ask Atlas a question",
        placeHolder: "e.g., How do I use a StatefulWidget in Flutter?",
      });
      if (!query) return;
      try {
        const bundlePath = vscode.workspace.getConfiguration("atlas").get("bundlePath", "knowledge.atlas");
        const result = await runAtlas(["solve", "--bundle", bundlePath, query, "--json"]);
        const data = JSON.parse(result);
        if (data.nodes && data.nodes.length > 0) {
          const items = data.nodes.map((n: any) => ({
            label: n.name,
            description: n.kind,
            detail: n.description?.slice(0, 100) || "",
          }));
          const picked = await vscode.window.showQuickPick(items, {
            placeHolder: `${data.total_matches} results (${(data.confidence * 100).toFixed(0)}% confidence)`,
          });
          if (picked) {
            vscode.window.showInformationMessage(picked.detail || picked.label);
          }
        } else {
          vscode.window.showInformationMessage("No matching knowledge found.");
        }
      } catch (e: any) {
        vscode.window.showErrorMessage(`Solve failed: ${e.message}`);
      }
    })
  );

  context.subscriptions.push(
    vscode.commands.registerCommand("atlas.decide", async () => {
      const query = await vscode.window.showInputBox({
        prompt: "Decision tree query",
        placeHolder: "e.g., widget, state management",
      });
      if (!query) return;
      const ctx = await vscode.window.showInputBox({
        prompt: "Context (key=value, space separated)",
        placeHolder: "e.g., answer=true framework=flutter",
      });
      const ctxArgs = ctx ? ctx.split(" ").flatMap((s) => ["-c", s]) : [];
      try {
        const bundlePath = vscode.workspace.getConfiguration("atlas").get("bundlePath", "knowledge.atlas");
        const result = await runAtlas([
          "decide",
          "--bundle",
          bundlePath,
          query,
          ...ctxArgs,
          "--json",
        ]);
        if (result.trim() === "null") {
          vscode.window.showInformationMessage("No matching decision tree found.");
          return;
        }
        const data = JSON.parse(result);
        vscode.window.showInformationMessage(
          `Decision: ${data.rationale}\n\nPath: ${data.path.join(" → ")}`
        );
      } catch (e: any) {
        vscode.window.showErrorMessage(`Decide failed: ${e.message}`);
      }
    })
  );

  context.subscriptions.push(
    vscode.commands.registerCommand("atlas.init", async () => {
      const name = await vscode.window.showInputBox({
        prompt: "Knowledge package name",
        placeHolder: "e.g., my-project-patterns",
      });
      if (!name) return;
      const templates = ["generic", "flutter", "rust", "typescript", "python"];
      const template = await vscode.window.showQuickPick(templates, {
        placeHolder: "Select a template",
      });
      if (!template) return;
      try {
        const result = await runAtlas(["init", name, "--template", template]);
        vscode.window.showInformationMessage(result);
      } catch (e: any) {
        vscode.window.showErrorMessage(`Init failed: ${e.message}`);
      }
    })
  );

  context.subscriptions.push(
    vscode.languages.registerHoverProvider("markdown", {
      provideHover(document, position) {
        const range = document.getWordRangeAtPosition(position, /concept:\w+|\w+:\w+/);
        if (!range) return;
        const word = document.getText(range);
        const hoverText = new vscode.MarkdownString();
        hoverText.appendMarkdown(`**Atlas Knowledge**\n\n`);
        hoverText.appendMarkdown(`Look up \`${word}\` in the knowledge graph?\n\n`);
        hoverText.appendMarkdown(`[Open in Atlas Explorer](command:atlas.solve?${encodeURIComponent(JSON.stringify([word]))})`);
        return new vscode.Hover(hoverText);
      },
    })
  );

  vscode.window.showInformationMessage("Atlas Knowledge Graph is ready. Press Ctrl+Alt+A to explore.");
}

export function deactivate() {}
