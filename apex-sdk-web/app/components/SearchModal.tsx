'use client'

import { useState, useRef, useEffect } from 'react'
import { useSearch } from '@/lib/hooks/useSearch'
import Link from 'next/link'

interface SearchModalProps {
  isOpen: boolean
  onClose: () => void
}

export function SearchModal({ isOpen, onClose }: SearchModalProps) {
  const [query, setQuery] = useState('')
  const { results, isLoading, search, clearSearch } = useSearch()
  const inputRef = useRef<HTMLInputElement>(null)

  useEffect(() => {
    if (isOpen && inputRef.current) {
      inputRef.current.focus()
    }
  }, [isOpen])

  useEffect(() => {
    const debounceTimer = setTimeout(() => {
      if (query.length > 2) {
        search(query)
      } else {
        clearSearch()
      }
    }, 300)

    return () => clearTimeout(debounceTimer)
  }, [query, search, clearSearch])

  useEffect(() => {
    const handleEscape = (e: KeyboardEvent) => {
      if (e.key === 'Escape') {
        onClose()
      }
    }

    if (isOpen) {
      document.addEventListener('keydown', handleEscape)
      return () => document.removeEventListener('keydown', handleEscape)
    }
  }, [isOpen, onClose])

  if (!isOpen) return null

  return (
    <div className="fixed inset-0 z-50 flex items-start justify-center pt-20">
      {/* Backdrop */}
      <div 
        className="fixed inset-0 bg-black/50 backdrop-blur-sm"
        onClick={onClose}
      />
      
      {/* Modal */}
      <div className="relative w-full max-w-2xl mx-4 bg-obsidian-light border border-obsidian-lighter rounded-lg shadow-xl">
        {/* Search Input */}
        <div className="p-4 border-b border-obsidian-lighter">
          <input
            ref={inputRef}
            type="text"
            placeholder="Search documentation..."
            value={query}
            onChange={(e) => setQuery(e.target.value)}
            className="w-full px-4 py-3 bg-obsidian border border-obsidian-lighter rounded-lg text-white placeholder-slate-gray focus:border-hyperBlue focus:outline-none"
          />
        </div>

        {/* Results */}
        <div className="max-h-96 overflow-y-auto">
          {isLoading ? (
            <div className="p-8 text-center text-slate-gray">
              Searching...
            </div>
          ) : query.length > 2 && results.length === 0 ? (
            <div className="p-8 text-center text-slate-gray">
              No results found for "{query}"
            </div>
          ) : query.length > 2 ? (
            <div className="p-2">
              {results.map((doc, index) => (
                <Link
                  key={doc.slug}
                  href={`/docs/${doc.slug}`}
                  onClick={onClose}
                  className="block p-4 rounded-lg hover:bg-obsidian transition-colors"
                >
                  <div className="flex items-start gap-3">
                    <div className="flex-shrink-0 mt-1 w-2 h-2 bg-hyperBlue rounded-full" />
                    <div className="flex-1 min-w-0">
                      <h3 className="font-medium text-white mb-1 truncate">
                        {doc.metadata.title}
                      </h3>
                      <p className="text-sm text-slate-gray line-clamp-2">
                        {doc.metadata.description}
                      </p>
                      <div className="flex items-center gap-2 mt-2">
                        <span className="text-xs bg-obsidian px-2 py-1 rounded text-slate-gray">
                          {doc.metadata.section}
                        </span>
                        {doc.readingTime && (
                          <span className="text-xs text-slate-gray">
                            {doc.readingTime} min read
                          </span>
                        )}
                      </div>
                    </div>
                  </div>
                </Link>
              ))}
            </div>
          ) : (
            <div className="p-8 text-center text-slate-gray">
              Type to search documentation...
            </div>
          )}
        </div>

        {/* Footer */}
        <div className="p-4 border-t border-obsidian-lighter text-center">
          <p className="text-xs text-slate-gray">
            Press <kbd className="px-2 py-1 bg-obsidian rounded text-xs">Esc</kbd> to close
          </p>
        </div>
      </div>
    </div>
  )
}