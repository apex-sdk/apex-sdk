// Theme Toggle Module
export class ThemeToggle {
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
}