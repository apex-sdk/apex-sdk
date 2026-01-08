// Main Application Entry Point
// Modular Apex SDK Documentation Website

// Import components
import { ThemeToggle } from './components/theme-toggle.js';
import { MobileMenu } from './components/mobile-menu.js';
import { ScrollEffects } from './components/scroll-effects.js';

// Import modules
import { ChainRenderer } from './modules/chain-renderer.js';
import { DocsViewer } from './modules/docs-viewer.js';

// Import utilities
import { ErrorHandler, Performance } from './utils/helpers.js';

class ApexSDKApp {
    constructor() {
        this.components = {};
        this.modules = {};
        this.isInitialized = false;
        
        this.init();
    }

    async init() {
        try {
            // Initialize error handling first
            ErrorHandler.init();
            
            // Wait for DOM to be ready
            if (document.readyState === 'loading') {
                document.addEventListener('DOMContentLoaded', () => this.initializeComponents());
            } else {
                this.initializeComponents();
            }
        } catch (error) {
            console.error('Failed to initialize Apex SDK App:', error);
            ErrorHandler.showUserFriendlyError('Failed to initialize the application. Please refresh the page.');
        }
    }

    initializeComponents() {
        try {
            // Initialize core components
            this.components.themeToggle = new ThemeToggle();
            this.components.mobileMenu = new MobileMenu();
            this.components.scrollEffects = new ScrollEffects();
            
            // Make scroll effects globally available for other modules
            window.scrollEffects = this.components.scrollEffects;

            // Initialize page-specific modules
            this.initializePageSpecificModules();
            
            // Initialize general features
            this.initializeGeneralFeatures();
            
            this.isInitialized = true;
            
            // Dispatch ready event
            document.dispatchEvent(new CustomEvent('apexsdk:ready', {
                detail: { app: this }
            }));
            
        } catch (error) {
            console.error('Failed to initialize components:', error);
            ErrorHandler.showUserFriendlyError('Some features may not work correctly. Please refresh the page.');
        }
    }

    initializePageSpecificModules() {
        // Initialize docs viewer if we're on the viewer page
        if (window.location.pathname.includes('viewer.html') || 
            document.getElementById('doc-content')) {
            this.modules.docsViewer = new DocsViewer();
        }

        // Initialize chain renderer if chains grid exists
        if (document.getElementById('chains-grid')) {
            this.modules.chainRenderer = new ChainRenderer();
        }
    }

    initializeGeneralFeatures() {
        // Set current year in footer
        this.setCurrentYear();
        
        // Initialize code showcase tabs if they exist
        this.initializeCodeTabs();
        
        // Initialize network explorer if it exists
        this.initializeNetworkExplorer();
        
        // Initialize lazy loading
        this.initializeLazyLoading();
        
        // Initialize keyboard shortcuts
        this.initializeKeyboardShortcuts();
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
                
                // Update active tab
                tabs.forEach(t => t.classList.remove('active'));
                tab.classList.add('active');
                
                // Update active code block
                codeBlocks.forEach(block => {
                    block.classList.remove('active');
                    if (block.dataset.content === targetTab) {
                        block.classList.add('active');
                        
                        // Trigger syntax highlighting for newly visible code
                        if (typeof hljs !== 'undefined') {
                            const codeElements = block.querySelectorAll('pre code');
                            codeElements.forEach(el => hljs.highlightElement(el));
                        }
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
                
                // Update active tab
                tabs.forEach(t => t.classList.remove('active'));
                tab.classList.add('active');
                
                // Update active panel
                panels.forEach(panel => {
                    panel.classList.remove('active');
                    if (panel.dataset.panel === ecosystem) {
                        panel.classList.add('active');
                    }
                });
            });
        });

        // Add hover effects to network cards
        const networkCards = document.querySelectorAll('.network-card');
        networkCards.forEach(card => {
            card.addEventListener('mouseenter', () => {
                card.style.transform = 'translateY(-8px) scale(1.02)';
            });
            
            card.addEventListener('mouseleave', () => {
                card.style.transform = '';
            });
        });
    }

    initializeLazyLoading() {
        // Lazy load images
        const images = document.querySelectorAll('img[data-src]');
        
        if (images.length > 0) {
            const imageObserver = new IntersectionObserver((entries, observer) => {
                entries.forEach(entry => {
                    if (entry.isIntersecting) {
                        const img = entry.target;
                        img.src = img.dataset.src;
                        img.removeAttribute('data-src');
                        img.classList.add('loaded');
                        observer.unobserve(img);
                    }
                });
            }, {
                rootMargin: '50px 0px'
            });

            images.forEach(img => imageObserver.observe(img));
        }
    }

    initializeKeyboardShortcuts() {
        document.addEventListener('keydown', (e) => {
            // Global keyboard shortcuts
            if (e.altKey && e.key === 't') {
                e.preventDefault();
                // Toggle theme
                if (this.components.themeToggle) {
                    const currentTheme = this.components.themeToggle.getCurrentTheme();
                    const newTheme = currentTheme === 'dark' ? 'light' : 'dark';
                    this.components.themeToggle.setTheme(newTheme);
                }
            }

            if (e.altKey && e.key === 'm') {
                e.preventDefault();
                // Toggle mobile menu
                if (this.components.mobileMenu) {
                    this.components.mobileMenu.toggleMenu();
                }
            }

            // Escape key to close modals/menus
            if (e.key === 'Escape') {
                if (this.components.mobileMenu && 
                    document.getElementById('navMenu')?.classList.contains('active')) {
                    this.components.mobileMenu.closeMenu();
                }
            }
        });
    }

    // Public API methods
    getComponent(name) {
        return this.components[name];
    }

    getModule(name) {
        return this.modules[name];
    }

    isReady() {
        return this.isInitialized;
    }

    // Method to reinitialize components (useful for dynamic content)
    reinitialize() {
        if (this.components.scrollEffects) {
            this.components.scrollEffects.destroy();
        }
        
        this.initializeComponents();
    }
}

// Initialize the application
const app = new ApexSDKApp();

// Make app globally available for debugging
if (typeof window !== 'undefined') {
    window.ApexSDKApp = app;
}

// Export for module systems
export default app;