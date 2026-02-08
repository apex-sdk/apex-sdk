'use client'

import { useState, useCallback } from 'react'

// Mock data for client-side search (in real app, this would come from an API)
const mockDocs = [
  {
    slug: 'quickstart',
    metadata: {
      title: 'Quick Start Guide',
      description: 'Get started with Apex SDK in less than 5 minutes',
      section: 'getting-started',
      tags: ['tutorial', 'beginner']
    },
    readingTime: '5'
  },
  {
    slug: 'installation',
    metadata: {
      title: 'Installation',
      description: 'Install and configure Apex SDK in your environment', 
      section: 'installation',
      tags: ['setup', 'install']
    },
    readingTime: '3'
  },
  {
    slug: 'api',
    metadata: {
      title: 'API Reference',
      description: 'Complete API documentation for Apex SDK',
      section: 'api', 
      tags: ['api', 'reference']
    },
    readingTime: '10'
  },
  {
    slug: 'cli-guide',
    metadata: {
      title: 'CLI Guide - Complete Command Reference',
      description: 'Complete guide to using the Apex CLI tool',
      section: 'cli-guide',
      tags: ['cli', 'commands']
    },
    readingTime: '8'
  },
  {
    slug: 'architecture',
    metadata: {
      title: 'Architecture Overview',
      description: 'Understand the core architecture and design principles',
      section: 'architecture',
      tags: ['architecture', 'design']
    },
    readingTime: '12'
  },
  {
    slug: 'contributing',
    metadata: {
      title: 'Contributing',
      description: 'Learn how to contribute to Apex SDK through bug reports, feature requests, and code contributions',
      section: 'contributing',
      tags: ['contributing', 'development', 'community']
    },
    readingTime: '7'
  },
  {
    slug: 'examples',
    metadata: {
      title: 'Examples & Tutorials',
      description: 'Working code samples and step-by-step tutorials demonstrating common patterns and use cases',
      section: 'examples',
      tags: ['examples', 'tutorials', 'code-samples']
    },
    readingTime: '15'
  },
  {
    slug: 'security',
    metadata: {
      title: 'Security Best Practices',
      description: 'Essential security guidelines and best practices for building secure cross-chain applications',
      section: 'security',
      tags: ['security', 'best-practices', 'safety']
    },
    readingTime: '20'
  },
  {
    slug: 'substrate',
    metadata: {
      title: 'Substrate Integration',
      description: 'Comprehensive guide to building applications on Substrate-based networks',
      section: 'substrate',
      tags: ['substrate', 'polkadot', 'kusama']
    },
    readingTime: '18'
  },
  {
    slug: 'testing',
    metadata: {
      title: 'Testing Guide',
      description: 'Comprehensive testing strategies for Apex SDK applications including unit tests and integration tests',
      section: 'testing',
      tags: ['testing', 'quality', 'unit-tests']
    },
    readingTime: '12'
  }
]

export interface SearchResult {
  slug: string
  metadata: {
    title: string
    description: string
    section: string
    tags: string[]
  }
  readingTime?: string
}

export const useSearch = () => {
  const [results, setResults] = useState<SearchResult[]>([])
  const [isLoading, setIsLoading] = useState(false)

  const search = useCallback((query: string) => {
    if (!query.trim()) {
      setResults([])
      return
    }

    setIsLoading(true)
    
    // Simulate async search
    setTimeout(() => {
      const filteredResults = mockDocs.filter(doc => 
        doc.metadata.title.toLowerCase().includes(query.toLowerCase()) ||
        doc.metadata.description.toLowerCase().includes(query.toLowerCase()) ||
        doc.metadata.tags.some(tag => 
          tag.toLowerCase().includes(query.toLowerCase())
        )
      ).slice(0, 8) // Limit results
      
      setResults(filteredResults)
      setIsLoading(false)
    }, 200)
  }, [])

  const clearSearch = useCallback(() => {
    setResults([])
  }, [])

  return {
    results,
    isLoading,
    search,
    clearSearch
  }
}