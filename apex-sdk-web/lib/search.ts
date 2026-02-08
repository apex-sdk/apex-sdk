import { DocContent, getAllDocs } from './content'

export interface SearchResult {
  title: string
  description: string
  href: string
  content: string
  score: number
}

export async function searchContent(query: string): Promise<SearchResult[]> {
  if (!query.trim()) return []
  
  const docs = await getAllDocs()
  const results: SearchResult[] = []
  
  const normalizedQuery = query.toLowerCase()
  
  for (const doc of docs) {
    const titleMatch = doc.metadata.title.toLowerCase().includes(normalizedQuery)
    const descriptionMatch = doc.metadata.description.toLowerCase().includes(normalizedQuery)
    const contentMatch = doc.content.toLowerCase().includes(normalizedQuery)
    const tagMatch = doc.metadata.tags?.some(tag => 
      tag.toLowerCase().includes(normalizedQuery)
    )
    
    if (titleMatch || descriptionMatch || contentMatch || tagMatch) {
      let score = 0
      
      // Weight different match types
      if (titleMatch) score += 100
      if (descriptionMatch) score += 50
      if (tagMatch) score += 30
      if (contentMatch) score += 10
      
      // Extract content snippet around the match
      const contentSnippet = extractSnippet(doc.content, normalizedQuery)
      
      results.push({
        title: doc.metadata.title,
        description: doc.metadata.description,
        href: `/docs/${doc.slug}`,
        content: contentSnippet,
        score
      })
    }
  }
  
  // Sort by relevance score
  return results.sort((a, b) => b.score - a.score).slice(0, 10)
}

function extractSnippet(content: string, query: string, maxLength: number = 200): string {
  const index = content.toLowerCase().indexOf(query.toLowerCase())
  
  if (index === -1) {
    // Return beginning of content if no match found
    return content.substring(0, maxLength) + (content.length > maxLength ? '...' : '')
  }
  
  const start = Math.max(0, index - 50)
  const end = Math.min(content.length, index + query.length + 150)
  
  let snippet = content.substring(start, end)
  
  // Clean up the snippet - remove markdown formatting
  snippet = snippet
    .replace(/#{1,6}\s+/g, '') // Remove markdown headers
    .replace(/\*\*(.*?)\*\*/g, '$1') // Remove bold formatting
    .replace(/\*(.*?)\*/g, '$1') // Remove italic formatting
    .replace(/`(.*?)`/g, '$1') // Remove inline code formatting
    .replace(/\[(.*?)\]\(.*?\)/g, '$1') // Replace links with just text
    .trim()
  
  if (start > 0) snippet = '...' + snippet
  if (end < content.length) snippet = snippet + '...'
  
  return snippet
}

export function highlightMatches(text: string, query: string): string {
  if (!query.trim()) return text
  
  const regex = new RegExp(`(${query.replace(/[.*+?^${}()|[\]\\]/g, '\\$&')})`, 'gi')
  return text.replace(regex, '<mark>$1</mark>')
}