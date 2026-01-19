"use client";

import React, { useState } from 'react';
import { Play, Terminal, Activity, RotateCw, Trash2 } from 'lucide-react';

// Token types for syntax highlighting
type TokenType = 'keyword' | 'builtin' | 'boolean' | 'string' | 'number' | 'comment' | 'function' | 'variable' | 'punctuation' | 'text';

interface Token {
  type: TokenType;
  value: string;
}

const tokenize = (code: string): Token[][] => {
  const keywords = ['perm', 'fn', 'do', 'end', 'if', 'else', 'for', 'while', 'return', 'give', 'elsif'];
  const builtins = ['log', 'get', 'len', 'sqrt', 'typeof', 'push', 'pop', 'abs', 'floor', 'ceil'];
  const booleans = ['on', 'off', 'empty'];

  return code.split('\n').map(line => {
    const tokens: Token[] = [];
    let i = 0;

    while (i < line.length) {
      // Comment
      if (line[i] === '#') {
        tokens.push({ type: 'comment', value: line.slice(i) });
        break;
      }

      // String
      if (line[i] === '"') {
        let end = i + 1;
        while (end < line.length && line[end] !== '"') end++;
        tokens.push({ type: 'string', value: line.slice(i, end + 1) });
        i = end + 1;
        continue;
      }

      // Number
      if (/\d/.test(line[i])) {
        let end = i;
        while (end < line.length && /\d/.test(line[end])) end++;
        tokens.push({ type: 'number', value: line.slice(i, end) });
        i = end;
        continue;
      }

      // Identifier/keyword
      if (/[a-zA-Z_]/.test(line[i])) {
        let end = i;
        while (end < line.length && /[a-zA-Z0-9_]/.test(line[end])) end++;
        const word = line.slice(i, end);

        if (keywords.includes(word)) {
          tokens.push({ type: 'keyword', value: word });
        } else if (builtins.includes(word)) {
          tokens.push({ type: 'builtin', value: word });
        } else if (booleans.includes(word)) {
          tokens.push({ type: 'boolean', value: word });
        } else if (tokens.length > 0 && tokens[tokens.length - 1].value === 'fn') {
          tokens.push({ type: 'function', value: word });
        } else {
          tokens.push({ type: 'variable', value: word });
        }
        i = end;
        continue;
      }

      // Punctuation
      if (/[\(\)\[\]\{\}\=\,\.\+\-\*\/\<\>\!]/.test(line[i])) {
        tokens.push({ type: 'punctuation', value: line[i] });
        i++;
        continue;
      }

      // Whitespace/other
      tokens.push({ type: 'text', value: line[i] });
      i++;
    }

    return tokens;
  });
};

const tokenColors: Record<TokenType, string> = {
  keyword: 'text-purple-400 font-bold',
  builtin: 'text-cyan-400',
  boolean: 'text-cyan-400 font-bold',
  string: 'text-emerald-400',
  number: 'text-orange-400',
  comment: 'text-gray-500 italic',
  function: 'text-yellow-400',
  variable: 'text-blue-300',
  punctuation: 'text-gray-400',
  text: 'text-gray-300',
};

export const InteractivePlayground: React.FC = () => {
  const [code, setCode] = useState(`# Hello Nebula!

perm active = on
perm count = 5

fn main() do
    log("Welcome to Nebula!")
    
    for i = 1, count do
        log("Count:", i)
    end
    
    if active do
        log("Status: Active")
    else
        log("Status: Inactive")
    end
    
    log("Done!")
end`);

  const [output, setOutput] = useState<string[]>([]);
  const [isRunning, setIsRunning] = useState(false);

  const executeInterpreter = async () => {
    if (isRunning) return;
    setIsRunning(true);
    setOutput([]);

    try {
      const lines = code.split('\n');
      const variables: Record<string, any> = {};
      let skipBlock = false;

      const getValue = (val: string) => {
        val = val.trim();
        if (val === 'on') return true;
        if (val === 'off') return false;
        if (!isNaN(Number(val))) return Number(val);
        if (val.startsWith('"')) return val.slice(1, -1);
        return variables[val];
      };

      for (let i = 0; i < lines.length; i++) {
        let line = lines[i].trim();
        if (!line || line.startsWith('#') || line.startsWith('fn ') || line === '}') continue;

        if (line.startsWith('perm ')) {
          const parts = line.replace('perm ', '').split('=');
          if (parts.length === 2) {
            variables[parts[0].trim()] = getValue(parts[1].trim());
          }
          continue;
        }

        if (line.startsWith('if ')) {
          const conditionPart = line.substring(3, line.indexOf(' do'));
          const [lhs] = conditionPart.split(' ');
          const result = Boolean(getValue(lhs));
          if (!result) skipBlock = true;
          continue;
        }

        if (line.startsWith('else')) {
          skipBlock = !skipBlock;
          continue;
        }

        if (line === 'end') {
          skipBlock = false;
          continue;
        }

        if (skipBlock) continue;

        if (line.startsWith('log(')) {
          const contentRaw = line.slice(4, -1);
          const parts = contentRaw.split(',').map(p => {
            const trimmed = p.trim();
            if (trimmed.startsWith('"')) return trimmed.replace(/"/g, '');
            if (variables[trimmed] !== undefined) return String(variables[trimmed]);
            return trimmed;
          });

          setOutput(prev => [...prev, parts.join(' ')]);
          await new Promise(resolve => setTimeout(resolve, 100));
          continue;
        }

        if (line.startsWith('for ')) {
          const match = line.match(/for\s+(\w+)\s*=\s*(\d+),\s*(\w+)\s*do/);
          if (match) {
            const [, varName, startStr, endVar] = match;
            const start = parseInt(startStr);
            const end = variables[endVar] ?? parseInt(endVar);

            let depth = 1;
            let endIdx = i + 1;
            while (endIdx < lines.length && depth > 0) {
              const l = lines[endIdx].trim();
              if (l.includes(' do') || l.startsWith('if ')) depth++;
              if (l === 'end') depth--;
              endIdx++;
            }

            for (let j = start; j <= end; j++) {
              variables[varName] = j;
              for (let k = i + 1; k < endIdx - 1; k++) {
                const loopLine = lines[k].trim();
                if (loopLine.startsWith('log(')) {
                  const contentRaw = loopLine.slice(4, -1);
                  const parts = contentRaw.split(',').map(p => {
                    const trimmed = p.trim();
                    if (trimmed.startsWith('"')) return trimmed.replace(/"/g, '');
                    if (variables[trimmed] !== undefined) return String(variables[trimmed]);
                    return trimmed;
                  });
                  setOutput(prev => [...prev, parts.join(' ')]);
                  await new Promise(resolve => setTimeout(resolve, 80));
                }
              }
            }
            i = endIdx - 1;
          }
          continue;
        }
      }

      setOutput(prev => [...prev, "> Program exited (0)"]);

    } catch (e) {
      setOutput(prev => [...prev, `> Error: ${e}`]);
    } finally {
      setIsRunning(false);
    }
  };

  const tokenizedCode = tokenize(code);

  return (
    <section className="w-full max-w-7xl mx-auto px-6 mb-32" id="playground">
      <div className="flex flex-col md:flex-row items-end justify-between mb-8 gap-4">
        <div>
          <div className="flex items-center gap-2 text-purple-400 mb-2">
            <Activity size={18} className="animate-pulse" />
            <span className="text-sm font-mono tracking-widest uppercase">Live Preview</span>
          </div>
          <h2
            className="text-3xl md:text-4xl font-bold text-white"
            style={{ textShadow: '0 0 30px rgba(147, 51, 234, 0.3)' }}
          >
            Interactive Playground
          </h2>
        </div>
      </div>

      {/* Main Container */}
      <div
        className="rounded-3xl overflow-hidden bg-[#08080e] border border-white/10"
        style={{ boxShadow: '0 25px 80px -20px rgba(0, 0, 0, 0.8), 0 0 60px rgba(147, 51, 234, 0.1)' }}
      >

        {/* Toolbar */}
        <div className="flex items-center justify-between px-6 py-4 border-b border-white/5 bg-[#06060a]">
          <div className="flex items-center gap-4">
            <div className="flex gap-1.5">
              <div className="w-3 h-3 rounded-full bg-red-500/60 shadow-[0_0_6px_rgba(239,68,68,0.5)]"></div>
              <div className="w-3 h-3 rounded-full bg-yellow-500/60 shadow-[0_0_6px_rgba(234,179,8,0.5)]"></div>
              <div className="w-3 h-3 rounded-full bg-green-500/60 shadow-[0_0_6px_rgba(34,197,94,0.5)]"></div>
            </div>
            <span className="text-sm font-mono text-gray-500 ml-2">playground.na</span>
          </div>

          <div className="flex gap-3">
            <button
              onClick={executeInterpreter}
              disabled={isRunning}
              className={`flex items-center gap-2 px-6 py-2 rounded-xl font-bold text-sm transition-all
                ${isRunning
                  ? 'bg-white/5 text-gray-500 cursor-wait'
                  : 'bg-purple-600 text-white hover:bg-purple-500 hover:scale-105 hover:shadow-[0_0_30px_rgba(147,51,234,0.5)] active:scale-95'}`}
            >
              {isRunning ? <RotateCw size={16} className="animate-spin" /> : <Play size={16} fill="currentColor" />}
              {isRunning ? 'Running...' : 'Run'}
            </button>
          </div>
        </div>

        <div className="grid grid-cols-1 lg:grid-cols-2 min-h-[500px]">

          {/* Editor with Syntax Highlighting */}
          <div className="border-r border-white/5 bg-[#0a0a10] relative overflow-hidden">
            {/* Syntax highlighted display */}
            <div className="absolute inset-0 p-6 font-mono text-sm leading-relaxed overflow-auto pointer-events-none">
              {tokenizedCode.map((lineTokens, lineIndex) => (
                <div key={lineIndex} className="flex">
                  <span className="text-white/20 w-8 text-right pr-4 select-none shrink-0">{lineIndex + 1}</span>
                  <span className="whitespace-pre">
                    {lineTokens.map((token, tokenIndex) => (
                      <span key={tokenIndex} className={tokenColors[token.type]}>
                        {token.value}
                      </span>
                    ))}
                    {lineTokens.length === 0 && '\u00A0'}
                  </span>
                </div>
              ))}
            </div>

            {/* Editable textarea (invisible text, visible caret) */}
            <textarea
              value={code}
              onChange={(e) => setCode(e.target.value)}
              className="w-full h-full min-h-[500px] p-6 pl-14 bg-transparent text-transparent caret-purple-400 resize-none outline-none font-mono text-sm leading-relaxed relative z-10"
              spellCheck={false}
            />
          </div>

          {/* Output Console */}
          <div className="bg-[#04040a] p-6 font-mono text-sm overflow-y-auto relative border-t lg:border-t-0 border-white/5">
            <div className="flex items-center justify-between text-gray-500 mb-4 pb-2 border-b border-white/5">
              <div className="flex items-center gap-2">
                <Terminal size={14} />
                <span className="text-xs uppercase tracking-wider">Console Output</span>
              </div>
              {output.length > 0 && (
                <button onClick={() => setOutput([])} className="hover:text-white transition-colors" title="Clear Console">
                  <Trash2 size={14} />
                </button>
              )}
            </div>

            <div className="space-y-2">
              {output.length === 0 && !isRunning && (
                <div className="text-gray-600 italic flex flex-col items-center justify-center h-40">
                  <Activity size={32} className="mb-2 opacity-50" />
                  <span>Ready to execute</span>
                </div>
              )}
              {output.map((line, i) => (
                <div key={i} className="flex gap-3 break-all animate-fadeIn">
                  <span className="text-purple-400 shrink-0">›</span>
                  <span className={line.startsWith('>') ? 'text-gray-500' : 'text-cyan-400'}>
                    {line}
                  </span>
                </div>
              ))}
              {isRunning && (
                <div className="w-2 h-5 bg-purple-500 animate-pulse inline-block align-middle ml-2 shadow-[0_0_10px_rgba(168,85,247,0.8)]"></div>
              )}
            </div>
          </div>

        </div>
      </div>
    </section>
  );
};