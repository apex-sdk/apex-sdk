// JavaScript for enhanced GitHub Pages functionality

document.addEventListener('DOMContentLoaded', function() {
    // Initialize all interactive features
    initNavigation();
    initScrollEffects();
    initCodeBlocks();
    initSearchFunctionality();
    initThemeToggle();
});

// Navigation functionality
function initNavigation() {
    // Mobile navigation toggle
    const navToggle = document.querySelector('.nav-toggle');
    const navLinks = document.querySelector('.nav-links');
    
    if (navToggle && navLinks) {
        navToggle.addEventListener('click', function() {
            navLinks.classList.toggle('active');
            
            // Animate hamburger menu
            const spans = navToggle.querySelectorAll('span');
            spans.forEach((span, index) => {
                if (navLinks.classList.contains('active')) {
                    if (index === 0) span.style.transform = 'rotate(45deg) translate(6px, 6px)';
                    if (index === 1) span.style.opacity = '0';
                    if (index === 2) span.style.transform = 'rotate(-45deg) translate(6px, -6px)';
                } else {
                    span.style.transform = 'none';
                    span.style.opacity = '1';
                }
            });
        });
    }

    // Smooth scrolling for anchor links
    document.querySelectorAll('a[href^="#"]').forEach(anchor => {
        anchor.addEventListener('click', function (e) {
            e.preventDefault();
            const target = document.querySelector(this.getAttribute('href'));
            if (target) {
                target.scrollIntoView({
                    behavior: 'smooth',
                    block: 'start'
                });
                
                // Close mobile menu if open
                if (navLinks && navLinks.classList.contains('active')) {
                    navLinks.classList.remove('active');
                }
            }
        });
    });

    // Highlight active navigation item
    const currentLocation = window.location.pathname;
    document.querySelectorAll('.nav-links a').forEach(link => {
        if (link.getAttribute('href') === currentLocation || 
            (currentLocation.includes(link.getAttribute('href')) && link.getAttribute('href') !== '/')) {
            link.classList.add('active');
        }
    });
}

// Scroll effects
function initScrollEffects() {
    // Navbar background on scroll
    const navbar = document.getElementById('navbar');
    if (navbar) {
        window.addEventListener('scroll', function() {
            if (window.scrollY > 50) {
                navbar.style.background = 'rgba(15, 23, 42, 0.98)';
                navbar.style.backdropFilter = 'blur(10px)';
            } else {
                navbar.style.background = 'rgba(15, 23, 42, 0.95)';
                navbar.style.backdropFilter = 'blur(10px)';
            }
        });
    }

    // Intersection Observer for animations
    const observerOptions = {
        threshold: 0.1,
        rootMargin: '0px 0px -50px 0px'
    };

    const observer = new IntersectionObserver((entries) => {
        entries.forEach(entry => {
            if (entry.isIntersecting) {
                entry.target.classList.add('animate-on-scroll');
                
                // Staggered animation for grid items
                if (entry.target.classList.contains('feature-grid') || 
                    entry.target.classList.contains('docs-grid')) {
                    const items = entry.target.querySelectorAll('.feature-card, .doc-card, .chain-category');
                    items.forEach((item, index) => {
                        setTimeout(() => {
                            item.classList.add('animate-on-scroll');
                        }, index * 100);
                    });
                }
            }
        });
    }, observerOptions);

    // Observe elements for animation
    document.querySelectorAll('.feature-card, .doc-card, .chain-category, .feature-grid, .docs-grid').forEach(el => {
        observer.observe(el);
    });

    // Back to top button
    createBackToTopButton();
}

// Enhanced code blocks
function initCodeBlocks() {
    // Add copy buttons to code blocks
    document.querySelectorAll('pre').forEach((pre) => {
        if (!pre.querySelector('.copy-btn')) {
            const copyBtn = document.createElement('button');
            copyBtn.className = 'copy-btn';
            copyBtn.innerHTML = '📋';
            copyBtn.title = 'Copy to clipboard';
            
            copyBtn.addEventListener('click', function() {
                const code = pre.querySelector('code');
                const text = code ? code.textContent : pre.textContent;
                
                navigator.clipboard.writeText(text).then(() => {
                    copyBtn.innerHTML = '✅';
                    copyBtn.title = 'Copied!';
                    setTimeout(() => {
                        copyBtn.innerHTML = '📋';
                        copyBtn.title = 'Copy to clipboard';
                    }, 2000);
                });
            });
            
            pre.style.position = 'relative';
            pre.appendChild(copyBtn);
        }
    });

    // Add language labels
    document.querySelectorAll('pre code[class*="language-"]').forEach(code => {
        const language = code.className.match(/language-(\w+)/)?.[1];
        if (language) {
            const pre = code.parentElement;
            pre.setAttribute('data-lang', language.toUpperCase());
            pre.classList.add('highlight');
        }
    });
}

// Search functionality
function initSearchFunctionality() {
    // Simple client-side search
    const searchContainer = createSearchBox();
    if (searchContainer) {
        document.querySelector('.nav-container').appendChild(searchContainer);
    }
}

function createSearchBox() {
    const searchContainer = document.createElement('div');
    searchContainer.className = 'search-container';
    searchContainer.innerHTML = `
        <input type="text" class="search-input" placeholder="Search docs..." />
        <div class="search-results"></div>
    `;

    const searchInput = searchContainer.querySelector('.search-input');
    const searchResults = searchContainer.querySelector('.search-results');

    let searchData = [];
    
    // Build search index from page content
    function buildSearchIndex() {
        const pages = [
            { title: 'Quick Start', url: 'QUICK_START.html', content: 'Getting started with Apex SDK in under 5 minutes' },
            { title: 'API Reference', url: 'API.html', content: 'Complete API documentation with examples' },
            { title: 'CLI Guide', url: 'CLI_GUIDE.html', content: 'Command line interface for project management' },
            { title: 'Substrate Features', url: 'Substrate_Features.html', content: 'Substrate blockchain integration guide' },
            { title: 'Testing Framework', url: 'TESTING_FRAMEWORK.html', content: 'Comprehensive testing tools and examples' },
            { title: 'Contributing', url: 'CONTRIBUTING.html', content: 'How to contribute to Apex SDK development' },
            { title: 'Security', url: 'SECURITY.html', content: 'Security policies and best practices' },
        ];
        
        searchData = pages;
    }

    buildSearchIndex();

    searchInput.addEventListener('input', function() {
        const query = this.value.toLowerCase().trim();
        
        if (query.length < 2) {
            searchResults.style.display = 'none';
            return;
        }

        const results = searchData.filter(item => 
            item.title.toLowerCase().includes(query) ||
            item.content.toLowerCase().includes(query)
        ).slice(0, 5);

        if (results.length > 0) {
            searchResults.innerHTML = results.map(result => 
                `<div class="search-result-item">
                    <a href="${result.url}">
                        <div class="search-result-title">${result.title}</div>
                        <div class="search-result-content">${result.content}</div>
                    </a>
                </div>`
            ).join('');
            searchResults.style.display = 'block';
        } else {
            searchResults.innerHTML = '<div class="search-no-results">No results found</div>';
            searchResults.style.display = 'block';
        }
    });

    // Hide results when clicking outside
    document.addEventListener('click', function(e) {
        if (!searchContainer.contains(e.target)) {
            searchResults.style.display = 'none';
        }
    });

    return searchContainer;
}

// Theme toggle functionality
function initThemeToggle() {
    // For future dark/light theme switching
    const preferredTheme = localStorage.getItem('apex-theme') || 'dark';
    document.documentElement.setAttribute('data-theme', preferredTheme);
}

// Back to top button
function createBackToTopButton() {
    const backToTop = document.createElement('button');
    backToTop.className = 'back-to-top';
    backToTop.innerHTML = '↑';
    backToTop.title = 'Back to top';
    backToTop.style.cssText = `
        position: fixed;
        bottom: 20px;
        right: 20px;
        width: 50px;
        height: 50px;
        border-radius: 50%;
        background: var(--primary-color);
        color: white;
        border: none;
        font-size: 18px;
        cursor: pointer;
        opacity: 0;
        transition: opacity 0.3s ease;
        z-index: 1000;
        box-shadow: var(--shadow-lg);
    `;

    document.body.appendChild(backToTop);

    window.addEventListener('scroll', function() {
        if (window.scrollY > 300) {
            backToTop.style.opacity = '1';
        } else {
            backToTop.style.opacity = '0';
        }
    });

    backToTop.addEventListener('click', function() {
        window.scrollTo({
            top: 0,
            behavior: 'smooth'
        });
    });
}

// Utility functions
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

// Enhanced error handling
window.addEventListener('error', function(e) {
    console.error('JavaScript error:', e.error);
    // Could implement error reporting here
});

// Service worker for offline functionality (if needed)
if ('serviceWorker' in navigator) {
    window.addEventListener('load', function() {
        // Uncomment to enable service worker
        // navigator.serviceWorker.register('/sw.js')
        //     .then(registration => console.log('SW registered'))
        //     .catch(error => console.log('SW registration failed'));
    });
}