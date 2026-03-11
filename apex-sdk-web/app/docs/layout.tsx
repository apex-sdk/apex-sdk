'use client';

import React, { useState } from 'react';
import Link from 'next/link';
import { usePathname } from 'next/navigation';
import Head from 'next/head';
import { Logo } from '../components/Logo';
import TableOfContents from '../components/TableOfContents';
import EnhancedSearch from '../components/EnhancedSearch';

// Generate breadcrumb schema for documentation pages
function generateBreadcrumbSchema(pathname: string) {
  const pathSegments = pathname.split('/').filter(Boolean);
  const breadcrumbs = [
    {
      '@type': 'ListItem',
      position: 1,
      name: 'Home',
      item: 'https://apexsdk.dev'
    }
  ];

  let currentPath = '';
  let position = 2;

  // Map path segments to readable names
  const pathNames: Record<string, string> = {
    'docs': 'Documentation',
    'api': 'API Reference',
    'quickstart': 'Quick Start',
    'installation': 'Installation',
    'cli-guide': 'CLI Guide',
    'architecture': 'Architecture',
    'testing': 'Testing',
    'security': 'Security',
    'evm': 'EVM Chains',
    'substrate': 'Substrate Chains',
    'examples': 'Examples',
    'contributing': 'Contributing',
    'ecosystem': 'Ecosystem',
    'research': 'Research',
    'testnets': 'Test Networks',
    'typed-metadata': 'Typed Metadata'
  };

  pathSegments.forEach((segment) => {
    currentPath += `/${segment}`;
    const name = pathNames[segment] || segment.charAt(0).toUpperCase() + segment.slice(1);

    breadcrumbs.push({
      '@type': 'ListItem',
      position,
      name,
      item: `https://apexsdk.dev${currentPath}`
    });

    position++;
  });

  return {
    '@context': 'https://schema.org',
    '@type': 'BreadcrumbList',
    itemListElement: breadcrumbs
  };
}

// Static navigation for client-side use - SEO optimized
const staticNavigation = [
  {
    title: 'Getting Started',
    items: [
      { title: 'Installation Guide', href: '/docs/installation', description: 'Install Apex SDK and set up your Rust blockchain development environment' },
      { title: 'Quick Start Tutorial', href: '/docs/quickstart', description: 'Build your first cross-chain application in 10 minutes with Apex SDK' },
      { title: 'CLI Guide', href: '/docs/cli-guide', description: 'Command-line tools for blockchain development and deployment' },
    ]
  },
  {
    title: 'Core Concepts',
    items: [
      { title: 'Architecture Overview', href: '/docs/architecture', description: 'Understand Apex SDK cross-chain architecture and design patterns' },
      { title: 'Testing Framework', href: '/docs/testing', description: 'Testing strategies and tools for blockchain application development' },
      { title: 'Security Best Practices', href: '/docs/security', description: 'Security guidelines and audit checklist for production deployments' },
    ]
  },
  {
    title: 'Network Integration',
    items: [
      { title: 'Asset Hub Integration', href: '/docs/substrate', description: 'Deep dive into Polkadot Asset Hub: assets, NFTs, and typed metadata' },
      { title: 'PolkaVM & Solidity', href: '/docs/evm', description: 'Deploy and interact with Solidity contracts using PolkaVM on system chains' },
      { title: 'Revive Protocol', href: '/docs/revive', description: 'Asset recovery and migration patterns unique to the Revive protocol' },
      { title: 'Typed Metadata', href: '/docs/typed-metadata', description: 'Type-safe blockchain metadata system for Substrate networks' },
      { title: 'Test Networks', href: '/docs/testnets', description: 'Development and testing blockchain networks configuration' },
    ]
  },
  {
    title: 'Examples & Guides',
    items: [
      { title: 'Code Examples', href: '/docs/examples', description: 'Production-ready code samples and tutorials for common use cases' },
      { title: 'Research & Innovation', href: '/docs/research', description: 'Cutting-edge blockchain features and research implementations' },
    ]
  },
  {
    title: 'Reference',
    items: [
      { title: 'API Reference', href: '/docs/api', description: 'Complete Apex SDK API documentation with examples and parameters' },
      { title: 'Ecosystem Overview', href: '/docs/ecosystem', description: 'Blockchain networks, tools, and integration partners' },
      { title: 'Contributing Guide', href: '/docs/contributing', description: 'How to contribute to Apex SDK open source development' },
    ]
  }
];

export default function DocsLayout({
  children,
}: {
  children: React.ReactNode;
}) {
  const [isMobileMenuOpen, setIsMobileMenuOpen] = useState(false);
  const [collapsedSections, setCollapsedSections] = useState<Set<string>>(new Set());

  const currentPath = typeof window !== 'undefined' ? window.location.pathname : '';

  // Generate breadcrumb schema for SEO
  const breadcrumbSchema = generateBreadcrumbSchema(currentPath);

  // Generate breadcrumb from current path
  const getBreadcrumb = (path: string) => {
    const segments = path.split('/').filter(Boolean);
    const breadcrumb = segments.map((segment, index) => {
      const href = '/' + segments.slice(0, index + 1).join('/');
      const title = segment.charAt(0).toUpperCase() + segment.slice(1).replace(/-/g, ' ');
      return { href, title };
    });
    return [{ href: '/', title: 'Home' }, ...breadcrumb];
  };

  const breadcrumbs = getBreadcrumb(currentPath);

  const toggleSection = (sectionTitle: string) => {
    const newCollapsed = new Set(collapsedSections);
    if (newCollapsed.has(sectionTitle)) {
      newCollapsed.delete(sectionTitle);
    } else {
      newCollapsed.add(sectionTitle);
    }
    setCollapsedSections(newCollapsed);
  };

  const isCurrentPage = (href: string) => currentPath === href;

  return (
    <div className="min-h-screen bg-obsidian text-white">
      {/* Breadcrumb Schema for SEO */}
      <script
        type="application/ld+json"
        dangerouslySetInnerHTML={{
          __html: JSON.stringify(breadcrumbSchema),
        }}
      />

      {/* Header */}
      <nav className="fixed top-0 w-full z-40 bg-obsidian-light/80 backdrop-blur-md border-b border-obsidian-lighter">
        <div className="max-w-7xl mx-auto px-6 h-16 flex items-center justify-between">
          <Link href="/" className="flex items-center space-x-3 group">
            <Logo className="w-8 h-8 group-hover:scale-110 transition-transform duration-200" />
            <span className="font-bold text-xl tracking-tight text-white group-hover:text-hyperBlue transition-colors">Apex SDK</span>
          </Link>
          <div className="hidden md:flex items-center space-x-6">
            {/* Search Box */}
            <div className="relative w-48">
              <EnhancedSearch
                placeholder="Search docs..."
                className="w-full"
                compact={true}
              />
            </div>

            {/* Navigation Links */}
            <div className="flex items-center space-x-6 text-sm text-white font-medium">
              <Link href="/docs" className="text-gradient-primary hover:opacity-80 transition-all duration-300 flex items-center space-x-2 px-3 py-2 rounded-md">
                <svg className="w-4 h-4" fill="currentColor" viewBox="0 0 24 24">
                  <path d="M19 3H5c-1.1 0-2 .9-2 2v14c0 1.1.9 2 2 2h14c1.1 0 2-.9 2-2V5c0-1.1-.9-2-2-2zm-5 14H7v-2h7v2zm3-4H7v-2h10v2zm0-4H7V7h10v2z" />
                </svg>
                <span>Documentation</span>
              </Link>
              <Link href="/docs/api" className="text-gradient-primary hover:text-gradient-secondary transition-all duration-300 px-3 py-2 rounded-md flex items-center space-x-2">
                <svg className="w-4 h-4" fill="currentColor" viewBox="0 0 24 24">
                  <path d="M9 12l2 2 4-4m6 2a9 9 0 11-18 0 9 9 0 0118 0z" />
                </svg>
                <span>API Reference</span>
              </Link>
              <a href="https://github.com/apex-sdk/apex-sdk" target="_blank" rel="noopener noreferrer" className="text-gradient-primary hover:text-gradient-secondary transition-all duration-300 px-3 py-2 rounded-md flex items-center space-x-2">
                <svg className="w-4 h-4" fill="currentColor" viewBox="0 0 24 24">
                  <path fillRule="evenodd" d="M12 2C6.477 2 2 6.484 2 12.017c0 4.425 2.865 8.18 6.839 9.504.5.092.682-.217.682-.483 0-.237-.008-.868-.013-1.703-2.782.605-3.369-1.343-3.369-1.343-.454-1.158-1.11-1.466-1.11-1.466-.908-.62.069-.608.069-.608 1.003.07 1.531 1.032 1.531 1.032.892 1.53 2.341 1.088 2.91.832.092-.647.35-1.088.636-1.338-2.22-.253-4.555-1.113-4.555-4.951 0-1.093.39-1.988 1.029-2.688-.103-.253-.446-1.272.098-2.65 0 0 .84-.27 2.75 1.026A9.564 9.564 0 0112 6.844c.85.004 1.705.115 2.504.337 1.909-1.296 2.747-1.027 2.747-1.027.546 1.379.202 2.398.1 2.651.64.7 1.028 1.595 1.028 2.688 0 3.848-2.339 4.695-4.566 4.943.359.309.678.92.678 1.855 0 1.338-.012 2.419-.012 2.747 0 .268.18.58.688.482A10.019 10.019 0 0022 12.017C22 6.484 17.522 2 12 2z" clipRule="evenodd" />
                </svg>
                <span>GitHub</span>
              </a>
              <a href="https://discord.gg/zCDFsBaZJN" target="_blank" rel="noopener noreferrer" className="text-gradient-primary hover:text-gradient-secondary transition-all duration-300 px-3 py-2 rounded-md flex items-center space-x-2">
                <svg className="w-4 h-4" fill="currentColor" viewBox="0 0 24 24">
                  <path d="M20.317 4.37a19.791 19.791 0 0 0-4.885-1.515.074.074 0 0 0-.079.037c-.21.375-.444.864-.608 1.25a18.27 18.27 0 0 0-5.487 0 12.64 12.64 0 0 0-.617-1.25.077.077 0 0 0-.079-.037A19.736 19.736 0 0 0 3.677 4.37a.07.07 0 0 0-.032.027C.533 9.046-.32 13.58.099 18.057a.082.082 0 0 0 .031.057 19.9 19.9 0 0 0 5.993 3.03.078.078 0 0 0 .084-.028c.462-.63.874-1.295 1.226-1.994a.076.076 0 0 0-.041-.106 13.107 13.107 0 0 1-1.872-.892.077.077 0 0 1-.008-.128 10.2 10.2 0 0 0 .372-.292.074.074 0 0 1 .077-.010c3.928 1.793 8.18 1.793 12.062 0a.074.074 0 0 1 .078.01c.120.098.246.198.373.292a.077.077 0 0 1-.006.127 12.299 12.299 0 0 1-1.873.892.077.077 0 0 0-.041.107c.36.698.772 1.362 1.225 1.993a.076.076 0 0 0 .084.028 19.839 19.839 0 0 0 6.002-3.03.077.077 0 0 0 .032-.054c.5-5.177-.838-9.674-3.549-13.66a.061.061 0 0 0-.031-.03zM8.02 15.33c-1.183 0-2.157-1.085-2.157-2.419 0-1.333.956-2.419 2.157-2.419 1.21 0 2.176 1.096 2.157 2.42 0 1.333-.956 2.418-2.157 2.418zm7.975 0c-1.183 0-2.157-1.085-2.157-2.419 0-1.333.955-2.419 2.157-2.419 1.21 0 2.176 1.096 2.157 2.42 0 1.333-.946 2.418-2.157 2.418z" />
                </svg>
                <span>Discord</span>
              </a>
            </div>
          </div>

          {/* Mobile menu button */}
          <button
            onClick={() => setIsMobileMenuOpen(!isMobileMenuOpen)}
            className="md:hidden p-2 text-slate-gray hover:text-white transition-colors"
          >
            <svg className="w-6 h-6" fill="none" stroke="currentColor" viewBox="0 0 24 24">
              {isMobileMenuOpen ? (
                <path strokeLinecap="round" strokeLinejoin="round" strokeWidth={2} d="M6 18L18 6M6 6l12 12" />
              ) : (
                <path strokeLinecap="round" strokeLinejoin="round" strokeWidth={2} d="M4 6h16M4 12h16M4 18h16" />
              )}
            </svg>
          </button>
        </div>

        {/* Mobile menu */}
        {isMobileMenuOpen && (
          <div className="md:hidden bg-obsidian-light border-t border-obsidian-lighter">
            <div className="px-6 py-4 space-y-4">
              {/* Mobile Search */}
              <div className="mb-4">
                <EnhancedSearch
                  placeholder="Search docs..."
                  className="w-full"
                  compact={true}
                />
              </div>
              <Link href="/docs" className="block text-hyperBlue">Documentation</Link>
              <Link href="/docs/api" className="block text-slate-gray hover:text-hyperBlue transition-colors">API Reference</Link>
              <a href="https://github.com/apex-sdk/apex-sdk" target="_blank" rel="noopener noreferrer" className="block text-slate-gray hover:text-hyperBlue transition-colors">GitHub</a>
              <a href="https://discord.gg/zCDFsBaZJN" target="_blank" rel="noopener noreferrer" className="block text-slate-gray hover:text-hyperBlue transition-colors">Discord</a>
            </div>
          </div>
        )}
      </nav>

      <div className="pt-16 flex">
        {/* Sidebar */}
        <aside className="w-64 min-h-screen bg-obsidian-light border-r border-obsidian-lighter fixed left-0 top-16 overflow-y-auto hidden md:block">
          <div className="p-6">
            {/* Enhanced Search */}
            <div className="mb-6">
              <EnhancedSearch placeholder="Search docs..." />
            </div>

            <nav className="space-y-4">
              {staticNavigation.map((section) => {
                const isCollapsed = collapsedSections.has(section.title);
                return (
                  <div key={section.title}>
                    <button
                      onClick={() => toggleSection(section.title)}
                      className="w-full flex items-center justify-between font-semibold text-sm uppercase tracking-wider text-slate-gray mb-3 hover:text-white transition-colors"
                    >
                      {section.title}
                      <svg
                        className={`w-3 h-3 transition-transform duration-200 ${isCollapsed ? 'transform rotate-0' : 'transform rotate-90'
                          }`}
                        fill="none"
                        stroke="currentColor"
                        viewBox="0 0 24 24"
                      >
                        <path strokeLinecap="round" strokeLinejoin="round" strokeWidth={2} d="M9 5l7 7-7 7" />
                      </svg>
                    </button>
                    {!isCollapsed && (
                      <ul className="space-y-1 mb-4">
                        {section.items.map((item) => {
                          const isCurrent = isCurrentPage(item.href);
                          return (
                            <li key={item.href}>
                              <Link
                                href={item.href}
                                className={`block text-sm py-2 px-3 rounded-md transition-all duration-300 ${isCurrent
                                    ? 'text-white bg-hyperBlue/10 border-l-2 border-hyperBlue font-medium'
                                    : 'text-white/70 hover:text-white hover:bg-obsidian-lighter/50'
                                  }`}
                                title={item.description}
                              >
                                {item.title}
                              </Link>
                            </li>
                          )
                        }
                        )}
                      </ul>
                    )}
                  </div>
                );
              })}
            </nav>
          </div>
        </aside>

        {/* Main content */}
        <main className="flex-1 md:ml-64">
          {/* Breadcrumb Navigation */}
          {currentPath !== '/docs' && (
            <div className="bg-obsidian-light border-b border-obsidian-lighter">
              <div className="max-w-4xl mx-auto px-6 py-3">
                <nav className="flex" aria-label="Breadcrumb">
                  <ol className="flex items-center space-x-2 text-sm">
                    {breadcrumbs.map((crumb, index) => (
                      <li key={crumb.href} className="flex items-center">
                        {index > 0 && (
                          <svg className="w-3 h-3 text-slate-gray mx-2" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                            <path strokeLinecap="round" strokeLinejoin="round" strokeWidth={2} d="M9 5l7 7-7 7" />
                          </svg>
                        )}
                        {index === breadcrumbs.length - 1 ? (
                          <span className="text-white font-medium">{crumb.title}</span>
                        ) : (
                          <Link
                            href={crumb.href}
                            className="text-slate-gray hover:text-hyperBlue transition-colors"
                          >
                            {crumb.title}
                          </Link>
                        )}
                      </li>
                    ))}
                  </ol>
                </nav>
              </div>
            </div>
          )}

          <div className="flex max-w-7xl mx-auto relative">
            {/* Main content */}
            <div className="flex-1 max-w-4xl px-4 md:px-6 py-8">
              {/* Mobile Table of Contents Toggle */}
              <div className="xl:hidden mb-6">
                <details className="bg-obsidian-light border border-obsidian-lighter rounded-lg overflow-hidden group">
                  <summary className="px-4 py-3 text-sm font-semibold text-white/80 cursor-pointer hover:bg-obsidian-lighter list-none flex items-center justify-between">
                    <span>On this page</span>
                    <svg className="w-4 h-4 transition-transform group-open:rotate-180" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                      <path strokeLinecap="round" strokeLinejoin="round" strokeWidth={2} d="M19 9l-7 7-7-7" />
                    </svg>
                  </summary>
                  <div className="p-4 border-t border-obsidian-lighter">
                    <TableOfContents className="!static !top-0" />
                  </div>
                </details>
              </div>

              <div className="prose">
                {children}
              </div>
            </div>

            {/* Table of Contents - Desktop */}
            <div className="hidden xl:block w-64 pl-8 pt-8">
              <TableOfContents />
            </div>
          </div>
        </main>
      </div>
    </div>
  );
}