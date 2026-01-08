// Scroll Effects Module
export class ScrollEffects {
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
                    // Unobserve after revealing to improve performance
                    observer.unobserve(entry.target);
                }
            });
        }, this.observerOptions);

        // Observe all scroll reveal elements
        const scrollElements = document.querySelectorAll('.scroll-reveal');
        scrollElements.forEach(el => {
            observer.observe(el);
        });

        // Store observer for potential cleanup
        this.intersectionObserver = observer;
    }

    removeLoadingClass() {
        // Remove loading class after a short delay to ensure animations are ready
        setTimeout(() => {
            document.body.classList.remove('loading');
        }, 100);
    }

    // Method to manually reveal elements (useful for dynamic content)
    revealElement(element) {
        if (element && !element.classList.contains('revealed')) {
            element.classList.add('revealed');
        }
    }

    // Cleanup method
    destroy() {
        if (this.intersectionObserver) {
            this.intersectionObserver.disconnect();
        }
    }
}