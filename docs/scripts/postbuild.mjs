// Regenerates sitemap.xml from every rendered chapter on each Pages deploy.
// Never hand-edit sitemap.xml: adding, renaming, or removing a page under
// docs/src/ (registered in SUMMARY.md) is enough. If the published site URL
// changes, update BASE_URL here together with site-url in docs/book.toml.
import { readFileSync, readdirSync, writeFileSync } from "node:fs";
import { join, dirname } from "node:path";
import { fileURLToPath } from "node:url";

const BASE_URL = "https://andreclaudino.github.io/scienceview";
const HERE = dirname(fileURLToPath(import.meta.url));
const SRC = join(HERE, "..", "src");
const OUT = join(HERE, "..", "book", "sitemap.xml");

function mdFiles(dir, base = "") {
  const entries = [];
  for (const entry of readdirSync(join(dir, base), { withFileTypes: true })) {
    const relative = base ? `${base}/${entry.name}` : entry.name;
    if (entry.isDirectory()) {
      entries.push(...mdFiles(dir, relative));
    } else if (entry.name.endsWith(".md") && entry.name !== "SUMMARY.md") {
      entries.push(relative);
    }
  }
  return entries;
}

const urls = mdFiles(SRC)
  .map((file) => file.replace(/\.md$/, ".html"))
  .map((file) => (file === "index.html" ? "" : file))
  .sort()
  .map((path) => `  <url><loc>${BASE_URL}/${path}</loc></url>`)
  .join("\n");

writeFileSync(
  OUT,
  `<?xml version="1.0" encoding="UTF-8"?>\n<urlset xmlns="http://www.sitemaps.org/schemas/sitemap/0.9">\n${urls}\n</urlset>\n`,
);
console.log(`Wrote ${OUT}`);
