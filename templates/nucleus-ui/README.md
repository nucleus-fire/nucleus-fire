# Nucleus UI

A complete, polished, modern Tailwind CSS-based UI component library for the Nucleus Framework.

![Nucleus UI](https://img.shields.io/badge/Nucleus_UI-v1.0.0-6366f1?style=flat-square)

## Features

- 🎨 **Beautiful by default** - Professionally designed with attention to detail
- 🌙 **Dark mode** - Full dark mode support out of the box
- ♿ **Accessible** - WCAG AA compliant with keyboard navigation
- 📱 **Responsive** - Mobile-first design that works everywhere
- ✨ **Animated** - Smooth micro-animations with reduced-motion support
- 🔧 **Customizable** - CSS variables for easy theming

## Quick Start

```bash
cd templates/nucleus-ui
npm install
npm run build:css
```

## Components

### Core Elements
| Component | Path | Variants |
|-----------|------|----------|
| Button | `components/buttons/button.ncl` | primary, secondary, outline, ghost, danger, success |
| Icon Button | `components/buttons/icon-button.ncl` | primary, secondary, ghost, danger |
| Button Group | `components/buttons/button-group.ncl` | horizontal, vertical |
| Badge | `components/feedback/badge.ncl` | solid, outline, soft × 6 colors |
| Avatar | `components/feedback/avatar.ncl` | 6 sizes, status indicators |
| Avatar Group | `components/feedback/avatar-group.ncl` | stacked avatars |
| Spinner | `components/feedback/spinner.ncl` | 5 sizes, 3 colors |
| Icon | `components/feedback/icon.ncl` | 5 sizes, 6 colors |
| Divider | `components/layout/divider.ncl` | solid, dashed, dotted |

### Forms
| Component | Path | Features |
|-----------|------|----------|
| Input | `components/forms/input.ncl` | label, error, hint, icons |
| Textarea | `components/forms/textarea.ncl` | auto-grow, char count |
| Select | `components/forms/select.ncl` | native, placeholder |
| Checkbox | `components/forms/checkbox.ncl` | label, description, indeterminate |
| Radio | `components/forms/radio.ncl` | label, description |
| Toggle | `components/forms/toggle.ncl` | switch with label |

### Feedback & Overlays
| Component | Path | Variants |
|-----------|------|----------|
| Alert | `components/feedback/alert.ncl` | info, success, warning, error |
| Modal | `components/overlays/modal.ncl` | sm, md, lg, xl, full |
| Card | `components/data-display/card.ncl` | default, elevated, bordered, ghost |
| Table | `components/data-display/table.ncl` | striped, hoverable, bordered |

### Navigation
| Component | Path | Features |
|-----------|------|----------|
| Tabs | `components/navigation/tabs.ncl` | underline, pills, boxed |
| Tab | `components/navigation/tab.ncl` | individual tab item |
| Breadcrumbs | `components/navigation/breadcrumbs.ncl` | with separators |
| Pagination | `components/navigation/pagination.ncl` | first/last, prev/next |
| Progress | `components/navigation/progress.ncl` | bar with label, indeterminate |

### Marketing Blocks
| Component | Path | Variants |
|-----------|------|----------|
| Hero | `blocks/marketing/hero.ncl` | centered, split, with-image |
| Features | `blocks/marketing/features.ncl` | grid, alternating, centered |
| Feature Card | `blocks/marketing/feature-card.ncl` | icon + title + description |
| CTA | `blocks/marketing/cta.ncl` | simple, split, gradient |
| Pricing | `blocks/marketing/pricing.ncl` | cards container |
| Pricing Card | `blocks/marketing/pricing-card.ncl` | tier with features list |
| Footer | `blocks/marketing/footer.ncl` | simple, mega, centered |

## Usage

```html
<!-- Import components in your view -->
<n:include src="nucleus-ui/src/components/buttons/button.ncl" />

<!-- Use the button -->
<n:button variant="primary" size="lg">
  Get Started
</n:button>

<!-- Button with loading state -->
<n:button variant="primary" loading="true">
  Saving...
</n:button>

<!-- Badge -->
<n:badge variant="soft" color="success" dot="true">
  Active
</n:button>

<!-- Avatar with status -->
<n:avatar 
  src="/images/user.jpg" 
  name="John Doe" 
  size="lg"
  status="online"
/>
```

## Customization

Override CSS variables in your project:

```css
:root {
  /* Custom primary color (indigo) */
  --color-primary-500: 99 102 241;
  --color-primary-600: 79 70 229;
  
  /* Custom surface colors */
  --color-surface-50: 250 250 250;
}
```

## Design Tokens

| Token | Description |
|-------|-------------|
| `--color-primary-*` | Primary brand colors (50-950) |
| `--color-surface-*` | Neutral/background colors |
| `--radius-*` | Border radius scale |
| `--shadow-*` | Shadow scale |
| `--duration-*` | Animation durations |

## Browser Support

- Chrome 90+
- Firefox 88+
- Safari 14+
- Edge 90+

## License

MIT © Nucleus Framework
