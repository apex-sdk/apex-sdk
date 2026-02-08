'use client';

import React, { useEffect, useState } from 'react';

interface TocItem {
  id: string;
  title: string;
  level: number;
}

interface TableOfContentsProps {
  className?: string;
}

export default function TableOfContents({ className = '' }: TableOfContentsProps) {
  const [toc, setToc] = useState<TocItem[]>([]);
  const [activeId, setActiveId] = useState('');

  useEffect(() => {
    // Generate table of contents from headings
    const headings = document.querySelectorAll('h1, h2, h3, h4, h5, h6');
    const tocItems: TocItem[] = [];

    headings.forEach((heading) => {
      if (heading.id) {
        tocItems.push({
          id: heading.id,
          title: heading.textContent || '',
          level: parseInt(heading.tagName.charAt(1))
        });
      }
    });

    setToc(tocItems);

    // Set up intersection observer for active heading
    const observer = new IntersectionObserver(
      (entries) => {
        entries.forEach((entry) => {
          if (entry.isIntersecting) {
            setActiveId(entry.target.id);
          }
        });
      },
      {
        rootMargin: '-80px 0px -80% 0px'
      }
    );

    headings.forEach((heading) => {
      if (heading.id) {
        observer.observe(heading);
      }
    });

    return () => observer.disconnect();
  }, []);

  if (toc.length === 0) {
    return null;
  }

  const scrollToHeading = (id: string) => {
    const element = document.getElementById(id);
    if (element) {
      const offset = 80; // Account for fixed header
      const elementPosition = element.offsetTop - offset;
      window.scrollTo({
        top: elementPosition,
        behavior: 'smooth'
      });
    }
  };

  return (
    <div className={`sticky top-24 ${className}`}>
      <div className="bg-obsidian-light border border-obsidian-lighter rounded-lg p-4">
        <h4 className="font-semibold text-white mb-3 text-sm uppercase tracking-wider">
          On this page
        </h4>
        <nav>
          <ul className="space-y-1">
            {toc.map((item) => (
              <li key={item.id}>
                <button
                  onClick={() => scrollToHeading(item.id)}
                  className={`block text-left w-full text-sm py-1 px-2 rounded transition-colors ${
                    item.level === 1 
                      ? 'font-semibold' 
                      : item.level === 2 
                        ? 'pl-4' 
                        : item.level === 3 
                          ? 'pl-6' 
                          : 'pl-8'
                  } ${
                    activeId === item.id
                      ? 'text-hyperBlue bg-hyperBlue/10'
                      : 'text-white/70 hover:text-white hover:bg-obsidian-lighter/50'
                  }`}
                >
                  {item.title}
                </button>
              </li>
            ))}
          </ul>
        </nav>
      </div>
    </div>
  );
}