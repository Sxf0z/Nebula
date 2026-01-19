"use client";

import React, { useState } from 'react';
import { Copy, Check, Terminal } from 'lucide-react';

export const Hero: React.FC = () => {
   const [copied, setCopied] = useState(false);

   const copyInstall = () => {
      navigator.clipboard.writeText("git clone https://github.com/Sxf0z/Nebula && cd Nebula && cargo build --release");
      setCopied(true);
      setTimeout(() => setCopied(false), 2000);
   };

   return (
      <section className="pt-40 pb-20 w-full max-w-7xl mx-auto px-6 flex flex-col items-center relative z-10">

         {/* Title Section */}
         <div className="text-center mb-16 flex flex-col items-center" style={{ animation: 'fadeIn 0.8s ease-out' }}>
            <h1
               className="text-7xl md:text-9xl font-extrabold tracking-tighter mb-4 relative z-10 text-white"
               style={{
                  fontFamily: "'Outfit', 'Inter', system-ui, sans-serif",
                  textShadow: '0 0 60px rgba(147, 51, 234, 0.5), 0 0 120px rgba(147, 51, 234, 0.3), 0 4px 20px rgba(0,0,0,0.8)',
               }}
            >
               Nebula
            </h1>
            <p className="text-2xl md:text-3xl font-light text-gray-400 max-w-3xl leading-relaxed">
               A <span className="text-purple-400 font-semibold">high-performance</span> scripting language for the modern era.
            </p>
         </div>

         {/* Code Block */}
         <div className="w-full max-w-5xl relative group" style={{ animation: 'fadeIn 0.8s ease-out 0.2s both' }}>

            {/* Glow Effect */}
            <div className="absolute -inset-2 bg-gradient-to-r from-purple-600/30 via-blue-600/20 to-cyan-600/30 rounded-2xl blur-xl opacity-40 group-hover:opacity-60 transition-opacity duration-1000"></div>

            {/* The Card */}
            <div
               className="relative rounded-2xl overflow-hidden border border-white/10 bg-[#0a0a12]"
               style={{ boxShadow: '0 25px 50px -12px rgba(0, 0, 0, 0.8), 0 0 40px rgba(147, 51, 234, 0.1)' }}
            >

               {/* Header */}
               <div className="flex items-center justify-between px-6 py-4 border-b border-white/5 bg-[#08080e]">
                  <div className="flex items-center gap-2">
                     <div className="flex gap-2 mr-4">
                        <div className="w-3 h-3 rounded-full bg-[#FF5F56] shadow-[0_0_8px_rgba(255,95,86,0.5)]"></div>
                        <div className="w-3 h-3 rounded-full bg-[#FFBD2E] shadow-[0_0_8px_rgba(255,189,46,0.5)]"></div>
                        <div className="w-3 h-3 rounded-full bg-[#27C93F] shadow-[0_0_8px_rgba(39,201,63,0.5)]"></div>
                     </div>
                     <div className="flex items-center gap-2 px-3 py-1 rounded-md bg-white/5 border border-white/5 text-xs text-gray-500 font-mono">
                        <Terminal size={12} />
                        <span>main.na</span>
                     </div>
                  </div>
                  <div className="flex items-center gap-4 text-xs font-mono text-gray-500">
                     <span>Ln 12, Col 4</span>
                     <span className="text-blue-400">UTF-8</span>
                  </div>
               </div>

               {/* Code Content with Syntax Highlighting */}
               <div className="p-8 font-mono text-sm md:text-base leading-relaxed overflow-x-auto">
                  <div className="grid grid-cols-[auto_1fr] gap-4">
                     <div className="text-white/20 text-right select-none space-y-1">
                        {Array.from({ length: 12 }).map((_, i) => <div key={i}>{i + 1}</div>)}
                     </div>
                     <div className="space-y-1">
                        <div><span className="text-gray-500"># Hello Nebula!</span></div>
                        <div className="h-4"></div>
                        <div><span className="text-purple-400 font-bold">perm</span> <span className="text-blue-300">active</span> <span className="text-gray-400">=</span> <span className="text-cyan-400 font-bold">on</span></div>
                        <div><span className="text-purple-400 font-bold">perm</span> <span className="text-blue-300">message</span> <span className="text-gray-400">=</span> <span className="text-emerald-400">"Welcome to Nebula!"</span></div>
                        <div className="h-4"></div>
                        <div><span className="text-purple-400 font-bold">fn</span> <span className="text-yellow-400">main</span><span className="text-gray-400">()</span> <span className="text-purple-400 font-bold">do</span></div>
                        <div className="pl-4"><span className="text-cyan-400">log</span><span className="text-gray-400">(</span><span className="text-blue-300">message</span><span className="text-gray-400">)</span></div>
                        <div className="pl-4"></div>
                        <div className="pl-4"><span className="text-purple-400 font-bold">for</span> <span className="text-blue-300">i</span> <span className="text-gray-400">=</span> <span className="text-orange-400">1</span><span className="text-gray-400">,</span> <span className="text-orange-400">5</span> <span className="text-purple-400 font-bold">do</span></div>
                        <div className="pl-8"><span className="text-cyan-400">log</span><span className="text-gray-400">(</span><span className="text-emerald-400">"Count:"</span><span className="text-gray-400">,</span> <span className="text-blue-300">i</span><span className="text-gray-400">)</span></div>
                        <div className="pl-4"><span className="text-purple-400 font-bold">end</span></div>
                        <div><span className="text-purple-400 font-bold">end</span></div>
                     </div>
                  </div>
               </div>

               {/* Status Bar */}
               <div className="bg-purple-600/10 border-t border-white/5 px-6 py-2 flex items-center justify-between text-xs font-mono text-purple-400">
                  <div className="flex items-center gap-2">
                     <div className="w-2 h-2 rounded-full bg-green-400 shadow-[0_0_8px_rgba(74,222,128,0.6)]"></div>
                     <span>Ready</span>
                  </div>
                  <div>Nebula v1.0.0</div>
               </div>

            </div>
         </div>

         {/* Quick Install Bar */}
         <div className="mt-12" style={{ animation: 'fadeIn 0.8s ease-out 0.4s both' }}>
            <div
               onClick={copyInstall}
               className="group relative cursor-pointer rounded-full pl-6 pr-2 py-2 flex items-center gap-4 bg-[#12121a] border border-white/10 hover:border-purple-500/50 transition-all"
               style={{ boxShadow: '0 10px 40px -10px rgba(0, 0, 0, 0.5)' }}
            >
               <div className="font-mono text-sm text-gray-400 group-hover:text-white transition-colors">
                  <span className="text-purple-400 mr-2">$</span>
                  git clone https://github.com/Sxf0z/Nebula
               </div>
               <button className="w-8 h-8 rounded-full bg-white/10 flex items-center justify-center text-white group-hover:bg-purple-600 transition-colors shadow-lg">
                  {copied ? <Check size={14} /> : <Copy size={14} />}
               </button>
            </div>
         </div>

         {/* Animation keyframes */}
         <style jsx>{`
        @keyframes fadeIn {
          from { opacity: 0; transform: translateY(20px); }
          to { opacity: 1; transform: translateY(0); }
        }
      `}</style>
      </section>
   );
};