<!--
  Nucleus UI - Icon Component
  
  SVG icon wrapper with consistent sizing and colors.
  Uses a simple icon system - icons are passed as slot content.
  
  Props:
    size: xs | sm | md | lg | xl (default: md)
    color: current | primary | secondary | success | warning | error (default: current)
    label: string - accessible label for decorative icons
-->

<n:component>
  <n:props>
    size: string = "md",
    color: string = "current",
    label: string = ""
  </n:props>

  <span
    class="nui-icon nui-icon--{size} nui-icon--{color} inline-flex shrink-0"
    role="{if label}img{else}presentation{/if}"
    aria-label="{label}"
    aria-hidden="{if label}false{else}true{/if}"
  >
    <n:slot />
  </span>

  <style scoped>
    .nui-icon {
      @apply items-center justify-center;
    }

    .nui-icon > :global(svg) {
      @apply h-full w-full;
    }

    /* ═══════════════════════════════════════════════════════════════════════════
       SIZE VARIANTS
       ═══════════════════════════════════════════════════════════════════════════ */
    
    .nui-icon--xs {
      @apply h-3 w-3;
    }
    
    .nui-icon--sm {
      @apply h-4 w-4;
    }
    
    .nui-icon--md {
      @apply h-5 w-5;
    }
    
    .nui-icon--lg {
      @apply h-6 w-6;
    }
    
    .nui-icon--xl {
      @apply h-8 w-8;
    }

    /* ═══════════════════════════════════════════════════════════════════════════
       COLOR VARIANTS
       ═══════════════════════════════════════════════════════════════════════════ */
    
    .nui-icon--current {
      @apply text-current;
    }

    .nui-icon--primary {
      @apply text-primary-600 dark:text-primary-400;
    }

    .nui-icon--secondary {
      @apply text-surface-600 dark:text-surface-400;
    }

    .nui-icon--success {
      @apply text-green-600 dark:text-green-400;
    }

    .nui-icon--warning {
      @apply text-yellow-600 dark:text-yellow-400;
    }

    .nui-icon--error {
      @apply text-red-600 dark:text-red-400;
    }
  </style>
</n:component>
