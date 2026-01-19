import fs from "fs";
import path from "path";
import matter from "gray-matter";
import { format } from "date-fns";

const docsDirectory = path.join(process.cwd(), "content");

export type Doc = {
    slug: string;
    title: string;
    date?: string;
    content: string;
};

export function getAllDocs(): Doc[] {
    const fileNames = fs.readdirSync(docsDirectory);
    const allDocsData = fileNames.map((fileName) => {
        const slug = fileName.replace(/\.mdx$/, "");
        const fullPath = path.join(docsDirectory, fileName);
        const fileContents = fs.readFileSync(fullPath, "utf8");
        const { data, content } = matter(fileContents);

        return {
            slug,
            title: data.title,
            date: data.date ? format(data.date, 'MMMM d, yyyy') : undefined,
            content,
        };
    });

    return allDocsData;
}

export function getDocBySlug(slug: string): Doc | undefined {
    try {
        const fullPath = path.join(docsDirectory, `${slug}.mdx`);
        const fileContents = fs.readFileSync(fullPath, "utf8");
        const { data, content } = matter(fileContents);

        return {
            slug,
            title: data.title,
            date: data.date ? format(data.date, 'MMMM d, yyyy') : undefined,
            content,
        };
    } catch (e) {
        return undefined;
    }
}
