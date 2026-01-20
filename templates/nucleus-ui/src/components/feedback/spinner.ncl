<!--
  Nucleus UI - Spinner Component
  
  Loading indicator with multiple sizes and colors.
  
  Props:
    size: xs | sm | md | lg | xl (default: md)
    color: primary | white | current (default: primary)
    label: string - accessible loading text (default: "Loading...")
-->

<n:component>
  <n:props>
    size: string = "md",
    color: string = "primary",
    label: string = "Loading..."
  </n:props>

  <div
    role="status"
    aria-label="{label}"
    class="nui-spinner nui-spinner--{size} nui-spinner--{color} inline-flex"
  >
    <svg class="animate-spin" viewBox="0 0 24 24" fill="none" xmlns="http://www.w3.org/2000/svg">
      <circle
        class="opacity-25"
        cx="12"
        cy="12"
        r="10"
        stroke="currentColor"
        stroke-width="4"
      />
      <path
        class="opacity-75"
        fill="currentColor"
        d="M4 12a8 8 0 018-8V0C5.373 0 0 5.373 0 12h4zm2 5.291A7.962 7.962 0 014 12H0c0 3.042 1.135 5.824 3 7.938l3-2.647z"
      />
    </svg>
    <span class="sr-only">{label}</span>
  </div>

  <style scoped>
    /* ═══════════════════════════════════════════════════════════════════════════
       SIZE VARIANTS
       ═══════════════════════════════════════════════════════════════════════════ */
    
    .nui-spinner--xs svg {
      @apply h-3 w-3;
    }
    
    .nui-spinner--sm svg {
      @apply h-4 w-4;
    }
    
    .nui-spinner--md svg {
      @apply h-6 w-6;
    }
    
    .nui-spinner--lg svg {
      @apply h-8 w-8;
    }
    
    .nui-spinner--xl svg {
      @apply h-12 w-12;
    }

    /* ═══════════════════════════════════════════════════════════════════════════
       COLOR VARIANTS
       ═══════════════════════════════════════════════════════════════════════════ */
    
    .nui-spinner--primary {
      @apply text-primary-600;
    }
    
    .dark .nui-spinner--primary {
      @apply text-primary-400;
    }

    .nui-spinner--white {
      @apply text-white;
    }

    .nui-spinner--current {
      @apply text-current;
    }
  </style>
</n:component>
