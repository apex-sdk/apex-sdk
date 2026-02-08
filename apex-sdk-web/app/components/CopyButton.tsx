'use client';

import React, { useState } from 'react';

interface CopyButtonProps {
  text: string;
  className?: string;
}

export default function CopyButton({ text, className = '' }: CopyButtonProps) {
  const [copied, setCopied] = useState(false);

  const handleCopy = async () => {
    try {
      if (navigator.clipboard && navigator.clipboard.writeText) {
        await navigator.clipboard.writeText(text);
      } else {
        // Fallback for browsers without clipboard API
        const textArea = document.createElement('textarea');
        textArea.value = text;
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
      console.error('Failed to copy text: ', err);
      // Still show copied state even if there's an error
      setCopied(true);
      setTimeout(() => setCopied(false), 2000);
    }
  };

  return (
    <button
      onClick={handleCopy}
      className={`relative p-2 rounded-md bg-surface hover:bg-surface-light transition-colors duration-200 text-muted hover:text-foreground ${className}`}
      title="Copy to clipboard"
      aria-label="Copy to clipboard"
    >
      <svg className="w-4 h-4" fill="currentColor" viewBox="0 0 24 24">
        {copied ? (
          <path fillRule="evenodd" d="M19.916 4.626a.75.75 0 01.208 1.04l-9 13.5a.75.75 0 01-1.154.114l-6-6a.75.75 0 011.06-1.06l5.353 5.353 8.493-12.739a.75.75 0 011.04-.208z" clipRule="evenodd"/>
        ) : (
          <path d="M8 5a3 3 0 013-3h2a3 3 0 013 3v.5h.5A1.5 1.5 0 0118 7v11a3 3 0 01-3 3H9a3 3 0 01-3-3V7a1.5 1.5 0 011.5-1.5H8V5zm1 1.5H7.5a.5.5 0 00-.5.5v11a2 2 0 002 2h6a2 2 0 002-2V7a.5.5 0 00-.5-.5H15V8a1 1 0 01-1 1H10a1 1 0 01-1-1V6.5zm2-3.5a2 2 0 00-2 2v.5h4V5a2 2 0 00-2-2h-2z"/>
        )}
      </svg>
      {copied && (
        <span className="absolute -top-8 left-1/2 transform -translate-x-1/2 bg-surface-light text-xs px-2 py-1 rounded whitespace-nowrap">
          Copied!
        </span>
      )}
    </button>
  );
}