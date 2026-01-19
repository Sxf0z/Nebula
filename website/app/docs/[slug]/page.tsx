import { notFound } from "next/navigation";
import { getDocBySlug, getAllDocs } from "@/lib/docs";
import { components } from "@/components/mdx";
import { CompileMDXResult, compileMDX } from "next-mdx-remote/rsc";
import rehypeHighlight from "rehype-highlight";
import rehypeSlug from "rehype-slug";
import remarkGfm from "remark-gfm";

// We need to allow dynamicparams to be true for new docs
export const dynamicParams = true;

export async function generateStaticParams() {
    const docs = getAllDocs();
    return docs.map((doc) => ({
        slug: doc.slug,
    }));
}

export default async function DocPage({
    params,
}: {
    params: Promise<{ slug: string }>;
}) {
    const { slug } = await params;

    // Explicitly decode the slug just in case, though usually not needed
    const decodedSlug = decodeURIComponent(slug);
    const doc = getDocBySlug(decodedSlug);

    if (!doc) {
        notFound();
    }

    // Use next-mdx-remote for compiling MDX
    const { content } = await compileMDX({
        source: doc.content,
        components: components,
        options: {
            parseFrontmatter: true,
            mdxOptions: {
                remarkPlugins: [remarkGfm],
                rehypePlugins: [rehypeHighlight, rehypeSlug],
            },
        },
    });

    return (
        <article className="prose prose-zinc dark:prose-invert max-w-none">
            <div className="mdx-content">
                {content}
            </div>
        </article>
    );
}
