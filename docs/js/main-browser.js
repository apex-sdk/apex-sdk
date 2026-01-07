// Apex SDK - Modular Main JavaScript File
// Organized into namespaces for better structure without ES6 modules

// Create global namespace
window.ApexSDK = window.ApexSDK || {};

// Theme Toggle Component
window.ApexSDK.ThemeToggle = class {
    constructor() {
        this.toggle = document.getElementById('themeToggle');
        this.html = document.documentElement;
        this.iconMoon = document.getElementById('theme-icon-moon');
        this.iconSun = document.getElementById('theme-icon-sun');
        
        this.init();
    }

    init() {
        if (!this.toggle) return;

        const savedTheme = localStorage.getItem('theme') || 'dark';
        this.setTheme(savedTheme);

        this.toggle.addEventListener('click', () => {
            const currentTheme = this.html.getAttribute('data-theme');
            const newTheme = currentTheme === 'dark' ? 'light' : 'dark';
            this.setTheme(newTheme);
        });
    }

    setTheme(theme) {
        this.html.setAttribute('data-theme', theme);
        localStorage.setItem('theme', theme);

        if (theme === 'dark') {
            if (this.iconMoon) this.iconMoon.style.display = 'none';
            if (this.iconSun) this.iconSun.style.display = 'block';
        } else {
            if (this.iconMoon) this.iconMoon.style.display = 'block';
            if (this.iconSun) this.iconSun.style.display = 'none';
        }
    }

    getCurrentTheme() {
        return this.html.getAttribute('data-theme');
    }
};

// Mobile Menu Component
window.ApexSDK.MobileMenu = class {
    constructor() {
        this.toggle = document.getElementById('mobileMenuToggle');
        this.menu = document.getElementById('navMenu');
        this.hamburgerLines = this.toggle?.querySelectorAll('.hamburger-line');
        
        this.init();
    }

    init() {
        if (!this.toggle || !this.menu) return;

        this.toggle.addEventListener('click', () => this.toggleMenu());

        this.menu.querySelectorAll('a').forEach(link => {
            link.addEventListener('click', () => {
                if (this.menu.classList.contains('active')) {
                    this.closeMenu();
                }
            });
        });

        document.addEventListener('click', (event) => {
            if (!this.menu.contains(event.target) && 
                !this.toggle.contains(event.target) && 
                this.menu.classList.contains('active')) {
                this.closeMenu();
            }
        });

        document.addEventListener('keydown', (event) => {
            if (event.key === 'Escape' && this.menu.classList.contains('active')) {
                this.closeMenu();
                this.toggle.focus();
            }
        });
    }

    toggleMenu() {
        const isExpanded = this.toggle.getAttribute('aria-expanded') === 'true';
        
        if (isExpanded) {
            this.closeMenu();
        } else {
            this.openMenu();
        }
    }

    openMenu() {
        this.toggle.setAttribute('aria-expanded', 'true');
        this.menu.classList.add('active');
        this.animateHamburger(true);
        
        const firstLink = this.menu.querySelector('a');
        if (firstLink) firstLink.focus();
    }

    closeMenu() {
        this.toggle.setAttribute('aria-expanded', 'false');
        this.menu.classList.remove('active');
        this.animateHamburger(false);
    }

    animateHamburger(isOpen) {
        if (!this.hamburgerLines) return;

        if (isOpen) {
            this.hamburgerLines[0].style.transform = 'rotate(45deg) translate(5px, 5px)';
            this.hamburgerLines[1].style.opacity = '0';
            this.hamburgerLines[2].style.transform = 'rotate(-45deg) translate(5px, -5px)';
        } else {
            this.hamburgerLines[0].style.transform = 'none';
            this.hamburgerLines[1].style.opacity = '1';
            this.hamburgerLines[2].style.transform = 'none';
        }
    }
};

// Scroll Effects Component
window.ApexSDK.ScrollEffects = class {
    constructor() {
        this.navbar = document.getElementById('navbar');
        this.observerOptions = {
            threshold: 0.1,
            rootMargin: '0px 0px -50px 0px'
        };
        
        this.init();
    }

    init() {
        this.initNavbarScroll();
        this.initIntersectionObserver();
        this.removeLoadingClass();
    }

    initNavbarScroll() {
        if (!this.navbar) return;

        let ticking = false;

        const updateNavbar = () => {
            if (window.scrollY > 20) {
                this.navbar.classList.add('scrolled');
            } else {
                this.navbar.classList.remove('scrolled');
            }
            ticking = false;
        };

        window.addEventListener('scroll', () => {
            if (!ticking) {
                requestAnimationFrame(updateNavbar);
                ticking = true;
            }
        }, { passive: true });
    }

    initIntersectionObserver() {
        const observer = new IntersectionObserver((entries) => {
            entries.forEach(entry => {
                if (entry.isIntersecting) {
                    entry.target.classList.add('revealed');
                    observer.unobserve(entry.target);
                }
            });
        }, this.observerOptions);

        const scrollElements = document.querySelectorAll('.scroll-reveal');
        scrollElements.forEach(el => {
            observer.observe(el);
        });

        this.intersectionObserver = observer;
    }

    removeLoadingClass() {
        setTimeout(() => {
            document.body.classList.remove('loading');
        }, 100);
    }

    destroy() {
        if (this.intersectionObserver) {
            this.intersectionObserver.disconnect();
        }
    }
};

// Chain Data and Renderer
window.ApexSDK.chainsData = [
    { name: 'Polkadot', type: 'Substrate', logo: 'polkadot.svg', url: 'https://polkadot.network', description: 'Scalable multi-chain network', status: 'active' },
    { name: 'Ethereum', type: 'EVM', logo: 'ethereum.svg', url: 'https://ethereum.org', description: 'Smart contract platform', status: 'active' },
    { name: 'Kusama', type: 'Substrate', logo: 'kusama.svg', url: 'https://kusama.network', description: 'Polkadot\'s canary network', status: 'active' },
    { name: 'Polygon', type: 'EVM', logo: 'polygon.svg', url: 'https://polygon.technology', description: 'Ethereum scaling solution', status: 'active' },
    { name: 'Moonbeam', type: 'Hybrid', logo: 'moonbeam.svg', url: 'https://moonbeam.network', description: 'Ethereum on Polkadot', status: 'active' },
    { name: 'Binance Smart Chain', type: 'EVM', logo: 'bsc.svg', url: 'https://www.bnbchain.org', description: 'High-performance blockchain', status: 'active' },
    { name: 'Astar', type: 'Hybrid', logo: 'astar.svg', url: 'https://astar.network', description: 'Multi-chain dApp hub', status: 'active' },
    { name: 'Avalanche', type: 'EVM', logo: 'avalanche.svg', url: 'https://www.avax.network', description: 'Fast consensus protocol', status: 'active' },
    { name: 'Acala', type: 'Substrate', logo: 'acala.svg', url: 'https://acala.network', description: 'DeFi hub for Polkadot', status: 'active' },
    { name: 'Arbitrum', type: 'EVM', logo: 'arbitrum.svg', url: 'https://arbitrum.io', description: 'Layer 2 for Ethereum', status: 'active' },
    { name: 'Moonriver', type: 'Hybrid', logo: 'moonriver.svg', url: 'https://moonbeam.network/networks/moonriver', description: 'Ethereum on Kusama', status: 'active' },
    { name: 'Optimism', type: 'EVM', logo: 'optimism.svg', url: 'https://optimism.io', description: 'Optimistic Ethereum', status: 'active' },
    { name: 'Parallel', type: 'Substrate', logo: 'parallel.svg', url: 'https://parallel.fi', description: 'DeFi super app', status: 'active' },
    { name: 'Fantom', type: 'EVM', logo: 'fantom.svg', url: 'https://fantom.foundation', description: 'High-speed consensus', status: 'active' },
    { name: 'Centrifuge', type: 'Substrate', logo: 'centrifuge.svg', url: 'https://centrifuge.io', description: 'Real-world assets on-chain', status: 'active' },
    { name: 'Base', type: 'EVM', logo: 'base.svg', url: 'https://base.org', description: 'Coinbase Layer 2', status: 'active' },
    { name: 'Sepolia', type: 'EVM', logo: 'sepolia.svg', url: 'https://sepolia.dev', description: 'Ethereum testnet', status: 'testnet' }
];

// Chain Renderer
window.ApexSDK.ChainRenderer = class {
    constructor(containerId = 'chains-grid') {
        this.container = document.getElementById(containerId);
        this.chains = window.ApexSDK.chainsData;
        this.currentFilter = 'all';
        
        this.init();
    }

    init() {
        if (!this.container) return;
        
        this.render();
        this.initFilter();
    }

    render(chainsToRender = this.chains) {
        if (!this.container) return;

        this.container.innerHTML = chainsToRender.map(chain => this.createChainCard(chain)).join('');
    }

    createChainCard(chain) {
        const statusClass = chain.status === 'testnet' ? 'testnet' : '';
        
        return `
            <a href="${chain.url}"
               target="_blank"
               rel="noopener"
               class="chain-card scroll-reveal ${statusClass}"
               data-type="${chain.type}"
               aria-label="Learn more about ${chain.name}">
                <div class="chain-logo">
                    <img src="assets/logos/${chain.logo}"
                         alt="${chain.name} logo"
                         loading="lazy"
                         onerror="this.style.display='none'; this.parentElement.innerHTML='<div class=&quot;chain-logo-fallback&quot;>${chain.name.charAt(0)}</div>';">
                </div>
                <div class="chain-info">
                    <div class="chain-name">${chain.name}</div>
                    <div class="chain-description">${chain.description}</div>
                    <div class="chain-type-badge ${chain.type.toLowerCase()}">${chain.type}</div>
                    ${chain.status === 'testnet' ? '<div class="chain-status-badge">Testnet</div>' : ''}
                </div>
                <div class="chain-arrow" aria-hidden="true">
                    <svg width="16" height="16" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
                        <path d="M7 17L17 7M17 7H7M17 7V17"/>
                    </svg>
                </div>
            </a>
        `;
    }

    initFilter() {
        const filterButtons = document.querySelectorAll('.chain-filter');
        
        filterButtons.forEach(button => {
            button.addEventListener('click', () => {
                const filter = button.dataset.filter;
                this.setActiveFilter(button, filter);
                this.filterChains(filter);
            });
        });

        this.filterChains('all');
    }

    setActiveFilter(activeButton, filter) {
        const filterButtons = document.querySelectorAll('.chain-filter');
        filterButtons.forEach(btn => {
            btn.classList.remove('active');
            btn.setAttribute('aria-selected', 'false');
        });
        activeButton.classList.add('active');
        activeButton.setAttribute('aria-selected', 'true');
        this.currentFilter = filter;
        
        // Announce filter change to screen readers
        const announcement = `Showing ${filter === 'all' ? 'all networks' : filter + ' networks'}`;
        this.announceToScreenReader(announcement);
    }

    filterChains(filter) {
        const chainCards = document.querySelectorAll('.chain-card');

        chainCards.forEach((card, index) => {
            const chainType = card.dataset.type;
            let shouldShow = false;

            switch (filter) {
                case 'all': shouldShow = true; break;
                case 'substrate': shouldShow = chainType === 'Substrate'; break;
                case 'evm': shouldShow = chainType === 'EVM'; break;
                case 'hybrid': shouldShow = chainType === 'Hybrid'; break;
                default: shouldShow = true;
            }

            if (shouldShow) {
                card.style.display = 'flex';
                card.setAttribute('aria-hidden', 'false');
                card.style.opacity = '0';
                
                setTimeout(() => {
                    card.style.transition = 'opacity 0.3s ease';
                    card.style.opacity = '1';
                }, index * 50);
            } else {
                card.style.transition = 'opacity 0.3s ease';
                card.style.opacity = '0';
                card.setAttribute('aria-hidden', 'true');
                
                setTimeout(() => {
                    card.style.display = 'none';
                }, 300);
            }
        });
    }

    // Announce to screen readers
    announceToScreenReader(message) {
        const announcer = document.createElement('div');
        announcer.setAttribute('aria-live', 'polite');
        announcer.setAttribute('aria-atomic', 'true');
        announcer.className = 'sr-only';
        announcer.textContent = message;
        
        document.body.appendChild(announcer);
        
        setTimeout(() => {
            document.body.removeChild(announcer);
        }, 1000);
    }
};

// Documentation Viewer
window.ApexSDK.DocsViewer = class {
    constructor() {
        this.docsList = [
            { name: 'Quick Start', file: 'QUICK_START.md' },
            { name: 'API Reference', file: 'API.md' },
            { name: 'CLI Guide', file: 'CLI_GUIDE.md' },
            { name: 'System Architecture', file: 'SYSTEM_ARCHITECTURE.md' },
            { name: 'Testing Framework', file: 'TESTING_FRAMEWORK.md' },
            { name: 'Roadmap', file: 'ROADMAP.md' },
            { name: 'Contributing', file: 'CONTRIBUTING.md' },
            { name: 'Security', file: 'SECURITY.md' }
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
    }

    buildNavigation() {
        this.navElement.innerHTML = this.docsList.map(doc => 
            `<a href="viewer.html?doc=${doc.file}" 
                class="doc-nav-item ${this.getActiveClass(doc.file)}"
                data-file="${doc.file}">
                ${doc.name}
            </a>`
        ).join('');

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
            const html = marked.parse(markdown);
            this.contentElement.innerHTML = html;
            
            if (typeof hljs !== 'undefined') {
                this.contentElement.querySelectorAll('pre code').forEach((block) => {
                    hljs.highlightElement(block);
                });
            }
            
            this.addCopyButtons();
        } else {
            this.contentElement.innerHTML = `<pre>${this.escapeHtml(markdown)}</pre>`;
        }
        
        this.contentElement.scrollTop = 0;
    }

    addCopyButtons() {
        const codeBlocks = this.contentElement.querySelectorAll('pre code');
        codeBlocks.forEach(block => {
            const pre = block.parentElement;
            const button = document.createElement('button');
            button.className = 'copy-code-btn';
            button.innerHTML = '📋';
            button.title = 'Copy code';
            
            button.addEventListener('click', async () => {
                try {
                    await navigator.clipboard.writeText(block.textContent);
                    button.innerHTML = '✅';
                    setTimeout(() => button.innerHTML = '📋', 2000);
                } catch (err) {
                    console.error('Failed to copy text: ', err);
                }
            });
            
            pre.style.position = 'relative';
            pre.appendChild(button);
        });
    }

    showLoadingState() {
        this.contentElement.innerHTML = '<div class="loading">Loading documentation...</div>';
    }

    showErrorState(filename, errorMessage) {
        this.contentElement.innerHTML = `
            <div class="error">
                <h2>Error Loading Document</h2>
                <p>Could not load <code>${this.escapeHtml(filename)}</code>.</p>
                <p><strong>Error:</strong> ${this.escapeHtml(errorMessage)}</p>
                <button onclick="location.reload()" class="btn btn-primary">Try Again</button>
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
    }

    performSearch(query) {
        const navItems = document.querySelectorAll('.doc-nav-item');

        navItems.forEach(item => {
            const text = item.textContent.toLowerCase();
            item.style.display = (!query || text.includes(query)) ? 'block' : 'none';
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
};

// Main Application
window.ApexSDK.App = class {
    constructor() {
        this.components = {};
        this.modules = {};
        this.isInitialized = false;
        
        this.init();
    }

    init() {
        if (document.readyState === 'loading') {
            document.addEventListener('DOMContentLoaded', () => this.initializeComponents());
        } else {
            this.initializeComponents();
        }
    }

    initializeComponents() {
        try {
            // Initialize error handling first
            if (window.ApexSDK.ErrorHandler) {
                this.errorHandler = new window.ApexSDK.ErrorHandler();
            }

            // Initialize performance optimization
            if (window.ApexSDK.Performance) {
                this.performance = new window.ApexSDK.Performance();
            }

            // Initialize core components
            this.components.themeToggle = new window.ApexSDK.ThemeToggle();
            this.components.mobileMenu = new window.ApexSDK.MobileMenu();
            this.components.scrollEffects = new window.ApexSDK.ScrollEffects();

            // Initialize page-specific modules
            this.initializePageSpecificModules();
            
            // Initialize general features
            this.initializeGeneralFeatures();
            
            this.isInitialized = true;
            
        } catch (error) {
            console.error('Failed to initialize components:', error);
        }
    }

    initializePageSpecificModules() {
        if (window.location.pathname.includes('viewer.html') || document.getElementById('doc-content')) {
            this.modules.docsViewer = new window.ApexSDK.DocsViewer();
        }

        if (document.getElementById('chains-grid')) {
            this.modules.chainRenderer = new window.ApexSDK.ChainRenderer();
        }
    }

    initializeGeneralFeatures() {
        this.setCurrentYear();
        this.initializeCodeTabs();
        this.initializeNetworkExplorer();
    }

    setCurrentYear() {
        const yearElement = document.getElementById('current-year');
        if (yearElement) {
            yearElement.textContent = new Date().getFullYear();
        }
    }

    initializeCodeTabs() {
        const tabs = document.querySelectorAll('.code-tab');
        const codeBlocks = document.querySelectorAll('.code-block');
        
        if (!tabs.length || !codeBlocks.length) return;

        tabs.forEach(tab => {
            tab.addEventListener('click', () => {
                const targetTab = tab.dataset.tab;
                
                tabs.forEach(t => t.classList.remove('active'));
                tab.classList.add('active');
                
                codeBlocks.forEach(block => {
                    block.classList.remove('active');
                    if (block.dataset.content === targetTab) {
                        block.classList.add('active');
                    }
                });
            });
        });
    }

    initializeNetworkExplorer() {
        const tabs = document.querySelectorAll('.explorer-tab');
        const panels = document.querySelectorAll('.ecosystem-panel');
        
        if (!tabs.length || !panels.length) return;

        tabs.forEach(tab => {
            tab.addEventListener('click', () => {
                const ecosystem = tab.dataset.ecosystem;
                
                tabs.forEach(t => t.classList.remove('active'));
                tab.classList.add('active');
                
                panels.forEach(panel => {
                    panel.classList.remove('active');
                    if (panel.dataset.panel === ecosystem) {
                        panel.classList.add('active');
                    }
                });
            });
        });
    }
};

// Initialize the application
const app = new window.ApexSDK.App();
window.ApexSDKApp = app;