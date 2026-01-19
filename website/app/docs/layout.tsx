"use client";

import { Sidebar } from "@/components/ui/sidebar";
import { motion } from "framer-motion";

export default function DocsLayout({
    children,
}: {
    children: React.ReactNode;
}) {
    return (
        <div className="min-h-screen" style={{ backgroundColor: "#16161e" }}>
            {/* Sidebar */}
            <Sidebar />

            {/* Main Content */}
            <main
                className="transition-all duration-300 ease-out ml-[280px]"
            >
                <div className="min-h-screen" style={{ backgroundColor: "#16161e" }}>
                    {/* Content Container */}
                    <div className="max-w-4xl mx-auto px-8 py-16 lg:py-20">
                        <motion.article
                            className="prose prose-zinc dark:prose-invert max-w-none"
                            initial={{ opacity: 0, y: 20 }}
                            animate={{ opacity: 1, y: 0 }}
                            transition={{ duration: 0.4, ease: "easeOut" }}
                        >
                            {children}
                        </motion.article>
                    </div>
                </div>
            </main>
        </div>
    );
}
