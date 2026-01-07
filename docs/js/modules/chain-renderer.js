// Chain Data Module
export const chainsData = [
    { 
        name: 'Polkadot', 
        type: 'Substrate', 
        logo: 'polkadot.svg', 
        url: 'https://polkadot.network', 
        description: 'Scalable multi-chain network',
        status: 'active'
    },
    { 
        name: 'Ethereum', 
        type: 'EVM', 
        logo: 'ethereum.svg', 
        url: 'https://ethereum.org', 
        description: 'Smart contract platform',
        status: 'active'
    },
    { 
        name: 'Kusama', 
        type: 'Substrate', 
        logo: 'kusama.svg', 
        url: 'https://kusama.network', 
        description: 'Polkadot\'s canary network',
        status: 'active'
    },
    { 
        name: 'Polygon', 
        type: 'EVM', 
        logo: 'polygon.svg', 
        url: 'https://polygon.technology', 
        description: 'Ethereum scaling solution',
        status: 'active'
    },
    { 
        name: 'Moonbeam', 
        type: 'Hybrid', 
        logo: 'moonbeam.svg', 
        url: 'https://moonbeam.network', 
        description: 'Ethereum on Polkadot',
        status: 'active'
    },
    { 
        name: 'Binance Smart Chain', 
        type: 'EVM', 
        logo: 'bsc.svg', 
        url: 'https://www.bnbchain.org', 
        description: 'High-performance blockchain',
        status: 'active'
    },
    { 
        name: 'Astar', 
        type: 'Hybrid', 
        logo: 'astar.svg', 
        url: 'https://astar.network', 
        description: 'Multi-chain dApp hub',
        status: 'active'
    },
    { 
        name: 'Avalanche', 
        type: 'EVM', 
        logo: 'avalanche.svg', 
        url: 'https://www.avax.network', 
        description: 'Fast consensus protocol',
        status: 'active'
    },
    { 
        name: 'Acala', 
        type: 'Substrate', 
        logo: 'acala.svg', 
        url: 'https://acala.network', 
        description: 'DeFi hub for Polkadot',
        status: 'active'
    },
    { 
        name: 'Arbitrum', 
        type: 'EVM', 
        logo: 'arbitrum.svg', 
        url: 'https://arbitrum.io', 
        description: 'Layer 2 for Ethereum',
        status: 'active'
    },
    { 
        name: 'Moonriver', 
        type: 'Hybrid', 
        logo: 'moonriver.svg', 
        url: 'https://moonbeam.network/networks/moonriver', 
        description: 'Ethereum on Kusama',
        status: 'active'
    },
    { 
        name: 'Optimism', 
        type: 'EVM', 
        logo: 'optimism.svg', 
        url: 'https://optimism.io', 
        description: 'Optimistic Ethereum',
        status: 'active'
    },
    { 
        name: 'Parallel', 
        type: 'Substrate', 
        logo: 'parallel.svg', 
        url: 'https://parallel.fi', 
        description: 'DeFi super app',
        status: 'active'
    },
    { 
        name: 'Fantom', 
        type: 'EVM', 
        logo: 'fantom.svg', 
        url: 'https://fantom.foundation', 
        description: 'High-speed consensus',
        status: 'active'
    },
    { 
        name: 'Centrifuge', 
        type: 'Substrate', 
        logo: 'centrifuge.svg', 
        url: 'https://centrifuge.io', 
        description: 'Real-world assets on-chain',
        status: 'active'
    },
    { 
        name: 'Base', 
        type: 'EVM', 
        logo: 'base.svg', 
        url: 'https://base.org', 
        description: 'Coinbase Layer 2',
        status: 'active'
    },
    { 
        name: 'Sepolia', 
        type: 'EVM', 
        logo: 'sepolia.svg', 
        url: 'https://sepolia.dev', 
        description: 'Ethereum testnet',
        status: 'testnet'
    }
];

// Chain Renderer Module
export class ChainRenderer {
    constructor(containerId = 'chains-grid') {
        this.container = document.getElementById(containerId);
        this.chains = chainsData;
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
        
        // Re-observe new elements for scroll animations
        const scrollElements = this.container.querySelectorAll('.scroll-reveal');
        if (window.scrollEffects && window.scrollEffects.intersectionObserver) {
            scrollElements.forEach(el => {
                window.scrollEffects.intersectionObserver.observe(el);
            });
        }
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

        // Show all chains by default
        this.filterChains('all');
    }

    setActiveFilter(activeButton, filter) {
        const filterButtons = document.querySelectorAll('.chain-filter');
        filterButtons.forEach(btn => btn.classList.remove('active'));
        activeButton.classList.add('active');
        this.currentFilter = filter;
    }

    filterChains(filter) {
        const chainCards = document.querySelectorAll('.chain-card');

        chainCards.forEach((card, index) => {
            const chainType = card.dataset.type;
            let shouldShow = false;

            switch (filter) {
                case 'all':
                    shouldShow = true;
                    break;
                case 'substrate':
                    shouldShow = chainType === 'Substrate';
                    break;
                case 'evm':
                    shouldShow = chainType === 'EVM';
                    break;
                case 'hybrid':
                    shouldShow = chainType === 'Hybrid';
                    break;
                default:
                    shouldShow = true;
            }

            if (shouldShow) {
                card.style.display = 'flex';
                card.style.opacity = '0';
                
                // Stagger animation for better UX
                setTimeout(() => {
                    card.style.transition = 'opacity 0.3s ease';
                    card.style.opacity = '1';
                }, index * 50);
            } else {
                card.style.transition = 'opacity 0.3s ease';
                card.style.opacity = '0';
                
                setTimeout(() => {
                    card.style.display = 'none';
                }, 300);
            }
        });
    }

    // Public method to add new chain
    addChain(chain) {
        this.chains.push(chain);
        this.render();
    }

    // Public method to get chains by type
    getChainsByType(type) {
        return this.chains.filter(chain => chain.type === type);
    }
}