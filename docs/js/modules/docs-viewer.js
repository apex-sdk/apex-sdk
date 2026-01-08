// Documentation Viewer Module
export class DocsViewer {
    constructor() {
        this.docsList = [
            { name: 'Quick Start', file: 'QUICK_START.md', path: 'QUICK_START.md' },
            { name: 'API Reference', file: 'API.md', path: 'API.md' },
            { name: 'CLI Guide', file: 'CLI_GUIDE.md', path: 'CLI_GUIDE.md' },
            { name: 'System Architecture', file: 'SYSTEM_ARCHITECTURE.md', path: 'SYSTEM_ARCHITECTURE.md' },
            { name: 'Testing Framework', file: 'TESTING_FRAMEWORK.md', path: 'TESTING_FRAMEWORK.md' },
            { name: 'Roadmap', file: 'ROADMAP.md', path: 'ROADMAP.md' },
            { name: 'Contributing', file: 'CONTRIBUTING.md', path: 'CONTRIBUTING.md' },
            { name: 'Security', file: 'SECURITY.md', path: 'SECURITY.md' }
        ];
        
        this.navElement = document.getElementById('doc-nav');
        this.contentElement = document.getElementById('doc-content');
        this.searchInput = document.getElementById('search-input');
        
        this.init();
    }

    init() {
        if (!this.navElement || !this.contentElement) return;
        
        this.buildNavigation();
        this.loadInitialDocument();
        this.setupSearch();
        this.setupKeyboardNavigation();
    }

    buildNavigation() {
        this.navElement.innerHTML = this.docsList.map(doc => 
            `<a href="viewer.html?doc=${doc.file}" 
                class="doc-nav-item ${this.getActiveClass(doc.file)}"
                data-file="${doc.file}"
                role="menuitem">
                ${doc.name}
            </a>`
        ).join('');

        // Add click handlers for client-side navigation
        this.navElement.addEventListener('click', (e) => {
            if (e.target.classList.contains('doc-nav-item')) {
                e.preventDefault();
                const file = e.target.dataset.file;
                this.loadDocument(file);
                this.updateUrl(file);
                this.setActiveNavItem(e.target);
            }
        });
    }

    loadInitialDocument() {
        const urlParams = new URLSearchParams(window.location.search);
        const docFile = urlParams.get('doc') || 'QUICK_START.md';
        this.loadDocument(docFile);
    }

    async loadDocument(filename) {
        if (!this.contentElement) return;

        try {
            this.showLoadingState();
            
            const response = await fetch(filename);
            if (!response.ok) {
                throw new Error(`HTTP error! status: ${response.status}`);
            }
            
            const markdown = await response.text();
            await this.renderMarkdown(markdown);
            this.updateDocumentTitle(filename);
            
        } catch (error) {
            console.error('Error loading document:', error);
            this.showErrorState(filename, error.message);
        }
    }

    async renderMarkdown(markdown) {
        if (typeof marked !== 'undefined') {
            // Configure marked for security and features
            marked.setOptions({
                breaks: true,
                gfm: true,
                headerIds: true,
                headerPrefix: 'doc-',
                sanitize: false // We trust our own markdown
            });
            
            const html = marked.parse(markdown);
            this.contentElement.innerHTML = html;
            
            // Highlight code blocks
            await this.highlightCode();
            
            // Add link handlers for internal navigation
            this.setupInternalLinks();
            
            // Add copy buttons to code blocks
            this.addCopyButtons();
            
        } else {
            // Fallback to plain text if marked is not available
            this.contentElement.innerHTML = `<pre>${this.escapeHtml(markdown)}</pre>`;
        }
        
        // Scroll to top of content
        this.contentElement.scrollTop = 0;
    }

    async highlightCode() {
        if (typeof hljs !== 'undefined') {
            const codeBlocks = this.contentElement.querySelectorAll('pre code');
            codeBlocks.forEach((block) => {
                hljs.highlightElement(block);
            });
        }
    }

    setupInternalLinks() {
        const links = this.contentElement.querySelectorAll('a[href^="#"]');
        links.forEach(link => {
            link.addEventListener('click', (e) => {
                e.preventDefault();
                const targetId = link.getAttribute('href').substring(1);
                const targetElement = document.getElementById(`doc-${targetId}`);
                if (targetElement) {
                    targetElement.scrollIntoView({ behavior: 'smooth' });
                }
            });
        });
    }

    addCopyButtons() {
        const codeBlocks = this.contentElement.querySelectorAll('pre code');
        codeBlocks.forEach(block => {
            const pre = block.parentElement;
            const button = document.createElement('button');
            button.className = 'copy-code-btn';
            button.innerHTML = `
                <svg width="16" height="16" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
                    <rect x="9" y="9" width="13" height="13" rx="2" ry="2"></rect>
                    <path d="M5 15H4a2 2 0 0 1-2-2V4a2 2 0 0 1 2-2h9a2 2 0 0 1 2 2v1"></path>
                </svg>
            `;
            button.title = 'Copy code';
            button.setAttribute('aria-label', 'Copy code to clipboard');
            
            button.addEventListener('click', async () => {
                try {
                    await navigator.clipboard.writeText(block.textContent);
                    button.innerHTML = `
                        <svg width="16" height="16" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
                            <polyline points="20,6 9,17 4,12"></polyline>
                        </svg>
                    `;
                    setTimeout(() => {
                        button.innerHTML = `
                            <svg width="16" height="16" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
                                <rect x="9" y="9" width="13" height="13" rx="2" ry="2"></rect>
                                <path d="M5 15H4a2 2 0 0 1-2-2V4a2 2 0 0 1 2-2h9a2 2 0 0 1 2 2v1"></path>
                            </svg>
                        `;
                    }, 2000);
                } catch (err) {
                    console.error('Failed to copy text: ', err);
                }
            });
            
            pre.style.position = 'relative';
            pre.appendChild(button);
        });
    }

    showLoadingState() {
        this.contentElement.innerHTML = `
            <div class="loading">
                <div class="loading-spinner"></div>
                <p>Loading documentation...</p>
            </div>
        `;
    }

    showErrorState(filename, errorMessage) {
        this.contentElement.innerHTML = `
            <div class="error">
                <h2>Error Loading Document</h2>
                <p>Could not load <code>${this.escapeHtml(filename)}</code>.</p>
                <p><strong>Error:</strong> ${this.escapeHtml(errorMessage)}</p>
                <button onclick="location.reload()" class="btn btn-primary">
                    Try Again
                </button>
            </div>
        `;
    }

    setupSearch() {
        if (!this.searchInput) return;

        let searchTimeout;
        
        this.searchInput.addEventListener('input', (e) => {
            clearTimeout(searchTimeout);
            searchTimeout = setTimeout(() => {
                this.performSearch(e.target.value.toLowerCase());
            }, 300);
        });

        // Clear search on escape
        this.searchInput.addEventListener('keydown', (e) => {
            if (e.key === 'Escape') {
                this.searchInput.value = '';
                this.performSearch('');
                this.searchInput.blur();
            }
        });
    }

    performSearch(query) {
        const navItems = document.querySelectorAll('.doc-nav-item');

        navItems.forEach(item => {
            const text = item.textContent.toLowerCase();
            if (!query || text.includes(query)) {
                item.style.display = 'block';
            } else {
                item.style.display = 'none';
            }
        });
    }

    setupKeyboardNavigation() {
        document.addEventListener('keydown', (e) => {
            if (e.ctrlKey || e.metaKey) {
                switch (e.key) {
                    case 'k':
                        e.preventDefault();
                        this.searchInput?.focus();
                        break;
                    case '/':
                        e.preventDefault();
                        this.searchInput?.focus();
                        break;
                }
            }
        });
    }

    getActiveClass(docFile) {
        const urlParams = new URLSearchParams(window.location.search);
        const currentDoc = urlParams.get('doc') || 'QUICK_START.md';
        return currentDoc === docFile ? 'active' : '';
    }

    setActiveNavItem(activeItem) {
        const navItems = document.querySelectorAll('.doc-nav-item');
        navItems.forEach(item => item.classList.remove('active'));
        activeItem.classList.add('active');
    }

    updateDocumentTitle(filename) {
        const docTitles = {
            'QUICK_START.md': 'Quick Start Guide',
            'API.md': 'API Reference',
            'CLI_GUIDE.md': 'CLI Guide',
            'SYSTEM_ARCHITECTURE.md': 'System Architecture',
            'TESTING_FRAMEWORK.md': 'Testing Framework',
            'ROADMAP.md': 'Roadmap',
            'CONTRIBUTING.md': 'Contributing',
            'SECURITY.md': 'Security'
        };
        
        const title = docTitles[filename] || 'Documentation';
        document.title = `${title} - Apex SDK`;
    }

    updateUrl(filename) {
        const newUrl = `${window.location.pathname}?doc=${filename}`;
        window.history.pushState({ doc: filename }, '', newUrl);
    }

    escapeHtml(text) {
        const div = document.createElement('div');
        div.textContent = text;
        return div.innerHTML;
    }
}