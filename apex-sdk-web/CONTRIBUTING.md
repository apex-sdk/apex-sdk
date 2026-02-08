# Contributing Guide

**Repository**: https://github.com/apex-sdk/apex-sdk-docs

## Development Workflow

1. **Fork & Clone**
   ```bash
   git clone https://github.com/apex-sdk/apex-sdk-docs.git
   cd apex-sdk-docs
   npm install
   ```

2. **Create Feature Branch**
   ```bash
   git checkout -b feature/your-feature-name
   ```

3. **Development**
   ```bash
   npm run dev
   # Make your changes
   ```

4. **Testing**
   ```bash
   npm test  # Runs lint + type-check + build
   ```

5. **Commit & Push**
   ```bash
   git add .
   git commit -m "feat: your feature description"
   git push origin feature/your-feature-name
   ```

## Code Style

- **ESLint**: Enforced automatically
- **TypeScript**: Strict mode enabled
- **Components**: Use functional components with hooks
- **Styling**: TailwindCSS utility classes

## Adding Documentation Pages

1. Create new page in `app/docs/your-page/page.tsx`
2. Follow existing page structure
3. Update navigation if needed
4. Test build locally: `npm run build`

## Pull Request Guidelines

- [x] All tests pass (`npm test`)
- [x] Clear, descriptive commit messages
- [x] Update documentation if needed
- [x] Link related issues

## Need Help?

Open an issue or reach out to the maintainers.