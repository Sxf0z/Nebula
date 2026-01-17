"use strict";
Object.defineProperty(exports, "__esModule", { value: true });
exports.deactivate = exports.activate = void 0;
const vscode = require("vscode");
const child_process_1 = require("child_process");
let diagnosticCollection;
let runnerTerminal;
function activate(context) {
    diagnosticCollection = vscode.languages.createDiagnosticCollection('nebula');
    context.subscriptions.push(diagnosticCollection);
    const runCommand = vscode.commands.registerCommand('nebula.run', async () => {
        const editor = vscode.window.activeTextEditor;
        if (!editor || editor.document.languageId !== 'nebula') {
            vscode.window.showErrorMessage('No Nebula file open');
            return;
        }
        await editor.document.save();
        const filePath = editor.document.fileName;
        const execPath = vscode.workspace.getConfiguration('nebula').get('executablePath') || 'nebula';
        if (!runnerTerminal || runnerTerminal.exitStatus !== undefined) {
            runnerTerminal = vscode.window.createTerminal('Nebula Runner');
        }
        runnerTerminal.show();
        runnerTerminal.sendText(`${execPath} --vm "${filePath}"`);
    });
    context.subscriptions.push(runCommand);
    const saveListener = vscode.workspace.onDidSaveTextDocument((document) => {
        if (document.languageId !== 'nebula')
            return;
        lintDocument(document);
    });
    context.subscriptions.push(saveListener);
    const debugProvider = vscode.debug.registerDebugConfigurationProvider('nebula', {
        provideDebugConfigurations() {
            return [{
                    type: 'nebula',
                    request: 'launch',
                    name: 'Run Nebula Script',
                    program: '${file}'
                }];
        },
        resolveDebugConfiguration(folder, config) {
            if (!config.program) {
                const editor = vscode.window.activeTextEditor;
                if (editor && editor.document.languageId === 'nebula') {
                    config.program = editor.document.fileName;
                }
            }
            return config;
        }
    });
    context.subscriptions.push(debugProvider);
    const debugAdapterFactory = vscode.debug.registerDebugAdapterDescriptorFactory('nebula', {
        createDebugAdapterDescriptor() {
            return new vscode.DebugAdapterInlineImplementation(new NebulaDebugAdapter());
        }
    });
    context.subscriptions.push(debugAdapterFactory);
}
exports.activate = activate;
function lintDocument(document) {
    const execPath = vscode.workspace.getConfiguration('nebula').get('executablePath') || 'nebula';
    const filePath = document.fileName;
    (0, child_process_1.exec)(`"${execPath}" "${filePath}"`, (error, stdout, stderr) => {
        const diagnostics = [];
        if (stderr) {
            const lines = stderr.split('\n');
            let errorMessage = '';
            let lineNumber = 0;
            for (let i = 0; i < lines.length; i++) {
                const line = lines[i];
                if (line.includes('[COSMIC FRACTURE]') || line.includes('[ERROR]')) {
                    if (i + 1 < lines.length) {
                        errorMessage = lines[i + 1].trim();
                    }
                }
                const lineMatch = line.match(/-->\s*line\s*(\d+)/);
                if (lineMatch) {
                    lineNumber = parseInt(lineMatch[1], 10) - 1;
                }
            }
            if (errorMessage && lineNumber >= 0) {
                const range = new vscode.Range(lineNumber, 0, lineNumber, 1000);
                const diagnostic = new vscode.Diagnostic(range, errorMessage, vscode.DiagnosticSeverity.Error);
                diagnostic.source = 'nebula';
                diagnostics.push(diagnostic);
            }
        }
        diagnosticCollection.set(document.uri, diagnostics);
    });
}
class NebulaDebugAdapter {
    constructor() {
        this.sendMessage = () => { };
        this.onDidSendMessage = (listener) => {
            this.sendMessage = listener;
            return { dispose: () => { } };
        };
    }
    handleMessage(message) {
        const msg = message;
        if (msg.command === 'initialize') {
            this.sendMessage({ type: 'response', request_seq: msg.seq, success: true, command: 'initialize', body: { supportsConfigurationDoneRequest: true } });
            this.sendMessage({ type: 'event', event: 'initialized' });
        }
        else if (msg.command === 'launch') {
            const program = msg.arguments?.program || '';
            const execPath = vscode.workspace.getConfiguration('nebula').get('executablePath') || 'nebula';
            if (!runnerTerminal || runnerTerminal.exitStatus !== undefined) {
                runnerTerminal = vscode.window.createTerminal('Nebula Debugger');
            }
            runnerTerminal.show();
            runnerTerminal.sendText(`${execPath} --vm "${program}"`);
            this.sendMessage({ type: 'response', request_seq: msg.seq, success: true, command: 'launch' });
            this.sendMessage({ type: 'event', event: 'terminated' });
        }
        else if (msg.command === 'configurationDone') {
            this.sendMessage({ type: 'response', request_seq: msg.seq, success: true, command: 'configurationDone' });
        }
        else if (msg.command === 'disconnect') {
            this.sendMessage({ type: 'response', request_seq: msg.seq, success: true, command: 'disconnect' });
        }
    }
    dispose() { }
}
function deactivate() {
    if (runnerTerminal) {
        runnerTerminal.dispose();
    }
}
exports.deactivate = deactivate;
//# sourceMappingURL=extension.js.map