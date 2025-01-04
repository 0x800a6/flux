class DocSearch {
    constructor() {
        this.searchIndex = null;
        this.searchResults = document.getElementById('searchResults');
        this.searchInput = document.getElementById('docsSearch');
        this.init();
    }

    async init() {
        try {
            const response = await fetch('/search-index.json');
            this.searchIndex = await response.json();
            this.setupListeners();
        } catch (error) {
            console.error('Failed to load search index:', error);
        }
    }

    setupListeners() {
        if (!this.searchInput) return;

        this.searchInput.addEventListener('input', debounce((e) => {
            const query = e.target.value;
            if (query.length < 2) {
                this.clearResults();
                return;
            }
            this.performSearch(query);
        }, 300));
    }

    performSearch(query) {
        if (!this.searchIndex) return;

        const results = this.searchIndex.filter(item => {
            const searchable = `${item.title} ${item.content}`.toLowerCase();
            return searchable.includes(query.toLowerCase());
        }).slice(0, 5);

        this.displayResults(results);
    }

    displayResults(results) {
        if (!this.searchResults) return;

        if (results.length === 0) {
            this.searchResults.innerHTML = `
                <div class="search-empty">
                    <p>No results found</p>
                </div>
            `;
            return;
        }

        this.searchResults.innerHTML = results.map(result => `
            <a href="${result.url}" class="search-result">
                <h6>${result.title}</h6>
                <p>${this.truncate(result.content, 100)}</p>
            </a>
        `).join('');
    }

    clearResults() {
        if (this.searchResults) {
            this.searchResults.innerHTML = '';
        }
    }

    truncate(text, length) {
        if (text.length <= length) return text;
        return text.substring(0, length) + '...';
    }
}

new DocSearch(); 