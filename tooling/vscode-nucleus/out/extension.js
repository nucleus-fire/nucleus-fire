"use strict";
var __createBinding = (this && this.__createBinding) || (Object.create ? (function(o, m, k, k2) {
    if (k2 === undefined) k2 = k;
    var desc = Object.getOwnPropertyDescriptor(m, k);
    if (!desc || ("get" in desc ? !m.__esModule : desc.writable || desc.configurable)) {
      desc = { enumerable: true, get: function() { return m[k]; } };
    }
    Object.defineProperty(o, k2, desc);
}) : (function(o, m, k, k2) {
    if (k2 === undefined) k2 = k;
    o[k2] = m[k];
}));
var __setModuleDefault = (this && this.__setModuleDefault) || (Object.create ? (function(o, v) {
    Object.defineProperty(o, "default", { enumerable: true, value: v });
}) : function(o, v) {
    o["default"] = v;
});
var __importStar = (this && this.__importStar) || (function () {
    var ownKeys = function(o) {
        ownKeys = Object.getOwnPropertyNames || function (o) {
            var ar = [];
            for (var k in o) if (Object.prototype.hasOwnProperty.call(o, k)) ar[ar.length] = k;
            return ar;
        };
        return ownKeys(o);
    };
    return function (mod) {
        if (mod && mod.__esModule) return mod;
        var result = {};
        if (mod != null) for (var k = ownKeys(mod), i = 0; i < k.length; i++) if (k[i] !== "default") __createBinding(result, mod, k[i]);
        __setModuleDefault(result, mod);
        return result;
    };
})();
Object.defineProperty(exports, "__esModule", { value: true });
exports.activate = activate;
exports.deactivate = deactivate;
const vscode = __importStar(require("vscode"));
const node_1 = require("vscode-languageclient/node");
const commands_1 = require("./commands");
let client;
async function activate(context) {
    console.log('Nucleus Framework extension is activating...');
    // Register all commands
    (0, commands_1.registerCommands)(context);
    // Start the LSP client if enabled
    const config = vscode.workspace.getConfiguration('nucleus');
    const lspEnabled = config.get('lsp.enabled', true);
    if (lspEnabled) {
        await startLanguageClient(context);
    }
    // Show welcome message on first activation
    const hasShownWelcome = context.globalState.get('nucleus.welcomeShown');
    if (!hasShownWelcome) {
        vscode.window.showInformationMessage('⚡ Nucleus Framework extension activated! Use snippets like "nview", "nfor", "ncomponent" for rapid development.', 'View Docs').then(selection => {
            if (selection === 'View Docs') {
                vscode.env.openExternal(vscode.Uri.parse('https://github.com/nucleus-lang/nucleus-lang/tree/main/docs'));
            }
        });
        context.globalState.update('nucleus.welcomeShown', true);
    }
    // Register restart LSP command
    context.subscriptions.push(vscode.commands.registerCommand('nucleus.restartLsp', async () => {
        if (client) {
            await client.stop();
        }
        await startLanguageClient(context);
        vscode.window.showInformationMessage('Nucleus Language Server restarted.');
    }));
    console.log('Nucleus Framework extension activated successfully.');
}
async function startLanguageClient(context) {
    const config = vscode.workspace.getConfiguration('nucleus');
    const lspPath = config.get('lsp.path', 'nucleus-lsp');
    // Define the server options
    const serverOptions = {
        command: lspPath,
        args: [],
        options: {
            env: {
                ...process.env,
                RUST_LOG: 'info'
            }
        }
    };
    // Define the client options
    const clientOptions = {
        documentSelector: [
            { scheme: 'file', language: 'ncl' }
        ],
        synchronize: {
            fileEvents: vscode.workspace.createFileSystemWatcher('**/*.ncl')
        },
        outputChannelName: 'Nucleus Language Server',
        traceOutputChannel: vscode.window.createOutputChannel('Nucleus LSP Trace')
    };
    // Create and start the client
    client = new node_1.LanguageClient('nucleus-lsp', 'Nucleus Language Server', serverOptions, clientOptions);
    try {
        await client.start();
        console.log('Nucleus Language Server started successfully.');
    }
    catch (error) {
        console.error('Failed to start Nucleus Language Server:', error);
        vscode.window.showWarningMessage(`Could not start Nucleus Language Server. Make sure 'nucleus-lsp' is installed and available in PATH. Error: ${error}`);
    }
    context.subscriptions.push(client);
}
async function deactivate() {
    if (client) {
        await client.stop();
    }
}
//# sourceMappingURL=extension.js.map