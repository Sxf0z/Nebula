import React from 'react';

export const Logo: React.FC<{ className?: string }> = ({ className = "w-10 h-10" }) => (
  <svg viewBox="0 0 100 100" className={className} fill="none" xmlns="http://www.w3.org/2000/svg">
    <defs>
      <linearGradient id="starGradient" x1="0%" y1="0%" x2="100%" y2="100%">
        <stop offset="0%" stopColor="#9333EA" />
        <stop offset="50%" stopColor="#3B82F6" />
        <stop offset="100%" stopColor="#06B6D4" />
      </linearGradient>
      <filter id="glow">
        <feGaussianBlur stdDeviation="2.5" result="coloredBlur" />
        <feMerge>
          <feMergeNode in="coloredBlur" />
          <feMergeNode in="SourceGraphic" />
        </feMerge>
      </filter>
    </defs>

    <path
      d="M50 8
         C52 8 54 18 55 22
         L62 38
         C63 40 65 41 67 41
         L85 41
         C90 41 92 48 88 51
         L73 62
         C71 63 70 66 71 68
         L76 85
         C78 90 72 94 68 91
         L53 82
         C51 81 49 81 47 82
         L32 91
         C28 94 22 90 24 85
         L29 68
         C30 66 29 63 27 62
         L12 51
         C8 48 10 41 15 41
         L33 41
         C35 41 37 40 38 38
         L45 22
         C46 18 48 8 50 8
         Z"
      fill="url(#starGradient)"
      filter="url(#glow)"
      stroke="url(#starGradient)"
      strokeWidth="2"
      strokeLinejoin="round"
    />

    <path
      d="M30 60 Q 50 50 70 60"
      stroke="rgba(255,255,255,0.2)"
      strokeWidth="3"
      strokeLinecap="round"
      fill="none"
    />
    <path
      d="M25 50 Q 50 40 75 50"
      stroke="rgba(255,255,255,0.1)"
      strokeWidth="2"
      strokeLinecap="round"
      fill="none"
    />
  </svg>
);