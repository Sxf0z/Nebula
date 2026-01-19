"use client";

import React, { useState } from 'react';
import { Download, Copy, Check, Apple, Monitor, Terminal } from 'lucide-react';

export const DownloadPage: React.FC = () => {
  const [copied, setCopied] = useState<string | null>(null);
  const [activeTab, setActiveTab] = useState<'windows' | 'macos' | 'linux'>('windows');

  const copyCommand = (cmd: string, id: string) => {
    navigator.clipboard.writeText(cmd);
    setCopied(id);
    setTimeout(() => setCopied(null), 2000);
  };

  const installCommands = {
    windows: [
      { id: 'w1', cmd: 'git clone https://github.com/Sxf0z/Nebula.git', desc: 'Clone the repository' },
      { id: 'w2', cmd: 'cd Nebula', desc: 'Navigate to directory' },
      { id: 'w3', cmd: 'cargo build --release', desc: 'Build with Cargo' },
      { id: 'w4', cmd: '.\\target\\release\\nebula.exe --help', desc: 'Verify installation' },
    ],
    macos: [
      { id: 'm1', cmd: 'git clone https://github.com/Sxf0z/Nebula.git', desc: 'Clone the repository' },
      { id: 'm2', cmd: 'cd Nebula', desc: 'Navigate to directory' },
      { id: 'm3', cmd: 'cargo build --release', desc: 'Build with Cargo' },
      { id: 'm4', cmd: './target/release/nebula --help', desc: 'Verify installation' },
    ],
    linux: [
      { id: 'l1', cmd: 'git clone https://github.com/Sxf0z/Nebula.git', desc: 'Clone the repository' },
      { id: 'l2', cmd: 'cd Nebula', desc: 'Navigate to directory' },
      { id: 'l3', cmd: 'cargo build --release', desc: 'Build with Cargo' },
      { id: 'l4', cmd: './target/release/nebula --help', desc: 'Verify installation' },
      { id: 'l5', cmd: 'sudo cp ./target/release/nebula /usr/local/bin/', desc: 'Install globally (optional)' },
    ],
  };

  return (
    <section className="pt-40 pb-20 w-full max-w-5xl mx-auto px-6">
      <div className="text-center mb-16">
        <h1
          className="text-5xl md:text-6xl font-extrabold text-white mb-4"
          style={{
            fontFamily: "'Outfit', system-ui, sans-serif",
            textShadow: '0 0 60px rgba(147, 51, 234, 0.4), 0 4px 20px rgba(0,0,0,0.8)'
          }}
        >
          Install Nebula
        </h1>
        <p className="text-xl text-gray-400 max-w-2xl mx-auto">
          Get started with Nebula in minutes. Build from source with Rust.
        </p>
      </div>

      {/* Platform Tabs */}
      <div className="flex justify-center gap-4 mb-8">
        {(['windows', 'macos', 'linux'] as const).map((platform) => (
          <button
            key={platform}
            onClick={() => setActiveTab(platform)}
            className={`px-6 py-3 rounded-xl font-semibold text-sm transition-all flex items-center gap-2 ${activeTab === platform
                ? 'bg-purple-600 text-white shadow-lg shadow-purple-500/30'
                : 'bg-[#1a1a24] text-gray-400 hover:text-white hover:bg-[#252533] border border-white/10'
              }`}
          >
            {platform === 'windows' && <Monitor size={18} />}
            {platform === 'macos' && <Apple size={18} />}
            {platform === 'linux' && <Terminal size={18} />}
            {platform.charAt(0).toUpperCase() + platform.slice(1)}
          </button>
        ))}
      </div>

      {/* Install Steps */}
      <div
        className="rounded-2xl p-8 bg-[#0f0f14] border border-white/10"
        style={{ boxShadow: '0 25px 50px -12px rgba(0, 0, 0, 0.5)' }}
      >
        <h2 className="text-xl font-bold text-white mb-6 flex items-center gap-3">
          <Download className="text-purple-400" />
          Installation Steps
        </h2>

        <div className="space-y-4">
          {installCommands[activeTab].map((step, index) => (
            <div key={step.id} className="flex items-start gap-4">
              <div className="w-8 h-8 rounded-full bg-purple-600/20 text-purple-400 flex items-center justify-center text-sm font-bold shrink-0">
                {index + 1}
              </div>
              <div className="flex-1">
                <p className="text-gray-400 text-sm mb-2">{step.desc}</p>
                <div className="flex items-center justify-between bg-[#0a0a10] rounded-xl px-4 py-3 border border-white/5 group hover:border-purple-500/30 transition-colors">
                  <code className="text-sm text-gray-300 font-mono">
                    <span className="text-purple-400">$</span> {step.cmd}
                  </code>
                  <button
                    onClick={() => copyCommand(step.cmd, step.id)}
                    className="text-gray-500 hover:text-white transition-colors ml-4"
                  >
                    {copied === step.id ? <Check size={16} className="text-green-400" /> : <Copy size={16} />}
                  </button>
                </div>
              </div>
            </div>
          ))}
        </div>
      </div>

      {/* Requirements */}
      <div
        className="mt-8 rounded-2xl p-6 bg-[#0f0f14] border border-white/10"
        style={{ boxShadow: '0 15px 40px -10px rgba(0, 0, 0, 0.4)' }}
      >
        <h3 className="font-bold text-white mb-4">Prerequisites</h3>
        <div className="grid md:grid-cols-3 gap-4">
          <div className="flex items-center gap-3 p-4 rounded-xl bg-[#0a0a10] border border-white/5">
            <div className="w-10 h-10 rounded-lg bg-orange-500/20 flex items-center justify-center">
              <span className="text-orange-400 font-bold">🦀</span>
            </div>
            <div>
              <p className="text-white font-medium">Rust 1.70+</p>
              <a href="https://rustup.rs" target="_blank" rel="noopener noreferrer" className="text-purple-400 text-sm hover:underline">rustup.rs</a>
            </div>
          </div>

          <div className="flex items-center gap-3 p-4 rounded-xl bg-[#0a0a10] border border-white/5">
            <div className="w-10 h-10 rounded-lg bg-gray-500/20 flex items-center justify-center">
              <span className="text-gray-400 font-bold">📦</span>
            </div>
            <div>
              <p className="text-white font-medium">Git</p>
              <a href="https://git-scm.com" target="_blank" rel="noopener noreferrer" className="text-purple-400 text-sm hover:underline">git-scm.com</a>
            </div>
          </div>

          <div className="flex items-center gap-3 p-4 rounded-xl bg-[#0a0a10] border border-white/5">
            <div className="w-10 h-10 rounded-lg bg-blue-500/20 flex items-center justify-center">
              <span className="text-blue-400 font-bold">⚙️</span>
            </div>
            <div>
              <p className="text-white font-medium">Cargo</p>
              <p className="text-gray-500 text-sm">Included with Rust</p>
            </div>
          </div>
        </div>
      </div>

      {/* Quick Start */}
      <div
        className="mt-8 rounded-2xl p-6 bg-gradient-to-r from-purple-600/10 to-cyan-600/10 border border-purple-500/20"
        style={{ boxShadow: '0 0 40px rgba(147, 51, 234, 0.1)' }}
      >
        <h3 className="font-bold text-white mb-4">🚀 Quick Start</h3>
        <p className="text-gray-400 mb-4">After installation, create a new file called <code className="text-purple-400 bg-white/5 px-2 py-1 rounded">hello.na</code>:</p>
        <div className="bg-[#0a0a10] rounded-xl p-4 font-mono text-sm border border-white/5 mb-4">
          <div><span className="text-gray-500"># hello.na</span></div>
          <div><span className="text-cyan-400">log</span>(<span className="text-emerald-400">"Hello, Nebula!"</span>)</div>
        </div>
        <p className="text-gray-400">Run it with:</p>
        <div className="bg-[#0a0a10] rounded-xl px-4 py-3 font-mono text-sm border border-white/5 mt-2 flex items-center justify-between">
          <code><span className="text-purple-400">$</span> nebula hello.na</code>
          <button
            onClick={() => copyCommand('nebula hello.na', 'quickstart')}
            className="text-gray-500 hover:text-white transition-colors"
          >
            {copied === 'quickstart' ? <Check size={16} className="text-green-400" /> : <Copy size={16} />}
          </button>
        </div>
      </div>
    </section>
  );
};