'use client';

import Link from 'next/link';
import CopyButton from './components/CopyButton';
import { Logo } from './components/Logo';

export default function Home() {
  return (
    <>
      {/* Structured Data for SEO */}
      <script
        type="application/ld+json"
        dangerouslySetInnerHTML={{
          __html: JSON.stringify({
            "@context": "https://schema.org",
            "@type": "SoftwareApplication",
            "name": "Apex SDK",
            "applicationCategory": "DeveloperApplication",
            "operatingSystem": "Linux, macOS, Windows",
            "description": "Enterprise Rust blockchain SDK for building cross-chain dApps on Ethereum, Polkadot and 15+ networks",
            "offers": {
              "@type": "Offer",
              "price": "0",
              "priceCurrency": "USD"
            },
            "programmingLanguage": "Rust",
            "author": {
              "@type": "Organization",
              "name": "Apex SDK Contributors",
              "url": "https://apex-sdk-docs.pages.dev"
            },
            "url": "https://apexsdk.dev",
            "softwareVersion": "0.1.6",
            "license": "https://github.com/apex-sdk/apex-sdk/blob/main/LICENSE",
            "downloadUrl": "https://crates.io/crates/apex-sdk",
            "documentation": "https://apexsdk.dev/docs/api",
            "codeRepository": "https://github.com/apex-sdk/apex-sdk",
            "aggregateRating": {
              "@type": "AggregateRating",
              "ratingValue": "4.8",
              "reviewCount": "12"
            }
          })
        }}
      ></script>

    <main className="min-h-screen bg-obsidian text-white selection:bg-hyperBlue-dim selection:text-hyperBlue font-sans pb-20">

      {/* Header/Nav */}
      <nav className="fixed top-0 w-full z-40 bg-obsidian-light/80 border-b border-obsidian-lighter">
        <div className="max-w-7xl mx-auto px-6 h-16 flex items-center justify-between">
          <Link href="/" className="flex items-center space-x-3 group">
            <Logo className="w-8 h-8 group-hover:scale-110 transition-transform duration-200" />
            <span className="font-bold text-xl tracking-tight text-white group-hover:text-hyperBlue transition-colors">Apex SDK</span>
          </Link>
          <div className="hidden md:flex items-center space-x-8 text-sm text-white font-medium">
            <Link href="/docs" className="text-gradient-primary hover:opacity-80 transition-all duration-300 flex items-center space-x-2 px-3 py-2 rounded-md">
              <svg className="w-4 h-4" fill="currentColor" viewBox="0 0 24 24">
                <path d="M19 3H5c-1.1 0-2 .9-2 2v14c0 1.1.9 2 2 2h14c1.1 0 2-.9 2-2V5c0-1.1-.9-2-2-2zm-5 14H7v-2h7v2zm3-4H7v-2h10v2zm0-4H7V7h10v2z"/>
              </svg>
              <span>Documentation</span>
            </Link>
            <Link href="/docs/api" className="text-gradient-primary hover:text-gradient-secondary transition-all duration-300 px-3 py-2 rounded-md flex items-center space-x-2">
              <svg className="w-4 h-4" fill="currentColor" viewBox="0 0 24 24">
                <path d="M9 12l2 2 4-4m6 2a9 9 0 11-18 0 9 9 0 0118 0z"/>
              </svg>
              <span>API Reference</span>
            </Link>
            <a href="https://github.com/apex-sdk/apex-sdk" target="_blank" rel="noopener noreferrer" className="text-gradient-primary hover:text-gradient-secondary transition-all duration-300 px-3 py-2 rounded-md flex items-center space-x-2">
              <svg className="w-4 h-4" fill="currentColor" viewBox="0 0 24 24">
                <path fillRule="evenodd" d="M12 2C6.477 2 2 6.484 2 12.017c0 4.425 2.865 8.18 6.839 9.504.5.092.682-.217.682-.483 0-.237-.008-.868-.013-1.703-2.782.605-3.369-1.343-3.369-1.343-.454-1.158-1.11-1.466-1.11-1.466-.908-.62.069-.608.069-.608 1.003.07 1.531 1.032 1.531 1.032.892 1.53 2.341 1.088 2.91.832.092-.647.35-1.088.636-1.338-2.22-.253-4.555-1.113-4.555-4.951 0-1.093.39-1.988 1.029-2.688-.103-.253-.446-1.272.098-2.65 0 0 .84-.27 2.75 1.026A9.564 9.564 0 0112 6.844c.85.004 1.705.115 2.504.337 1.909-1.296 2.747-1.027 2.747-1.027.546 1.379.202 2.398.1 2.651.64.7 1.028 1.595 1.028 2.688 0 3.848-2.339 4.695-4.566 4.943.359.309.678.92.678 1.855 0 1.338-.012 2.419-.012 2.747 0 .268.18.58.688.482A10.019 10.019 0 0022 12.017C22 6.484 17.522 2 12 2z" clipRule="evenodd" />
              </svg>
              <span>GitHub</span>
            </a>
          </div>
        </div>
      </nav>

      {/* Hero Section */}
      <section className="min-h-screen flex flex-col justify-center pt-20 pb-24 px-6 relative overflow-hidden">
        {/* Background Gradients */}
        <div className="absolute inset-0" style={{
          background: 'linear-gradient(135deg, var(--background) 0%, #1a1f2e 50%, var(--surface) 100%)'
        }} />
        <div className="absolute top-1/3 left-1/2 -translate-x-1/2 w-[800px] h-[400px] bg-blue-500/10 blur-[100px] rounded-full pointer-events-none" />

        <div className="max-w-5xl mx-auto text-center relative z-10">
          <h1 className="text-6xl md:text-8xl lg:text-9xl font-black tracking-tight mb-8 text-gradient-hero bg-clip-text">
            The definitive Rust framework for Multichain Development
          </h1>

          <div className="mb-6">
            <h2 className="text-xl md:text-2xl text-white/90 font-medium mb-2">
              Build secure, scalable, and interoperable blockchain apps for Substrate, EVM, and beyond
            </h2>
            <p className="text-lg md:text-xl text-white/70 max-w-3xl mx-auto leading-relaxed">
              Apex SDK is the open-source standard for multichain dApp development. Type-safe APIs, blazing fast Rust performance, and unified tooling for the next generation of Web3 builders.
            </p>
          </div>

          <div className="flex flex-col sm:flex-row items-center justify-center gap-6 mt-12">
            {/* Cargo Add Command */}
            <div className="group relative">
              <div className="flex items-center border border-gray-700 rounded-lg overflow-hidden transition-all duration-300 hover:shadow-lg" style={{
                background: 'linear-gradient(135deg, rgba(31, 40, 51, 0.8) 0%, rgba(44, 52, 64, 0.4) 100%)'
              }}
              onMouseEnter={(e: React.MouseEvent<HTMLDivElement>) => {
                e.currentTarget.style.borderImage = 'linear-gradient(135deg, #2176FF, #FF5A1F) 1';
                e.currentTarget.style.boxShadow = '0 10px 25px -5px rgba(33, 118, 255, 0.3), 0 10px 10px -5px rgba(255, 90, 31, 0.1)';
              }}
              onMouseLeave={(e: React.MouseEvent<HTMLDivElement>) => {
                e.currentTarget.style.borderImage = '';
                e.currentTarget.style.boxShadow = '';
              }}>
                <div className="px-4 py-3 font-mono text-sm text-white border-r border-gray-700 select-all group-hover:bg-gray-800/50 transition-all duration-300" style={{
                  background: 'rgba(15, 23, 42, 0.8)'
                }}>
                  cargo add apex-sdk
                </div>
                <CopyButton 
                  text="cargo add apex-sdk"
                  className="px-3 py-3 text-white/60 hover:text-white transition-all duration-300 bg-transparent"
                />
              </div>
            </div>

            <Link href="/docs/quickstart" className="px-6 py-3 border border-gray-700 rounded-lg text-white transition-all duration-300 font-medium hover:shadow-lg" style={{
              background: 'linear-gradient(135deg, rgba(44, 52, 64, 0.4) 0%, rgba(31, 40, 51, 0.8) 100%)'
            }}
            onMouseEnter={(e) => {
              e.currentTarget.style.borderImage = 'linear-gradient(135deg, #2176FF, #FF5A1F) 1';
              e.currentTarget.style.boxShadow = '0 10px 25px -5px rgba(33, 118, 255, 0.3), 0 10px 10px -5px rgba(255, 90, 31, 0.1)';
            }}
            onMouseLeave={(e) => {
              e.currentTarget.style.borderImage = '';
              e.currentTarget.style.boxShadow = '';
            }}>
              Quick Start Guide
            </Link>
          </div>
        </div>
      </section>

      {/* Features Section */}
      <section className="py-24 px-6 relative" id="features" style={{
        background: 'linear-gradient(180deg, rgba(44, 52, 64, 0.3) 0%, rgba(31, 40, 51, 0.1) 100%)'
      }}>
        <div className="max-w-7xl mx-auto">
          <div className="text-center mb-16">
            <h2 className="text-4xl md:text-5xl font-bold tracking-tight mb-6">
              Core Features of Apex SDK
            </h2>
            <p className="text-xl text-white/70 max-w-3xl mx-auto">
              Everything you need to build production-ready cross-chain blockchain applications
            </p>
          </div>

          <div className="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 gap-8">
            {/* Type-Safe Multi-Chain Transactions */}
            <div className="border border-gray-700 rounded-xl p-8 transition-all duration-300" style={{
              background: 'linear-gradient(135deg, rgba(31, 40, 51, 0.8) 0%, rgba(44, 52, 64, 0.4) 100%)'
            }}
            onMouseEnter={(e) => {
              e.currentTarget.style.borderImage = 'linear-gradient(135deg, #2176FF, #FF5A1F) 1';
              e.currentTarget.style.boxShadow = '0 10px 25px -5px rgba(33, 118, 255, 0.3), 0 10px 10px -5px rgba(255, 90, 31, 0.1)';
            }}
            onMouseLeave={(e) => {
              e.currentTarget.style.borderImage = '';
              e.currentTarget.style.boxShadow = '';
            }}>
              <h3 className="text-xl font-semibold mb-4 text-hyperBlue">Type-Safe Multi-Chain Transactions</h3>
              <p className="text-white/70 mb-4">
                Execute transactions across EVM and Substrate chains with compile-time safety. No more runtime errors from type mismatches.
              </p>
              <Link href="/docs/api#transaction-pipeline" className="text-hyperBlue hover:underline text-sm font-medium">
                Learn about Transaction Pipeline →
              </Link>
            </div>

            {/* Enterprise-Grade Security */}
            <div className="border border-gray-700 rounded-xl p-8 transition-all duration-300" style={{
              background: 'linear-gradient(135deg, rgba(31, 40, 51, 0.8) 0%, rgba(44, 52, 64, 0.4) 100%)'
            }}
            onMouseEnter={(e) => {
              e.currentTarget.style.borderImage = 'linear-gradient(135deg, #2176FF, #FF5A1F) 1';
              e.currentTarget.style.boxShadow = '0 10px 25px -5px rgba(33, 118, 255, 0.3), 0 10px 10px -5px rgba(255, 90, 31, 0.1)';
            }}
            onMouseLeave={(e) => {
              e.currentTarget.style.borderImage = '';
              e.currentTarget.style.boxShadow = '';
            }}>
              <h3 className="text-xl font-semibold mb-4 text-hyperBlue">Enterprise-Grade Security</h3>
              <p className="text-white/70 mb-4">
                Built-in security best practices, audit trails, and secure key management for production deployments.
              </p>
              <Link href="/docs/security" className="text-hyperBlue hover:underline text-sm font-medium">
                Security Guide →
              </Link>
            </div>

            {/* Unified API for EVM & Substrate */}
            <div className="border border-gray-700 rounded-xl p-8 transition-all duration-300" style={{
              background: 'linear-gradient(135deg, rgba(31, 40, 51, 0.8) 0%, rgba(44, 52, 64, 0.4) 100%)'
            }}
            onMouseEnter={(e) => {
              e.currentTarget.style.borderImage = 'linear-gradient(135deg, #2176FF, #FF5A1F) 1';
              e.currentTarget.style.boxShadow = '0 10px 25px -5px rgba(33, 118, 255, 0.3), 0 10px 10px -5px rgba(255, 90, 31, 0.1)';
            }}
            onMouseLeave={(e) => {
              e.currentTarget.style.borderImage = '';
              e.currentTarget.style.boxShadow = '';
            }}>
              <h3 className="text-xl font-semibold mb-4 text-hyperBlue">Unified API for EVM & Substrate</h3>
              <p className="text-white/70 mb-4">
                One consistent API for all blockchain interactions. Switch between Ethereum and Polkadot with minimal code changes.
              </p>
              <Link href="/docs/architecture" className="text-hyperBlue hover:underline text-sm font-medium">
                Architecture Overview →
              </Link>
            </div>

            {/* Performance Optimized */}
            <div className="border border-gray-700 rounded-xl p-8 transition-all duration-300" style={{
              background: 'linear-gradient(135deg, rgba(31, 40, 51, 0.8) 0%, rgba(44, 52, 64, 0.4) 100%)'
            }}
            onMouseEnter={(e) => {
              e.currentTarget.style.borderImage = 'linear-gradient(135deg, #2176FF, #FF5A1F) 1';
              e.currentTarget.style.boxShadow = '0 10px 25px -5px rgba(33, 118, 255, 0.3), 0 10px 10px -5px rgba(255, 90, 31, 0.1)';
            }}
            onMouseLeave={(e) => {
              e.currentTarget.style.borderImage = '';
              e.currentTarget.style.boxShadow = '';
            }}>
              <h3 className="text-xl font-semibold mb-4 text-hyperBlue">Performance Optimized</h3>
              <p className="text-white/70 mb-4">
                Zero-cost abstractions and efficient runtime. Process thousands of transactions with minimal resource usage.
              </p>
              <Link href="/docs/examples" className="text-hyperBlue hover:underline text-sm font-medium">
                View Benchmarks →
              </Link>
            </div>

            {/* Developer Experience */}
            <div className="border border-gray-700 rounded-xl p-8 transition-all duration-300" style={{
              background: 'linear-gradient(135deg, rgba(31, 40, 51, 0.8) 0%, rgba(44, 52, 64, 0.4) 100%)'
            }}
            onMouseEnter={(e) => {
              e.currentTarget.style.borderImage = 'linear-gradient(135deg, #2176FF, #FF5A1F) 1';
              e.currentTarget.style.boxShadow = '0 10px 25px -5px rgba(33, 118, 255, 0.3), 0 10px 10px -5px rgba(255, 90, 31, 0.1)';
            }}
            onMouseLeave={(e) => {
              e.currentTarget.style.borderImage = '';
              e.currentTarget.style.boxShadow = '';
            }}>
              <h3 className="text-xl font-semibold mb-4 text-hyperBlue">Developer Experience</h3>
              <p className="text-white/70 mb-4">
                Rich IDE support, comprehensive documentation, and powerful CLI tools. Get productive from day one.
              </p>
              <Link href="/docs/cli-guide" className="text-hyperBlue hover:underline text-sm font-medium">
                CLI Guide →
              </Link>
            </div>

            {/* Production Ready */}
            <div className="border border-gray-700 rounded-xl p-8 transition-all duration-300" style={{
              background: 'linear-gradient(135deg, rgba(31, 40, 51, 0.8) 0%, rgba(44, 52, 64, 0.4) 100%)'
            }}
            onMouseEnter={(e) => {
              e.currentTarget.style.borderImage = 'linear-gradient(135deg, #2176FF, #FF5A1F) 1';
              e.currentTarget.style.boxShadow = '0 10px 25px -5px rgba(33, 118, 255, 0.3), 0 10px 10px -5px rgba(255, 90, 31, 0.1)';
            }}
            onMouseLeave={(e) => {
              e.currentTarget.style.borderImage = '';
              e.currentTarget.style.boxShadow = '';
            }}>
              <h3 className="text-xl font-semibold mb-4 text-hyperBlue">Production Ready</h3>
              <p className="text-white/70 mb-4">
                Battle-tested in production environments. Comprehensive testing suite with 119+ passing tests.
              </p>
              <Link href="/docs/testing" className="text-hyperBlue hover:underline text-sm font-medium">
                Testing Guide →
              </Link>
            </div>
          </div>
        </div>
      </section>

      {/* Supported Networks Section */}
      <section className="py-24 px-6" id="networks">
        <div className="max-w-7xl mx-auto">
          <div className="text-center mb-16">
            <h2 className="text-4xl md:text-5xl font-bold tracking-tight mb-6">
              Supported Blockchain Networks
            </h2>
            <p className="text-xl text-white/70 max-w-3xl mx-auto">
              Build once, deploy everywhere. Apex SDK supports 15+ blockchain networks out of the box.
            </p>
          </div>

          <div className="grid grid-cols-1 md:grid-cols-2 gap-12">
            {/* EVM Networks */}
            <div className="border border-gray-700 rounded-xl p-8" style={{
              background: 'linear-gradient(135deg, rgba(31, 40, 51, 0.8) 0%, rgba(44, 52, 64, 0.4) 100%)'
            }}>
              <h3 className="text-2xl font-semibold mb-6 text-hyperBlue">Ethereum & EVM-Compatible Chains</h3>
              <div className="grid grid-cols-2 gap-4 mb-6">
                <div className="text-white/80">• Ethereum</div>
                <div className="text-white/80">• Polygon</div>
                <div className="text-white/80">• BSC</div>
                <div className="text-white/80">• Arbitrum</div>
                <div className="text-white/80">• Optimism</div>
                <div className="text-white/80">• Avalanche</div>
                <div className="text-white/80">• Fantom</div>
                <div className="text-white/80">• Moonbeam</div>
              </div>
              <Link href="/docs/evm" className="text-hyperBlue hover:underline font-medium">
                EVM Integration Guide →
              </Link>
            </div>

            {/* Substrate Networks */}
            <div className="border border-gray-700 rounded-xl p-8" style={{
              background: 'linear-gradient(135deg, rgba(31, 40, 51, 0.8) 0%, rgba(44, 52, 64, 0.4) 100%)'
            }}>
              <h3 className="text-2xl font-semibold mb-6 text-hyperBlue">Polkadot & Substrate Parachains</h3>
              <div className="grid grid-cols-2 gap-4 mb-6">
                <div className="text-white/80">• Polkadot</div>
                <div className="text-white/80">• Kusama</div>
                <div className="text-white/80">• Acala</div>
                <div className="text-white/80">• Moonriver</div>
                <div className="text-white/80">• Astar</div>
                <div className="text-white/80">• Parallel</div>
                <div className="text-white/80">• Centrifuge</div>
                <div className="text-white/80">• Custom</div>
              </div>
              <Link href="/docs/substrate" className="text-hyperBlue hover:underline font-medium">
                Substrate Integration Guide →
              </Link>
            </div>
          </div>
        </div>
      </section>

      {/* Use Cases Section */}
      <section className="py-24 px-6 relative" id="use-cases" style={{
        background: 'linear-gradient(180deg, rgba(44, 52, 64, 0.3) 0%, rgba(31, 40, 51, 0.1) 100%)'
      }}>
        <div className="max-w-7xl mx-auto">
          <div className="text-center mb-16">
            <h2 className="text-4xl md:text-5xl font-bold tracking-tight mb-6">
              Real-World Use Cases
            </h2>
            <p className="text-xl text-white/70 max-w-3xl mx-auto">
              See how developers are using Apex SDK to build the next generation of blockchain applications
            </p>
          </div>

          <div className="grid grid-cols-1 lg:grid-cols-3 gap-8">
            {/* DeFi Protocol Development */}
            <div className="border border-gray-700 rounded-xl p-8 transition-all duration-300" style={{
              background: 'linear-gradient(135deg, rgba(31, 40, 51, 0.8) 0%, rgba(44, 52, 64, 0.4) 100%)'
            }}
            onMouseEnter={(e) => {
              e.currentTarget.style.borderImage = 'linear-gradient(135deg, #2176FF, #FF5A1F) 1';
              e.currentTarget.style.boxShadow = '0 10px 25px -5px rgba(33, 118, 255, 0.3), 0 10px 10px -5px rgba(255, 90, 31, 0.1)';
            }}
            onMouseLeave={(e) => {
              e.currentTarget.style.borderImage = '';
              e.currentTarget.style.boxShadow = '';
            }}>
              <h3 className="text-xl font-semibold mb-4 text-hyperBlue">DeFi Protocol Development</h3>
              <p className="text-white/70 mb-4">
                Build decentralized exchanges, lending protocols, and yield farms with cross-chain liquidity management.
              </p>
              <Link href="/docs/examples#defi" className="text-hyperBlue hover:underline text-sm font-medium">
                DeFi Examples →
              </Link>
            </div>

            {/* NFT Marketplace Creation */}
            <div className="border border-gray-700 rounded-xl p-8 transition-all duration-300" style={{
              background: 'linear-gradient(135deg, rgba(31, 40, 51, 0.8) 0%, rgba(44, 52, 64, 0.4) 100%)'
            }}
            onMouseEnter={(e) => {
              e.currentTarget.style.borderImage = 'linear-gradient(135deg, #2176FF, #FF5A1F) 1';
              e.currentTarget.style.boxShadow = '0 10px 25px -5px rgba(33, 118, 255, 0.3), 0 10px 10px -5px rgba(255, 90, 31, 0.1)';
            }}
            onMouseLeave={(e) => {
              e.currentTarget.style.borderImage = '';
              e.currentTarget.style.boxShadow = '';
            }}>
              <h3 className="text-xl font-semibold mb-4 text-hyperBlue">NFT Marketplace Creation</h3>
              <p className="text-white/70 mb-4">
                Create multi-chain NFT marketplaces with seamless cross-chain transfers and unified metadata standards.
              </p>
              <Link href="/docs/examples#nft" className="text-hyperBlue hover:underline text-sm font-medium">
                NFT Examples →
              </Link>
            </div>

            {/* Cross-Chain Bridge Building */}
            <div className="border border-gray-700 rounded-xl p-8 transition-all duration-300" style={{
              background: 'linear-gradient(135deg, rgba(31, 40, 51, 0.8) 0%, rgba(44, 52, 64, 0.4) 100%)'
            }}
            onMouseEnter={(e) => {
              e.currentTarget.style.borderImage = 'linear-gradient(135deg, #2176FF, #FF5A1F) 1';
              e.currentTarget.style.boxShadow = '0 10px 25px -5px rgba(33, 118, 255, 0.3), 0 10px 10px -5px rgba(255, 90, 31, 0.1)';
            }}
            onMouseLeave={(e) => {
              e.currentTarget.style.borderImage = '';
              e.currentTarget.style.boxShadow = '';
            }}>
              <h3 className="text-xl font-semibold mb-4 text-hyperBlue">Cross-Chain Bridge Building</h3>
              <p className="text-white/70 mb-4">
                Implement secure cross-chain bridges with built-in validation, monitoring, and recovery mechanisms.
              </p>
              <Link href="/docs/examples#bridges" className="text-hyperBlue hover:underline text-sm font-medium">
                Bridge Examples →
              </Link>
            </div>
          </div>
        </div>
      </section>

      {/* Footer */}
      <footer className="fixed bottom-0 left-0 right-0 z-30 border-t border-obsidian-lighter py-4 px-6 bg-obsidian-light/95 backdrop-blur-sm">
        <div className="max-w-7xl mx-auto flex flex-col md:flex-row justify-between items-center text-sm text-white/60">
          <div className="mb-6 md:mb-0 flex items-center space-x-3">
            <Logo className="w-6 h-6" />
            <div>
              <span className="font-bold text-white block">Apex SDK Protocol</span>
              <span className="text-xs">© 2026 Apex SDK</span>
            </div>
          </div>
          <div className="flex space-x-8">
            <Link href="/docs" className="text-gradient-primary hover:text-gradient-secondary transition-all duration-300 px-2 py-1 rounded-md">Documentation</Link>
            <Link href="/docs/api" className="text-gradient-primary hover:text-gradient-secondary transition-all duration-300 px-2 py-1 rounded-md">API Reference</Link>
            <a 
              href="https://github.com/apex-sdk/apex-sdk" 
              target="_blank" 
              rel="noopener noreferrer" 
              className="text-gradient-primary hover:text-gradient-secondary transition-all duration-300 p-2 rounded-md"
              title="GitHub Repository"
            >
              <svg className="w-5 h-5" fill="currentColor" viewBox="0 0 24 24" aria-hidden="true">
                <path fillRule="evenodd" d="M12 2C6.477 2 2 6.484 2 12.017c0 4.425 2.865 8.18 6.839 9.504.5.092.682-.217.682-.483 0-.237-.008-.868-.013-1.703-2.782.605-3.369-1.343-3.369-1.343-.454-1.158-1.11-1.466-1.11-1.466-.908-.62.069-.608.069-.608 1.003.07 1.531 1.032 1.531 1.032.892 1.53 2.341 1.088 2.91.832.092-.647.35-1.088.636-1.338-2.22-.253-4.555-1.113-4.555-4.951 0-1.093.39-1.988 1.029-2.688-.103-.253-.446-1.272.098-2.65 0 0 .84-.27 2.75 1.026A9.564 9.564 0 0112 6.844c.85.004 1.705.115 2.504.337 1.909-1.296 2.747-1.027 2.747-1.027.546 1.379.202 2.398.1 2.651.64.7 1.028 1.595 1.028 2.688 0 3.848-2.339 4.695-4.566 4.943.359.309.678.92.678 1.855 0 1.338-.012 2.419-.012 2.747 0 .268.18.58.688.482A10.019 10.019 0 0022 12.017C22 6.484 17.522 2 12 2z" clipRule="evenodd" />
              </svg>
            </a>
            <a 
              href="https://discord.gg/zCDFsBaZJN" 
              className="text-gradient-primary hover:text-gradient-secondary transition-all duration-300 p-2 rounded-md"
              title="Discord Community"
            >
              <svg className="w-5 h-5" fill="currentColor" viewBox="0 0 24 24" aria-hidden="true">
                <path d="M20.317 4.37a19.791 19.791 0 0 0-4.885-1.515.074.074 0 0 0-.079.037c-.21.375-.444.864-.608 1.25a18.27 18.27 0 0 0-5.487 0 12.64 12.64 0 0 0-.617-1.25.077.077 0 0 0-.079-.037A19.736 19.736 0 0 0 3.677 4.37a.07.07 0 0 0-.032.027C.533 9.046-.32 13.58.099 18.057a.082.082 0 0 0 .031.057 19.9 19.9 0 0 0 5.993 3.03.078.078 0 0 0 .084-.028c.462-.63.874-1.295 1.226-1.994a.076.076 0 0 0-.041-.106 13.107 13.107 0 0 1-1.872-.892.077.077 0 0 1-.008-.128 10.2 10.2 0 0 0 .372-.292.074.074 0 0 1 .077-.010c3.928 1.793 8.18 1.793 12.062 0a.074.074 0 0 1 .078.01c.120.098.246.198.373.292a.077.077 0 0 1-.006.127 12.299 12.299 0 0 1-1.873.892.077.077 0 0 0-.041.107c.36.698.772 1.362 1.225 1.993a.076.076 0 0 0 .084.028 19.839 19.839 0 0 0 6.002-3.03.077.077 0 0 0 .032-.054c.5-5.177-.838-9.674-3.549-13.66a.061.061 0 0 0-.031-.03zM8.02 15.33c-1.183 0-2.157-1.085-2.157-2.419 0-1.333.956-2.419 2.157-2.419 1.21 0 2.176 1.096 2.157 2.42 0 1.333-.956 2.418-2.157 2.418zm7.975 0c-1.183 0-2.157-1.085-2.157-2.419 0-1.333.955-2.419 2.157-2.419 1.21 0 2.176 1.096 2.157 2.42 0 1.333-.946 2.418-2.157 2.418z"/>
              </svg>
            </a>
            <a 
              href="https://x.com/apexsdk" 
              className="text-gradient-primary hover:text-gradient-secondary transition-all duration-300 p-2 rounded-md"
              title="Follow us on X"
            >
              <svg className="w-5 h-5" fill="currentColor" viewBox="0 0 24 24" aria-hidden="true">
                <path d="M18.244 2.25h3.308l-7.227 8.26 8.502 11.24H16.17l-5.214-6.817L4.99 21.75H1.68l7.73-8.835L1.254 2.25H8.08l4.713 6.231zm-1.161 17.52h1.833L7.084 4.126H5.117z"/>
              </svg>
            </a>
          </div>
        </div>
      </footer>
    </main>
    </>
  );
}
