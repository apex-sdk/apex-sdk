import React from 'react';

interface DocumentationPageProps {
  title: string;
  description: string;
  children: React.ReactNode;
  lastUpdated?: string;
  difficulty?: 'beginner' | 'intermediate' | 'advanced';
  estimatedReadTime?: string;
}

interface SectionProps {
  title: string;
  children: React.ReactNode;
  id?: string;
  level?: 1 | 2 | 3 | 4;
}

interface CodeExampleProps {
  title?: string;
  language: string;
  code: string;
  description?: string;
}

interface ParameterTableProps {
  parameters: {
    name: string;
    type: string;
    required: boolean;
    description: string;
    defaultValue?: string;
  }[];
}

interface NoteProps {
  type: 'info' | 'warning' | 'error' | 'tip';
  title?: string;
  children: React.ReactNode;
}

export function DocumentationPage({ 
  title, 
  description, 
  children,
  lastUpdated,
  difficulty,
  estimatedReadTime
}: DocumentationPageProps) {
  const difficultyColors = {
    beginner: 'text-green-400 bg-green-400/10',
    intermediate: 'text-yellow-400 bg-yellow-400/10',
    advanced: 'text-red-400 bg-red-400/10'
  };

  return (
    <article className="prose prose-invert prose-slate max-w-none">
      {/* Page Header */}
      <header className="border-b border-obsidian-lighter pb-6 mb-8">
        <div className="flex flex-wrap items-center gap-4 mb-4">
          {difficulty && (
            <span className={`px-2 py-1 text-xs font-medium rounded-full ${difficultyColors[difficulty]}`}>
              {difficulty.charAt(0).toUpperCase() + difficulty.slice(1)}
            </span>
          )}
          {estimatedReadTime && (
            <span className="text-xs text-slate-gray">
              {estimatedReadTime}
            </span>
          )}
          {lastUpdated && (
            <span className="text-xs text-slate-gray">
              Updated {lastUpdated}
            </span>
          )}
        </div>
        
        <h1 id="overview" className="text-4xl font-bold text-white mb-4">
          {title}
        </h1>
        
        <p className="text-lg text-white/80 leading-relaxed">
          {description}
        </p>
      </header>

      {/* Page Content */}
      <div className="space-y-8">
        {children}
      </div>
    </article>
  );
}

export function Section({ title, children, id, level = 2 }: SectionProps) {
  const sectionId = id || title.toLowerCase().replace(/[^a-z0-9]+/g, '-');

  const getHeadingElement = () => {
    switch (level) {
      case 1:
        return (
          <h1 id={sectionId} className="text-3xl font-semibold text-white">
            {title}
          </h1>
        );
      case 2:
        return (
          <h2 id={sectionId} className="text-2xl font-semibold text-white">
            {title}
          </h2>
        );
      case 3:
        return (
          <h3 id={sectionId} className="text-xl font-semibold text-white">
            {title}
          </h3>
        );
      case 4:
        return (
          <h4 id={sectionId} className="text-lg font-semibold text-white">
            {title}
          </h4>
        );
      default:
        return (
          <h2 id={sectionId} className="text-2xl font-semibold text-white">
            {title}
          </h2>
        );
    }
  };

  return (
    <section className="space-y-4">
      {getHeadingElement()}
      <div className="space-y-4">
        {children}
      </div>
    </section>
  );
}

export function CodeExample({ title, language, code, description }: CodeExampleProps) {
  return (
    <div className="space-y-2">
      {title && (
        <h4 className="font-medium text-white">{title}</h4>
      )}
      {description && (
        <p className="text-sm text-white/70">{description}</p>
      )}
      <div className="relative">
        <pre className="bg-obsidian-darker border border-obsidian-lighter rounded-lg p-4 overflow-x-auto">
          <code className={`language-${language} text-sm`}>
            {code}
          </code>
        </pre>
      </div>
    </div>
  );
}

export function ParameterTable({ parameters }: ParameterTableProps) {
  return (
    <div className="overflow-x-auto">
      <table className="min-w-full border border-obsidian-lighter rounded-lg">
        <thead>
          <tr className="bg-obsidian-light">
            <th className="px-4 py-3 text-left text-sm font-medium text-white border-b border-obsidian-lighter">
              Parameter
            </th>
            <th className="px-4 py-3 text-left text-sm font-medium text-white border-b border-obsidian-lighter">
              Type
            </th>
            <th className="px-4 py-3 text-left text-sm font-medium text-white border-b border-obsidian-lighter">
              Required
            </th>
            <th className="px-4 py-3 text-left text-sm font-medium text-white border-b border-obsidian-lighter">
              Description
            </th>
            <th className="px-4 py-3 text-left text-sm font-medium text-white border-b border-obsidian-lighter">
              Default
            </th>
          </tr>
        </thead>
        <tbody>
          {parameters.map((param, index) => (
            <tr key={param.name} className={index % 2 === 0 ? 'bg-obsidian-darker/30' : ''}>
              <td className="px-4 py-3 text-sm font-mono text-hyperBlue border-b border-obsidian-lighter">
                {param.name}
              </td>
              <td className="px-4 py-3 text-sm font-mono text-white/80 border-b border-obsidian-lighter">
                {param.type}
              </td>
              <td className="px-4 py-3 text-sm border-b border-obsidian-lighter">
                <span className={`px-2 py-1 text-xs rounded-full ${
                  param.required 
                    ? 'text-red-400 bg-red-400/10' 
                    : 'text-green-400 bg-green-400/10'
                }`}>
                  {param.required ? 'Required' : 'Optional'}
                </span>
              </td>
              <td className="px-4 py-3 text-sm text-white/80 border-b border-obsidian-lighter">
                {param.description}
              </td>
              <td className="px-4 py-3 text-sm font-mono text-white/60 border-b border-obsidian-lighter">
                {param.defaultValue || '—'}
              </td>
            </tr>
          ))}
        </tbody>
      </table>
    </div>
  );
}

export function Note({ type, title, children }: NoteProps) {
  const styles = {
    info: {
      container: 'border-blue-400/30 bg-blue-400/5',
      icon: 'text-blue-400',
      title: 'text-blue-400'
    },
    warning: {
      container: 'border-yellow-400/30 bg-yellow-400/5',
      icon: 'text-yellow-400',
      title: 'text-yellow-400'
    },
    error: {
      container: 'border-red-400/30 bg-red-400/5',
      icon: 'text-red-400',
      title: 'text-red-400'
    },
    tip: {
      container: 'border-green-400/30 bg-green-400/5',
      icon: 'text-green-400',
      title: 'text-green-400'
    }
  };

  const style = styles[type];
  const icons = {
    info: 'ⓘ',
    warning: '⚠',
    error: '✕',
    tip: '💡'
  };

  return (
    <div className={`border rounded-lg p-4 ${style.container}`}>
      <div className="flex items-start space-x-3">
        <span className={`text-lg ${style.icon}`}>
          {icons[type]}
        </span>
        <div className="flex-1">
          {title && (
            <h5 className={`font-semibold mb-2 ${style.title}`}>
              {title}
            </h5>
          )}
          <div className="text-white/80">
            {children}
          </div>
        </div>
      </div>
    </div>
  );
}