import Link from 'next/link';
// import { Card } from '../components/Card';

export default function DocsPage() {
  const quickStartItems = [
    {
      title: 'Quick Start',
      description: '5-minute setup guide to build your first cross-chain app',
      href: '/docs/quickstart',
      time: '5 min'
    },
    {
      title: 'CLI Guide',
      description: 'Master the command-line tools and project setup',
      href: '/docs/cli-guide',
      time: '10 min'
    },
    {
      title: 'API Reference',
      description: 'Complete API documentation with examples',
      href: '/docs/api',
      time: 'Reference'
    }
  ];

  const coreGuides = [
    {
      title: 'Testing Framework',
      description: 'Write comprehensive tests for your dApps',
      href: '/docs/testing',
      icon: 'Test'
    },
    {
      title: 'Security Guide',
      description: 'Security best practices and guidelines',
      href: '/docs/security',
      icon: 'Security'
    },
    {
      title: 'System Architecture',
      description: 'Understanding Apex SDK architecture',
      href: '/docs/architecture',
      icon: 'Build'
    },
    {
      title: 'Examples',
      description: 'Working code samples and tutorials',
      href: '/docs/examples',
      icon: 'Learn'
    }
  ];

  return (
    <div className="prose prose-invert max-w-none">
      <div className="mb-8">
        <h1 className="text-4xl font-bold text-white mb-4">Apex SDK Documentation</h1>
        <p className="text-xl text-slate-gray leading-relaxed">
          Build cross-chain applications with confidence using Rust's type safety and performance.
        </p>
      </div>

      {/* Quick Start Section */}
      <section className="mb-12">
        <h2 className="text-2xl font-bold text-white mb-6">Get Started</h2>
        <div className="grid md:grid-cols-3 gap-6 not-prose">
          {quickStartItems.map((item) => (
            <Link key={item.href} href={item.href} className="block">
              <div className="bg-obsidian-light border border-obsidian-lighter rounded-lg p-6 hover:border-hyperBlue/50 transition-colors h-full">
                <div className="flex items-center justify-between mb-3">
                  <h3 className="font-semibold text-white">{item.title}</h3>
                  <span className="text-xs px-2 py-1 bg-vividOrange/10 text-vividOrange rounded">{item.time}</span>
                </div>
                <p className="text-slate-gray text-sm">{item.description}</p>
              </div>
            </Link>
          ))}
        </div>
      </section>

      {/* Core Guides */}
      <section className="mb-12">
        <h2 className="text-2xl font-bold text-white mb-6">Core Guides</h2>
        <div className="grid md:grid-cols-2 gap-6 not-prose">
          {coreGuides.map((item) => (
            <Link key={item.href} href={item.href} className="block">
              <div className="bg-obsidian-light border border-obsidian-lighter rounded-lg p-6 hover:border-hyperBlue/50 transition-colors h-full flex items-start space-x-4">
                <div className="text-2xl">{item.icon}</div>
                <div>
                  <h3 className="font-semibold text-white mb-2">{item.title}</h3>
                  <p className="text-slate-gray text-sm">{item.description}</p>
                </div>
              </div>
            </Link>
          ))}
        </div>
      </section>

      {/* Additional Resources */}
      <section>
        <h2 className="text-2xl font-bold text-white mb-6">Resources</h2>
        <div className="bg-obsidian-light border border-obsidian-lighter rounded-lg p-6">
          <div className="grid md:grid-cols-2 gap-6">
            <div>
              <h3 className="font-semibold text-white mb-3">Community</h3>
              <ul className="space-y-2">
                <li>
                  <a href="https://discord.gg/zCDFsBaZJN" className="text-hyperBlue hover:underline">
                    Discord Community
                  </a>
                </li>
                <li>
                  <a href="https://github.com/apex-sdk/apex-sdk" className="text-hyperBlue hover:underline">
                    GitHub Repository
                  </a>
                </li>
                <li>
                  <Link href="/docs/contributing" className="text-hyperBlue hover:underline">
                    Contributing Guidelines
                  </Link>
                </li>
              </ul>
            </div>
            <div>
              <h3 className="font-semibold text-white mb-3">Advanced Topics</h3>
              <ul className="space-y-2">
                <li>
                  <Link href="/docs/substrate" className="text-hyperBlue hover:underline">
                    Substrate Features
                  </Link>
                </li>
                <li>
                  <Link href="/docs/evm" className="text-hyperBlue hover:underline">
                    EVM Integration
                  </Link>
                </li>
                <li>
                  <Link href="/docs/research" className="text-hyperBlue hover:underline">
                    Research Initiatives
                  </Link>
                </li>
              </ul>
            </div>
          </div>
        </div>
      </section>
    </div>
  );
}
