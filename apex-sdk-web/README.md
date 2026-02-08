# Apex SDK: System Chain Standard Library

> The unified standard for building system chain applications on Polkadot Asset Hub, Revive, and PolkaVM.

![Next.js](https://img.shields.io/badge/Next.js-16.1.1-black) ![React](https://img.shields.io/badge/React-19.2.3-blue) ![TailwindCSS](https://img.shields.io/badge/TailwindCSS-4.0-06B6D4) ![TypeScript](https://img.shields.io/badge/TypeScript-5.x-3178C6)

## Quick Start

```bash
# Install dependencies
npm install

# Start development server
npm run dev
# Open http://localhost:3000
```

## Available Scripts

| Script | Description |
|--------|-------------|
| `npm run dev` | Start development server |
| `npm run build` | Build for production |
| `npm run start` | Start production server |
| `npm run lint` | Run ESLint checks |
| `npm test` | Run full test suite |

## Tech Stack

- **Framework**: Next.js 16 (App Router)
- **Styling**: TailwindCSS v4
- **Language**: TypeScript
- **Target**: System Chain Standard (Asset Hub, Revive, PolkaVM)

## Development

### Prerequisites
- Node.js 20+
- npm or yarn


### CI/CD
GitHub Actions workflow automatically runs:
- ESLint code quality checks
- TypeScript type validation
- Production build verification

## Deployment

Builds to static files in `out/` directory, ready for any static hosting service.

```bash
npm run build
# Deploy the 'out/' folder
```