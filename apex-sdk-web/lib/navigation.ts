import { getAllDocs, type DocContent } from './content'

export interface NavItem {
  title: string
  href: string
  description: string
  items?: NavItem[]
}

export interface NavSection {
  title: string
  items: NavItem[]
}

export async function generateNavigation(): Promise<NavSection[]> {
  const docs = await getAllDocs()
  const sections = new Map<string, DocContent[]>()

  // Group docs by section
  docs.forEach(doc => {
    const section = doc.metadata.section || 'other'
    if (!sections.has(section)) {
      sections.set(section, [])
    }
    sections.get(section)!.push(doc)
  })

  // Define section order and display names
  const sectionConfig: Record<string, { title: string; order: number }> = {
    'getting-started': { title: 'Getting Started', order: 1 },
    'core-concepts': { title: 'Core Concepts', order: 2 },
    'guides': { title: 'Guides', order: 3 },
    'reference': { title: 'Reference', order: 4 },
    'examples': { title: 'Examples', order: 5 },
    'community': { title: 'Community', order: 6 },
    'other': { title: 'Other', order: 7 }
  }

  // Sort sections by configured order
  const sortedSections = Array.from(sections.entries()).sort((a, b) => {
    const orderA = sectionConfig[a[0]]?.order || 999
    const orderB = sectionConfig[b[0]]?.order || 999
    return orderA - orderB
  })

  const navigation: NavSection[] = []

  for (const [sectionKey, sectionDocs] of sortedSections) {
    const config = sectionConfig[sectionKey]
    const sectionTitle = config?.title || sectionKey.charAt(0).toUpperCase() + sectionKey.slice(1)
    
    // Sort docs within section by order, then by title
    const sortedDocs = sectionDocs.sort((a, b) => {
      const orderA = a.metadata.order || 999
      const orderB = b.metadata.order || 999
      if (orderA !== orderB) return orderA - orderB
      return a.metadata.title.localeCompare(b.metadata.title)
    })

    const items: NavItem[] = sortedDocs.map(doc => ({
      title: doc.metadata.title,
      href: `/docs/${doc.slug}`,
      description: doc.metadata.description
    }))

    navigation.push({
      title: sectionTitle,
      items
    })
  }

  return navigation
}

// Fallback static navigation for build time and client-side
export const staticNavigation: NavSection[] = [
  {
    title: 'Getting Started',
    items: [
      { title: 'Installation Guide', href: '/docs/installation', description: 'Install and configure Apex SDK' },
      { title: 'Quick Start', href: '/docs/quickstart', description: 'Build your first cross-chain application' },
    ]
  },
  {
    title: 'Core Concepts',
    items: [
      { title: 'Architecture Overview', href: '/docs/architecture', description: 'Understand the core architecture' },
    ]
  },
  {
    title: 'Reference',
    items: [
      { title: 'API Reference', href: '/docs/api', description: 'Complete API documentation' },
    ]
  }
]

export function findCurrentNavItem(navigation: NavSection[], currentPath: string): NavItem | null {
  for (const section of navigation) {
    for (const item of section.items) {
      if (item.href === currentPath) {
        return item
      }
      if (item.items) {
        for (const subItem of item.items) {
          if (subItem.href === currentPath) {
            return subItem
          }
        }
      }
    }
  }
  return null
}

export function getAdjacentNavItems(navigation: NavSection[], currentPath: string) {
  const allItems: NavItem[] = []
  
  // Flatten all navigation items
  navigation.forEach(section => {
    section.items.forEach(item => {
      allItems.push(item)
      if (item.items) {
        allItems.push(...item.items)
      }
    })
  })

  const currentIndex = allItems.findIndex(item => item.href === currentPath)
  
  return {
    previous: currentIndex > 0 ? allItems[currentIndex - 1] : null,
    next: currentIndex < allItems.length - 1 ? allItems[currentIndex + 1] : null
  }
}