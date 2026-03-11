'use client';

import React, { useState, useEffect } from 'react';
import Link from 'next/link';

interface NavigationItem {
  title: string;
  href: string;
  description?: string;
}

interface NavigationSection {
  title: string;
  items: NavigationItem[];
}

// Static navigation data
const staticNavigation: NavigationSection[] = [
  {
    title: 'Getting Started',
    items: [
      {
        title: 'Quick Start',
        href: '/docs/quickstart',
        description: 'Get up and running in 5 minutes'
      },
      {
        title: 'Installation',
        href: '/docs/installation',
        description: 'Install and configure Apex SDK'
      },
      {
        title: 'Examples',
        href: '/docs/examples',
        description: 'Code examples and tutorials'
      }
    ]
  },
  {
    title: 'Core Concepts',
    items: [
      {
        title: 'Architecture',
        href: '/docs/architecture',
        description: 'System architecture overview'
      },
      {
        title: 'EVM Support',
        href: '/docs/evm',
        description: 'Ethereum Virtual Machine integration'
      },
      {
        title: 'Substrate',
        href: '/docs/substrate',
        description: 'Substrate framework support'
      }
    ]
  },
  {
    title: 'API Reference',
    items: [
      {
        title: 'API Documentation',
        href: '/docs/api',
        description: 'Complete API reference'
      },
      {
        title: 'Typed Metadata',
        href: '/docs/typed-metadata',
        description: 'Type-safe metadata handling'
      }
    ]
  },
  {
    title: 'Guides',
    items: [
      {
        title: 'Testing',
        href: '/docs/testing',
        description: 'Testing strategies and tools'
      },
      {
        title: 'Security',
        href: '/docs/security',
        description: 'Security best practices'
      },
      {
        title: 'Contributing',
        href: '/docs/contributing',
        description: 'Contribute to the project'
      }
    ]
  },
  {
    title: 'Resources',
    items: [
      {
        title: 'CLI Guide',
        href: '/docs/cli-guide',
        description: 'Command-line interface reference'
      },
      {
        title: 'Testnets',
        href: '/docs/testnets',
        description: 'Test network configurations'
      },
      {
        title: 'Ecosystem',
        href: '/docs/ecosystem',
        description: 'Related tools and libraries'
      },
      {
        title: 'Research',
        href: '/docs/research',
        description: 'Research papers and articles'
      }
    ]
  }
];

interface MobileNavigationProps {
  isOpen: boolean;
  onClose: () => void;
  currentPath: string;
}

export default function MobileNavigation({ isOpen, onClose, currentPath }: MobileNavigationProps) {
  const [collapsedSections, setCollapsedSections] = useState<Set<string>>(new Set());

  // Prevent body scroll when mobile menu is open
  useEffect(() => {
    if (isOpen) {
      document.body.style.overflow = 'hidden';
    } else {
      document.body.style.overflow = 'unset';
    }

    return () => {
      document.body.style.overflow = 'unset';
    };
  }, [isOpen]);

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

  if (!isOpen) return null;

  return (
    <div className="fixed inset-0 z-50 md:hidden">
      {/* Backdrop */}
      <div 
        className="fixed inset-0 bg-black/50 backdrop-blur-sm"
        onClick={onClose}
      />
      
      {/* Navigation Panel */}
      <div className="fixed top-0 left-0 bottom-0 w-80 bg-obsidian-light border-r border-obsidian-lighter overflow-y-auto">
        {/* Header */}
        <div className="sticky top-0 bg-obsidian-light border-b border-obsidian-lighter p-4 flex items-center justify-between">
          <h2 className="font-semibold text-white">Navigation</h2>
          <button
            onClick={onClose}
            className="p-2 text-slate-gray hover:text-white transition-colors rounded-md"
          >
            <svg className="w-5 h-5" fill="none" stroke="currentColor" viewBox="0 0 24 24">
              <path strokeLinecap="round" strokeLinejoin="round" strokeWidth={2} d="M6 18L18 6M6 6l12 12" />
            </svg>
          </button>
        </div>

        {/* Navigation Content */}
        <div className="p-4">
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
                      className={`w-3 h-3 transition-transform duration-200 ${
                        isCollapsed ? 'transform rotate-0' : 'transform rotate-90'
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
                              onClick={onClose}
                              className={`block text-sm py-3 px-3 rounded-md transition-all duration-300 ${
                                isCurrent 
                                  ? 'text-white bg-hyperBlue/10 border-l-2 border-hyperBlue font-medium'
                                  : 'text-white/70 hover:text-white hover:bg-obsidian-lighter/50'
                              }`}
                            >
                              {item.title}
                              {item.description && (
                                <div className="text-xs text-white/50 mt-1">
                                  {item.description}
                                </div>
                              )}
                            </Link>
                          </li>
                        );
                      })}
                    </ul>
                  )}
                </div>
              );
            })}
          </nav>
        </div>
      </div>
    </div>
  );
}