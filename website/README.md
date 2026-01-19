# Nebula Documentation Website

The official documentation and landing page for [Nebula](https://github.com/Sxf0z/Nebula) - a high-performance scripting language.

## Development

```bash
# Install dependencies
npm install

# Run development server
npm run dev

# Build for production
npm run build
```

## Deployment

This site is configured for deployment on **Netlify**.

### Deploy to Netlify

1. Connect your GitHub repository to Netlify
2. Set the build settings:
   - **Base directory**: `website`
   - **Build command**: `npm run build`
   - **Publish directory**: `website/.next`
3. Deploy!

Or use the Netlify CLI:

```bash
# Install Netlify CLI
npm install -g netlify-cli

# Login and deploy
netlify login
netlify deploy --prod
```

## Structure

```
website/
├── app/                 # Next.js App Router
│   ├── page.tsx        # Landing page
│   ├── docs/           # Documentation pages
│   └── globals.css     # Global styles
├── components/
│   ├── landing/        # Landing page components
│   ├── ui/             # UI components (sidebar, etc.)
│   └── mdx.tsx         # MDX component styling
├── content/            # MDX documentation files
├── lib/                # Utilities
└── public/             # Static assets
```

## Tech Stack

- **Framework**: Next.js 15
- **Styling**: Tailwind CSS
- **Content**: MDX
- **Fonts**: Geist, Outfit
