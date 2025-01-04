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

    // Clear existing content first
    toc.innerHTML = '<h6>On this page</h6>';

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
    const searchModal = document.getElementById('searchModal');
    const modalSearch = document.getElementById('modalSearch');
    const closeSearch = document.getElementById('closeSearch');
    const searchResults = document.getElementById('searchResults');
    
    
    if (!searchInput) return;

    // Open search modal with keyboard shortcut (Ctrl/Cmd + K)
    document.addEventListener('keydown', (e) => {
        if ((e.ctrlKey || e.metaKey) && e.key === 'k') {
            e.preventDefault();
            openSearchModal();
        }
    });

    searchInput.addEventListener('click', openSearchModal);

    // Close modal with ESC
    document.addEventListener('keydown', (e) => {
        if (e.key === 'Escape' && searchModal.style.display === 'block') {
            closeSearchModal();
        }
    });

    closeSearch.addEventListener('click', closeSearchModal);

    let selectedIndex = -1;
    let searchItems = [];

    modalSearch.addEventListener('input', debounce(async (e) => {
        const searchTerm = e.target.value.toLowerCase();
        if (searchTerm.length < 2) {
            searchResults.innerHTML = '';
            return;
        }

        const results = await searchDocs(searchTerm);
        displaySearchResults(results, searchResults);
    }, 300));

    // Keyboard navigation
    modalSearch.addEventListener('keydown', (e) => {
        const items = searchResults.querySelectorAll('li');
        
        switch(e.key) {
            case 'ArrowDown':
                e.preventDefault();
                selectedIndex = Math.min(selectedIndex + 1, items.length - 1);
                updateSelection(items);
                break;
            case 'ArrowUp':
                e.preventDefault();
                selectedIndex = Math.max(selectedIndex - 1, -1);
                updateSelection(items);
                break;
            case 'Enter':
                e.preventDefault();
                if (selectedIndex >= 0 && searchItems[selectedIndex]) {
                    window.location.href = searchItems[selectedIndex].url;
                }
                break;
        }
    });

    function openSearchModal() {
        searchModal.style.display = 'block';
        modalSearch.focus();
        document.body.style.overflow = 'hidden';
    }

    function closeSearchModal() {
        searchModal.style.display = 'none';
        document.body.style.overflow = '';
    }

    function updateSelection(items) {
        items.forEach((item, index) => {
            item.classList.toggle('selected', index === selectedIndex);
        });
    }
}

async function searchDocs(query) {
    try {
        // Create a case-insensitive regex pattern from the search query
        // Escape special regex characters and create pattern
        const pattern = new RegExp(query.replace(/[.*+?^${}()|[\]\\]/g, '\\$&'), 'i');
        
        const results = [];
        document.querySelectorAll('.docs-nav a').forEach(link => {
            const text = link.textContent;
            const url = link.getAttribute('href');
            
            // Use regex test instead of includes()
            if (pattern.test(text)) {
                // Highlight the matching part of the text
                const highlightedText = text.replace(pattern, match => `<mark>${match}</mark>`);
                results.push({ 
                    title: highlightedText, 
                    url: url,
                    icon: link.querySelector('i')?.className || 'fas fa-file-alt'
                });
            }
        });
        return results;
    } catch (e) {
        // In case of invalid regex, fall back to normal string search
        console.warn('Invalid regex pattern, falling back to normal search');
        return fallbackSearch(query);
    }
}

function fallbackSearch(query) {
    const results = [];
    const searchTerm = query.toLowerCase();
    document.querySelectorAll('.docs-nav a').forEach(link => {
        const text = link.textContent.toLowerCase();
        const url = link.getAttribute('href');
        if (text.includes(searchTerm)) {
            results.push({ 
                title: link.textContent, 
                url: url,
                icon: link.querySelector('i')?.className || 'fas fa-file-alt'
            });
        }
    });
    return results;
}

function displaySearchResults(results, container) {
    container.innerHTML = '';
    
    if (results.length === 0) {
        container.innerHTML = '<div class="no-results">No results found</div>';
        return;
    }

    const ul = document.createElement('ul');
    results.forEach((result) => {
        const li = document.createElement('li');
        li.innerHTML = `
            <a href="${result.url}" class="search-result-item">
                <i class="${result.icon}"></i>
                <span>${result.title}</span>
            </a>
        `;
        ul.appendChild(li);
    });
    
    container.appendChild(ul);
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