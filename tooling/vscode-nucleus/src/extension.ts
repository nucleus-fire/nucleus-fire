import * as vscode from 'vscode';
import { LanguageClient, LanguageClientOptions, ServerOptions } from 'vscode-languageclient/node';
import { registerCommands } from './commands';

let client: LanguageClient | undefined;

export async function activate(context: vscode.ExtensionContext) {
    console.log('Nucleus Framework extension is activating...');

    // Register all commands
    registerCommands(context);

    // Start the LSP client if enabled
    const config = vscode.workspace.getConfiguration('nucleus');
    const lspEnabled = config.get<boolean>('lsp.enabled', true);
    
    if (lspEnabled) {
        await startLanguageClient(context);
    }

    // Show welcome message on first activation
    const hasShownWelcome = context.globalState.get('nucleus.welcomeShown');
    if (!hasShownWelcome) {
        vscode.window.showInformationMessage(
            '⚡ Nucleus Framework extension activated! Use snippets like "nview", "nfor", "ncomponent" for rapid development.',
            'View Docs'
        ).then(selection => {
            if (selection === 'View Docs') {
                vscode.env.openExternal(vscode.Uri.parse('https://github.com/nucleus-lang/nucleus-lang/tree/main/docs'));
            }
        });
        context.globalState.update('nucleus.welcomeShown', true);
    }

    // Register restart LSP command
    context.subscriptions.push(
        vscode.commands.registerCommand('nucleus.restartLsp', async () => {
            if (client) {
                await client.stop();
            }
            await startLanguageClient(context);
            vscode.window.showInformationMessage('Nucleus Language Server restarted.');
        })
    );

    console.log('Nucleus Framework extension activated successfully.');
}

async function startLanguageClient(context: vscode.ExtensionContext) {
    const config = vscode.workspace.getConfiguration('nucleus');
    const lspPath = config.get<string>('lsp.path', 'nucleus-lsp');

    // Define the server options
    const serverOptions: ServerOptions = {
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
    const clientOptions: LanguageClientOptions = {
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
    client = new LanguageClient(
        'nucleus-lsp',
        'Nucleus Language Server',
        serverOptions,
        clientOptions
    );

    try {
        await client.start();
        console.log('Nucleus Language Server started successfully.');
    } catch (error) {
        console.error('Failed to start Nucleus Language Server:', error);
        vscode.window.showWarningMessage(
            `Could not start Nucleus Language Server. Make sure 'nucleus-lsp' is installed and available in PATH. Error: ${error}`
        );
    }

    context.subscriptions.push(client);
}

export async function deactivate(): Promise<void> {
    if (client) {
        await client.stop();
    }
}
