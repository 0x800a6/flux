const fs = require('fs');
const path = require('path');
const { walk } = require('./utils/walk');

const srcDir = path.join(__dirname, '..', 'src');
const docsDir = path.join(__dirname, '..', 'www', '_docs');
const codebaseDir = path.join(docsDir, 'codebase');

/**
 * Extracts documentation comments from Rust files
 * @param {string} content - File content
 * @returns {Object} Extracted documentation
 */
function extractDocs(content) {
    const docs = {
        description: '',
        functions: [],
        structs: [],
        traits: [],
        enums: [],
        modules: [],
        examples: [],
        dependencies: [],
        tests: []
    };

    // Extract dependencies from Cargo.toml references
    const dependencyRegex = /use\s+([a-zA-Z_][a-zA-Z0-9_:]*)\s*;/g;
    let depMatches;
    while ((depMatches = dependencyRegex.exec(content)) !== null) {
        const dep = depMatches[1].split('::')[0];
        if (!docs.dependencies.includes(dep)) {
            docs.dependencies.push(dep);
        }
    }

    // Extract module-level documentation
    const moduleDocRegex = /^(?:\/\/\/[^\n]*\n)+/;
    const moduleDoc = content.match(moduleDocRegex);
    if (moduleDoc) {
        docs.description = moduleDoc[0]
            .split('\n')
            .map(line => line.replace('///', '').trim())
            .join('\n');
    }

    // Extract test functions
    const testRegex = /#\[test\][^{]*fn\s+([a-zA-Z_][a-zA-Z0-9_]*)/g;
    let testMatches;
    while ((testMatches = testRegex.exec(content)) !== null) {
        docs.tests.push({
            name: testMatches[1],
            description: 'Unit test for functionality verification'
        });
    }

    // Match doc comments and associated items with improved regex
    const docRegex = /\/\/\/[^\n]*(?:\n\/\/\/[^\n]*)*/g;
    const itemRegex = /(?:pub(?:\([^)]+\))?\s+)?(?:fn|struct|trait|impl|enum)\s+([a-zA-Z_][a-zA-Z0-9_]*)/;
    const argRegex = /\((.*?)\)/;
    const returnRegex = /\s*->\s*(.+?)(?:\s*where\s*|{|$)/;
    const whereRegex = /where\s+([^{]+)/;

    let matches = content.match(new RegExp(docRegex.source + '\\s*' + itemRegex.source, 'g')) || [];
    
    matches.forEach(match => {
        const docLines = match.match(docRegex)[0]
            .split('\n')
            .map(line => line.replace('///', '').trim());
        
        const itemMatch = match.match(itemRegex);
        
        if (itemMatch) {
            const [fullMatch, name] = itemMatch;
            const doc = {
                name,
                description: '',
                arguments: [],
                returns: '',
                examples: [],
                whereClause: '',
                visibility: fullMatch.includes('pub') ? 'public' : 'private',
                deprecated: docLines.some(line => line.includes('#[deprecated')),
                since: '',
                attributes: []
            };

            // Extract attributes
            const attrRegex = /#\[([^\]]+)\]/g;
            let attrMatch;
            while ((attrMatch = attrRegex.exec(match)) !== null) {
                doc.attributes.push(attrMatch[1]);
            }

            // Parse documentation sections with improved handling
            let currentSection = 'description';
            let codeBlock = false;

            docLines.forEach(line => {
                if (line.startsWith('```')) {
                    codeBlock = !codeBlock;
                    if (currentSection === 'examples') {
                        doc.examples.push(line);
                    }
                    return;
                }

                if (line.startsWith('# Arguments')) {
                    currentSection = 'arguments';
                } else if (line.startsWith('# Returns')) {
                    currentSection = 'returns';
                } else if (line.startsWith('# Example')) {
                    currentSection = 'examples';
                } else if (line.startsWith('# Since')) {
                    currentSection = 'since';
                } else if (line.trim()) {
                    switch (currentSection) {
                        case 'description':
                            doc.description += line + '\n';
                            break;
                        case 'arguments':
                            if (line.startsWith('* `')) {
                                const [_, argName, argDesc] = line.match(/\* `([^`]+)`\s*-\s*(.+)/) || [];
                                if (argName && argDesc) {
                                    doc.arguments.push({ 
                                        name: argName, 
                                        description: argDesc,
                                        type: argName.includes(':') ? argName.split(':')[1].trim() : ''
                                    });
                                }
                            }
                            break;
                        case 'returns':
                            doc.returns += line + '\n';
                            break;
                        case 'examples':
                            doc.examples.push(line);
                            break;
                        case 'since':
                            doc.since = line.trim();
                            break;
                    }
                }
            });

            // Extract where clause if present
            const whereMatch = match.match(whereRegex);
            if (whereMatch) {
                doc.whereClause = whereMatch[1].trim();
            }

            // Clean up multiline strings
            doc.description = doc.description.trim();
            doc.returns = doc.returns.trim();

            // Add to appropriate category
            if (match.includes('fn ')) {
                docs.functions.push(doc);
            } else if (match.includes('struct ')) {
                docs.structs.push(doc);
            } else if (match.includes('trait ')) {
                docs.traits.push(doc);
            } else if (match.includes('enum ')) {
                docs.enums.push(doc);
            }
        }
    });

    return docs;
}

/**
 * Generates markdown documentation for a Rust file
 * @param {string} filePath - Path to Rust file
 * @returns {string} Markdown content
 */
function generateMarkdown(filePath) {
    const content = fs.readFileSync(filePath, 'utf8');
    const docs = extractDocs(content);
    const relativePath = path.relative(srcDir, filePath);
    const moduleName = path.basename(filePath, '.rs');
    
    let markdown = `---
layout: docs
title: ${moduleName}
description: ${docs.description.split('\n')[0] || `Documentation for ${relativePath}`}
category: Codebase
---

# ${moduleName}

\`${relativePath}\`

${docs.description}

`;

    if (docs.structs.length > 0) {
        markdown += '\n## Structures\n\n';
        docs.structs.forEach(struct => {
            markdown += `### ${struct.name}\n\n${struct.description}\n\n`;
            if (struct.arguments.length > 0) {
                markdown += '#### Fields\n\n';
                struct.arguments.forEach(arg => {
                    markdown += `- \`${arg.name}\` - ${arg.description}\n`;
                });
                markdown += '\n';
            }
        });
    }

    if (docs.functions.length > 0) {
        markdown += '\n## Functions\n\n';
        docs.functions.forEach(func => {
            markdown += `### ${func.name}\n\n${func.description}\n\n`;
            if (func.arguments.length > 0) {
                markdown += '#### Arguments\n\n';
                func.arguments.forEach(arg => {
                    markdown += `- \`${arg.name}\` - ${arg.description}\n`;
                });
                markdown += '\n';
            }
            if (func.returns) {
                markdown += `#### Returns\n\n${func.returns}\n\n`;
            }
            if (func.examples.length > 0) {
                markdown += '#### Example\n\n```rust\n';
                markdown += func.examples.join('\n');
                markdown += '\n```\n\n';
            }
        });
    }

    if (docs.traits.length > 0) {
        markdown += '\n## Traits\n\n';
        docs.traits.forEach(trait => {
            markdown += `### ${trait.name}\n\n${trait.description}\n\n`;
        });
    }

    if (docs.enums.length > 0) {
        markdown += '\n## Enums\n\n';
        docs.enums.forEach(enum_ => {
            markdown += `### ${enum_.name}\n\n${enum_.description}\n\n`;
        });
    }

    return markdown;
}

// Create docs directories if they don't exist
fs.mkdirSync(docsDir, { recursive: true });
fs.mkdirSync(codebaseDir, { recursive: true });

// Generate documentation for all Rust files
const rustFiles = walk(srcDir, '.rs');
rustFiles.forEach(file => {
    const relativePath = path.relative(srcDir, file);
    const docsPath = path.join(codebaseDir, `${relativePath.replace(/\//g, '_')}.md`);
    
    const markdown = generateMarkdown(file);
    fs.writeFileSync(docsPath, markdown);
});

// Generate index page for codebase documentation
const indexContent = `---
layout: docs
title: Codebase Documentation
description: API documentation for the Flux Shell codebase
category: Codebase
---

# Codebase Documentation

This section contains the API documentation automatically generated from the source code.

## Modules

${rustFiles.map(file => {
    const name = path.basename(file, '.rs');
    const relativePath = path.relative(srcDir, file);
    return `- [${name}](/docs/codebase/${relativePath.replace(/\//g, '_')}) - ${relativePath}`;
}).join('\n')}
`;

fs.writeFileSync(path.join(codebaseDir, 'index.md'), indexContent);

// Update the navigation
const navContent = `
<div class="nav-group">
    <h6>Codebase</h6>
    <ul>
        <li><a href="/docs/codebase">Overview</a></li>
        ${rustFiles.map(file => {
            const name = path.basename(file, '.rs');
            const relativePath = path.relative(srcDir, file);
            return `<li><a href="/docs/codebase/${relativePath.replace(/\//g, '_')}">${name}</a></li>`;
        }).join('\n        ')}
    </ul>
</div>
`;

// Update the docs layout with the navigation
const layoutPath = path.join(__dirname, '..', 'www', '_layouts', 'docs.html');
let layoutContent = fs.readFileSync(layoutPath, 'utf8');
layoutContent = layoutContent.replace(
    /<div id="codebase-nav">[\s\S]*?<\/div>/g,
    navContent
);

fs.writeFileSync(layoutPath, layoutContent);

console.log('Documentation generated successfully!');


