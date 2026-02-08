#!/usr/bin/env node

import fs from 'fs'
import path from 'path'
import readline from 'readline'
import { fileURLToPath } from 'url'

const __filename = fileURLToPath(import.meta.url)
const __dirname = path.dirname(__filename)

const rl = readline.createInterface({
  input: process.stdin,
  output: process.stdout
})

function ask(question) {
  return new Promise(resolve => {
    rl.question(question, resolve)
  })
}

function slugify(text) {
  return text
    .toLowerCase()
    .replace(/[^a-z0-9]+/g, '-')
    .replace(/^-+|-+$/g, '')
}

function formatDate(date) {
  return date.toISOString().split('T')[0]
}

const templates = {
  'basic-guide': {
    name: 'Basic Guide',
    description: 'A standard documentation guide with step-by-step instructions'
  },
  'api-reference': {
    name: 'API Reference',
    description: 'API documentation for functions, methods, or endpoints'
  },
  'tutorial': {
    name: 'Tutorial',
    description: 'A hands-on tutorial with complete examples'
  }
}

const sections = {
  'getting-started': 'Getting Started',
  'installation': 'Installation',
  'api': 'API Reference', 
  'cli-guide': 'CLI Guide',
  'architecture': 'Architecture',
  'examples': 'Examples & Tutorials',
  'testing': 'Testing',
  'contributing': 'Contributing',
  'security': 'Security',
  'research': 'Research',
  'ecosystem': 'Ecosystem',
  'substrate': 'Substrate',
  'evm': 'EVM',
  'testnets': 'Testnets',
  'typed-metadata': 'Typed Metadata'
}

async function main() {
  console.log('📝 Apex SDK Documentation Generator\n')
  
  // Choose template
  console.log('Available templates:')
  Object.entries(templates).forEach(([key, template]) => {
    console.log(`  ${key}: ${template.name} - ${template.description}`)
  })
  
  const templateType = await ask('\nChoose template type: ')
  
  if (!templates[templateType]) {
    console.error(`❌ Invalid template type: ${templateType}`)
    process.exit(1)
  }
  
  // Get document details
  const title = await ask('Document title: ')
  const description = await ask('Description (80-160 chars): ')
  
  console.log('\nAvailable sections:')
  Object.entries(sections).forEach(([key, name]) => {
    console.log(`  ${key}: ${name}`)
  })
  
  const section = await ask('Section: ')
  
  if (!sections[section]) {
    console.error(`❌ Invalid section: ${section}`)
    process.exit(1)
  }
  
  const orderStr = await ask('Order (number): ')
  const order = parseInt(orderStr, 10)
  
  const tagsStr = await ask('Tags (comma-separated): ')
  const tags = tagsStr.split(',').map(tag => tag.trim()).filter(Boolean)
  
  // Generate filename and path
  const filename = slugify(title) + '.mdx'
  const contentDir = path.resolve(__dirname, '../content/docs')
  const filePath = path.join(contentDir, filename)
  
  if (fs.existsSync(filePath)) {
    console.error(`❌ File already exists: ${filename}`)
    process.exit(1)
  }
  
  // Load template
  const templatePath = path.resolve(__dirname, '../templates', `${templateType}.mdx`)
  
  if (!fs.existsSync(templatePath)) {
    console.error(`❌ Template not found: ${templateType}`)
    process.exit(1)
  }
  
  let content = fs.readFileSync(templatePath, 'utf8')
  
  // Replace placeholders
  const replacements = {
    'Document Title': title,
    'Brief description of what this document covers (80-160 characters)': description,
    '"getting-started"': `"${section}"`,
    'order: 1': `order: ${order}`,
    '["tag1", "tag2"]': JSON.stringify(tags),
    '"2024-01-01"': `"${formatDate(new Date())}"`,
    'API Reference: Function/Method Name': title,
    'Complete reference for the function/method including parameters and examples': description,
    'Tutorial: Build a Feature': title,
    'Complete tutorial for building a specific feature from start to finish': description
  }
  
  Object.entries(replacements).forEach(([search, replace]) => {
    content = content.replace(new RegExp(search, 'g'), replace)
  })
  
  // Write file
  fs.writeFileSync(filePath, content)
  
  console.log(`\n✅ Created: ${filename}`)
  console.log(`📍 Path: ${filePath}`)
  console.log(`🔗 URL: /docs/${slugify(title)}`)
  
  rl.close()
}

main().catch(console.error)