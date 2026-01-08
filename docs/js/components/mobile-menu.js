// Mobile Menu Module
export class MobileMenu {
    constructor() {
        this.toggle = document.getElementById('mobileMenuToggle');
        this.menu = document.getElementById('navMenu');
        this.hamburgerLines = this.toggle?.querySelectorAll('.hamburger-line');
        
        this.init();
    }

    init() {
        if (!this.toggle || !this.menu) return;

        this.toggle.addEventListener('click', () => this.toggleMenu());

        // Close menu when clicking on nav links
        this.menu.querySelectorAll('a').forEach(link => {
            link.addEventListener('click', () => {
                if (this.menu.classList.contains('active')) {
                    this.closeMenu();
                }
            });
        });

        // Close menu when clicking outside
        document.addEventListener('click', (event) => {
            if (!this.menu.contains(event.target) && 
                !this.toggle.contains(event.target) && 
                this.menu.classList.contains('active')) {
                this.closeMenu();
            }
        });

        // Handle escape key
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
        
        // Focus first menu item for accessibility
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
}