'use client';

import React, { useState, useEffect, useMemo } from 'react';
import Link from 'next/link';

interface SearchResult {
  title: string;
  href: string;
  content: string;
  section: string;
  type: 'page' | 'section' | 'method' | 'example';
}

interface EnhancedSearchProps {
  className?: string;
  placeholder?: string;
  compact?: boolean; // New prop for header version
}

// Mock search data - in real implementation, this would come from an API or indexed content
const searchData: SearchResult[] = [
  {
    title: 'Quick Start Guide',
    href: '/docs/quickstart',
    content: 'Get up and running with Apex SDK in 5 minutes. Build your first cross-chain application.',
    section: 'Getting Started',
    type: 'page'
  },
  {
    title: 'API Reference',
    href: '/docs/api',
    content: 'Complete API documentation for all Apex SDK classes and methods.',
    section: 'Reference',
    type: 'page'
  },
  {
    title: 'ApexClient.new()',
    href: '/docs/api#core-classes',
    content: 'Creates a new ApexClient instance with the specified network configuration.',
    section: 'API Reference',
    type: 'method'
  },
  {
    title: 'Cross-Chain Transfer',
    href: '/docs/api#cross-chain',
    content: 'Transfer assets between different blockchain networks using bridges.',
    section: 'API Reference',
    type: 'example'
  },
  {
    title: 'Installation',
    href: '/docs/installation',
    content: 'System requirements and installation guide for Apex SDK.',
    section: 'Getting Started',
    type: 'page'
  },
  {
    title: 'Smart Contract Integration',
    href: '/docs/examples#smart-contracts',
    content: 'Deploy and interact with smart contracts using Apex SDK.',
    section: 'Examples',
    type: 'example'
  },
  {
    title: 'Error Handling',
    href: '/docs/api#error-handling',
    content: 'Comprehensive error handling patterns for robust applications.',
    section: 'API Reference',
    type: 'section'
  },
  {
    title: 'Testing Guide',
    href: '/docs/testing',
    content: 'Unit testing and integration testing strategies for Apex applications.',
    section: 'Guides',
    type: 'page'
  },
  {
    title: 'Security Best Practices',
    href: '/docs/security',
    content: 'Security guidelines and best practices for production applications.',
    section: 'Guides',
    type: 'page'
  },
  {
    title: 'Event Monitoring',
    href: '/docs/api#events',
    content: 'Monitor blockchain events in real-time with filtering and processing.',
    section: 'API Reference',
    type: 'section'
  }
];

export default function EnhancedSearch({ 
  className = '', 
  placeholder = 'Search documentation...', 
  compact = false 
}: EnhancedSearchProps) {
  const [query, setQuery] = useState('');
  const [isOpen, setIsOpen] = useState(false);
  const [selectedIndex, setSelectedIndex] = useState(-1);

  // Filter and rank search results
  const searchResults = useMemo(() => {
    if (!query.trim()) return [];

    const normalizedQuery = query.toLowerCase().trim();
    
    return searchData
      .map(item => {
        let score = 0;
        const titleMatch = item.title.toLowerCase().includes(normalizedQuery);
        const contentMatch = item.content.toLowerCase().includes(normalizedQuery);
        const sectionMatch = item.section.toLowerCase().includes(normalizedQuery);
        
        // Calculate relevance score
        if (item.title.toLowerCase().startsWith(normalizedQuery)) score += 100;
        else if (titleMatch) score += 50;
        if (contentMatch) score += 20;
        if (sectionMatch) score += 10;
        
        // Boost certain types
        if (item.type === 'page') score += 5;
        if (item.type === 'method') score += 3;
        
        return { ...item, score };
      })
      .filter(item => item.score > 0)
      .sort((a, b) => b.score - a.score)
      .slice(0, 8);
  }, [query]);

  // Handle keyboard navigation
  useEffect(() => {
    const handleKeyDown = (e: KeyboardEvent) => {
      if (!isOpen) return;

      switch (e.key) {
        case 'ArrowDown':
          e.preventDefault();
          setSelectedIndex(prev => 
            prev < searchResults.length - 1 ? prev + 1 : 0
          );
          break;
        case 'ArrowUp':
          e.preventDefault();
          setSelectedIndex(prev => 
            prev > 0 ? prev - 1 : searchResults.length - 1
          );
          break;
        case 'Enter':
          e.preventDefault();
          if (selectedIndex >= 0 && selectedIndex < searchResults.length) {
            window.location.href = searchResults[selectedIndex].href;
          }
          break;
        case 'Escape':
          setIsOpen(false);
          setSelectedIndex(-1);
          break;
      }
    };

    document.addEventListener('keydown', handleKeyDown);
    return () => document.removeEventListener('keydown', handleKeyDown);
  }, [isOpen, selectedIndex, searchResults]);

  // Reset selection when query changes
  useEffect(() => {
    setSelectedIndex(-1);
  }, [query]);

  const getTypeIcon = (type: SearchResult['type']) => {
    switch (type) {
      case 'page':
        return (
          <svg className="w-4 h-4" fill="none" stroke="currentColor" viewBox="0 0 24 24">
            <path strokeLinecap="round" strokeLinejoin="round" strokeWidth={2} d="M9 12h6m-6 4h6m2 5H7a2 2 0 01-2-2V5a2 2 0 012-2h5.586a1 1 0 01.707.293l5.414 5.414a1 1 0 01.293.707V19a2 2 0 01-2 2z" />
          </svg>
        );
      case 'method':
        return (
          <svg className="w-4 h-4" fill="none" stroke="currentColor" viewBox="0 0 24 24">
            <path strokeLinecap="round" strokeLinejoin="round" strokeWidth={2} d="M10 20l4-16m4 4l4 4-4 4M6 16l-4-4 4-4" />
          </svg>
        );
      case 'example':
        return (
          <svg className="w-4 h-4" fill="none" stroke="currentColor" viewBox="0 0 24 24">
            <path strokeLinecap="round" strokeLinejoin="round" strokeWidth={2} d="M13 10V3L4 14h7v7l9-11h-7z" />
          </svg>
        );
      case 'section':
        return (
          <svg className="w-4 h-4" fill="none" stroke="currentColor" viewBox="0 0 24 24">
            <path strokeLinecap="round" strokeLinejoin="round" strokeWidth={2} d="M7 7h.01M7 3h5c.512 0 1.024.195 1.414.586l7 7a2 2 0 010 2.828l-7 7a2 2 0 01-2.828 0l-7-7A1.994 1.994 0 013 12V7a4 4 0 014-4z" />
          </svg>
        );
    }
  };

  const highlightMatch = (text: string, query: string) => {
    if (!query.trim()) return text;
    
    const regex = new RegExp(`(${query.replace(/[.*+?^${}()|[\\]\\\\]/g, '\\$&')})`, 'gi');
    const parts = text.split(regex);
    
    return parts.map((part, index) => 
      regex.test(part) ? (
        <mark key={index} className="bg-hyperBlue/20 text-hyperBlue rounded px-0.5">
          {part}
        </mark>
      ) : (
        part
      )
    );
  };

  return (
    <div className={`relative ${className}`}>
      <div className="relative">
        <input
          type="text"
          placeholder={placeholder}
          value={query}
          onChange={(e) => {
            setQuery(e.target.value);
            setIsOpen(true);
          }}
          onFocus={() => setIsOpen(true)}
          className={`w-full border rounded-lg pl-10 pr-4 text-white placeholder-slate-gray focus:outline-none focus:ring-2 focus:ring-hyperBlue/50 focus:border-hyperBlue transition-colors ${
            compact 
              ? 'bg-obsidian-light/50 border-obsidian-lighter py-1.5 text-sm' 
              : 'bg-obsidian-darker border-obsidian-lighter py-2'
          }`}
        />
        <svg
          className="absolute left-3 top-1/2 transform -translate-y-1/2 w-4 h-4 text-slate-gray"
          fill="none"
          stroke="currentColor"
          viewBox="0 0 24 24"
        >
          <path strokeLinecap="round" strokeLinejoin="round" strokeWidth={2} d="M21 21l-6-6m2-5a7 7 0 11-14 0 7 7 0 0114 0z" />
        </svg>
        
        {query && (
          <button
            onClick={() => {
              setQuery('');
              setIsOpen(false);
            }}
            className="absolute right-3 top-1/2 transform -translate-y-1/2 text-slate-gray hover:text-white transition-colors"
          >
            <svg className="w-4 h-4" fill="none" stroke="currentColor" viewBox="0 0 24 24">
              <path strokeLinecap="round" strokeLinejoin="round" strokeWidth={2} d="M6 18L18 6M6 6l12 12" />
            </svg>
          </button>
        )}
      </div>

      {/* Search Results */}
      {isOpen && searchResults.length > 0 && (
        <div className="absolute top-full left-0 right-0 mt-2 bg-obsidian-light border border-obsidian-lighter rounded-lg shadow-lg z-50 max-h-96 overflow-y-auto">
          <div className="py-2">
            {searchResults.map((result, index) => (
              <Link
                key={`${result.href}-${index}`}
                href={result.href}
                className={`block px-4 py-3 hover:bg-obsidian-lighter transition-colors ${
                  selectedIndex === index ? 'bg-obsidian-lighter' : ''
                }`}
                onClick={() => {
                  setIsOpen(false);
                  setQuery('');
                }}
              >
                <div className="flex items-start space-x-3">
                  <div className="text-slate-gray mt-0.5">
                    {getTypeIcon(result.type)}
                  </div>
                  <div className="flex-1 min-w-0">
                    <div className="flex items-center justify-between">
                      <h4 className="text-sm font-medium text-white truncate">
                        {highlightMatch(result.title, query)}
                      </h4>
                      <span className="text-xs text-slate-gray ml-2 capitalize">
                        {result.type}
                      </span>
                    </div>
                    <p className="text-xs text-white/70 mt-1 line-clamp-2">
                      {highlightMatch(result.content, query)}
                    </p>
                    <p className="text-xs text-slate-gray mt-1">
                      {result.section}
                    </p>
                  </div>
                </div>
              </Link>
            ))}
          </div>
          
          {/* Search tips */}
          <div className="border-t border-obsidian-lighter px-4 py-3">
            <div className="text-xs text-slate-gray">
              <div className="flex items-center justify-between">
                <span>Use ↑↓ to navigate, ↵ to select, ESC to close</span>
                <span>{searchResults.length} result{searchResults.length !== 1 ? 's' : ''}</span>
              </div>
            </div>
          </div>
        </div>
      )}

      {/* No results */}
      {isOpen && query && searchResults.length === 0 && (
        <div className="absolute top-full left-0 right-0 mt-2 bg-obsidian-light border border-obsidian-lighter rounded-lg shadow-lg z-50">
          <div className="px-4 py-6 text-center">
            <svg className="w-8 h-8 text-slate-gray mx-auto mb-2" fill="none" stroke="currentColor" viewBox="0 0 24 24">
              <path strokeLinecap="round" strokeLinejoin="round" strokeWidth={2} d="M9.172 16.172a4 4 0 015.656 0M9 12h6m-6 4h6m2 5H7a2 2 0 01-2-2V5a2 2 0 012-2h5.586a1 1 0 01.707.293l5.414 5.414a1 1 0 01.293.707V19a2 2 0 01-2 2z" />
            </svg>
            <p className="text-sm text-white/70 mb-1">No results found</p>
            <p className="text-xs text-slate-gray">
              Try searching for "quickstart", "API", or "examples"
            </p>
          </div>
        </div>
      )}

      {/* Click outside to close */}
      {isOpen && (
        <div
          className="fixed inset-0 z-40"
          onClick={() => setIsOpen(false)}
        />
      )}
    </div>
  );
}