import * as vscode from 'vscode';
import { exec } from 'child_process';

let diagnosticCollection: vscode.DiagnosticCollection;
let runnerTerminal: vscode.Terminal | undefined;

export function activate(context: vscode.ExtensionContext) {
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
        const execPath = vscode.workspace.getConfiguration('nebula').get<string>('executablePath') || 'nebula';

        if (!runnerTerminal || runnerTerminal.exitStatus !== undefined) {
            runnerTerminal = vscode.window.createTerminal('Nebula Runner');
        }

        runnerTerminal.show();
        runnerTerminal.sendText(`${execPath} --vm "${filePath}"`);
    });

    context.subscriptions.push(runCommand);

    const saveListener = vscode.workspace.onDidSaveTextDocument((document) => {
        if (document.languageId !== 'nebula') return;
        lintDocument(document);
    });

    context.subscriptions.push(saveListener);

    const debugProvider = vscode.debug.registerDebugConfigurationProvider('nebula', {
        provideDebugConfigurations(): vscode.DebugConfiguration[] {
            return [{
                type: 'nebula',
                request: 'launch',
                name: 'Run Nebula Script',
                program: '${file}'
            }];
        },
        resolveDebugConfiguration(
            folder: vscode.WorkspaceFolder | undefined,
            config: vscode.DebugConfiguration
        ): vscode.DebugConfiguration | undefined {
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
        createDebugAdapterDescriptor(): vscode.ProviderResult<vscode.DebugAdapterDescriptor> {
            return new vscode.DebugAdapterInlineImplementation(new NebulaDebugAdapter());
        }
    });

    context.subscriptions.push(debugAdapterFactory);
}

function lintDocument(document: vscode.TextDocument) {
    const execPath = vscode.workspace.getConfiguration('nebula').get<string>('executablePath') || 'nebula';
    const filePath = document.fileName;

    exec(`"${execPath}" "${filePath}"`, (error, stdout, stderr) => {
        const diagnostics: vscode.Diagnostic[] = [];

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

class NebulaDebugAdapter implements vscode.DebugAdapter {
    private sendMessage: (message: vscode.DebugProtocolMessage) => void = () => { };

    onDidSendMessage: vscode.Event<vscode.DebugProtocolMessage> = (listener) => {
        this.sendMessage = listener;
        return { dispose: () => { } };
    };

    handleMessage(message: vscode.DebugProtocolMessage): void {
        const msg = message as { command?: string; seq?: number; arguments?: { program?: string } };

        if (msg.command === 'initialize') {
            this.sendMessage({ type: 'response', request_seq: msg.seq, success: true, command: 'initialize', body: { supportsConfigurationDoneRequest: true } } as vscode.DebugProtocolMessage);
            this.sendMessage({ type: 'event', event: 'initialized' } as vscode.DebugProtocolMessage);
        } else if (msg.command === 'launch') {
            const program = msg.arguments?.program || '';
            const execPath = vscode.workspace.getConfiguration('nebula').get<string>('executablePath') || 'nebula';

            if (!runnerTerminal || runnerTerminal.exitStatus !== undefined) {
                runnerTerminal = vscode.window.createTerminal('Nebula Debugger');
            }
            runnerTerminal.show();
            runnerTerminal.sendText(`${execPath} --vm "${program}"`);

            this.sendMessage({ type: 'response', request_seq: msg.seq, success: true, command: 'launch' } as vscode.DebugProtocolMessage);
            this.sendMessage({ type: 'event', event: 'terminated' } as vscode.DebugProtocolMessage);
        } else if (msg.command === 'configurationDone') {
            this.sendMessage({ type: 'response', request_seq: msg.seq, success: true, command: 'configurationDone' } as vscode.DebugProtocolMessage);
        } else if (msg.command === 'disconnect') {
            this.sendMessage({ type: 'response', request_seq: msg.seq, success: true, command: 'disconnect' } as vscode.DebugProtocolMessage);
        }
    }

    dispose(): void { }
}

export function deactivate() {
    if (runnerTerminal) {
        runnerTerminal.dispose();
    }
}
