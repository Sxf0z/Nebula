"use client";

import React from 'react';
import { Github, Heart } from 'lucide-react';
import Image from 'next/image';
import Link from 'next/link';

export const Footer: React.FC = () => {
    return (
        <footer className="relative z-10 border-t border-white/5 bg-[#0a0a10] mt-20">
            <div className="max-w-7xl mx-auto px-6 py-16">
                <div className="grid grid-cols-1 md:grid-cols-4 gap-12">

                    {/* Brand */}
                    <div className="md:col-span-1">
                        <div className="flex items-center gap-3 mb-4">
                            <Image
                                src="/Logo.png"
                                alt="Nebula Logo"
                                width={32}
                                height={32}
                                className="rounded-sm"
                            />
                            <span className="font-bold text-xl text-white">Nebula</span>
                        </div>
                        <p className="text-gray-500 text-sm leading-relaxed mb-4">
                            High-performance scripting language designed for modern development.
                        </p>
                        <div className="flex items-center gap-4">
                            <a
                                href="https://github.com/Sxf0z/Nebula"
                                target="_blank"
                                rel="noopener noreferrer"
                                className="text-gray-500 hover:text-white transition-colors"
                            >
                                <Github size={20} />
                            </a>
                            <a
                                href="https://discord.gg/3RBEyCUgpX"
                                target="_blank"
                                rel="noopener noreferrer"
                                className="text-gray-500 hover:text-white transition-colors"
                            >
                                <svg width="20" height="20" viewBox="0 0 24 24" fill="currentColor">
                                    <path d="M20.317 4.3698a19.7913 19.7913 0 00-4.8851-1.5152.0741.0741 0 00-.0785.0371c-.211.3753-.4447.8648-.6083 1.2495-1.8447-.2762-3.68-.2762-5.4868 0-.1636-.3933-.4058-.8742-.6177-1.2495a.077.077 0 00-.0785-.037 19.7363 19.7363 0 00-4.8852 1.515.0699.0699 0 00-.0321.0277C.5334 9.0458-.319 13.5799.0992 18.0578a.0824.0824 0 00.0312.0561c2.0528 1.5076 4.0413 2.4228 5.9929 3.0294a.0777.0777 0 00.0842-.0276c.4616-.6304.8731-1.2952 1.226-1.9942a.076.076 0 00-.0416-.1057c-.6528-.2476-1.2743-.5495-1.8722-.8923a.077.077 0 01-.0076-.1277c.1258-.0943.2517-.1923.3718-.2914a.0743.0743 0 01.0776-.0105c3.9278 1.7933 8.18 1.7933 12.0614 0a.0739.0739 0 01.0785.0095c.1202.099.246.1981.3728.2924a.077.077 0 01-.0066.1276 12.2986 12.2986 0 01-1.873.8914.0766.0766 0 00-.0407.1067c.3604.698.7719 1.3628 1.225 1.9932a.076.076 0 00.0842.0286c1.961-.6067 3.9495-1.5219 6.0023-3.0294a.077.077 0 00.0313-.0552c.5004-5.177-.8382-9.6739-3.5485-13.6604a.061.061 0 00-.0312-.0286zM8.02 15.3312c-1.1825 0-2.1569-1.0857-2.1569-2.419 0-1.3332.9555-2.4189 2.157-2.4189 1.2108 0 2.1757 1.0952 2.1568 2.419 0 1.3332-.9555 2.4189-2.1569 2.4189zm7.9748 0c-1.1825 0-2.1569-1.0857-2.1569-2.419 0-1.3332.9554-2.4189 2.1569-2.4189 1.2108 0 2.1757 1.0952 2.1568 2.419 0 1.3332-.946 2.4189-2.1568 2.4189Z" />
                                </svg>
                            </a>
                        </div>
                    </div>

                    {/* Links */}
                    <div>
                        <h4 className="font-semibold text-white mb-4">Documentation</h4>
                        <ul className="space-y-3 text-sm">
                            <li><Link href="/docs/introduction" className="text-gray-500 hover:text-white transition-colors">Introduction</Link></li>
                            <li><Link href="/docs/installation" className="text-gray-500 hover:text-white transition-colors">Installation</Link></li>
                            <li><Link href="/docs/basics" className="text-gray-500 hover:text-white transition-colors">Basics</Link></li>
                            <li><Link href="/docs/functions" className="text-gray-500 hover:text-white transition-colors">Functions</Link></li>
                        </ul>
                    </div>

                    <div>
                        <h4 className="font-semibold text-white mb-4">Resources</h4>
                        <ul className="space-y-3 text-sm">
                            <li><Link href="/docs/cli" className="text-gray-500 hover:text-white transition-colors">CLI Reference</Link></li>
                            <li><Link href="/docs/builtins" className="text-gray-500 hover:text-white transition-colors">Built-in Functions</Link></li>
                            <li><Link href="/docs/advanced" className="text-gray-500 hover:text-white transition-colors">Advanced</Link></li>
                            <li><Link href="/docs/benchmarks" className="text-gray-500 hover:text-white transition-colors">Benchmarks</Link></li>
                        </ul>
                    </div>

                    <div>
                        <h4 className="font-semibold text-white mb-4">Community</h4>
                        <ul className="space-y-3 text-sm">
                            <li><a href="https://github.com/Sxf0z/Nebula" target="_blank" rel="noopener noreferrer" className="text-gray-500 hover:text-white transition-colors">GitHub</a></li>
                            <li><a href="https://discord.gg/3RBEyCUgpX" target="_blank" rel="noopener noreferrer" className="text-gray-500 hover:text-white transition-colors">Discord</a></li>
                            <li><a href="https://github.com/Sxf0z/Nebula/issues" target="_blank" rel="noopener noreferrer" className="text-gray-500 hover:text-white transition-colors">Report Issues</a></li>
                            <li><a href="https://github.com/Sxf0z/Nebula/blob/main/LICENSE" target="_blank" rel="noopener noreferrer" className="text-gray-500 hover:text-white transition-colors">License (MIT)</a></li>
                        </ul>
                    </div>

                </div>

                {/* Bottom */}
                <div className="mt-12 pt-8 border-t border-white/5 flex flex-col md:flex-row items-center justify-between gap-4">
                    <p className="text-gray-600 text-sm">
                        © 2026 Nebula. Built with <Heart size={14} className="inline text-red-500" /> and Rust.
                    </p>
                    <p className="text-gray-600 text-sm font-mono">
                        v1.0.0
                    </p>
                </div>
            </div>
        </footer>
    );
};