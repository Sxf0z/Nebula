"use client";

import React, { useState, useEffect } from 'react';
import { LandingNavbar } from '@/components/landing/Navbar';
import { Hero } from '@/components/landing/Hero';
import { Features } from '@/components/landing/Features';
import { InteractivePlayground } from '@/components/landing/InteractivePlayground';
import { Footer } from '@/components/landing/Footer';
import { DownloadPage } from '@/components/landing/DownloadPage';

export default function Home() {
  const [currentPage, setCurrentPage] = useState<'home' | 'download'>('home');
  const [stars, setStars] = useState<Array<{ id: number, top: string, left: string, size: number, delay: number }>>([]);

  // Generate random stars on mount
  useEffect(() => {
    const generatedStars = Array.from({ length: 50 }).map((_, i) => ({
      id: i,
      top: `${Math.random() * 100}%`,
      left: `${Math.random() * 100}%`,
      size: Math.random() * 2 + 1,
      delay: Math.random() * 5,
    }));
    setStars(generatedStars);
  }, []);

  return (
    <div className="min-h-screen bg-[#0a0a12] text-white overflow-x-hidden relative selection:bg-purple-500/30 selection:text-white font-sans">

      {/* Background Effects */}
      <div className="fixed inset-0 z-0 pointer-events-none overflow-hidden">

        {/* Deep space gradient */}
        <div className="absolute inset-0 bg-gradient-to-b from-[#02010a] via-[#050314] to-[#0a0a14]"></div>

        {/* Static star field */}
        <div
          className="absolute inset-0"
          style={{
            backgroundImage: `
              radial-gradient(1.5px 1.5px at 20px 30px, rgba(255,255,255,0.4), transparent),
              radial-gradient(1.5px 1.5px at 40px 70px, rgba(255,255,255,0.3), transparent),
              radial-gradient(1px 1px at 90px 40px, rgba(255,255,255,0.5), transparent),
              radial-gradient(1.5px 1.5px at 130px 80px, rgba(255,255,255,0.4), transparent),
              radial-gradient(1px 1px at 160px 120px, rgba(255,255,255,0.3), transparent)
            `,
            backgroundSize: '200px 200px',
            opacity: 0.6,
          }}
        ></div>

        {/* Twinkling stars */}
        {stars.map((star) => (
          <div
            key={star.id}
            className="absolute rounded-full bg-white animate-pulse"
            style={{
              top: star.top,
              left: star.left,
              width: `${star.size}px`,
              height: `${star.size}px`,
              animationDelay: `${star.delay}s`,
              animationDuration: '3s',
              boxShadow: `0 0 ${star.size * 3}px rgba(255,255,255,0.8)`,
            }}
          />
        ))}

        {/* Shooting stars */}
        <div
          className="absolute w-[150px] h-[2px] bg-gradient-to-r from-transparent via-white to-transparent opacity-0"
          style={{
            top: '15%',
            right: '10%',
            transform: 'rotate(-45deg)',
            animation: 'shootingStar 4s ease-out infinite',
            animationDelay: '0s',
          }}
        />
        <div
          className="absolute w-[100px] h-[1px] bg-gradient-to-r from-transparent via-cyan-400 to-transparent opacity-0"
          style={{
            top: '35%',
            right: '30%',
            transform: 'rotate(-45deg)',
            animation: 'shootingStar 5s ease-out infinite',
            animationDelay: '2s',
          }}
        />
        <div
          className="absolute w-[180px] h-[2px] bg-gradient-to-r from-transparent via-purple-400 to-transparent opacity-0"
          style={{
            top: '55%',
            left: '5%',
            transform: 'rotate(-45deg)',
            animation: 'shootingStar 6s ease-out infinite',
            animationDelay: '4s',
          }}
        />

        {/* Orbital rings */}
        <div
          className="absolute top-1/2 left-1/2 -translate-x-1/2 -translate-y-1/2 w-[80vh] h-[80vh] rounded-full border border-white/5 opacity-20 pointer-events-none"
          style={{ animation: 'spin 60s linear infinite' }}
        />
        <div
          className="absolute top-1/2 left-1/2 -translate-x-1/2 -translate-y-1/2 w-[120vh] h-[120vh] rounded-full border border-white/5 opacity-10 pointer-events-none border-dashed"
          style={{ animation: 'spin 80s linear infinite reverse' }}
        />

        {/* Nebula glow orbs */}
        <div className="absolute top-[-20%] left-[-10%] w-[60vw] h-[60vw] bg-purple-600/15 rounded-full blur-[150px] opacity-50" />
        <div className="absolute bottom-[-10%] right-[-10%] w-[50vw] h-[50vw] bg-cyan-600/10 rounded-full blur-[120px] opacity-40" />
        <div className="absolute top-[40%] left-[30%] w-[30vw] h-[30vw] bg-blue-600/10 rounded-full blur-[100px] opacity-30" />
      </div>

      <LandingNavbar onNavigate={setCurrentPage} currentPage={currentPage} />

      <main className="relative z-10 flex flex-col items-center w-full min-h-screen">
        {currentPage === 'home' ? (
          <div className="w-full flex flex-col items-center">
            <Hero />
            <Features />
            <InteractivePlayground />
          </div>
        ) : (
          <DownloadPage />
        )}
      </main>

      <Footer />

      {/* Global animations */}
      <style jsx global>{`
        @keyframes spin {
          from { transform: translate(-50%, -50%) rotate(0deg); }
          to { transform: translate(-50%, -50%) rotate(360deg); }
        }
        @keyframes shootingStar {
          0% { transform: translateX(0) rotate(-45deg); opacity: 0; }
          5% { opacity: 1; }
          20% { transform: translateX(-300px) rotate(-45deg); opacity: 0; }
          100% { opacity: 0; }
        }
      `}</style>
    </div>
  );
}
