"use client";

import React from 'react';
import { Zap, Shield, Code, FileText, MessageSquare, Layers } from 'lucide-react';
import Link from 'next/link';

export const Features: React.FC = () => {
    return (
        <section className="w-full max-w-7xl mx-auto px-6 mb-32 relative">

            <div className="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-4 gap-6 relative z-10">

                {/* Core Feature 1 - Performance */}
                <div className="lg:col-span-2 rounded-3xl p-8 relative overflow-hidden group bg-[#1a1a24] border border-white/10 hover:border-purple-500/30 transition-all">
                    <div className="absolute top-0 right-0 w-64 h-64 bg-purple-600/10 rounded-full blur-[80px] -mr-16 -mt-16 group-hover:bg-purple-600/20 transition-colors"></div>
                    <div className="relative z-10 h-full flex flex-col">
                        <div className="w-12 h-12 rounded-xl bg-purple-600 flex items-center justify-center text-white mb-6 shadow-lg shadow-purple-500/20">
                            <Zap size={24} />
                        </div>
                        <h3 className="text-2xl font-bold mb-3 text-white">High Performance</h3>
                        <p className="text-gray-400 text-lg leading-relaxed mb-8">
                            Up to 4x faster than Python. NanBoxed values, string interning, and peephole optimization make Nebula blazing fast.
                        </p>
                        <div className="mt-auto">
                            <div className="h-1 w-full bg-white/5 rounded-full overflow-hidden">
                                <div className="h-full bg-purple-600 w-[85%]"></div>
                            </div>
                            <div className="flex justify-between text-xs font-mono text-gray-500 mt-2">
                                <span>Performance vs Python</span>
                                <span className="text-purple-400">4x faster</span>
                            </div>
                        </div>
                    </div>
                </div>

                {/* Core Feature 2 - State Logic */}
                <div className="rounded-3xl p-8 relative overflow-hidden group bg-[#1a1a24] border border-white/10 hover:border-cyan-500/30 transition-all">
                    <div className="w-12 h-12 rounded-xl bg-cyan-600 flex items-center justify-center text-white mb-6 shadow-lg shadow-cyan-500/20">
                        <Shield size={24} />
                    </div>
                    <h3 className="text-xl font-bold mb-3 text-white">State Logic</h3>
                    <p className="text-gray-400 text-sm leading-relaxed">
                        Use <code className="text-cyan-400 bg-white/5 px-1 rounded">on</code>/<code className="text-cyan-400 bg-white/5 px-1 rounded">off</code> for booleans and <code className="text-cyan-400 bg-white/5 px-1 rounded">empty</code> for null. Clean and intuitive.
                    </p>
                </div>

                {/* Core Feature 3 - Clean Syntax */}
                <div className="rounded-3xl p-8 relative overflow-hidden group bg-[#1a1a24] border border-white/10 hover:border-blue-500/30 transition-all">
                    <div className="w-12 h-12 rounded-xl bg-blue-600 flex items-center justify-center text-white mb-6 shadow-lg shadow-blue-500/20">
                        <Code size={24} />
                    </div>
                    <h3 className="text-xl font-bold mb-3 text-white">Clean Syntax</h3>
                    <p className="text-gray-400 text-sm leading-relaxed">
                        Python-like readability with <code className="text-blue-400 bg-white/5 px-1 rounded">do</code>...<code className="text-blue-400 bg-white/5 px-1 rounded">end</code> blocks. No curly braces, no semicolons.
                    </p>
                </div>

                {/* Documentation Showcase */}
                <Link
                    href="/docs/introduction"
                    className="lg:col-span-2 rounded-3xl p-8 flex flex-row items-center gap-8 relative overflow-hidden group bg-[#1a1a24] border border-white/10 hover:border-cyan-500/50 transition-colors cursor-pointer"
                >
                    <div className="absolute inset-0 bg-gradient-to-r from-cyan-600/5 to-transparent opacity-0 group-hover:opacity-100 transition-opacity"></div>
                    <div className="flex-1 relative z-10">
                        <h3 className="text-2xl font-bold mb-2 text-white flex items-center gap-2">
                            <FileText size={24} className="text-cyan-400" /> Documentation
                        </h3>
                        <p className="text-gray-400 mb-4">
                            Comprehensive guides, syntax reference, and examples to get you started quickly.
                        </p>
                        <span className="text-sm font-bold text-cyan-400 flex items-center gap-1">
                            Read the Docs <Layers size={14} />
                        </span>
                    </div>
                    <div className="hidden md:block w-1/3 h-24 bg-white/5 rounded-lg border border-white/10 p-3 rotate-3 group-hover:rotate-0 transition-transform">
                        <div className="space-y-2">
                            <div className="w-3/4 h-2 bg-white/20 rounded"></div>
                            <div className="w-full h-2 bg-white/10 rounded"></div>
                            <div className="w-5/6 h-2 bg-white/10 rounded"></div>
                        </div>
                    </div>
                </Link>

                {/* Community Showcase */}
                <a
                    href="https://discord.gg/3RBEyCUgpX"
                    target="_blank"
                    rel="noopener noreferrer"
                    className="lg:col-span-2 rounded-3xl p-8 flex flex-row items-center gap-8 relative overflow-hidden group bg-[#1a1a24] border border-white/10 hover:border-purple-500/50 transition-colors cursor-pointer"
                >
                    <div className="absolute inset-0 bg-gradient-to-r from-purple-600/5 to-transparent opacity-0 group-hover:opacity-100 transition-opacity"></div>
                    <div className="hidden md:block w-1/3 h-24 relative">
                        <div className="absolute top-0 right-0 w-10 h-10 rounded-full bg-purple-600 border-2 border-[#1a1a24] z-20"></div>
                        <div className="absolute top-4 right-6 w-10 h-10 rounded-full bg-cyan-600 border-2 border-[#1a1a24] z-10"></div>
                        <div className="absolute top-8 right-12 w-10 h-10 rounded-full bg-blue-600 border-2 border-[#1a1a24] z-0"></div>
                    </div>
                    <div className="flex-1 relative z-10 text-right md:text-left">
                        <h3 className="text-2xl font-bold mb-2 text-white flex items-center gap-2 justify-end md:justify-start">
                            Community <MessageSquare size={24} className="text-purple-400" />
                        </h3>
                        <p className="text-gray-400 mb-4">
                            Join our Discord community. Get help, share projects, and contribute to Nebula.
                        </p>
                        <span className="text-sm font-bold text-purple-400 flex items-center gap-1 justify-end md:justify-start">
                            Join Discord →
                        </span>
                    </div>
                </a>

            </div>
        </section>
    );
};