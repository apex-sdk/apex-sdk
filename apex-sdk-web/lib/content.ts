import fs from 'fs'
import path from 'path'
import matter from 'gray-matter'

export interface DocMetadata {
  title: string
  description: string
  section: string
  order?: number
  lastUpdated: string
  tags?: string[]
  author?: string
  draft?: boolean
}

export interface DocContent {
  metadata: DocMetadata
  content: string
  slug: string
  readingTime?: number
}

const contentDirectory = path.join(process.cwd(), 'content')

export function getDocSlugs(): string[] {
  const docsPath = path.join(contentDirectory, 'docs')
  
  if (!fs.existsSync(docsPath)) {
    return []
  }

  const getAllSlugs = (dir: string, basePath = ''): string[] => {
    const files = fs.readdirSync(dir)
    let slugs: string[] = []

    for (const file of files) {
      const filePath = path.join(dir, file)
      const stat = fs.statSync(filePath)

      if (stat.isDirectory()) {
        slugs = [...slugs, ...getAllSlugs(filePath, path.join(basePath, file))]
      } else if (file.endsWith('.mdx') || file.endsWith('.md')) {
        const slug = path.join(basePath, file.replace(/\.(mdx?|md)$/, ''))
        slugs.push(slug.replace(/\\/g, '/')) // Normalize path separators
      }
    }

    return slugs
  }

  return getAllSlugs(docsPath)
}

export function getDocBySlug(slug: string): DocContent | null {
  try {
    const realSlug = slug.replace(/\/$/, '') // Remove trailing slash
    const fullPath = path.join(contentDirectory, 'docs', `${realSlug}.mdx`)
    
    // Try .mdx first, then .md
    let fileContents: string
    let actualPath: string
    
    if (fs.existsSync(fullPath)) {
      actualPath = fullPath
      fileContents = fs.readFileSync(fullPath, 'utf8')
    } else {
      const mdPath = path.join(contentDirectory, 'docs', `${realSlug}.md`)
      if (fs.existsSync(mdPath)) {
        actualPath = mdPath
        fileContents = fs.readFileSync(mdPath, 'utf8')
      } else {
        return null
      }
    }

    const { data, content } = matter(fileContents)
    
    // Calculate reading time (rough estimate: 200 words per minute)
    const wordCount = content.split(/\s+/).length
    const readingTime = Math.ceil(wordCount / 200)

    return {
      metadata: data as DocMetadata,
      content,
      slug: realSlug,
      readingTime
    }
  } catch (error) {
    console.error(`Error reading doc ${slug}:`, error)
    return null
  }
}

export function getAllDocs(): DocContent[] {
  const slugs = getDocSlugs()
  return slugs
    .map((slug) => getDocBySlug(slug))
    .filter((doc): doc is DocContent => doc !== null)
    .filter((doc) => !doc.metadata.draft) // Filter out draft content
    .sort((a, b) => {
      // Sort by section first, then by order, then by title
      if (a.metadata.section !== b.metadata.section) {
        return a.metadata.section.localeCompare(b.metadata.section)
      }
      if (a.metadata.order !== undefined && b.metadata.order !== undefined) {
        return a.metadata.order - b.metadata.order
      }
      return a.metadata.title.localeCompare(b.metadata.title)
    })
}

export function getDocsBySection(section: string): DocContent[] {
  return getAllDocs().filter((doc) => doc.metadata.section === section)
}

export function searchDocs(query: string): DocContent[] {
  const allDocs = getAllDocs()
  const lowercaseQuery = query.toLowerCase()

  return allDocs.filter((doc) => {
    const searchableText = `
      ${doc.metadata.title}
      ${doc.metadata.description}
      ${doc.content}
      ${(doc.metadata.tags || []).join(' ')}
    `.toLowerCase()

    return searchableText.includes(lowercaseQuery)
  })
}

// Navigation utilities
export function generateBreadcrumbs(slug: string): Array<{ title: string; href: string }> {
  const parts = slug.split('/').filter(Boolean)
  const breadcrumbs = [{ title: 'Documentation', href: '/docs' }]

  let currentPath = '/docs'
  for (const part of parts) {
    currentPath += `/${part}`
    const doc = getDocBySlug(part)
    breadcrumbs.push({
      title: doc?.metadata.title || part.charAt(0).toUpperCase() + part.slice(1),
      href: currentPath
    })
  }

  return breadcrumbs
}

export function getRelatedDocs(currentSlug: string, limit = 3): DocContent[] {
  const currentDoc = getDocBySlug(currentSlug)
  if (!currentDoc) return []

  const allDocs = getAllDocs().filter(doc => doc.slug !== currentSlug)
  
  // Find related docs based on section and tags
  const sectionMatches = allDocs.filter(doc => 
    doc.metadata.section === currentDoc.metadata.section
  )

  const tagMatches = allDocs.filter(doc => {
    const currentTags = currentDoc.metadata.tags || []
    const docTags = doc.metadata.tags || []
    return currentTags.some(tag => docTags.includes(tag))
  })

  // Combine and deduplicate, prioritizing section matches
  const related = [...new Set([...sectionMatches, ...tagMatches])]
  
  return related.slice(0, limit)
}