'use client'

import { useState, useEffect } from 'react'
import { SearchResult, searchContent } from '@/lib/search'

interface SearchBoxProps {
  onResults?: (results: SearchResult[]) => void
}

export function SearchBox({ onResults }: SearchBoxProps) {
  const [query, setQuery] = useState('')
  const [results, setResults] = useState<SearchResult[]>([])
  const [isLoading, setIsLoading] = useState(false)
  const [isOpen, setIsOpen] = useState(false)

  useEffect(() => {
    const searchContent = async () => {
      if (!query.trim()) {
        setResults([])
        onResults?.([])
        return
      }

      setIsLoading(true)
      try {
        // For now, we'll implement a simple client-side search
        // In production, this would call the API
        const mockResults: SearchResult[] = [
          {
            title: 'Installation Guide',
            description: 'Install and configure Apex SDK for cross-chain development',
            href: '/docs/installation',
            content: 'Install and configure Apex SDK for cross-chain development across EVM and Substrate networks...',
            score: 100
          },
          {
            title: 'Quick Start',
            description: 'Build your first cross-chain application in minutes with Apex SDK',
            href: '/docs/quickstart',
            content: 'Build your first cross-chain application in minutes with Apex SDK. This guide will walk you through...',
            score: 80
          }
        ].filter(result => 
          result.title.toLowerCase().includes(query.toLowerCase()) ||
          result.description.toLowerCase().includes(query.toLowerCase())
        )

        setResults(mockResults)
        onResults?.(mockResults)
      } catch (error) {
        console.error('Search error:', error)
        setResults([])
        onResults?.([])
      } finally {
        setIsLoading(false)
      }
    }

    const debounceTimer = setTimeout(searchContent, 300)
    return () => clearTimeout(debounceTimer)
  }, [query, onResults])

  return (
    <div className="relative">
      <div className="relative">
        <input
          type="text"
          placeholder="Search documentation..."
          value={query}
          onChange={(e) => setQuery(e.target.value)}
          onFocus={() => setIsOpen(true)}
          className="w-full px-4 py-2 pl-10 pr-4 text-sm text-gray-900 bg-white border border-gray-300 rounded-lg focus:ring-2 focus:ring-blue-500 focus:border-blue-500"
        />
        <div className="absolute inset-y-0 left-0 flex items-center pl-3 pointer-events-none">
          <svg
            className="w-4 h-4 text-gray-400"
            fill="none"
            stroke="currentColor"
            viewBox="0 0 24 24"
          >
            <path
              strokeLinecap="round"
              strokeLinejoin="round"
              strokeWidth={2}
              d="M21 21l-6-6m2-5a7 7 0 11-14 0 7 7 0 0114 0z"
            />
          </svg>
        </div>
        {isLoading && (
          <div className="absolute inset-y-0 right-0 flex items-center pr-3">
            <div className="w-4 h-4 border-2 border-gray-300 border-t-blue-500 rounded-full animate-spin"></div>
          </div>
        )}
      </div>

      {isOpen && query && (
        <div className="absolute z-50 w-full mt-1 bg-white border border-gray-200 rounded-lg shadow-lg max-h-96 overflow-y-auto">
          {results.length > 0 ? (
            <div className="py-2">
              {results.map((result, index) => (
                <a
                  key={index}
                  href={result.href}
                  className="block px-4 py-3 hover:bg-gray-50 transition-colors"
                  onClick={() => {
                    setIsOpen(false)
                    setQuery('')
                  }}
                >
                  <h3 className="text-sm font-medium text-gray-900 mb-1">
                    {result.title}
                  </h3>
                  <p className="text-xs text-gray-600 mb-1">
                    {result.description}
                  </p>
                  <p className="text-xs text-gray-500 truncate">
                    {result.content}
                  </p>
                </a>
              ))}
            </div>
          ) : (
            <div className="px-4 py-3 text-sm text-gray-500">
              {isLoading ? 'Searching...' : 'No results found'}
            </div>
          )}
        </div>
      )}

      {isOpen && (
        <div
          className="fixed inset-0 z-40"
          onClick={() => setIsOpen(false)}
        />
      )}
    </div>
  )
}