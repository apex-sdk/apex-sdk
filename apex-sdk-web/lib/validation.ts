import fs from 'fs'
import path from 'path'
import matter from 'gray-matter'

export interface ValidationRule {
  name: string
  description: string
  check: (filePath: string, content: string, metadata: any) => string[]
}

export interface ValidationResult {
  filePath: string
  errors: Array<{
    rule: string
    message: string
  }>
}

// Required frontmatter fields
const requiredFields = ['title', 'description', 'section']

// Validation rules
export const validationRules: ValidationRule[] = [
  {
    name: 'required-frontmatter',
    description: 'Check for required frontmatter fields',
    check: (filePath, content, metadata) => {
      const errors: string[] = []
      
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
      const errors: string[] = []
      
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
      const errors: string[] = []
      
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
      const errors: string[] = []
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
        'typed-metadata'
      ]
      
      if (metadata.section && !validSections.includes(metadata.section)) {
        errors.push(`Invalid section: ${metadata.section}. Valid sections: ${validSections.join(', ')}`)
      }
      
      return errors
    }
  },
  
  {
    name: 'order-number',
    description: 'Check if order is a valid number',
    check: (filePath, content, metadata) => {
      const errors: string[] = []
      
      if (metadata.order !== undefined) {
        const order = Number(metadata.order)
        if (isNaN(order) || order < 0) {
          errors.push('Order must be a positive number')
        }
      }
      
      return errors
    }
  },
  
  {
    name: 'content-structure',
    description: 'Check basic content structure',
    check: (filePath, content, metadata) => {
      const errors: string[] = []
      
      // Check for at least one heading
      if (!content.includes('#')) {
        errors.push('Content should include at least one heading')
      }
      
      // Check minimum content length
      if (content.trim().length < 100) {
        errors.push('Content is too short (minimum 100 characters)')
      }
      
      return errors
    }
  },
  
  {
    name: 'consistent-casing',
    description: 'Check for consistent casing in titles',
    check: (filePath, content, metadata) => {
      const errors: string[] = []
      
      if (metadata.title) {
        // Check if title uses proper title case
        const words = metadata.title.split(' ')
        interface TitleCaseOptions {
            lowercaseWords: string[]
        }

        interface WordProcessingContext {
            word: string
            index: number
            totalWords: number
            options: TitleCaseOptions
        }

        const titleCaseOptions: TitleCaseOptions = {
            lowercaseWords: ['a', 'an', 'and', 'as', 'at', 'but', 'by', 'for', 'if', 'in', 'of', 'on', 'or', 'the', 'to', 'up', 'via']
        }

        const properTitleCase: string = words.map((word: string, index: number): string => {
            const context: WordProcessingContext = {
                word,
                index,
                totalWords: words.length,
                options: titleCaseOptions
            }
            
            if (context.index === 0 || context.index === context.totalWords - 1) {
                return context.word.charAt(0).toUpperCase() + context.word.slice(1).toLowerCase()
            }
            
            if (context.options.lowercaseWords.includes(context.word.toLowerCase())) {
                return context.word.toLowerCase()
            }
            
            return context.word.charAt(0).toUpperCase() + context.word.slice(1).toLowerCase()
        }).join(' ')
        
        if (metadata.title !== properTitleCase && !metadata.title.toUpperCase().includes('SDK') && !metadata.title.toUpperCase().includes('API')) {
          errors.push(`Title should use proper title case. Suggested: "${properTitleCase}"`)
        }
      }
      
      return errors
    }
  }
]

export async function validateContent(contentDir: string): Promise<ValidationResult[]> {
  const results: ValidationResult[] = []
  
  async function processDirectory(dir: string) {
    const entries = fs.readdirSync(dir, { withFileTypes: true })
    
    for (const entry of entries) {
      const fullPath = path.join(dir, entry.name)
      
      if (entry.isDirectory()) {
        await processDirectory(fullPath)
      } else if (entry.name.endsWith('.mdx')) {
        const content = fs.readFileSync(fullPath, 'utf8')
        const { data: metadata, content: mdxContent } = matter(content)
        
        const errors: Array<{ rule: string; message: string }> = []
        
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

export function formatValidationResults(results: ValidationResult[]): string {
  if (results.length === 0) {
    return 'All content validation checks passed!'
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