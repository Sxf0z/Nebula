"use client";

import Link from "next/link";
import { ArrowLeft } from "lucide-react";

export default function NotFound() {
    return (
        <div className="relative min-h-screen flex flex-col items-center justify-center overflow-hidden bg-background text-foreground">

            {/* Dynamic Starry Background - Monochrome */}
            <div className="absolute inset-0 bg-black">

                {/* Static Stars Layer */}
                <div className="absolute inset-0 z-0 opacity-100"
                    style={{
                        backgroundImage: 'radial-gradient(1.5px 1.5px at 20px 20px, white 100%, transparent 0)',
                        backgroundSize: '40px 40px'
                    }}
                />

                <div className="absolute inset-0 z-0 opacity-60"
                    style={{
                        backgroundImage: 'radial-gradient(1px 1px at 80px 40px, white 100%, transparent 0)',
                        backgroundSize: '120px 120px'
                    }}
                />

                {/* Glowing Stars */}
                <div className="absolute inset-0 z-0 animate-pulse opacity-40"
                    style={{
                        backgroundImage: 'radial-gradient(3px 3px at 150px 100px, white 50%, transparent 0)',
                        backgroundSize: '250px 250px'
                    }}
                />

                {/* Shooting Stars Animation - Pure White */}
                <div className="absolute top-0 left-1/2 w-[2px] h-[150px] bg-gradient-to-b from-transparent via-white to-transparent rotate-[45deg] animate-[shooting-star_4s_infinite_ease-in-out]"
                    style={{ top: '-150px', left: '25%' }} />
                <div className="absolute top-0 left-1/2 w-[3px] h-[200px] bg-gradient-to-b from-transparent via-white to-transparent rotate-[45deg] animate-[shooting-star_6s_infinite_2s_ease-in-out] opacity-70"
                    style={{ top: '-100px', left: '70%' }} />
            </div>

            <div className="relative z-10 text-center space-y-4">
                <div className="inline-block border border-red-500/50 bg-red-900/10 px-4 py-1 text-red-400 font-mono text-sm uppercase tracking-widest animate-pulse">
                    Critical Failure
                </div>
                <h1 className="text-8xl md:text-9xl font-bold font-mono tracking-tighter text-white select-none">
                    404
                </h1>

                <div className="space-y-4">
                    <h2 className="text-2xl md:text-3xl font-mono uppercase tracking-widest text-white/80">
                        Signal Lost
                    </h2>
                    <p className="text-zinc-400 max-w-md mx-auto font-mono text-sm">
                        The requested coordinates are empty.
                        System cannot locate vector.
                    </p>
                </div>

                <Link
                    href="/"
                    className="inline-flex items-center gap-2 mt-8 px-8 py-3 border border-white/20 text-white font-mono font-bold uppercase tracking-tight hover:bg-white hover:text-black transition-all hover:scale-105 duration-200"
                >
                    <ArrowLeft className="w-4 h-4" />
                    Return to Orbit
                </Link>
            </div>

            <style jsx global>{`
        @keyframes shooting-star {
            0% {
                transform: translateX(0) translateY(0) rotate(45deg);
                opacity: 1;
            }
            100% {
                transform: translateX(500px) translateY(500px) rotate(45deg);
                opacity: 0;
            }
        }
      `}</style>
        </div>
    );
}
