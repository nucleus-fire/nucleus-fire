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
exports.registerCommands = registerCommands;
const vscode = __importStar(require("vscode"));
function registerCommands(context) {
    const config = vscode.workspace.getConfiguration('nucleus');
    const cliPath = config.get('cli.path', 'nucleus');
    // Helper to run nucleus commands in terminal
    function runInTerminal(command, name = 'Nucleus') {
        const terminal = vscode.window.createTerminal({
            name,
            cwd: vscode.workspace.workspaceFolders?.[0]?.uri.fsPath
        });
        terminal.show();
        terminal.sendText(command);
        return terminal;
    }
    // Helper to prompt for input
    async function promptInput(prompt, placeholder) {
        return vscode.window.showInputBox({
            prompt,
            placeHolder: placeholder
        });
    }
    // nucleus dev - Start development server
    context.subscriptions.push(vscode.commands.registerCommand('nucleus.dev', () => {
        runInTerminal(`${cliPath} dev`, 'Nucleus Dev Server');
        vscode.window.showInformationMessage('⚡ Nucleus dev server starting...');
    }));
    // nucleus run - Run server
    context.subscriptions.push(vscode.commands.registerCommand('nucleus.run', async () => {
        const port = config.get('dev.port', 3000);
        runInTerminal(`${cliPath} run --port ${port}`, 'Nucleus Server');
    }));
    // nucleus build - Build for production
    context.subscriptions.push(vscode.commands.registerCommand('nucleus.build', () => {
        runInTerminal(`${cliPath} build`, 'Nucleus Build');
        vscode.window.showInformationMessage('📦 Building for production...');
    }));
    // nucleus new - Create new project
    context.subscriptions.push(vscode.commands.registerCommand('nucleus.new', async () => {
        const projectName = await promptInput('Enter project name', 'my-nucleus-app');
        if (!projectName)
            return;
        const template = await vscode.window.showQuickPick([
            { label: 'default', description: 'Full-featured template with views and examples' },
            { label: 'api', description: 'API-only template without views' },
            { label: 'minimal', description: 'Minimal template with no examples' }
        ], { placeHolder: 'Select project template' });
        const command = template
            ? `${cliPath} new ${projectName} --template ${template.label}`
            : `${cliPath} new ${projectName}`;
        runInTerminal(command, 'Nucleus New Project');
    }));
    // nucleus test - Run tests
    context.subscriptions.push(vscode.commands.registerCommand('nucleus.test', () => {
        runInTerminal(`${cliPath} test`, 'Nucleus Tests');
    }));
    // nucleus generate scaffold
    context.subscriptions.push(vscode.commands.registerCommand('nucleus.generate.scaffold', async () => {
        const name = await promptInput('Enter model name (e.g., Post, User)', 'Post');
        if (!name)
            return;
        const fields = await promptInput('Enter fields (e.g., title:string body:text views:int)', 'name:string description:text');
        const command = fields
            ? `${cliPath} generate scaffold ${name} ${fields}`
            : `${cliPath} generate scaffold ${name}`;
        runInTerminal(command, 'Nucleus Generate');
        vscode.window.showInformationMessage(`Generating scaffold for ${name}...`);
    }));
    // nucleus generate model
    context.subscriptions.push(vscode.commands.registerCommand('nucleus.generate.model', async () => {
        const name = await promptInput('Enter model name', 'User');
        if (!name)
            return;
        const fields = await promptInput('Enter fields (e.g., name:string email:string)', 'name:string email:string');
        const command = fields
            ? `${cliPath} generate model ${name} ${fields}`
            : `${cliPath} generate model ${name}`;
        runInTerminal(command, 'Nucleus Generate');
    }));
    // nucleus generate migration
    context.subscriptions.push(vscode.commands.registerCommand('nucleus.generate.migration', async () => {
        const name = await promptInput('Enter migration name', 'add_column_to_users');
        if (!name)
            return;
        runInTerminal(`${cliPath} db new ${name}`, 'Nucleus Migration');
    }));
    // nucleus db new - Create migration
    context.subscriptions.push(vscode.commands.registerCommand('nucleus.db.new', async () => {
        const name = await promptInput('Enter migration name', 'create_users');
        if (!name)
            return;
        runInTerminal(`${cliPath} db new ${name}`, 'Nucleus DB');
        vscode.window.showInformationMessage(`Creating migration: ${name}`);
    }));
    // nucleus db up - Run migrations
    context.subscriptions.push(vscode.commands.registerCommand('nucleus.db.up', () => {
        runInTerminal(`${cliPath} db up`, 'Nucleus DB');
        vscode.window.showInformationMessage('Running migrations...');
    }));
    // nucleus db down - Rollback
    context.subscriptions.push(vscode.commands.registerCommand('nucleus.db.down', async () => {
        const confirm = await vscode.window.showWarningMessage('This will rollback the last migration. Continue?', 'Yes', 'No');
        if (confirm !== 'Yes')
            return;
        runInTerminal(`${cliPath} db down`, 'Nucleus DB');
    }));
    // nucleus db status
    context.subscriptions.push(vscode.commands.registerCommand('nucleus.db.status', () => {
        runInTerminal(`${cliPath} db status`, 'Nucleus DB');
    }));
    // nucleus deploy - Interactive deployment
    context.subscriptions.push(vscode.commands.registerCommand('nucleus.deploy', async () => {
        const target = await vscode.window.showQuickPick([
            { label: 'interactive', description: 'Interactive deployment wizard' },
            { label: 'docker', description: 'Generate Dockerfile for self-hosting' },
            { label: 'fly', description: 'Deploy to Fly.io' },
            { label: 'railway', description: 'Deploy to Railway' },
            { label: 'render', description: 'Deploy to Render' },
            { label: 'manual', description: 'Generate all deployment configs' }
        ], { placeHolder: 'Select deployment target' });
        if (!target)
            return;
        const command = target.label === 'interactive'
            ? `${cliPath} deploy`
            : `${cliPath} deploy --target ${target.label}`;
        runInTerminal(command, 'Nucleus Deploy');
        vscode.window.showInformationMessage('🚀 Starting deployment...');
    }));
    // nucleus export - Static export
    context.subscriptions.push(vscode.commands.registerCommand('nucleus.export', () => {
        runInTerminal(`${cliPath} export`, 'Nucleus Export');
        vscode.window.showInformationMessage('📦 Starting static export...');
    }));
}
//# sourceMappingURL=commands.js.map