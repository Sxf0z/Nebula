"use client";

import { useState } from "react";
import Link from "next/link";
import Image from "next/image";
import { usePathname } from "next/navigation";
import { cn } from "@/lib/utils";
import { motion, AnimatePresence } from "framer-motion";
import {
    Book,
    Code,
    Box,
    Layers,
    Zap,
    Menu,
    ChevronRight,
    Github,
    Terminal,
} from "lucide-react";

interface SidebarProps extends React.HTMLAttributes<HTMLDivElement> { }

export function Sidebar({ className }: SidebarProps) {
    const pathname = usePathname();
    const [isOpen, setIsOpen] = useState(true);
    const [expandedSections, setExpandedSections] = useState<string[]>([
        "Getting Started",
        "Language Guide",
        "Engine & Core",
    ]);

    const toggle = () => setIsOpen(!isOpen);

    const toggleSection = (title: string) => {
        setExpandedSections((prev) =>
            prev.includes(title)
                ? prev.filter((t) => t !== title)
                : [...prev, title]
        );
    };

    const sections = [
        {
            title: "Getting Started",
            items: [
                { title: "Introduction", href: "/docs/introduction", icon: Book },
                { title: "Installation", href: "/docs/installation", icon: Terminal },
                { title: "CLI Reference", href: "/docs/cli", icon: Code },
            ],
        },
        {
            title: "Language Guide",
            items: [
                { title: "Basics", href: "/docs/basics", icon: Code },
                { title: "Data Structures", href: "/docs/data-structures", icon: Layers },
                { title: "Control Flow", href: "/docs/control-flow", icon: Layers },
                { title: "Functions", href: "/docs/functions", icon: Code },
                { title: "Builtins", href: "/docs/builtins", icon: Box },
                { title: "Advanced", href: "/docs/advanced", icon: Zap },
            ],
        },
        {
            title: "Engine & Core",
            items: [
                { title: "Internals", href: "/docs/internals", icon: Box },
                { title: "Benchmarks", href: "/docs/benchmarks", icon: Zap },
                { title: "AI & ML", href: "/docs/ai-ml", icon: Layers },
            ],
        },
    ];

    return (
        <>
            {/* Toggle Button - Always Hamburger Icon */}
            <motion.button
                onClick={toggle}
                className={cn(
                    "fixed top-5 z-50 p-2.5 bg-[#1a1a24] border border-[#2a2a3a] hover:bg-[#252533] rounded-md transition-all duration-300",
                    isOpen ? "left-[290px]" : "left-5"
                )}
                whileHover={{ scale: 1.05 }}
                whileTap={{ scale: 0.95 }}
                aria-label="Toggle sidebar"
            >
                <Menu className="w-4 h-4 text-[#a0a0b0]" />
            </motion.button>

            {/* Sidebar Panel */}
            <AnimatePresence>
                {isOpen && (
                    <motion.aside
                        initial={{ x: -300, opacity: 0 }}
                        animate={{ x: 0, opacity: 1 }}
                        exit={{ x: -300, opacity: 0 }}
                        transition={{
                            type: "spring",
                            stiffness: 300,
                            damping: 30,
                        }}
                        className={cn(
                            "fixed left-0 top-0 z-40 h-screen w-[280px] bg-[#13131a] border-r border-[#1e1e2a] flex flex-col",
                            className
                        )}
                    >
                        {/* Header with Logo */}
                        <div className="p-5 border-b border-[#1e1e2a]">
                            <Link href="/" className="flex items-center gap-3 group">
                                <Image
                                    src="/Logo.png"
                                    alt="Nebula Logo"
                                    width={36}
                                    height={36}
                                    className="rounded-sm"
                                />
                                <div className="flex flex-col">
                                    <span className="text-[#e8e8f0] font-semibold text-sm tracking-wide">
                                        Nebula
                                    </span>
                                    <span className="text-[10px] text-[#808090] font-mono uppercase tracking-widest">
                                        Documentation
                                    </span>
                                </div>
                            </Link>
                        </div>

                        {/* Navigation */}
                        <nav className="flex-1 overflow-y-auto py-4 px-3">
                            <div className="space-y-1">
                                {sections.map((section) => (
                                    <div key={section.title} className="mb-4">
                                        {/* Section Header */}
                                        <button
                                            onClick={() => toggleSection(section.title)}
                                            className="w-full flex items-center justify-between px-3 py-2 text-[11px] font-semibold uppercase tracking-wider text-[#707080] hover:text-[#a0a0b0] transition-colors"
                                        >
                                            <span>{section.title}</span>
                                            <motion.div
                                                animate={{
                                                    rotate: expandedSections.includes(section.title)
                                                        ? 90
                                                        : 0,
                                                }}
                                                transition={{ duration: 0.2 }}
                                            >
                                                <ChevronRight className="w-3 h-3" />
                                            </motion.div>
                                        </button>

                                        {/* Section Items */}
                                        <AnimatePresence>
                                            {expandedSections.includes(section.title) && (
                                                <motion.div
                                                    initial={{ height: 0, opacity: 0 }}
                                                    animate={{ height: "auto", opacity: 1 }}
                                                    exit={{ height: 0, opacity: 0 }}
                                                    transition={{ duration: 0.2 }}
                                                    className="overflow-hidden"
                                                >
                                                    <div className="space-y-0.5 mt-1">
                                                        {section.items.map((item, index) => {
                                                            const isActive = pathname === item.href;
                                                            return (
                                                                <motion.div
                                                                    key={item.href}
                                                                    initial={{ x: -10, opacity: 0 }}
                                                                    animate={{ x: 0, opacity: 1 }}
                                                                    transition={{ delay: index * 0.03 }}
                                                                >
                                                                    <Link
                                                                        href={item.href}
                                                                        className={cn(
                                                                            "group flex items-center gap-3 px-3 py-2 text-[13px] rounded-md transition-all duration-200",
                                                                            isActive
                                                                                ? "bg-[#252533] text-[#e8e8f0]"
                                                                                : "text-[#808090] hover:text-[#c0c0d0] hover:bg-[#1a1a24]"
                                                                        )}
                                                                    >
                                                                        <item.icon
                                                                            className={cn(
                                                                                "w-4 h-4 transition-all",
                                                                                isActive
                                                                                    ? "text-[#e8e8f0]"
                                                                                    : "text-[#606070] group-hover:text-[#909090]"
                                                                            )}
                                                                        />
                                                                        <span>{item.title}</span>
                                                                    </Link>
                                                                </motion.div>
                                                            );
                                                        })}
                                                    </div>
                                                </motion.div>
                                            )}
                                        </AnimatePresence>
                                    </div>
                                ))}
                            </div>
                        </nav>

                        {/* Footer */}
                        <div className="p-4 border-t border-[#1e1e2a]">
                            {/* GitHub Link */}
                            <a
                                href="https://github.com/Sxf0z/Nebula"
                                target="_blank"
                                rel="noopener noreferrer"
                                className="flex items-center gap-2 px-3 py-2 text-[12px] text-[#707080] hover:text-[#a0a0b0] hover:bg-[#1a1a24] rounded-md transition-all"
                            >
                                <Github className="w-4 h-4" />
                                <span>View on GitHub</span>
                            </a>

                            <div className="mt-3 px-3 text-[10px] text-[#505060] font-mono">
                                v1.0.0
                            </div>
                        </div>
                    </motion.aside>
                )}
            </AnimatePresence>

            {/* Mobile Backdrop */}
            <AnimatePresence>
                {isOpen && (
                    <motion.div
                        initial={{ opacity: 0 }}
                        animate={{ opacity: 1 }}
                        exit={{ opacity: 0 }}
                        onClick={toggle}
                        className="fixed inset-0 bg-black/50 z-30 md:hidden"
                    />
                )}
            </AnimatePresence>
        </>
    );
}
