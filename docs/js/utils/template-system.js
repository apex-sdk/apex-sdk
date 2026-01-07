// Simple Template System for Static HTML
class SimpleTemplate {
    constructor() {
        this.cache = new Map();
    }

    async loadTemplate(templatePath) {
        if (this.cache.has(templatePath)) {
            return this.cache.get(templatePath);
        }

        try {
            const response = await fetch(templatePath);
            if (!response.ok) {
                throw new Error(`Failed to load template: ${templatePath}`);
            }
            const template = await response.text();
            this.cache.set(templatePath, template);
            return template;
        } catch (error) {
            console.error('Template loading error:', error);
            return '';
        }
    }

    replaceVariables(template, variables = {}) {
        let result = template;
        
        Object.keys(variables).forEach(key => {
            const placeholder = `{{${key}}}`;
            result = result.replace(new RegExp(placeholder, 'g'), variables[key] || '');
        });

        // Remove any unreplaced placeholders
        result = result.replace(/\{\{[^}]+\}\}/g, '');
        
        return result;
    }

    async render(templatePath, variables = {}) {
        const template = await this.loadTemplate(templatePath);
        return this.replaceVariables(template, variables);
    }

    async includePartial(partialPath, targetSelector, variables = {}) {
        const partial = await this.render(partialPath, variables);
        const targetElement = document.querySelector(targetSelector);
        
        if (targetElement) {
            targetElement.innerHTML = partial;
        } else {
            console.warn(`Target element not found: ${targetSelector}`);
        }
    }

    async loadPartials(partials) {
        const promises = partials.map(partial => 
            this.includePartial(partial.path, partial.target, partial.variables)
        );
        
        await Promise.all(promises);
    }
}

// Global template instance
window.templateSystem = new SimpleTemplate();