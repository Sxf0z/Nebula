"use client";

import React, { useState, useEffect } from 'react';
import { Github, Menu, X, Download, Sun, Moon } from 'lucide-react';
import Image from 'next/image';
import Link from 'next/link';

interface NavbarProps {
  onNavigate: (page: 'home' | 'download') => void;
  currentPage: 'home' | 'download';
}

export const LandingNavbar: React.FC<NavbarProps> = ({ onNavigate, currentPage }) => {
  const [mobileMenuOpen, setMobileMenuOpen] = useState(false);
  const [scrolled, setScrolled] = useState(false);
  const [isDark, setIsDark] = useState(true);

  useEffect(() => {
    const handleScroll = () => setScrolled(window.scrollY > 20);
    window.addEventListener('scroll', handleScroll);
    return () => window.removeEventListener('scroll', handleScroll);
  }, []);

  const toggleTheme = () => {
    const html = document.documentElement;
    if (html.classList.contains('dark')) {
      html.classList.remove('dark');
      localStorage.setItem('theme', 'light');
      setIsDark(false);
    } else {
      html.classList.add('dark');
      localStorage.setItem('theme', 'dark');
      setIsDark(true);
    }
  };

  return (
    <>
      <header className={`fixed top-6 left-0 right-0 z-50 transition-all duration-300 flex justify-center px-4 ${scrolled ? 'transform translate-y-0' : ''}`}>
        <div className={`
          rounded-2xl px-6 py-3 flex items-center justify-between w-full max-w-6xl
          transition-all duration-300 backdrop-blur-xl border border-white/10
          ${scrolled ? 'shadow-lg bg-[#16161e]/90' : 'bg-[#16161e]/70'}
        `}>

          {/* Brand */}
          <div
            className="flex items-center gap-3 cursor-pointer group"
            onClick={() => onNavigate('home')}
          >
            <div className="relative group-hover:scale-110 transition-transform duration-300">
              <Image
                src="/Logo.png"
                alt="Nebula Logo"
                width={36}
                height={36}
                className="rounded-sm"
              />
            </div>
            <span className="font-sans font-bold text-xl tracking-tight text-white group-hover:text-purple-400 transition-colors">Nebula</span>
          </div>

          {/* Desktop Nav */}
          <nav className="hidden md:flex items-center gap-10 text-sm font-medium">
            <button
              onClick={() => onNavigate('home')}
              className={`transition-colors relative group ${currentPage === 'home' ? 'text-white font-semibold' : 'text-gray-400 hover:text-white'}`}
            >
              Overview
              {currentPage === 'home' && <span className="absolute -bottom-1 left-1/2 -translate-x-1/2 w-1.5 h-1.5 rounded-full bg-purple-500 shadow-[0_0_10px_#9333EA]"></span>}
            </button>

            <Link
              href="/docs/introduction"
              className="text-gray-400 hover:text-white transition-colors"
            >
              Documentation
            </Link>

            <button
              onClick={() => onNavigate('download')}
              className={`transition-colors relative group ${currentPage === 'download' ? 'text-white font-semibold' : 'text-gray-400 hover:text-white'}`}
            >
              Download
              {currentPage === 'download' && <span className="absolute -bottom-1 left-1/2 -translate-x-1/2 w-1.5 h-1.5 rounded-full bg-cyan-500 shadow-[0_0_10px_#06B6D4]"></span>}
            </button>

            <a
              href="https://discord.gg/3RBEyCUgpX"
              target="_blank"
              rel="noopener noreferrer"
              className="text-gray-400 hover:text-white transition-colors"
            >
              Community
            </a>
          </nav>

          {/* Actions */}
          <div className="hidden md:flex items-center gap-4">
            <a
              href="https://github.com/Sxf0z/Nebula"
              target="_blank"
              rel="noopener noreferrer"
              className="text-gray-400 hover:text-white transition-colors"
            >
              <Github size={20} />
            </a>

            <a
              href="https://discord.gg/3RBEyCUgpX"
              target="_blank"
              rel="noopener noreferrer"
              className="text-gray-400 hover:text-white transition-colors"
            >
              <svg width="20" height="20" viewBox="0 0 24 24" fill="currentColor">
                <path d="M20.317 4.3698a19.7913 19.7913 0 00-4.8851-1.5152.0741.0741 0 00-.0785.0371c-.211.3753-.4447.8648-.6083 1.2495-1.8447-.2762-3.68-.2762-5.4868 0-.1636-.3933-.4058-.8742-.6177-1.2495a.077.077 0 00-.0785-.037 19.7363 19.7363 0 00-4.8852 1.515.0699.0699 0 00-.0321.0277C.5334 9.0458-.319 13.5799.0992 18.0578a.0824.0824 0 00.0312.0561c2.0528 1.5076 4.0413 2.4228 5.9929 3.0294a.0777.0777 0 00.0842-.0276c.4616-.6304.8731-1.2952 1.226-1.9942a.076.076 0 00-.0416-.1057c-.6528-.2476-1.2743-.5495-1.8722-.8923a.077.077 0 01-.0076-.1277c.1258-.0943.2517-.1923.3718-.2914a.0743.0743 0 01.0776-.0105c3.9278 1.7933 8.18 1.7933 12.0614 0a.0739.0739 0 01.0785.0095c.1202.099.246.1981.3728.2924a.077.077 0 01-.0066.1276 12.2986 12.2986 0 01-1.873.8914.0766.0766 0 00-.0407.1067c.3604.698.7719 1.3628 1.225 1.9932a.076.076 0 00.0842.0286c1.961-.6067 3.9495-1.5219 6.0023-3.0294a.077.077 0 00.0313-.0552c.5004-5.177-.8382-9.6739-3.5485-13.6604a.061.061 0 00-.0312-.0286zM8.02 15.3312c-1.1825 0-2.1569-1.0857-2.1569-2.419 0-1.3332.9555-2.4189 2.157-2.4189 1.2108 0 2.1757 1.0952 2.1568 2.419 0 1.3332-.9555 2.4189-2.1569 2.4189zm7.9748 0c-1.1825 0-2.1569-1.0857-2.1569-2.419 0-1.3332.9554-2.4189 2.1569-2.4189 1.2108 0 2.1757 1.0952 2.1568 2.419 0 1.3332-.946 2.4189-2.1568 2.4189Z" />
              </svg>
            </a>

            <button
              onClick={() => onNavigate('download')}
              className="group relative px-5 py-2 rounded-xl overflow-hidden text-sm font-bold text-white transition-all hover:scale-105 active:scale-95 shadow-md bg-purple-600 hover:bg-purple-500"
            >
              <span className="relative flex items-center gap-2">
                Get Started <Download size={14} />
              </span>
            </button>
          </div>

          {/* Mobile Toggle */}
          <div className="flex items-center gap-4 md:hidden">
            <button className="text-white p-2" onClick={() => setMobileMenuOpen(!mobileMenuOpen)}>
              {mobileMenuOpen ? <X /> : <Menu />}
            </button>
          </div>
        </div>
      </header>

      {/* Mobile Menu */}
      {mobileMenuOpen && (
        <div className="fixed inset-0 z-40 bg-[#16161e]/95 backdrop-blur-2xl flex flex-col items-center justify-center p-6 md:hidden">
          <button className="absolute top-8 right-8 text-white" onClick={() => setMobileMenuOpen(false)}>
            <X size={32} />
          </button>
          <div className="flex flex-col gap-8 text-center text-xl font-medium text-white">
            <button onClick={() => { onNavigate('home'); setMobileMenuOpen(false); }}>Overview</button>
            <Link href="/docs/introduction" onClick={() => setMobileMenuOpen(false)}>Documentation</Link>
            <button onClick={() => { onNavigate('download'); setMobileMenuOpen(false); }}>Download</button>
            <a href="https://discord.gg/3RBEyCUgpX" target="_blank" rel="noopener noreferrer" className="text-gray-400">Community</a>
            <button
              onClick={() => { onNavigate('download'); setMobileMenuOpen(false); }}
              className="bg-purple-600 text-white py-3 px-8 rounded-full font-bold shadow-lg"
            >
              Install Nebula
            </button>
          </div>
        </div>
      )}
    </>
  );
};