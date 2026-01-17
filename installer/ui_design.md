# Nebula Installer UI Design

## Visual Style
Modern glassy aesthetic with depth. Inspired by VS Code, JetBrains Toolbox, and Windows 11 design language. Minimal chrome, maximum content focus.

## Color Palette

| Role | Light Mode | Dark Mode |
|------|------------|-----------|
| Background | `#FAFAFA` | `#1E1E1E` |
| Surface | `#FFFFFF` | `#252526` |
| Surface Elevated | `#FFFFFF` + 8dp shadow | `#2D2D30` |
| Primary | `#6B4EE6` (Nebula Purple) | `#8B6FF0` |
| Primary Hover | `#5A3DD4` | `#9D85F5` |
| Accent | `#00D4AA` (Electric Teal) | `#00E6B8` |
| Text Primary | `#1A1A1A` | `#E5E5E5` |
| Text Secondary | `#6B6B6B` | `#9D9D9D` |
| Text Disabled | `#B0B0B0` | `#5A5A5A` |
| Border | `#E0E0E0` | `#3C3C3C` |
| Error | `#E53935` | `#FF6B6B` |
| Success | `#43A047` | `#69F0AE` |

## Typography

| Element | Font | Weight | Size |
|---------|------|--------|------|
| Title | Segoe UI | Semibold | 24px |
| Heading | Segoe UI | Semibold | 16px |
| Body | Segoe UI | Regular | 14px |
| Caption | Segoe UI | Regular | 12px |
| Button | Segoe UI | Semibold | 14px |
| Code/Path | Cascadia Code, Consolas | Regular | 13px |

## Window Layout

**Dimensions:** 560 x 420px (fixed)
**Border Radius:** 8px
**Shadow:** 0 8px 32px rgba(0,0,0,0.15)

### Screen 1: Welcome
```
┌──────────────────────────────────────────────┐
│  [Nebula Logo]                               │
│                                              │
│  Welcome to Nebula                           │
│  v1.0.0                                      │
│                                              │
│  A fast, simple programming language         │
│                                              │
│                                              │
│              [Cancel]  [Next →]              │
└──────────────────────────────────────────────┘
```

### Screen 2: License
```
┌──────────────────────────────────────────────┐
│  License Agreement                           │
│                                              │
│  ┌────────────────────────────────────────┐  │
│  │ MIT License                            │  │
│  │                                        │  │
│  │ Copyright (c) 2026 Nebula             │  │
│  │ ...                                    │  │
│  └────────────────────────────────────────┘  │
│                                              │
│  [✓] I accept the license terms              │
│                                              │
│          [← Back]  [Cancel]  [Next →]        │
└──────────────────────────────────────────────┘
```

### Screen 3: Install Path
```
┌──────────────────────────────────────────────┐
│  Installation Location                       │
│                                              │
│  Install to:                                 │
│  ┌──────────────────────────────┐ [Browse]   │
│  │ C:\Users\Dev\Nebula          │            │
│  └──────────────────────────────┘            │
│                                              │
│  Space required: 12 MB                       │
│  Space available: 243 GB                     │
│                                              │
│  [✓] Add Nebula to PATH                      │
│  [ ] Create desktop shortcut                 │
│  [ ] Associate .na files                     │
│                                              │
│          [← Back]  [Cancel]  [Next →]        │
└──────────────────────────────────────────────┘
```

### Screen 4: IDE Extensions
```
┌──────────────────────────────────────────────┐
│  IDE Extensions                              │
│                                              │
│  Install Nebula support for your editors:    │
│                                              │
│  [✓] VS Code              ● Detected         │
│  [ ] JetBrains IDEs       ○ Not found        │
│  [✓] Neovim               ● Detected         │
│  [ ] Sublime Text         ○ Not found        │
│                                              │
│  ─────────────────────────────────────────   │
│  [✓] Install recommended extensions          │
│                                              │
│  Extensions are optional and can be          │
│  installed later from each IDE.              │
│                                              │
│          [← Back]  [Cancel]  [Install]       │
└──────────────────────────────────────────────┘
```

**IDE Row States:**
- Detected: Checkbox enabled, "● Detected" in green
- Not Found: Checkbox disabled, greyed out, "○ Not found" in secondary text
- Installing: Spinner icon, "Installing..."
- Done: Checkmark icon, "✓ Installed"

### Screen 5: Progress
```
┌──────────────────────────────────────────────┐
│  Installing Nebula                           │
│                                              │
│  ████████████████░░░░░░░░░  67%              │
│                                              │
│  Copying runtime files...                    │
│                                              │
│                                              │
│                                              │
│                                              │
│                           [Cancel]           │
└──────────────────────────────────────────────┘
```

**Progress Bar:** 4px height, rounded ends, animated shimmer

### Screen 6: Success
```
┌──────────────────────────────────────────────┐
│                                              │
│         [✓ Success Icon]                     │
│                                              │
│  Nebula installed successfully               │
│                                              │
│  Installed to: C:\Users\Dev\Nebula           │
│  Extensions: VS Code, Neovim                 │
│                                              │
│  Get started:                                │
│  > nebula --help                             │
│                                              │
│                                              │
│                            [Finish]          │
└──────────────────────────────────────────────┘
```

## Button Styles

### Primary Button
| State | Background | Text | Border |
|-------|------------|------|--------|
| Default | `#6B4EE6` | `#FFFFFF` | none |
| Hover | `#5A3DD4` | `#FFFFFF` | none |
| Active | `#4A2DC4` | `#FFFFFF` | none |
| Disabled | `#E0E0E0` | `#9D9D9D` | none |

### Secondary Button
| State | Background | Text | Border |
|-------|------------|------|--------|
| Default | transparent | `#6B4EE6` | 1px `#6B4EE6` |
| Hover | `#6B4EE610` | `#5A3DD4` | 1px `#5A3DD4` |
| Disabled | transparent | `#B0B0B0` | 1px `#E0E0E0` |

**Border Radius:** 6px
**Padding:** 10px 20px
**Min Width:** 80px

## Checkbox Style
- 18x18px box, 2px border radius
- Unchecked: 2px border `#6B6B6B`
- Checked: Filled `#6B4EE6`, white checkmark
- Disabled: Background `#F0F0F0`, border `#D0D0D0`

## Animations

| Element | Animation | Duration | Easing |
|---------|-----------|----------|--------|
| Screen transition | Fade + slide right | 200ms | ease-out |
| Button hover | Background fade | 150ms | ease |
| Progress bar | Width + shimmer | continuous | linear |
| Success checkmark | Scale from 0 | 300ms | spring |
| Spinner | Rotate | 800ms | linear |

## Icons
- Style: Outlined, 1.5px stroke
- Size: 20px (standard), 48px (hero)
- Format: SVG embedded
- Colors: Inherit from text color

## Dark Mode
- Auto-detect from Windows theme
- No manual toggle in installer (follows system)
- All colors swap per palette above

## Accessibility
- Minimum contrast ratio: 4.5:1
- Focus indicators: 2px solid `#6B4EE6` outline
- Tab navigation: All interactive elements
- Screen reader: All elements labeled
- Minimum touch target: 32x32px
