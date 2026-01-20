<!--
  Nucleus UI - Badge Component
  
  A small status indicator for labels, counts, and tags.
  
  Props:
    variant: solid | outline | soft (default: solid)
    color: primary | secondary | success | warning | error | info (default: primary)
    size: sm | md | lg (default: md)
    dot: boolean - show status dot instead of content
    removable: boolean - show remove button
-->

<n:component>
  <n:props>
    variant: string = "solid",
    color: string = "primary",
    size: string = "md",
    dot: bool = false,
    removable: bool = false
  </n:props>

  <span
    class="
      nui-badge
      nui-badge--{variant}
      nui-badge--{color}
      nui-badge--{size}
      inline-flex items-center
      font-medium
      transition-colors duration-200
    "
  >
    {if dot}
      <span class="nui-badge__dot animate-pulse-subtle"></span>
    {/if}
    
    <n:slot />
    
    {if removable}
      <button
        type="button"
        class="nui-badge__remove ml-1 -mr-0.5 hover:bg-black/10 dark:hover:bg-white/10 rounded-full p-0.5 transition-colors"
        aria-label="Remove"
      >
        <svg class="h-3 w-3" fill="none" viewBox="0 0 24 24" stroke="currentColor" stroke-width="2">
          <path stroke-linecap="round" stroke-linejoin="round" d="M6 18L18 6M6 6l12 12" />
        </svg>
      </button>
    {/if}
  </span>

  <style scoped>
    /* ═══════════════════════════════════════════════════════════════════════════
       SIZE VARIANTS
       ═══════════════════════════════════════════════════════════════════════════ */
    
    .nui-badge--sm {
      @apply h-5 px-1.5 text-2xs rounded;
    }
    
    .nui-badge--md {
      @apply h-6 px-2 text-xs rounded-md;
    }
    
    .nui-badge--lg {
      @apply h-7 px-2.5 text-sm rounded-md;
    }

    /* Status Dot */
    .nui-badge__dot {
      @apply h-1.5 w-1.5 rounded-full mr-1.5;
    }

    /* ═══════════════════════════════════════════════════════════════════════════
       SOLID VARIANTS
       ═══════════════════════════════════════════════════════════════════════════ */
    
    .nui-badge--solid.nui-badge--primary {
      @apply bg-primary-600 text-white;
    }
    .nui-badge--solid.nui-badge--primary .nui-badge__dot {
      @apply bg-white;
    }

    .nui-badge--solid.nui-badge--secondary {
      @apply bg-surface-600 text-white;
    }
    .nui-badge--solid.nui-badge--secondary .nui-badge__dot {
      @apply bg-white;
    }

    .nui-badge--solid.nui-badge--success {
      @apply bg-green-600 text-white;
    }
    .nui-badge--solid.nui-badge--success .nui-badge__dot {
      @apply bg-white;
    }

    .nui-badge--solid.nui-badge--warning {
      @apply bg-yellow-500 text-yellow-950;
    }
    .nui-badge--solid.nui-badge--warning .nui-badge__dot {
      @apply bg-yellow-950;
    }

    .nui-badge--solid.nui-badge--error {
      @apply bg-red-600 text-white;
    }
    .nui-badge--solid.nui-badge--error .nui-badge__dot {
      @apply bg-white;
    }

    .nui-badge--solid.nui-badge--info {
      @apply bg-blue-600 text-white;
    }
    .nui-badge--solid.nui-badge--info .nui-badge__dot {
      @apply bg-white;
    }

    /* ═══════════════════════════════════════════════════════════════════════════
       SOFT VARIANTS (subtle background)
       ═══════════════════════════════════════════════════════════════════════════ */
    
    .nui-badge--soft.nui-badge--primary {
      @apply bg-primary-100 text-primary-700;
    }
    .dark .nui-badge--soft.nui-badge--primary {
      @apply bg-primary-950 text-primary-300;
    }
    .nui-badge--soft.nui-badge--primary .nui-badge__dot {
      @apply bg-primary-600;
    }

    .nui-badge--soft.nui-badge--secondary {
      @apply bg-surface-100 text-surface-700;
    }
    .dark .nui-badge--soft.nui-badge--secondary {
      @apply bg-surface-800 text-surface-300;
    }
    .nui-badge--soft.nui-badge--secondary .nui-badge__dot {
      @apply bg-surface-600;
    }

    .nui-badge--soft.nui-badge--success {
      @apply bg-green-100 text-green-700;
    }
    .dark .nui-badge--soft.nui-badge--success {
      @apply bg-green-950 text-green-300;
    }
    .nui-badge--soft.nui-badge--success .nui-badge__dot {
      @apply bg-green-600;
    }

    .nui-badge--soft.nui-badge--warning {
      @apply bg-yellow-100 text-yellow-800;
    }
    .dark .nui-badge--soft.nui-badge--warning {
      @apply bg-yellow-950 text-yellow-300;
    }
    .nui-badge--soft.nui-badge--warning .nui-badge__dot {
      @apply bg-yellow-600;
    }

    .nui-badge--soft.nui-badge--error {
      @apply bg-red-100 text-red-700;
    }
    .dark .nui-badge--soft.nui-badge--error {
      @apply bg-red-950 text-red-300;
    }
    .nui-badge--soft.nui-badge--error .nui-badge__dot {
      @apply bg-red-600;
    }

    .nui-badge--soft.nui-badge--info {
      @apply bg-blue-100 text-blue-700;
    }
    .dark .nui-badge--soft.nui-badge--info {
      @apply bg-blue-950 text-blue-300;
    }
    .nui-badge--soft.nui-badge--info .nui-badge__dot {
      @apply bg-blue-600;
    }

    /* ═══════════════════════════════════════════════════════════════════════════
       OUTLINE VARIANTS
       ═══════════════════════════════════════════════════════════════════════════ */
    
    .nui-badge--outline.nui-badge--primary {
      @apply border border-primary-300 text-primary-700 bg-transparent;
    }
    .dark .nui-badge--outline.nui-badge--primary {
      @apply border-primary-700 text-primary-300;
    }
    .nui-badge--outline.nui-badge--primary .nui-badge__dot {
      @apply bg-primary-600;
    }

    .nui-badge--outline.nui-badge--secondary {
      @apply border border-surface-300 text-surface-700 bg-transparent;
    }
    .dark .nui-badge--outline.nui-badge--secondary {
      @apply border-surface-600 text-surface-300;
    }
    .nui-badge--outline.nui-badge--secondary .nui-badge__dot {
      @apply bg-surface-600;
    }

    .nui-badge--outline.nui-badge--success {
      @apply border border-green-300 text-green-700 bg-transparent;
    }
    .dark .nui-badge--outline.nui-badge--success {
      @apply border-green-700 text-green-300;
    }
    .nui-badge--outline.nui-badge--success .nui-badge__dot {
      @apply bg-green-600;
    }

    .nui-badge--outline.nui-badge--warning {
      @apply border border-yellow-400 text-yellow-700 bg-transparent;
    }
    .dark .nui-badge--outline.nui-badge--warning {
      @apply border-yellow-600 text-yellow-300;
    }
    .nui-badge--outline.nui-badge--warning .nui-badge__dot {
      @apply bg-yellow-600;
    }

    .nui-badge--outline.nui-badge--error {
      @apply border border-red-300 text-red-700 bg-transparent;
    }
    .dark .nui-badge--outline.nui-badge--error {
      @apply border-red-700 text-red-300;
    }
    .nui-badge--outline.nui-badge--error .nui-badge__dot {
      @apply bg-red-600;
    }

    .nui-badge--outline.nui-badge--info {
      @apply border border-blue-300 text-blue-700 bg-transparent;
    }
    .dark .nui-badge--outline.nui-badge--info {
      @apply border-blue-700 text-blue-300;
    }
    .nui-badge--outline.nui-badge--info .nui-badge__dot {
      @apply bg-blue-600;
    }
  </style>
</n:component>
