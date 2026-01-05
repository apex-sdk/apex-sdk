// Performance Optimization Module
window.ApexSDK.Performance = class {
    constructor() {
        this.imageCache = new Map();
        this.intersectionObserver = null;
        this.performanceMetrics = {
            loadStart: performance.now(),
            firstPaint: null,
            domContentLoaded: null,
            resourcesLoaded: []
        };
        
        this.init();
    }

    init() {
        this.setupLazyLoading();
        this.setupResourceCaching();
        this.measurePerformance();
        this.optimizeImages();
        this.setupPreloadHints();
    }

    setupLazyLoading() {
        // Modern intersection observer for lazy loading
        if ('IntersectionObserver' in window) {
            this.intersectionObserver = new IntersectionObserver((entries) => {
                entries.forEach(entry => {
                    if (entry.isIntersecting) {
                        this.loadLazyElement(entry.target);
                        this.intersectionObserver.unobserve(entry.target);
                    }
                });
            }, {
                rootMargin: '50px 0px',
                threshold: 0.01
            });

            // Observe lazy elements
            this.observeLazyElements();
        } else {
            // Fallback for older browsers
            this.loadAllLazyElements();
        }
    }

    observeLazyElements() {
        // Images with data-src attribute
        document.querySelectorAll('img[data-src]').forEach(img => {
            this.intersectionObserver.observe(img);
        });

        // Background images with data-bg attribute
        document.querySelectorAll('[data-bg]').forEach(el => {
            this.intersectionObserver.observe(el);
        });

        // Content sections for lazy component loading
        document.querySelectorAll('[data-lazy-load]').forEach(el => {
            this.intersectionObserver.observe(el);
        });
    }

    loadLazyElement(element) {
        const errorHandler = window.ApexSDK.errorHandler;
        
        if (element.tagName === 'IMG' && element.dataset.src) {
            this.loadLazyImage(element);
        } else if (element.dataset.bg) {
            element.style.backgroundImage = `url(${element.dataset.bg})`;
            element.classList.add('bg-loaded');
        } else if (element.dataset.lazyLoad) {
            this.loadLazyComponent(element);
        }
    }

    loadLazyImage(img) {
        const src = img.dataset.src;
        const errorHandler = window.ApexSDK.errorHandler;

        // Check cache first
        if (this.imageCache.has(src)) {
            const cachedImage = this.imageCache.get(src);
            if (cachedImage.complete) {
                img.src = src;
                img.classList.add('loaded');
                return;
            }
        }

        // Create new image for loading
        const imageLoader = new Image();
        
        imageLoader.onload = () => {
            img.src = src;
            img.classList.add('loaded');
            img.removeAttribute('data-src');
            this.imageCache.set(src, imageLoader);
        };

        imageLoader.onerror = () => {
            img.classList.add('load-error');
            if (errorHandler) {
                errorHandler.handleImageError(img);
            }
        };

        imageLoader.src = src;
    }

    loadLazyComponent(element) {
        const componentType = element.dataset.lazyLoad;
        const errorHandler = window.ApexSDK.errorHandler;

        try {
            switch (componentType) {
                case 'chain-list':
                    this.loadChainList(element);
                    break;
                case 'code-example':
                    this.loadCodeExample(element);
                    break;
                case 'documentation':
                    this.loadDocumentation(element);
                    break;
                default:
                    console.warn(`Unknown lazy component type: ${componentType}`);
            }
        } catch (error) {
            if (errorHandler) {
                errorHandler.handleError(error, 'Lazy Component Loading');
            }
        }
    }

    async loadChainList(element) {
        try {
            if (window.ApexSDK.ChainRenderer) {
                const renderer = new window.ApexSDK.ChainRenderer();
                await renderer.renderChainGrid();
                element.classList.add('loaded');
            }
        } catch (error) {
            element.classList.add('load-error');
            throw error;
        }
    }

    async loadCodeExample(element) {
        try {
            // Check if highlight.js is available
            if (window.hljs) {
                const codeBlocks = element.querySelectorAll('pre code');
                codeBlocks.forEach(block => {
                    window.hljs.highlightElement(block);
                });
            }
            element.classList.add('loaded');
        } catch (error) {
            element.classList.add('load-error');
            throw error;
        }
    }

    async loadDocumentation(element) {
        const errorHandler = window.ApexSDK.errorHandler;

        try {
            if (window.ApexSDK.DocsViewer) {
                const viewer = new window.ApexSDK.DocsViewer();
                await viewer.loadContent(element);
                element.classList.add('loaded');
            }
        } catch (error) {
            element.classList.add('load-error');
            throw error;
        }
    }

    loadAllLazyElements() {
        // Fallback for browsers without Intersection Observer
        document.querySelectorAll('img[data-src], [data-bg], [data-lazy-load]').forEach(el => {
            this.loadLazyElement(el);
        });
    }

    setupResourceCaching() {
        // Preload critical resources
        this.preloadCriticalResources();
        
        // Setup service worker if available
        this.setupServiceWorker();
        
        // Cache API responses
        this.setupAPICache();
    }

    preloadCriticalResources() {
        const criticalResources = [
            '/docs/css/modular.css',
            '/docs/js/main-browser.js',
            '/docs/assets/fonts/inter-variable.woff2'
        ];

        criticalResources.forEach(resource => {
            const link = document.createElement('link');
            link.rel = 'preload';
            link.href = resource;
            
            if (resource.endsWith('.css')) {
                link.as = 'style';
            } else if (resource.endsWith('.js')) {
                link.as = 'script';
            } else if (resource.includes('fonts')) {
                link.as = 'font';
                link.type = 'font/woff2';
                link.crossOrigin = 'anonymous';
            }
            
            document.head.appendChild(link);
        });
    }

    setupServiceWorker() {
        if ('serviceWorker' in navigator) {
            navigator.serviceWorker.register('/docs/sw.js')
                .then(registration => {
                    console.log('Service Worker registered:', registration.scope);
                })
                .catch(error => {
                    console.log('Service Worker registration failed:', error);
                });
        }
    }

    setupAPICache() {
        this.apiCache = new Map();
        this.cacheTimeout = 5 * 60 * 1000; // 5 minutes
    }

    async cachedFetch(url, options = {}) {
        const cacheKey = `${url}:${JSON.stringify(options)}`;
        const cached = this.apiCache.get(cacheKey);

        if (cached && Date.now() - cached.timestamp < this.cacheTimeout) {
            return cached.response.clone();
        }

        try {
            const response = await fetch(url, options);
            
            if (response.ok) {
                this.apiCache.set(cacheKey, {
                    response: response.clone(),
                    timestamp: Date.now()
                });
            }
            
            return response;
        } catch (error) {
            if (cached) {
                console.warn('Using cached response due to network error');
                return cached.response.clone();
            }
            throw error;
        }
    }

    optimizeImages() {
        // Add modern image format support detection
        this.supportsWebP = this.checkWebPSupport();
        this.supportsAVIF = this.checkAVIFSupport();

        // Update image sources if modern formats are supported
        if (this.supportsWebP || this.supportsAVIF) {
            this.upgradeImageSources();
        }
    }

    checkWebPSupport() {
        try {
            return document.createElement('canvas')
                .toDataURL('image/webp')
                .indexOf('data:image/webp') === 0;
        } catch (e) {
            return false;
        }
    }

    checkAVIFSupport() {
        return new Promise(resolve => {
            const avif = new Image();
            avif.onload = avif.onerror = () => {
                resolve(avif.height === 2);
            };
            avif.src = 'data:image/avif;base64,AAAAIGZ0eXBhdmlmAAAAAGF2aWZtaWYxbWlhZk1BMUIAAADybWV0YQAAAAAAAAAoaGRscgAAAAAAAAAAcGljdAAAAAAAAAAAAAAAAGxpYmF2aWYAAAAADnBpdG0AAAAAAAEAAAAeaWxvYwAAAABEAAABAAEAAAABAAABGgAAAB0AAAAoaWluZgAAAAAAAQAAABppbmZlAgAAAAABAABhdjAxQ29sb3IAAAAAamlwcnAAAABLaXBjbwAAABRpc3BlAAAAAAAAAAIAAAACAAAAEHBpeGkAAAAAAwgICAAAAAxhdjFDgQ0MAAAAABNjb2xybmNseAACAAIAAYAAAAAXaXBtYQAAAAAAAAABAAEEAQKDBAAAACVtZGF0EgAKCBgABogQEAwgMg8f8D///8WfhwB8+ErK42A=';
        });
    }

    upgradeImageSources() {
        document.querySelectorAll('img[data-src]').forEach(img => {
            let src = img.dataset.src;
            
            if (this.supportsAVIF && src.endsWith('.jpg') || src.endsWith('.png')) {
                const avifSrc = src.replace(/\.(jpg|png)$/, '.avif');
                img.dataset.src = avifSrc;
                img.dataset.fallback = src;
            } else if (this.supportsWebP && (src.endsWith('.jpg') || src.endsWith('.png'))) {
                const webpSrc = src.replace(/\.(jpg|png)$/, '.webp');
                img.dataset.src = webpSrc;
                img.dataset.fallback = src;
            }
        });
    }

    setupPreloadHints() {
        // Resource hints for better performance
        this.addResourceHints();
        
        // Prefetch likely next pages
        this.setupPrefetching();
    }

    addResourceHints() {
        const hints = [
            { rel: 'dns-prefetch', href: '//fonts.googleapis.com' },
            { rel: 'dns-prefetch', href: '//api.github.com' },
            { rel: 'preconnect', href: '//fonts.gstatic.com', crossorigin: true }
        ];

        hints.forEach(hint => {
            const link = document.createElement('link');
            Object.assign(link, hint);
            if (hint.crossorigin) link.crossOrigin = hint.crossorigin;
            document.head.appendChild(link);
        });
    }

    setupPrefetching() {
        // Prefetch likely navigation targets
        const prefetchTargets = [
            '/docs/QUICK_START.html',
            '/docs/API.html',
            '/docs/DOCUMENTATION_HUB.html'
        ];

        // Only prefetch on fast connections
        if (navigator.connection && navigator.connection.effectiveType === '4g') {
            prefetchTargets.forEach(target => {
                const link = document.createElement('link');
                link.rel = 'prefetch';
                link.href = target;
                document.head.appendChild(link);
            });
        }
    }

    measurePerformance() {
        // Mark performance milestones
        this.performanceMetrics.domContentLoaded = performance.now();

        // Listen for paint events
        if ('PerformanceObserver' in window) {
            const paintObserver = new PerformanceObserver((entryList) => {
                const entries = entryList.getEntries();
                entries.forEach(entry => {
                    if (entry.name === 'first-contentful-paint') {
                        this.performanceMetrics.firstPaint = entry.startTime;
                    }
                });
            });

            paintObserver.observe({ entryTypes: ['paint'] });

            // Resource timing observer
            const resourceObserver = new PerformanceObserver((entryList) => {
                const entries = entryList.getEntries();
                entries.forEach(entry => {
                    this.performanceMetrics.resourcesLoaded.push({
                        name: entry.name,
                        duration: entry.duration,
                        size: entry.transferSize
                    });
                });
            });

            resourceObserver.observe({ entryTypes: ['resource'] });
        }

        // Report performance metrics
        window.addEventListener('load', () => {
            this.reportPerformanceMetrics();
        });
    }

    reportPerformanceMetrics() {
        const loadTime = performance.now() - this.performanceMetrics.loadStart;
        
        console.group('🚀 Apex SDK Performance Metrics');
        console.log(`Total Load Time: ${Math.round(loadTime)}ms`);
        
        if (this.performanceMetrics.firstPaint) {
            console.log(`First Paint: ${Math.round(this.performanceMetrics.firstPaint)}ms`);
        }
        
        console.log(`DOM Content Loaded: ${Math.round(this.performanceMetrics.domContentLoaded)}ms`);
        
        const totalResourceSize = this.performanceMetrics.resourcesLoaded
            .reduce((total, resource) => total + (resource.size || 0), 0);
        
        console.log(`Total Resource Size: ${Math.round(totalResourceSize / 1024)}KB`);
        console.groupEnd();

        // Performance budget warnings
        if (loadTime > 3000) {
            console.warn('⚠️ Page load time exceeds 3 seconds');
        }

        if (totalResourceSize > 1024 * 1024) {
            console.warn('⚠️ Total resource size exceeds 1MB');
        }
    }

    // Public method to manually trigger lazy loading check
    checkLazyElements() {
        if (this.intersectionObserver) {
            this.observeLazyElements();
        }
    }

    // Clean up resources
    destroy() {
        if (this.intersectionObserver) {
            this.intersectionObserver.disconnect();
        }
        this.imageCache.clear();
        this.apiCache.clear();
    }
};