// Error Handling and Fallbacks Module
window.ApexSDK.ErrorHandler = class {
    constructor() {
        this.errorQueue = [];
        this.isOnline = navigator.onLine;
        this.retryAttempts = new Map();
        
        this.init();
    }

    init() {
        // Global error handlers
        window.addEventListener('error', (e) => this.handleError(e.error, 'JavaScript Error'));
        window.addEventListener('unhandledrejection', (e) => this.handleError(e.reason, 'Promise Rejection'));
        
        // Network status monitoring
        window.addEventListener('online', () => this.handleNetworkChange(true));
        window.addEventListener('offline', () => this.handleNetworkChange(false));
        
        // Resource loading error handler
        this.monitorResourceLoading();
    }

    handleError(error, type = 'Unknown Error') {
        console.error(`[${type}]:`, error);
        
        const errorInfo = {
            type,
            message: error.message || error,
            stack: error.stack,
            timestamp: new Date().toISOString(),
            url: window.location.href,
            userAgent: navigator.userAgent
        };

        this.errorQueue.push(errorInfo);
        
        // Show user-friendly error message
        this.showUserFriendlyError(this.getUserFriendlyMessage(error, type));
        
        // Attempt recovery if possible
        this.attemptErrorRecovery(error, type);
    }

    getUserFriendlyMessage(error, type) {
        if (!this.isOnline) {
            return 'You appear to be offline. Please check your internet connection.';
        }

        if (type === 'Network Error' || error.message?.includes('fetch')) {
            return 'A network error occurred. Please try again.';
        }

        if (error.message?.includes('marked') || error.message?.includes('hljs')) {
            return 'Documentation rendering failed. Content may display without formatting.';
        }

        if (type === 'Resource Loading') {
            return 'Some resources failed to load. The site may not work as expected.';
        }

        return 'Something went wrong. Please refresh the page if problems persist.';
    }

    showUserFriendlyError(message, duration = 5000) {
        // Remove existing error messages
        document.querySelectorAll('.global-error-message').forEach(el => el.remove());

        // Create error notification
        const errorDiv = document.createElement('div');
        errorDiv.className = 'global-error-message';
        errorDiv.setAttribute('role', 'alert');
        errorDiv.innerHTML = `
            <div class="error-content">
                <span>${message}</span>
                <button onclick="this.parentElement.parentElement.remove()" aria-label="Close error message">×</button>
            </div>
        `;
        
        document.body.appendChild(errorDiv);
        
        // Auto remove after duration
        setTimeout(() => {
            if (errorDiv.parentElement) {
                errorDiv.remove();
            }
        }, duration);
    }

    handleNetworkChange(isOnline) {
        this.isOnline = isOnline;
        
        if (isOnline) {
            this.showSuccessMessage('Connection restored');
            this.retryFailedRequests();
        } else {
            this.showUserFriendlyError('You are now offline. Some features may not work.', 0);
        }
    }

    showSuccessMessage(message) {
        const successDiv = document.createElement('div');
        successDiv.className = 'global-success-message';
        successDiv.setAttribute('role', 'alert');
        successDiv.innerHTML = `
            <div class="success-content">
                <span>${message}</span>
                <button onclick="this.parentElement.parentElement.remove()" aria-label="Close success message">×</button>
            </div>
        `;
        
        document.body.appendChild(successDiv);
        
        setTimeout(() => {
            if (successDiv.parentElement) {
                successDiv.remove();
            }
        }, 3000);
    }

    monitorResourceLoading() {
        // Monitor CSS loading
        document.querySelectorAll('link[rel="stylesheet"]').forEach(link => {
            link.addEventListener('error', () => {
                this.handleError(new Error(`Failed to load stylesheet: ${link.href}`), 'Resource Loading');
                this.loadFallbackCSS();
            });
        });

        // Monitor script loading
        document.querySelectorAll('script[src]').forEach(script => {
            script.addEventListener('error', () => {
                this.handleError(new Error(`Failed to load script: ${script.src}`), 'Resource Loading');
            });
        });

        // Monitor image loading
        this.setupImageErrorHandling();
    }

    setupImageErrorHandling() {
        document.addEventListener('error', (e) => {
            if (e.target.tagName === 'IMG') {
                this.handleImageError(e.target);
            }
        }, true);
    }

    handleImageError(img) {
        // Add error class for styling
        img.classList.add('image-error');
        
        // Try to load fallback image
        if (!img.dataset.fallbackAttempted) {
            img.dataset.fallbackAttempted = 'true';
            
            if (img.src.includes('/logos/')) {
                // For chain logos, create a text fallback
                const fallbackDiv = document.createElement('div');
                fallbackDiv.className = 'chain-logo-fallback';
                fallbackDiv.textContent = img.alt.charAt(0).toUpperCase();
                img.parentElement.replaceChild(fallbackDiv, img);
            } else {
                // For other images, hide them
                img.style.display = 'none';
            }
        }
    }

    loadFallbackCSS() {
        // Create minimal fallback styles
        const fallbackStyles = document.createElement('style');
        fallbackStyles.textContent = `
            body { font-family: system-ui, sans-serif; margin: 0; padding: 20px; line-height: 1.6; }
            .container { max-width: 1200px; margin: 0 auto; }
            .btn { display: inline-block; padding: 10px 20px; background: #007bff; color: white; text-decoration: none; border-radius: 4px; }
            .error { background: #f8d7da; color: #721c24; padding: 15px; border-radius: 4px; margin: 10px 0; }
            .sr-only { position: absolute; width: 1px; height: 1px; padding: 0; margin: -1px; overflow: hidden; clip: rect(0,0,0,0); }
        `;
        document.head.appendChild(fallbackStyles);
    }

    attemptErrorRecovery(error, type) {
        const errorKey = `${type}:${error.message}`;
        const attempts = this.retryAttempts.get(errorKey) || 0;

        if (attempts < 3) {
            this.retryAttempts.set(errorKey, attempts + 1);
            
            setTimeout(() => {
                if (type === 'Network Error') {
                    this.retryNetworkOperation(error);
                } else if (type.includes('Resource')) {
                    this.retryResourceLoading(error);
                }
            }, Math.pow(2, attempts) * 1000); // Exponential backoff
        }
    }

    retryNetworkOperation(error) {
        // This would be implemented based on specific network operations
        console.log('Retrying network operation:', error.message);
    }

    retryResourceLoading(error) {
        // Attempt to reload failed resources
        if (error.message.includes('stylesheet')) {
            const link = document.createElement('link');
            link.rel = 'stylesheet';
            link.href = error.message.split(': ')[1];
            document.head.appendChild(link);
        }
    }

    retryFailedRequests() {
        // Retry any queued requests when coming back online
        this.errorQueue
            .filter(error => error.type === 'Network Error')
            .forEach(error => {
                this.attemptErrorRecovery(new Error(error.message), error.type);
            });
    }

    // Public method to safely execute async operations
    async safeAsync(asyncFn, fallback = null, context = 'Operation') {
        try {
            return await asyncFn();
        } catch (error) {
            this.handleError(error, `${context} Error`);
            return fallback;
        }
    }

    // Public method to safely execute sync operations
    safeSync(syncFn, fallback = null, context = 'Operation') {
        try {
            return syncFn();
        } catch (error) {
            this.handleError(error, `${context} Error`);
            return fallback;
        }
    }
};

// Initialize error handler
window.ApexSDK.errorHandler = new window.ApexSDK.ErrorHandler();