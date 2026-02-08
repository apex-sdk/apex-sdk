'use client';

import { usePathname } from 'next/navigation';

interface JsonLdProps {
  data: Record<string, unknown>;
}

export function JsonLd({ data }: JsonLdProps) {
  return (
    <script
      type="application/ld+json"
      dangerouslySetInnerHTML={{ __html: JSON.stringify(data) }}
    />
  );
}

export function OrganizationJsonLd() {
  const organizationData = {
    "@context": "https://schema.org",
    "@type": "Organization",
    "name": "Apex SDK Protocol",
    "alternateName": "Apex SDK",
    "url": "https://apex-sdk-docs.pages.dev",
    "logo": {
      "@type": "ImageObject",
      "url": "https://apex-sdk-docs.pages.dev/assets/logo.png",
      "width": "200",
      "height": "60"
    },
    "description": "The definitive Rust framework for Multichain Development. Apex SDK empowers developers to build secure, scalable, and interoperable blockchain applications across Substrate, EVM, and beyond. The definitive Rust framework for Multichain Development.",
    "foundingDate": "2024",
    "industry": "Blockchain Technology",
    "knowsAbout": [
      "Multichain Development",
      "Cross-chain Solutions",
      "Rust Programming",
      "Smart Contracts",
      "DeFi",
      "Web3",
      "Substrate",
      "Ethereum",
      "Polkadot"
    ],
    "sameAs": [
      "https://github.com/apex-sdk/apex-sdk",
      "https://discord.gg/zCDFsBaZJN"
    ]
  };

  return <JsonLd data={organizationData} />;
}

export function SoftwareJsonLd() {
  const softwareData = {
    "@context": "https://schema.org",
    "@type": "SoftwareApplication",
    "name": "Apex SDK Protocol",
    "alternateName": "Apex SDK",
    "applicationCategory": "DeveloperApplication",
    "operatingSystem": ["Linux", "macOS", "Windows"],
    "programmingLanguage": "Rust",
    "url": "https://apex-sdk-docs.pages.dev",
    "downloadUrl": "https://github.com/apex-sdk/apex-sdk",
    "author": "Apex SDK Contributors",
    "publisher": "Apex SDK Contributors",
    "description": "The definitive Rust framework for Multichain Development. Build secure, scalable, and interoperable blockchain applications for Substrate, EVM, and more. Type-safe APIs, blazing fast performance, and unified standards for dApp development. The definitive Rust framework for Multichain Development.",
    "keywords": ["Rust", "blockchain", "multichain", "SDK", "Substrate", "EVM", "Web3", "dApp", "cross-chain", "crypto", "open source"],
    "features": [
      "Multichain development",
      "Type-safe APIs",
      "Substrate integration",
      "EVM compatibility",
      "WebAssembly support",
      "Real-time synchronization",
      "Developer-friendly CLI",
      "Open source"
    ],
    "softwareVersion": "1.0.0",
    "license": "https://github.com/apex-sdk/apex-sdk/blob/main/LICENSE",
    "codeRepository": "https://github.com/apex-sdk/apex-sdk",
    "supportingData": {
      "@type": "DataDownload",
      "name": "Documentation",
      "url": "https://apex-sdk-docs.pages.dev/docs"
    }
  };

  return <JsonLd data={softwareData} />;
}

export function WebSiteJsonLd() {
  // const pathname = usePathname(); // TODO: Use pathname when needed
  
  const websiteData = {
    "@context": "https://schema.org",
    "@type": "WebSite", 
    "name": "Apex SDK Protocol",
    "alternateName": "Apex SDK Documentation",
    "url": "https://apex-sdk-docs.pages.dev",
    "description": "The definitive Rust framework for Multichain Development. Official documentation and resources for Apex SDK Protocol, empowering developers to build next-generation blockchain applications. The definitive Rust framework for Multichain Development.",
    "publisher": "Apex SDK Contributors",
    "keywords": ["Rust", "blockchain", "multichain", "SDK", "Substrate", "EVM", "Web3", "dApp", "cross-chain", "crypto", "open source"],
    "potentialAction": {
      "@type": "SearchAction",
      "target": {
        "@type": "EntryPoint",
        "urlTemplate": "https://apex-sdk-docs.pages.dev/docs?search={search_term_string}"
      },
      "query-input": "required name=search_term_string"
    },
    "mainEntity": {
      "@type": "SoftwareApplication",
      "name": "Apex SDK Protocol"
    }
  };

  return <JsonLd data={websiteData} />;
}

export function BreadcrumbJsonLd() {
  const pathname = usePathname();
  const pathSegments = pathname.split('/').filter(Boolean);
  
  if (pathSegments.length === 0) return null;
  
  const breadcrumbItems = [
    {
      "@type": "ListItem",
      "position": 1,
      "name": "Home",
      "item": "https://apex-sdk-docs.pages.dev"
    }
  ];
  
  let currentPath = '';
  pathSegments.forEach((segment, index) => {
    currentPath += `/${segment}`;
    breadcrumbItems.push({
      "@type": "ListItem",
      "position": index + 2,
      "name": segment.charAt(0).toUpperCase() + segment.slice(1).replace(/-/g, ' '),
      "item": `https://apex-sdk-docs.pages.dev${currentPath}`
    });
  });
  
  const breadcrumbData = {
    "@context": "https://schema.org",
    "@type": "BreadcrumbList", 
    "itemListElement": breadcrumbItems
  };
  
  return <JsonLd data={breadcrumbData} />;
}