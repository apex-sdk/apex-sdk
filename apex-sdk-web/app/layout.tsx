import type { Metadata } from "next";
import { Inter, JetBrains_Mono } from "next/font/google";
import "./globals.css";
import { OrganizationJsonLd, SoftwareJsonLd, WebSiteJsonLd, BreadcrumbJsonLd } from './components/JsonLd';

const inter = Inter({
  variable: "--font-inter",
  subsets: ["latin"],
  display: 'swap',
  weight: ['300', '400', '500', '600', '700'],
  style: ['normal', 'italic'],
});

const jetbrainsMono = JetBrains_Mono({
  variable: "--font-jetbrains-mono",
  subsets: ["latin"],
  display: 'swap',
  weight: ['300', '400', '500', '600', '700'],
  style: ['normal', 'italic'],
});

export const metadata: Metadata = {
  metadataBase: new URL('https://apexsdk.dev'),
  title: {
    default: "Apex SDK - Rust Blockchain SDK for Multi-Chain Development",
    template: "%s | Apex SDK"
  },
  description: "Apex SDK: Enterprise Rust blockchain SDK for building cross-chain dApps on Ethereum, Polkadot & 15+ networks. Type-safe, fast & production-ready. Start now!",
  keywords: [
    "rust blockchain sdk", "cross-chain rust library", "polkadot rust sdk", "ethereum rust sdk",
    "substrate rust framework", "blockchain development rust", "multi-chain sdk rust", "evm rust library",
    "rust crypto sdk", "apex sdk", "blockchain transaction rust", "rust defi development",
    "substrate framework tutorial", "cross-chain development guide", "rust blockchain tutorial",
    "blockchain sdk comparison", "multi-chain wallet development", "rust smart contract development"
  ],
  authors: [{ name: "Apex SDK Contributors", url: "https://apex-sdk-docs.pages.dev" }],
  creator: "Apex SDK Contributors",
  publisher: "Apex SDK Contributors",
  category: "technology",
  classification: "Blockchain Development Tools",
  robots: {
    index: true,
    follow: true,
    googleBot: {
      index: true,
      follow: true,
      'max-video-preview': -1,
      'max-image-preview': 'large',
      'max-snippet': -1,
    },
  },
  icons: {
    icon: [
      { url: '/assets/favicon-16x16.png', sizes: '16x16', type: 'image/png' },
      { url: '/assets/favicon-32x32.png', sizes: '32x32', type: 'image/png' },
    ],
    apple: '/assets/apple-touch-icon.png',
  },
  manifest: '/site.webmanifest',
  openGraph: {
    type: "website",
    url: "https://apexsdk.dev",
    title: "Apex SDK - Rust Blockchain SDK for Multi-Chain Development",
    description: "Enterprise Rust blockchain SDK for building cross-chain dApps across 15+ blockchain networks. Type-safe, fast & production-ready. Download now!",
    siteName: "Apex SDK",
    locale: "en_US",
    images: [
      {
        url: 'https://apex-sdk-og.kherld-hussein.workers.dev/?title=Apex%20SDK&subtitle=Enterprise%20Rust%20blockchain%20SDK%20for%20building%20cross-chain%20dApps%20across%2015%2B%20blockchain%20networks.',
        width: 1200,
        height: 630,
        alt: 'Apex SDK - Cross-Chain Blockchain Development with Rust',
        type: 'image/png',
      },
    ],
  },
  twitter: {
    card: "summary_large_image",
    site: "@apexsdk",
    creator: "@apexsdk",
    title: "Apex SDK - Rust Blockchain SDK for Multi-Chain Development",
    description: "Enterprise Rust blockchain SDK for building cross-chain dApps. Type-safe, fast & production-ready. 15+ networks supported.",
    images: {
      url: 'https://apex-sdk-og.kherld-hussein.workers.dev/?title=Apex%20SDK&subtitle=Enterprise%20Rust%20blockchain%20SDK%20for%20building%20cross-chain%20dApps%20across%2015%2B%20blockchain%20networks.',
      alt: 'Apex SDK - Cross-Chain Blockchain Development with Rust',
    },
  },
  alternates: {
    canonical: "https://apexsdk.dev",
  },
  other: {
    'google-site-verification': 'YOUR_GOOGLE_VERIFICATION_CODE',
    'msvalidate.01': 'YOUR_BING_VERIFICATION_CODE',
  },
};

export default function RootLayout({
  children,
}: Readonly<{
  children: React.ReactNode;
}>) {
  return (
    <html lang="en">
      <body
        className={`${inter.variable} ${jetbrainsMono.variable} antialiased bg-obsidian text-white font-sans`}
      >
        <OrganizationJsonLd />
        <SoftwareJsonLd />
        <WebSiteJsonLd />
        <BreadcrumbJsonLd />
        {children}
      </body>
    </html>
  );
}
