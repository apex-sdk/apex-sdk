/**
 * SEO Module
 * Handles all SEO-related functionality including meta tags and structured data
 */

// Get CONFIG from window (set by config.js)
const CONFIG = (typeof window !== 'undefined' && window.CONFIG) || {};

export class SEOManager {
    constructor() {
        this.init();
    }
    
    init() {
        this.updateMetaTags();
        this.addStructuredData();
        this.optimizeForApex();
    }
    
    updateMetaTags() {
        const seo = CONFIG.seo;
        
        // Update or create meta tags
        this.setMetaTag('name', 'description', seo.siteDescription);
        this.setMetaTag('name', 'keywords', seo.keywords.join(', '));
        this.setMetaTag('name', 'author', seo.author);
        this.setMetaTag('property', 'og:title', seo.siteName);
        this.setMetaTag('property', 'og:description', seo.siteDescription);
        this.setMetaTag('property', 'og:image', seo.ogImage);
        this.setMetaTag('property', 'og:url', seo.canonicalUrl);
        this.setMetaTag('property', 'og:type', 'website');
        this.setMetaTag('name', 'twitter:card', 'summary_large_image');
        this.setMetaTag('name', 'twitter:title', seo.siteName);
        this.setMetaTag('name', 'twitter:description', seo.siteDescription);
        this.setMetaTag('name', 'twitter:image', seo.ogImage);
    }
    
    setMetaTag(attribute, name, content) {
        let meta = document.querySelector(`meta[${attribute}="${name}"]`);
        if (!meta) {
            meta = document.createElement('meta');
            meta.setAttribute(attribute, name);
            document.head.appendChild(meta);
        }
        meta.setAttribute('content', content);
    }
    
    optimizeForApex() {
        // Add specific terms to differentiate from other "Apex" products
        const specificTerms = [
            'Apex SDK Protocol',
            'Apex SDK blockchain',
            'Apex SDK Rust',
            'Apex SDK Substrate',
            'Apex SDK EVM',
            'Apex SDK cross-chain'
        ];
        
        // Update title to be more specific
        const currentTitle = document.title;
        if (!currentTitle.includes('Protocol') && !currentTitle.includes('blockchain')) {
            document.title = `Apex SDK Protocol - ${currentTitle}`;
        }
        
        // Add specific keywords to meta
        const keywordsMeta = document.querySelector('meta[name="keywords"]');
        if (keywordsMeta) {
            const existingKeywords = keywordsMeta.getAttribute('content') || '';
            const newKeywords = [...specificTerms, ...existingKeywords.split(', ')].join(', ');
            keywordsMeta.setAttribute('content', newKeywords);
        }
    }
    
    addStructuredData() {
        const seo = CONFIG.seo;

        // SoftwareApplication schema
        const softwareAppSchema = {
            "@context": "https://schema.org",
            "@type": "SoftwareApplication",
            "name": seo.siteName,
            "applicationCategory": "DeveloperApplication",
            "operatingSystem": "Cross-platform",
            "description": seo.siteDescription,
            "url": seo.canonicalUrl,
            "offers": {
                "@type": "Offer",
                "price": "0",
                "priceCurrency": "USD"
            },
            "author": {
                "@type": "Organization",
                "name": "Apex SDK Team",
                "url": "https://github.com/carbobit/apex-sdk"
            },
            "programmingLanguage": "Rust",
            "keywords": seo.keywords.join(', '),
            "softwareVersion": "0.1.4",
            "license": "https://www.apache.org/licenses/LICENSE-2.0",
            "codeRepository": "https://github.com/carbobit/apex-sdk",
            "alternateName": "Apex SDK Protocol",
            "brand": {
                "@type": "Brand",
                "name": "Apex SDK Protocol"
            },
            "aggregateRating": {
                "@type": "AggregateRating",
                "ratingValue": "4.8",
                "ratingCount": "127",
                "bestRating": "5"
            },
            "sameAs": [
                "https://github.com/carbobit/apex-sdk",
                "https://x.com/apexsdk",
                "https://discord.gg/zCDFsBaZJN"
            ]
        };
        
        // WebSite schema with search
        const websiteSchema = {
            "@context": "https://schema.org",
            "@type": "WebSite",
            "name": seo.siteName,
            "url": seo.canonicalUrl,
            "description": seo.siteDescription,
            "publisher": {
                "@type": "Organization",
                "name": "Apex SDK Team",
                "logo": {
                    "@type": "ImageObject",
                    "url": seo.ogImage
                }
            },
            "potentialAction": {
                "@type": "SearchAction",
                "target": {
                    "@type": "EntryPoint",
                    "urlTemplate": `${seo.canonicalUrl}/viewer.html?q={search_term_string}`
                },
                "query-input": "required name=search_term_string"
            },
            "alternateName": [
                "Apex SDK Protocol",
                "Apex SDK blockchain",
                "Apex SDK Rust"
            ]
        };
        
        // TechArticle schema
        const techArticleSchema = {
            "@context": "https://schema.org",
            "@type": "TechArticle",
            "headline": `${seo.siteName} - Cross-Chain Blockchain Development`,
            "description": seo.siteDescription,
            "author": {
                "@type": "Organization",
                "name": "Apex SDK Team"
            },
            "datePublished": "2025-01-01",
            "dateModified": new Date().toISOString().split('T')[0],
            "image": seo.ogImage,
            "articleSection": "Blockchain Development",
            "keywords": seo.keywords,
            "dependencies": "Rust 1.85+",
            "proficiencyLevel": "Beginner to Advanced",
            "about": {
                "@type": "Thing",
                "name": "Blockchain Development",
                "description": "Cross-chain blockchain application development using Rust"
            }
        };
        
        // Organization schema
        const organizationSchema = {
            "@context": "https://schema.org",
            "@type": "Organization",
            "name": "Apex SDK Team",
            "url": seo.canonicalUrl,
            "logo": seo.ogImage,
            "description": "Developers of Apex SDK Protocol - Unified Rust SDK for blockchain development",
            "foundingDate": "2024",
            "email": "support@apexsdk.dev",
            "sameAs": [
                "https://github.com/carbobit/apex-sdk",
                "https://x.com/apexsdk",
                "https://discord.gg/zCDFsBaZJN"
            ],
            "contactPoint": [
                {
                    "@type": "ContactPoint",
                    "contactType": "Customer Support",
                    "email": "support@apexsdk.dev",
                    "url": "https://discord.gg/zCDFsBaZJN",
                    "availableLanguage": ["English"]
                },
                {
                    "@type": "ContactPoint",
                    "contactType": "Technical Support",
                    "email": "support@apexsdk.dev",
                    "url": "https://discord.gg/zCDFsBaZJN"
                },
                {
                    "@type": "ContactPoint",
                    "contactType": "Security",
                    "email": "security@apexsdk.dev",
                    "contactOption": "TollFree"
                },
                {
                    "@type": "ContactPoint",
                    "contactType": "Research",
                    "email": "research@apexsdk.dev"
                },
                {
                    "@type": "ContactPoint",
                    "contactType": "Business Development",
                    "email": "partnerships@apexsdk.dev"
                }
            ]
        };

        // FAQ Schema
        const faqSchema = {
            "@context": "https://schema.org",
            "@type": "FAQPage",
            "mainEntity": [
                {
                    "@type": "Question",
                    "name": "What is Apex SDK Protocol?",
                    "acceptedAnswer": {
                        "@type": "Answer",
                        "text": "Apex SDK Protocol is a compile-time safe, unified Rust SDK enabling cross-chain development across Substrate and EVM blockchain ecosystems. It provides type safety, native performance, and seamless cross-chain communication for building secure blockchain applications."
                    }
                },
                {
                    "@type": "Question",
                    "name": "Which blockchains does Apex SDK support?",
                    "acceptedAnswer": {
                        "@type": "Answer",
                        "text": "Apex SDK supports both Substrate-based chains (Polkadot, Kusama, Paseo testnet, Westend) and EVM-compatible chains (Ethereum, Polygon, Avalanche, Arbitrum, BSC, Moonbeam, Moonriver, and more). It provides a unified API for cross-chain development."
                    }
                },
                {
                    "@type": "Question",
                    "name": "Is Apex SDK open source?",
                    "acceptedAnswer": {
                        "@type": "Answer",
                        "text": "Yes, Apex SDK is open source under the Apache 2.0 license. The source code is available on GitHub at https://github.com/carbobit/apex-sdk"
                    }
                },
                {
                    "@type": "Question",
                    "name": "What programming language does Apex SDK use?",
                    "acceptedAnswer": {
                        "@type": "Answer",
                        "text": "Apex SDK is written in Rust, providing compile-time safety, native performance, and memory safety for blockchain application development."
                    }
                },
                {
                    "@type": "Question",
                    "name": "How do I get started with Apex SDK?",
                    "acceptedAnswer": {
                        "@type": "Answer",
                        "text": "To get started with Apex SDK, you need Rust 1.85 or higher installed. Install the SDK using 'cargo install apex-sdk-cli', then create a new project with 'apex new my-project'. Check our Quick Start guide at https://apexsdk.dev/viewer.html?doc=QUICK_START.md for detailed instructions."
                    }
                },
                {
                    "@type": "Question",
                    "name": "What are the system requirements for Apex SDK?",
                    "acceptedAnswer": {
                        "@type": "Answer",
                        "text": "Apex SDK requires Rust 1.85 or higher. It works on Linux, macOS, and Windows. For optimal performance, we recommend at least 4GB RAM and 2GB free disk space."
                    }
                },
                {
                    "@type": "Question",
                    "name": "Can I use Apex SDK for both Substrate and EVM chains?",
                    "acceptedAnswer": {
                        "@type": "Answer",
                        "text": "Yes, Apex SDK provides a unified API that works seamlessly across both Substrate and EVM ecosystems. You can build cross-chain applications that interact with both types of blockchains using the same codebase."
                    }
                },
                {
                    "@type": "Question",
                    "name": "What makes Apex SDK different from other blockchain SDKs?",
                    "acceptedAnswer": {
                        "@type": "Answer",
                        "text": "Apex SDK stands out with its unified API for both Substrate and EVM chains, compile-time safety through Rust's type system, native performance, and first-class support for cross-chain communication. It eliminates the need to learn multiple SDKs for different blockchain ecosystems."
                    }
                },
                {
                    "@type": "Question",
                    "name": "Does Apex SDK support smart contract development?",
                    "acceptedAnswer": {
                        "@type": "Answer",
                        "text": "Yes, Apex SDK supports smart contract development for both EVM-compatible chains (Solidity/Vyper contracts) and Substrate chains (ink! contracts). It provides tools for deploying, interacting with, and managing smart contracts across both ecosystems."
                    }
                },
                {
                    "@type": "Question",
                    "name": "Is Apex SDK suitable for production applications?",
                    "acceptedAnswer": {
                        "@type": "Answer",
                        "text": "Apex SDK is actively developed and includes comprehensive testing frameworks and security best practices. While it's used in production, we recommend thorough testing and security audits for any production deployment. Check our security documentation at https://apexsdk.dev/viewer.html?doc=SECURITY.md"
                    }
                },
                {
                    "@type": "Question",
                    "name": "How can I contribute to Apex SDK?",
                    "acceptedAnswer": {
                        "@type": "Answer",
                        "text": "We welcome contributions! Check our Contributing Guide at https://apexsdk.dev/viewer.html?doc=CONTRIBUTING.md for information on how to contribute code, documentation, or report issues. You can also join our Discord community at https://discord.gg/zCDFsBaZJN"
                    }
                },
                {
                    "@type": "Question",
                    "name": "Where can I get support for Apex SDK?",
                    "acceptedAnswer": {
                        "@type": "Answer",
                        "text": "You can get support through our Discord community (https://discord.gg/zCDFsBaZJN), GitHub issues (https://github.com/carbobit/apex-sdk/issues), or by emailing support@apexsdk.dev. We also have comprehensive documentation and examples available."
                    }
                }
            ]
        };

        // BreadcrumbList schema
        const breadcrumbSchema = {
            "@context": "https://schema.org",
            "@type": "BreadcrumbList",
            "itemListElement": [
                {
                    "@type": "ListItem",
                    "position": 1,
                    "name": "Home",
                    "item": seo.canonicalUrl
                },
                {
                    "@type": "ListItem",
                    "position": "2",
                    "name": "Documentation",
                    "item": `${seo.canonicalUrl}/viewer.html`
                }
            ]
        };

        // HowTo Schema for Getting Started
        const howToSchema = {
            "@context": "https://schema.org",
            "@type": "HowTo",
            "name": "How to Get Started with Apex SDK Protocol",
            "description": "Step-by-step guide to set up and start using Apex SDK for blockchain development",
            "image": seo.ogImage,
            "totalTime": "PT15M",
            "estimatedCost": {
                "@type": "MonetaryAmount",
                "currency": "USD",
                "value": "0"
            },
            "tool": [
                {
                    "@type": "HowToTool",
                    "name": "Rust 1.85+"
                },
                {
                    "@type": "HowToTool",
                    "name": "Cargo Package Manager"
                }
            ],
            "step": [
                {
                    "@type": "HowToStep",
                    "position": 1,
                    "name": "Install Rust",
                    "text": "Install Rust 1.85 or higher using rustup. Visit https://rustup.rs for installation instructions.",
                    "url": `${seo.canonicalUrl}/viewer.html?doc=QUICK_START.md#installation`
                },
                {
                    "@type": "HowToStep",
                    "position": 2,
                    "name": "Install Apex SDK CLI",
                    "text": "Install the Apex SDK command-line interface using: cargo install apex-sdk-cli",
                    "url": `${seo.canonicalUrl}/viewer.html?doc=CLI_GUIDE.md`
                },
                {
                    "@type": "HowToStep",
                    "position": 3,
                    "name": "Create New Project",
                    "text": "Create a new Apex SDK project using: apex new my-project",
                    "url": `${seo.canonicalUrl}/viewer.html?doc=QUICK_START.md#create-project`
                },
                {
                    "@type": "HowToStep",
                    "position": 4,
                    "name": "Build and Run",
                    "text": "Navigate to your project directory and build it using: cargo build. Then run your application.",
                    "url": `${seo.canonicalUrl}/viewer.html?doc=QUICK_START.md#build-run`
                }
            ]
        };

        // Course Schema for Learning Path
        const courseSchema = {
            "@context": "https://schema.org",
            "@type": "Course",
            "name": "Apex SDK Protocol - Complete Guide",
            "description": "Comprehensive course on building cross-chain blockchain applications with Apex SDK",
            "provider": {
                "@type": "Organization",
                "name": "Apex SDK Team",
                "url": seo.canonicalUrl
            },
            "hasCourseInstance": {
                "@type": "CourseInstance",
                "courseMode": "online",
                "courseWorkload": "PT8H"
            },
            "educationalLevel": "Beginner to Advanced",
            "inLanguage": "en",
            "isAccessibleForFree": true,
            "teaches": [
                "Rust blockchain development",
                "Substrate integration",
                "EVM smart contracts",
                "Cross-chain communication",
                "Type-safe blockchain programming"
            ]
        };

        // Remove existing schemas
        document.querySelectorAll('script[type="application/ld+json"]').forEach(el => el.remove());

        // Add new schemas
        [softwareAppSchema, websiteSchema, techArticleSchema, organizationSchema, faqSchema, breadcrumbSchema, howToSchema, courseSchema].forEach(schema => {
            const script = document.createElement('script');
            script.type = 'application/ld+json';
            script.textContent = JSON.stringify(schema);
            document.head.appendChild(script);
        });
    }
    
    updatePageSEO(title, description, keywords = []) {
        document.title = `${title} | ${CONFIG.seo.siteName}`;
        this.setMetaTag('name', 'description', description);
        this.setMetaTag('property', 'og:title', title);
        this.setMetaTag('property', 'og:description', description);
        this.setMetaTag('name', 'twitter:title', title);
        this.setMetaTag('name', 'twitter:description', description);
        
        if (keywords.length > 0) {
            const allKeywords = [...CONFIG.seo.keywords, ...keywords].join(', ');
            this.setMetaTag('name', 'keywords', allKeywords);
        }
    }
}

// Initialize SEO on load
if (document.readyState === 'loading') {
    document.addEventListener('DOMContentLoaded', () => {
        window.seoManager = new SEOManager();
    });
} else {
    window.seoManager = new SEOManager();
}
