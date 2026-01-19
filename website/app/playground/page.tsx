"use client";

import { useState, useEffect } from "react";
import { Play, Loader2 } from "lucide-react";
import Link from "next/link";

const DEFAULT_CODE = `# High-Performance Scripting
perm MAX_CYCLES = 1000

fn calculate_trajectory(v) do
    if v < MAX_CYCLES do
        give v * 2
    else
        log("Limit Reached")
        give 0
    end
end

# Executing strict logic
log("Result:", calculate_trajectory(500))
`;

export default function PlaygroundPage() {
    const [code, setCode] = useState(DEFAULT_CODE);
    const [output, setOutput] = useState("");
    const [error, setError] = useState<string | null>(null);
    const [isLoading, setIsLoading] = useState(false);

    // Debounced Auto-Run
    useEffect(() => {
        const timer = setTimeout(() => {
            if (code.trim()) {
                runCode();
            }
        }, 1500); // 1.5s debounce to prevent lag

        return () => clearTimeout(timer);
    }, [code]);

    const runCode = async () => {
        setIsLoading(true);
        // Don't clear output immediately during auto-run to avoid flickering
        // setOutput(""); 
        setError(null);

        try {
            const res = await fetch("/api/execute", {
                method: "POST",
                headers: { "Content-Type": "application/json" },
                body: JSON.stringify({ code }),
            });

            const data = await res.json();

            if (data.error) {
                setError(data.error);
                // Keep old output if error, or clear it? 
                // Better to show error.
            } else {
                setOutput(data.output);
            }
        } catch (e) {
            setError("Failed to connect to execution server.");
        } finally {
            setIsLoading(false);
        }
    };

    return (
        <div className="min-h-screen bg-background text-foreground flex flex-col">
            {/* Header */}
            <header className="h-14 border-b border-border flex items-center justify-between px-6 bg-[#14141F]">
                <Link href="/" className="font-mono text-sm uppercase tracking-widest text-white/80 hover:text-white transition-colors">
                    &larr; Nebula_Lang
                </Link>
                <h1 className="font-mono text-xs uppercase tracking-widest text-muted-foreground">
                    // Playground
                </h1>
                <button
                    onClick={runCode}
                    disabled={isLoading}
                    className="h-8 px-4 bg-white text-black font-mono text-xs font-bold uppercase tracking-tight flex items-center gap-2 hover:bg-zinc-200 transition-colors disabled:opacity-50"
                >
                    {isLoading ? (
                        <Loader2 className="w-3 h-3 animate-spin" />
                    ) : (
                        <Play className="w-3 h-3" />
                    )}
                    Run
                </button>
            </header>

            {/* Main Content */}
            <div className="flex-1 grid md:grid-cols-2 divide-x divide-border">
                {/* Editor Pane */}
                <div className="flex flex-col">
                    <div className="h-10 border-b border-border flex items-center px-4 bg-[#1E1E2E]">
                        <span className="text-xs font-mono text-muted-foreground uppercase">main.na</span>
                    </div>
                    <textarea
                        value={code}
                        onChange={(e) => setCode(e.target.value)}
                        spellCheck={false}
                        className="flex-1 p-4 bg-[#0A0A10] font-mono text-sm text-zinc-300 resize-none focus:outline-none"
                        placeholder="// Write your Nebula code here..."
                    />
                </div>

                {/* Output Pane */}
                <div className="flex flex-col">
                    <div className="h-10 border-b border-border flex items-center px-4 bg-[#1E1E2E]">
                        <span className="text-xs font-mono text-muted-foreground uppercase">Output</span>
                    </div>
                    <div className="flex-1 p-4 bg-[#0A0A10] font-mono text-sm overflow-auto">
                        {error && (
                            <pre className="text-red-400 whitespace-pre-wrap mb-2">{error}</pre>
                        )}
                        <pre className="text-zinc-300 whitespace-pre-wrap">{output}</pre>
                        {!output && !error && !isLoading && (
                            <span className="text-muted-foreground">// Click &quot;Run&quot; to execute</span>
                        )}
                    </div>
                </div>
            </div>
        </div>
    );
}
