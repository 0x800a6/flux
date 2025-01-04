document.addEventListener('DOMContentLoaded', () => {
    // Initialize code syntax highlighting with highlight.js
    hljs.highlightAll();

    // Initialize tooltips
    const tooltips = document.querySelectorAll('[data-bs-toggle="tooltip"]');
    tooltips.forEach(tooltip => new bootstrap.Tooltip(tooltip));

    // Table of Contents generation
    generateTableOfContents();

    // Copy code buttons
    setupCodeCopyButtons();

    // Search functionality
    setupSearch();
});

function generateTableOfContents() {
    const article = document.querySelector('.docs-article');
    const toc = document.getElementById('tableOfContents');
    
    if (!article || !toc) return;

    const headings = article.querySelectorAll('h2, h3');
    const tocList = document.createElement('ul');
    tocList.className = 'toc-list';

    headings.forEach(heading => {
        const li = document.createElement('li');
        const a = document.createElement('a');
        
        // Create ID if not exists
        if (!heading.id) {
            heading.id = heading.textContent.toLowerCase()
                .replace(/[^a-z0-9]+/g, '-');
        }

        a.href = `#${heading.id}`;
        a.textContent = heading.textContent;
        a.className = heading.tagName === 'H3' ? 'toc-sub-item' : 'toc-item';
        
        li.appendChild(a);
        tocList.appendChild(li);
    });

    toc.appendChild(tocList);
}

function setupCodeCopyButtons() {
    document.querySelectorAll('pre code').forEach(block => {
        const button = document.createElement('button');
        button.className = 'copy-button';
        button.innerHTML = '<i class="far fa-copy"></i>';
        
        block.parentNode.appendChild(button);
        
        button.addEventListener('click', async () => {
            await navigator.clipboard.writeText(block.textContent);
            button.innerHTML = '<i class="fas fa-check"></i>';
            setTimeout(() => {
                button.innerHTML = '<i class="far fa-copy"></i>';
            }, 2000);
        });
    });
}

function setupSearch() {
    const searchInput = document.getElementById('docsSearch');
    if (!searchInput) return;

    searchInput.addEventListener('input', debounce(async (e) => {
        const query = e.target.value;
        if (query.length < 2) return;

        try {
            const results = await searchDocs(query);
            displaySearchResults(results);
        } catch (error) {
            console.error('Search failed:', error);
        }
    }, 300));
}

async function searchDocs(query) {
    // Implement your search logic here
    // This could use Lunr.js, Algolia, or a custom solution
}

function debounce(func, wait) {
    let timeout;
    return function executedFunction(...args) {
        const later = () => {
            clearTimeout(timeout);
            func(...args);
        };
        clearTimeout(timeout);
        timeout = setTimeout(later, wait);
    };
} 