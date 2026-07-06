import { readFileSync, writeFileSync } from 'fs';
import { resolve } from 'path';
import { parse } from 'toml';
import type { IndexHtmlTransformHook } from 'vite';
import type { Plugin } from 'vite';

export function cargoMetadata() {
  return {
    name: 'vite-plugin-cargo-metadata',
    transformIndexHtml: {
      order: 'pre',
      handler(html: string) {
        const cargoTomlPath = resolve(process.cwd(), 'Cargo.toml');
        let cargoToml;
        try {
          cargoToml = parse(readFileSync(cargoTomlPath, 'utf-8'));
        } catch (e) {
          console.error(`Failed to parse Cargo.toml at ${cargoTomlPath}:`, e);
          return html;
        }

        const metadata = cargoToml.package?.metadata?.html || {};
        const version = cargoToml.package?.version;
        
        let transformedHtml = html;

        // Inject mobile-min-width into the viewport meta tag's data-min-width
        if (metadata['mobile-min-width']) {
          transformedHtml = transformedHtml.replace(
            /data-min-width="[^"]*"/,
            `data-min-width="${metadata['mobile-min-width']}"`
          );
        }

        // Inject mobile-min-height into the viewport meta tag's data-min-height
        if (metadata['mobile-min-height']) {
          transformedHtml = transformedHtml.replace(
            /data-min-height="[^"]*"/,
            `data-min-height="${metadata['mobile-min-height']}"`
          );
        }

        // Replace <title>
        if (metadata.title) {
          transformedHtml = transformedHtml.replace(
            /<title>(.*?)<\/title>/,
            `<title>${metadata.title}</title>`
          );
        }

        // Add or replace metadata tags
        const headInsertions: string[] = [];
        
        if (metadata.description) {
          headInsertions.push(`<meta name="description" content="${metadata.description}">`);
        }
        
        if (metadata.keywords) {
          headInsertions.push(`<meta name="keywords" content="${metadata.keywords}">`);
        }

        if (version) {
          headInsertions.push(
            `<meta name="version" content="${version}">`
          );
        }

        if (headInsertions.length > 0) {
          transformedHtml = transformedHtml.replace(
            /<\/head>/,
            `  ${headInsertions.join('')}
  </head>`
          );
        }

        return transformedHtml;
      }
    } as { order: 'pre', handler: IndexHtmlTransformHook },

    writeBundle() {
      const cargoTomlPath = resolve(process.cwd(), 'Cargo.toml');
      let cargoToml;
      try {
        cargoToml = parse(readFileSync(cargoTomlPath, 'utf-8'));
      } catch (e) {
        console.error(`Failed to parse Cargo.toml at ${cargoTomlPath}:`, e);
        return;
      }

      const metadata = cargoToml.package?.metadata?.html || {};
      const cname = metadata.cname;
      if (!cname) return;

      const cnamePath = resolve(process.cwd(), 'docs', 'CNAME');
      writeFileSync(cnamePath, cname + '\n', 'utf-8');
      console.log(`Wrote ${cnamePath}`);
    }
  } satisfies Plugin;
}
