'use client';

import React, { useState } from 'react';

interface EnhancedCodeExampleProps {
  title?: string;
  language: string;
  code: string;
  description?: string;
  highlightLines?: number[];
  showLineNumbers?: boolean;
  maxHeight?: string;
  runnable?: boolean;
  onRun?: () => void;
}

export default function EnhancedCodeExample({
  title,
  language,
  code,
  description,
  highlightLines = [],
  showLineNumbers = true,
  maxHeight = 'none',
  runnable = false,
  onRun
}: EnhancedCodeExampleProps) {
  const [copied, setCopied] = useState(false);
  const [isExpanded, setIsExpanded] = useState(false);

  const copyToClipboard = async () => {
    try {
      if (navigator.clipboard && navigator.clipboard.writeText) {
        await navigator.clipboard.writeText(code);
      } else {
        // Fallback for browsers without clipboard API
        const textArea = document.createElement('textarea');
        textArea.value = code;
        textArea.style.position = 'fixed';
        textArea.style.left = '-999999px';
        textArea.style.top = '-999999px';
        document.body.appendChild(textArea);
        textArea.focus();
        textArea.select();
        document.execCommand('copy');
        document.body.removeChild(textArea);
      }
      setCopied(true);
      setTimeout(() => setCopied(false), 2000);
    } catch (err) {
      console.error('Failed to copy code:', err);
      // Still show copied state to provide user feedback
      setCopied(true);
      setTimeout(() => setCopied(false), 2000);
    }
  };

  const lines = code.split('\n');
  const shouldTruncate = lines.length > 20 && maxHeight !== 'none';
  const displayLines = shouldTruncate && !isExpanded ? lines.slice(0, 20) : lines;

  return (
    <div className="space-y-2 my-6">
      {/* Header */}
      {(title || description) && (
        <div className="space-y-1">
          {title && (
            <h4 className="font-medium text-white text-lg">{title}</h4>
          )}
          {description && (
            <p className="text-sm text-white/70">{description}</p>
          )}
        </div>
      )}

      {/* Code block */}
      <div className="relative group">
        <div className="absolute top-3 right-3 z-10 flex items-center space-x-2">
          {/* Language badge */}
          <span className="px-2 py-1 text-xs font-medium bg-obsidian-darker text-white/60 rounded border border-obsidian-lighter">
            {language}
          </span>
          
          {/* Run button */}
          {runnable && (
            <button
              onClick={onRun}
              className="px-3 py-1 text-xs font-medium bg-hyperBlue hover:bg-hyperBlue/80 text-white rounded transition-colors"
            >
              Run
            </button>
          )}
          
          {/* Copy button */}
          <button
            onClick={copyToClipboard}
            className="px-3 py-1 text-xs font-medium bg-obsidian-darker hover:bg-obsidian-lighter text-white rounded transition-colors border border-obsidian-lighter"
          >
            {copied ? (
              <span className="flex items-center space-x-1">
                <svg className="w-3 h-3" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                  <path strokeLinecap="round" strokeLinejoin="round" strokeWidth={2} d="M5 13l4 4L19 7" />
                </svg>
                <span>Copied</span>
              </span>
            ) : (
              <span className="flex items-center space-x-1">
                <svg className="w-3 h-3" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                  <path strokeLinecap="round" strokeLinejoin="round" strokeWidth={2} d="M8 16H6a2 2 0 01-2-2V6a2 2 0 012-2h8a2 2 0 012 2v2m-6 12h8a2 2 0 002-2v-8a2 2 0 00-2-2h-8a2 2 0 00-2 2v8a2 2 0 002 2z" />
                </svg>
                <span>Copy</span>
              </span>
            )}
          </button>
        </div>

        <pre 
          className="bg-obsidian-darker border border-obsidian-lighter rounded-lg p-4 pt-12 overflow-x-auto text-sm leading-relaxed"
          style={{ 
            maxHeight: isExpanded ? 'none' : maxHeight === 'none' ? 'none' : maxHeight 
          }}
        >
          <code className={`language-${language}`}>
            {showLineNumbers ? (
              <div className="table w-full">
                {displayLines.map((line, index) => (
                  <div 
                    key={index} 
                    className={`table-row ${
                      highlightLines.includes(index + 1) 
                        ? 'bg-hyperBlue/10 border-l-2 border-hyperBlue' 
                        : ''
                    }`}
                  >
                    <div className="table-cell w-8 text-right pr-4 text-white/40 select-none">
                      {index + 1}
                    </div>
                    <div className="table-cell text-white">
                      {line || '\n'}
                    </div>
                  </div>
                ))}
              </div>
            ) : (
              <div className="text-white">
                {displayLines.join('\n')}
              </div>
            )}
          </code>
        </pre>

        {/* Expand/collapse button */}
        {shouldTruncate && (
          <div className="text-center mt-2">
            <button
              onClick={() => setIsExpanded(!isExpanded)}
              className="text-sm text-hyperBlue hover:text-hyperBlue/80 transition-colors"
            >
              {isExpanded ? 'Show less' : `Show all ${lines.length} lines`}
            </button>
          </div>
        )}
      </div>
    </div>
  );
}

// Specialized components for different code types
export function RustExample(props: Omit<EnhancedCodeExampleProps, 'language'>) {
  return <EnhancedCodeExample {...props} language="rust" />;
}

export function BashExample(props: Omit<EnhancedCodeExampleProps, 'language'>) {
  return <EnhancedCodeExample {...props} language="bash" showLineNumbers={false} />;
}

export function JSONExample(props: Omit<EnhancedCodeExampleProps, 'language'>) {
  return <EnhancedCodeExample {...props} language="json" />;
}

export function TOMLExample(props: Omit<EnhancedCodeExampleProps, 'language'>) {
  return <EnhancedCodeExample {...props} language="toml" />;
}