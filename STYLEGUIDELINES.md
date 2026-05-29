# UI / Design Guidelines

## General Style
Design the application using a GitHub-inspired interface style.

The UI should feel:
- clean
- minimal
- professional
- developer-focused
- modern but not flashy
- highly readable
- consistent across all screens

Avoid:
- excessive gradients
- glassmorphism
- oversized rounded corners
- neon colors
- overly animated interfaces
- mobile-app-like oversized spacing

---

# Theme System

Support:
- Light mode
- Dark mode

Use a color palette similar to GitHub / Primer Design System.

## Preferred Color Style

### Dark Theme
- Background: deep neutral dark
- Panels: slightly elevated dark gray
- Borders: subtle gray separators
- Text: soft white / light gray
- Accent: GitHub blue

### Light Theme
- Background: off-white
- Panels: white
- Borders: subtle gray
- Text: dark gray
- Accent: GitHub blue

---

# Typography

Use GitHub-like typography.

Preferred font stack:

css -apple-system, BlinkMacSystemFont, "Segoe UI", Helvetica, Arial, sans-serif 

Monospace/code font:

css ui-monospace, SFMono-Regular, Menlo, Consolas, monospace 

Typography should prioritize:
- readability
- compactness
- clean spacing
- developer-oriented density

---

# Layout

Preferred layout style:
- sidebar + content panel
- collapsible sections
- tabbed panels where useful
- resizable panes if appropriate

Use:
- subtle borders
- low visual noise
- compact spacing
- clear hierarchy

Avoid:
- huge padding
- oversized cards
- unnecessary floating elements

---

# Components

Buttons:
- subtle
- rectangular or slightly rounded
- minimal shadows

Inputs:
- GitHub-like bordered inputs
- focus ring on active state

Panels:
- soft border separation
- consistent padding
- no excessive nesting

Tables:
- compact
- readable
- developer-tool style

---

# Icons

Use simple monochrome icons similar to:
- Octicons
- Lucide
- Heroicons

Avoid cartoonish icons.

---

# Interaction Style

Animations should be:
- fast
- subtle
- functional

Avoid:
- bouncing
- dramatic transitions
- distracting effects

---

# UX Principles

Prioritize:
- clarity
- speed
- usability
- keyboard accessibility
- information density
- developer productivity

The interface should feel like:
- GitHub
- VSCode
- Linear
- modern developer tooling

---

# Technical Notes

Preferred technologies:
- TailwindCSS
- CSS variables
- theme tokens
- responsive layouts
- reusable components

Use semantic color tokens instead of hardcoded colors.

Example:

css --bg-primary --bg-secondary --text-primary --border-default --accent-color 

---

# Overall Goal

The application should feel like a professional developer tool with a polished GitHub-style UI and strong readability in both dark and light modes.