#!/usr/bin/env node

import fs from 'fs'
import path from 'path'
import matter from 'gray-matter'
import { fileURLToPath } from 'url'

const __filename = fileURLToPath(import.meta.url)
const __dirname = path.dirname(__filename)

// Validation rules and functions inline to avoid TypeScript import issues
const requiredFields = ['title', 'description', 'section']

const validationRules = [
  {
    name: 'required-frontmatter',
    description: 'Check for required frontmatter fields',
    check: (filePath, content, metadata) => {
      const errors = []
      
      requiredFields.forEach(field => {
        if (!metadata[field]) {
          errors.push(`Missing required field: ${field}`)
        }
      })
      
      return errors
    }
  },
  
  {
    name: 'title-length',
    description: 'Check title length is reasonable',
    check: (filePath, content, metadata) => {
      const errors = []
      
      if (metadata.title) {
        if (metadata.title.length < 10) {
          errors.push('Title is too short (minimum 10 characters)')
        }
        if (metadata.title.length > 60) {
          errors.push('Title is too long (maximum 60 characters)')
        }
      }
      
      return errors
    }
  },
  
  {
    name: 'description-length',
    description: 'Check description length is reasonable',
    check: (filePath, content, metadata) => {
      const errors = []
      
      if (metadata.description) {
        if (metadata.description.length < 20) {
          errors.push('Description is too short (minimum 20 characters)')
        }
        if (metadata.description.length > 160) {
          errors.push('Description is too long (maximum 160 characters)')
        }
      }
      
      return errors
    }
  },
  
  {
    name: 'valid-section',
    description: 'Check if section is valid',
    check: (filePath, content, metadata) => {
      const errors = []
      const validSections = [
        'getting-started',
        'installation',
        'api',
        'cli-guide',
        'architecture',
        'examples',
        'testing',
        'contributing',
        'security',
        'research',
        'ecosystem',
        'substrate',
        'evm',
        'testnets',
        'typed-metadata',
        'revive'
      ]
      
      if (metadata.section && !validSections.includes(metadata.section)) {
        errors.push(`Invalid section: ${metadata.section}. Valid sections: ${validSections.join(', ')}`)
      }
      
      return errors
    }
  },
  
  {
    name: 'content-structure',
    description: 'Check basic content structure',
    check: (filePath, content, metadata) => {
      const errors = []
      
      if (!content.includes('#')) {
        errors.push('Content should include at least one heading')
      }
      
      if (content.trim().length < 100) {
        errors.push('Content is too short (minimum 100 characters)')
      }
      
      return errors
    }
  }
]

async function validateContent(contentDir) {
  const results = []
  
  async function processDirectory(dir) {
    const entries = fs.readdirSync(dir, { withFileTypes: true })
    
    for (const entry of entries) {
      const fullPath = path.join(dir, entry.name)
      
      if (entry.isDirectory()) {
        await processDirectory(fullPath)
      } else if (entry.name.endsWith('.mdx')) {
        const content = fs.readFileSync(fullPath, 'utf8')
        const { data: metadata, content: mdxContent } = matter(content)
        
        const errors = []
        
        for (const rule of validationRules) {
          const ruleErrors = rule.check(fullPath, mdxContent, metadata)
          ruleErrors.forEach(error => {
            errors.push({
              rule: rule.name,
              message: error
            })
          })
        }
        
        if (errors.length > 0) {
          results.push({
            filePath: path.relative(contentDir, fullPath),
            errors
          })
        }
      }
    }
  }
  
  await processDirectory(contentDir)
  return results
}

function formatValidationResults(results) {
  if (results.length === 0) {
    return '✅ All content validation checks passed!'
  }
  
  let output = `Found ${results.length} file(s) with validation errors:\n\n`
  
  results.forEach(result => {
    output += `📄 ${result.filePath}\n`
    result.errors.forEach(error => {
      output += `  ❌ [${error.rule}] ${error.message}\n`
    })
    output += '\n'
  })
  
  return output
}

async function main() {
  console.log('🔍 Validating MDX content...\n')
  
  const contentDir = path.resolve(__dirname, '../content')
  const results = await validateContent(contentDir)
  
  const output = formatValidationResults(results)
  console.log(output)
  
  // Exit with error code if validation failed
  if (results.length > 0) {
    process.exit(1)
  }
}

main().catch(console.error)