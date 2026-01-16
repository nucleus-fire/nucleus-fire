# Accessibility Guide

> Build inclusive applications that work for everyone with Nucleus's WCAG 2.1 AA compliant components.

## Overview

Nucleus is designed with accessibility as a first-class concern. All built-in components follow WCAG 2.1 AA guidelines, ensuring your applications are usable by:

- People using screen readers
- Keyboard-only users
- Users with visual impairments
- Users with motor disabilities
- Users with cognitive disabilities

## WCAG 2.1 AA Compliance

### The POUR Principles

| Principle | Description | Implementation |
|-----------|-------------|----------------|
| **Perceivable** | Content can be perceived | Alt text, color contrast, captions |
| **Operable** | Interface is navigable | Keyboard access, skip links, focus management |
| **Understandable** | Content is readable | Clear labels, error messages, predictable navigation |
| **Robust** | Works with assistive tech | Semantic HTML, ARIA attributes |

## Built-in Features

### Skip Links

Every Nucleus page includes a skip link for keyboard users:

```html
<!-- Automatically added to layout -->
<a href="#main-content" class="skip-link">Skip to main content</a>
```

Users can press Tab on page load to reveal and activate the skip link.

### Focus Management

Visible focus indicators are provided for all interactive elements:

```css
/* Focus visible for keyboard users */
:focus-visible {
    outline: 2px solid #6366f1;
    outline-offset: 2px;
}
```

### Screen Reader Announcements

Live regions announce dynamic content changes:

```html
<div id="a11y-announcer" aria-live="polite" aria-atomic="true"></div>
```

Use the announcer in JavaScript:

```javascript
function announce(message) {
    const announcer = document.getElementById('a11y-announcer');
    announcer.textContent = message;
}

// Example: Form submission
announce('Form submitted successfully');
```

## Forms

Nucleus forms are built with accessibility in mind:

### Labels and Help Text

```xml
<TextInput 
    name="email" 
    label="Email Address" 
    help="We'll never share your email"
    required="true"
/>
```

Renders as:

```html
<div class="form-field">
    <label for="email">
        Email Address <span class="required-indicator" aria-hidden="true">*</span>
    </label>
    <input 
        type="text" 
        id="email" 
        name="email"
        aria-describedby="email-help"
        required
    >
    <span id="email-help" class="field-help">We'll never share your email</span>
</div>
```

### Error Messages

```xml
<TextInput 
    name="password" 
    label="Password" 
    error="Password must be at least 8 characters"
/>
```

Renders with proper ARIA:

```html
<input 
    id="password"
    aria-invalid="true"
    aria-describedby="password-error"
>
<span id="password-error" class="field-error" role="alert">
    Password must be at least 8 characters
</span>
```

### Fieldsets for Groups

```xml
<FormGroup legend="Personal Information">
    <TextInput name="firstName" label="First Name" />
    <TextInput name="lastName" label="Last Name" />
</FormGroup>
```

## Keyboard Navigation

### Standard Keys

| Key | Action |
|-----|--------|
| `Tab` | Move to next focusable element |
| `Shift + Tab` | Move to previous focusable element |
| `Enter` | Activate buttons, follow links |
| `Space` | Activate checkboxes, buttons |
| `Arrow Keys` | Navigate within components (tabs, menus) |
| `Escape` | Close modals, dismiss popups |

### Focus Trap for Modals

When modals are open, focus is trapped within:

```javascript
// Automatically applied to [role="dialog"]
// Tab cycles through modal content only
// Escape closes the modal
// Focus returns to trigger element on close
```

## Color and Contrast

### Minimum Contrast Ratios

| Content Type | Ratio (AA) | Ratio (AAA) |
|--------------|------------|-------------|
| Normal text | 4.5:1 | 7:1 |
| Large text (18px+) | 3:1 | 4.5:1 |
| UI components | 3:1 | 3:1 |

### Nucleus Color Palette

Our default colors meet AA requirements:

```css
:root {
    --text-primary: #ededed;    /* 14.7:1 on #000 ✅ */
    --text-secondary: #a1a1aa;  /* 7.2:1 on #000 ✅ */
    --accent: #00dc82;          /* 10.5:1 on #000 ✅ */
}
```

### Don't Rely on Color Alone

Always provide additional indicators:

```xml
<!-- ❌ Color only -->
<span style="color: red">Error</span>

<!-- ✅ Color + icon + text -->
<span class="status-error">
    <span aria-hidden="true">⚠️</span>
    Error: Invalid email
</span>
```

## Reduced Motion

Respect user preferences for reduced motion:

```css
@media (prefers-reduced-motion: reduce) {
    * {
        animation-duration: 0.01ms !important;
        transition-duration: 0.01ms !important;
    }
}
```

This is automatically applied via `accessibility.css`.

## Screen Reader Tips

### Hiding Decorative Content

```html
<!-- Decorative icons -->
<span aria-hidden="true">⚡</span>

<!-- Decorative images -->
<img src="decoration.svg" alt="" role="presentation">
```

### Providing Context

```html
<!-- Buttons with icons need labels -->
<button aria-label="Close dialog">
    <span aria-hidden="true">×</span>
</button>

<!-- Links need descriptive text -->
<a href="/docs">
    Read documentation
    <span class="sr-only"> about Nucleus Framework</span>
</a>
```

### Status Messages

```html
<div role="status" aria-live="polite">
    3 items in your cart
</div>

<div role="alert" aria-live="assertive">
    Error: Payment failed
</div>
```

## Testing Accessibility

### Automated Testing

1. **Lighthouse** - Built into Chrome DevTools
   ```bash
   # Run via CLI
   lighthouse http://localhost:3000 --only-categories=accessibility
   ```

2. **axe DevTools** - Browser extension for detailed reports

3. **WAVE** - Web accessibility evaluation tool

### Manual Testing

1. **Keyboard Navigation**
   - Tab through all interactive elements
   - Verify focus is visible
   - Test skip links

2. **Screen Reader**
   - VoiceOver (Mac): `Cmd + F5`
   - NVDA (Windows): Free download
   - JAWS (Windows): Commercial

3. **Color Contrast**
   - Use browser dev tools
   - Test with colorblind simulators

## Checklist

### Before Launch

- [ ] All images have alt text
- [ ] Color contrast meets 4.5:1 (normal) or 3:1 (large)
- [ ] All forms have labels
- [ ] Error messages are linked to inputs
- [ ] Keyboard navigation works throughout
- [ ] Focus indicators are visible
- [ ] Skip link present and functional
- [ ] Page has proper heading hierarchy (one h1)
- [ ] Landmarks (`main`, `nav`, `footer`) are used
- [ ] Reduced motion is respected
- [ ] Screen reader testing passed

### Continuous

- [ ] Run Lighthouse on each deploy
- [ ] Test new features with keyboard
- [ ] Review ARIA usage for correctness

## Resources

- [WCAG 2.1 Guidelines](https://www.w3.org/WAI/WCAG21/quickref/)
- [MDN ARIA Guide](https://developer.mozilla.org/en-US/docs/Web/Accessibility/ARIA)
- [WebAIM Contrast Checker](https://webaim.org/resources/contrastchecker/)
- [Inclusive Components](https://inclusive-components.design/)

## Component Reference

### Accessible Components

| Component | Keyboard | Screen Reader | Focus |
|-----------|----------|---------------|-------|
| Button | ✅ Enter/Space | ✅ Role, state | ✅ |
| TextInput | ✅ Tab | ✅ Label, error | ✅ |
| Select | ✅ Arrow keys | ✅ Options | ✅ |
| Checkbox | ✅ Space | ✅ Checked state | ✅ |
| Modal | ✅ Escape, Tab trap | ✅ Dialog role | ✅ |
| Tabs | ✅ Arrow keys | ✅ Tablist role | ✅ |
| Wizard | ✅ Tab, Arrows | ✅ Step progress | ✅ |

## Related Guides

- [Forms Guide](#07_forms_and_validation) - Accessible form patterns
- [Components Guide](#26_components_guide) - All component accessibility
