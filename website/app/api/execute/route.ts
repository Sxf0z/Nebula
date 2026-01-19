import { NextResponse } from 'next/server';
import { spawn } from 'child_process';
import fs from 'fs/promises';
import path from 'path';
import os from 'os';

export async function POST(request: Request) {
    let tempFilePath: string | null = null;

    try {
        const { code } = await request.json();

        if (!code || typeof code !== 'string') {
            return NextResponse.json({ output: '', error: 'No code provided.' }, { status: 400 });
        }

        // Create a temporary file
        const tempDir = os.tmpdir();
        const tempFileName = `nebula_playground_${Date.now()}.na`;
        tempFilePath = path.join(tempDir, tempFileName);

        await fs.writeFile(tempFilePath, code, 'utf-8');

        // Path to the Nebula binary (adjust based on your deployment)
        // For local dev, this assumes the binary is in the project root's target/release folder.
        const nebulaPath = path.resolve(process.cwd(), '..', 'target', 'release', 'nebula.exe');

        const result = await new Promise<{ output: string; error: string | null }>((resolve) => {
            let stdout = '';
            let stderr = '';

            const proc = spawn(nebulaPath, ['--vm', tempFilePath!], {
                timeout: 5000, // 5 second timeout
            });

            proc.stdout.on('data', (data) => {
                stdout += data.toString();
            });

            proc.stderr.on('data', (data) => {
                stderr += data.toString();
            });

            proc.on('close', (exitCode) => {
                if (exitCode !== 0 && stderr) {
                    resolve({ output: stdout, error: stderr });
                } else {
                    resolve({ output: stdout, error: null });
                }
            });

            proc.on('error', (err) => {
                resolve({ output: '', error: `Execution failed: ${err.message}` });
            });
        });

        return NextResponse.json(result);
    } catch (e: unknown) {
        const message = e instanceof Error ? e.message : 'Unknown error';
        return NextResponse.json({ output: '', error: `Server error: ${message}` }, { status: 500 });
    } finally {
        // Cleanup temp file
        if (tempFilePath) {
            try {
                await fs.unlink(tempFilePath);
            } catch { /* ignore cleanup errors */ }
        }
    }
}
