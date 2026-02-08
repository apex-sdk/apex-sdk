import { notFound } from 'next/navigation'
import { getDocBySlug, getDocSlugs } from '@/lib/content'
import { Metadata } from 'next'
import { MDXRemote } from 'next-mdx-remote/rsc'

interface DocPageProps {
  params: Promise<{ slug: string }>
}

// Enhanced MDX components for better typography
const components = {
  h1: (props: any) => (
    <h1 className="text-4xl font-bold text-white mb-6 mt-8 first:mt-0 leading-tight" {...props} />
  ),
  h2: (props: any) => (
    <h2 className="text-3xl font-semibold text-white mb-4 mt-8 leading-tight border-b border-obsidian-lighter pb-2" {...props} />
  ),
  h3: (props: any) => (
    <h3 className="text-2xl font-semibold text-white mb-3 mt-6 leading-tight" {...props} />
  ),
  h4: (props: any) => (
    <h4 className="text-xl font-semibold text-white mb-2 mt-4 leading-tight" {...props} />
  ),
  p: (props: any) => (
    <p className="text-slate-gray mb-4 leading-relaxed" {...props} />
  ),
  strong: (props: any) => (
    <strong className="text-white font-semibold" {...props} />
  ),
  em: (props: any) => (
    <em className="text-white/90 italic" {...props} />
  ),
  a: (props: any) => (
    <a className="text-hyperBlue hover:text-hyperBlue-hover transition-colors font-medium border-b border-transparent hover:border-hyperBlue" {...props} />
  ),
  code: (props: any) => (
    <code className="bg-obsidian-light text-hyperBlue px-2 py-1 rounded text-sm font-mono font-medium" {...props} />
  ),
  pre: (props: any) => (
    <div className="relative">
      <pre className="bg-obsidian border border-obsidian-lighter rounded-lg p-4 overflow-x-auto text-sm leading-relaxed" {...props} />
      <div className="absolute top-0 left-0 right-0 h-px" style={{
        background: 'linear-gradient(to right, transparent, #2176FF, transparent)'
      }}></div>
    </div>
  ),
  blockquote: (props: any) => (
    <blockquote className="border-l-4 border-hyperBlue bg-obsidian-light pl-6 py-4 my-6 rounded-r-lg" {...props} />
  ),
  ul: (props: any) => (
    <ul className="space-y-1 mb-6 ml-0" {...props} />
  ),
  ol: (props: any) => (
    <ol className="space-y-1 mb-6 ml-0 counter-reset-list" {...props} />
  ),
  li: (props: any) => (
    <li className="flex items-start text-slate-gray leading-relaxed mb-1">
      <span className="flex-shrink-0 w-1.5 h-1.5 bg-hyperBlue rounded-full mt-2 mr-3"></span>
      <span {...props} />
    </li>
  ),
  table: (props: any) => (
    <div className="overflow-x-auto my-6">
      <table className="min-w-full bg-obsidian-light border border-obsidian-lighter rounded-lg overflow-hidden" {...props} />
    </div>
  ),
  thead: (props: any) => (
    <thead className="bg-obsidian" {...props} />
  ),
  th: (props: any) => (
    <th className="px-4 py-3 text-left text-white font-semibold text-sm uppercase tracking-wider border-b border-hyperBlue" {...props} />
  ),
  td: (props: any) => (
    <td className="px-4 py-3 text-slate-gray border-b border-obsidian-lighter last:border-b-0" {...props} />
  ),
  hr: (props: any) => (
    <hr className="border-0 h-px my-8" style={{
      background: 'linear-gradient(to right, transparent, #2C3440, transparent)'
    }} {...props} />
  ),
  img: (props: any) => (
    <img className="rounded-lg shadow-lg my-6 max-w-full h-auto" {...props} />
  ),
};

export async function generateStaticParams() {
  const slugs = getDocSlugs()
  return slugs.map((slug) => ({
    slug: slug
  }))
}

export async function generateMetadata({ params }: DocPageProps): Promise<Metadata> {
  const { slug } = await params
  
  const content = getDocBySlug(slug)
  if (!content) {
    return {
      title: 'Page Not Found',
      description: 'The requested page could not be found.',
    }
  }

  return {
    title: content.metadata.title,
    description: content.metadata.description,
    keywords: content.metadata.tags,
  }
}

export default async function DocPage({ params }: DocPageProps) {
  const { slug } = await params
  
  const content = getDocBySlug(slug)
  if (!content) {
    notFound()
  }
  
  return (
    <article className="prose prose-invert max-w-none">
      {/* Document Header */}
      <header className="mb-8 pb-6 border-b border-obsidian-lighter">
        <h1 className="text-4xl font-bold text-white mb-4 leading-tight">
          {content.metadata.title}
        </h1>
        {content.metadata.description && (
          <p className="text-xl text-slate-light leading-relaxed mb-4">
            {content.metadata.description}
          </p>
        )}
        
        {/* Metadata */}
        <div className="flex flex-wrap items-center gap-4 text-sm text-slate-gray">
          {content.metadata.lastUpdated && (
            <div className="flex items-center gap-2">
              <svg className="w-4 h-4" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                <path strokeLinecap="round" strokeLinejoin="round" strokeWidth={2} d="M8 7V3m8 4V3m-9 8h10M5 21h14a2 2 0 002-2V7a2 2 0 00-2-2H5a2 2 0 00-2 2v12a2 2 0 002 2z" />
              </svg>
              <span>Updated {content.metadata.lastUpdated}</span>
            </div>
          )}
          
          {content.readingTime && (
            <div className="flex items-center gap-2">
              <svg className="w-4 h-4" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                <path strokeLinecap="round" strokeLinejoin="round" strokeWidth={2} d="M12 8v4l3 3m6-3a9 9 0 11-18 0 9 9 0 0118 0z" />
              </svg>
              <span>{content.readingTime} min read</span>
            </div>
          )}
          
          {content.metadata.tags && content.metadata.tags.length > 0 && (
            <div className="flex items-center gap-2 flex-wrap">
              <svg className="w-4 h-4" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                <path strokeLinecap="round" strokeLinejoin="round" strokeWidth={2} d="M7 7h.01M7 3h5c.512 0 1.024.195 1.414.586l7 7a2 2 0 010 2.828l-7 7a2 2 0 01-2.828 0l-7-7A1.99 1.99 0 013 12V7a4 4 0 014-4z" />
              </svg>
              {content.metadata.tags.map((tag) => (
                <span
                  key={tag}
                  className="px-2 py-1 bg-obsidian-light border border-obsidian-lighter rounded text-xs text-hyperBlue"
                >
                  {tag}
                </span>
              ))}
            </div>
          )}
        </div>
      </header>
      
      {/* Document Content */}
      <div className="prose-enhanced">
        <MDXRemote source={content.content} components={components} />
      </div>
      
      {/* Document Footer */}
      <footer className="mt-12 pt-8 border-t border-obsidian-lighter">
        <div className="flex flex-col sm:flex-row justify-between items-start sm:items-center gap-4">
          <div className="text-sm text-slate-gray">
            {content.metadata.author && (
              <p>Written by {content.metadata.author}</p>
            )}
          </div>
          
          <div className="flex items-center gap-4 text-sm">
            <a
              href={`https://github.com/apex-sdk/apex-sdk-docs/edit/main/content/docs/${slug}.mdx`}
              className="text-hyperBlue hover:text-hyperBlue-hover transition-colors flex items-center gap-2"
              target="_blank"
              rel="noopener noreferrer"
            >
              <svg className="w-4 h-4" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                <path strokeLinecap="round" strokeLinejoin="round" strokeWidth={2} d="M11 5H6a2 2 0 00-2 2v11a2 2 0 002 2h11a2 2 0 002-2v-5m-1.414-9.414a2 2 0 112.828 2.828L11.828 15H9v-2.828l8.586-8.586z" />
              </svg>
              Edit this page
            </a>
          </div>
        </div>
      </footer>
    </article>
  )
}