'use client'

import { useState, useEffect, forwardRef, useImperativeHandle } from 'react'
import { SearchModal } from './SearchModal'

export interface GlobalSearchRef {
  openSearch: () => void
}

const GlobalSearch = forwardRef<GlobalSearchRef>((props, ref) => {
  const [isOpen, setIsOpen] = useState(false)

  useImperativeHandle(ref, () => ({
    openSearch: () => setIsOpen(true)
  }))

  useEffect(() => {
    const handleKeyDown = (e: KeyboardEvent) => {
      if ((e.metaKey || e.ctrlKey) && e.key === 'k') {
        e.preventDefault()
        setIsOpen(true)
      }
    }

    document.addEventListener('keydown', handleKeyDown)
    return () => document.removeEventListener('keydown', handleKeyDown)
  }, [])

  return (
    <>
      <button 
        onClick={() => setIsOpen(true)}
        className="flex items-center gap-2 px-4 py-2 bg-obsidian border border-obsidian-lighter rounded-lg text-slate-gray hover:border-hyperBlue transition-colors"
      >
        <svg className="w-4 h-4" fill="none" stroke="currentColor" viewBox="0 0 24 24">
          <path strokeLinecap="round" strokeLinejoin="round" strokeWidth={2} d="m21 21-6-6m2-5a7 7 0 11-14 0 7 7 0 0114 0z" />
        </svg>
        <span>Search docs</span>
        <kbd className="ml-auto px-2 py-1 text-xs bg-obsidian-lighter rounded">⌘K</kbd>
      </button>
      
      <SearchModal 
        isOpen={isOpen} 
        onClose={() => setIsOpen(false)} 
      />
    </>
  )
})

GlobalSearch.displayName = 'GlobalSearch'

export default GlobalSearch
