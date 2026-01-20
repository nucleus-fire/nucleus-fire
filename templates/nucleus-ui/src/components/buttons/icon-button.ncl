<!--
  Nucleus UI - Icon Button Component
  
  A compact button for icon-only actions with optional tooltip.
  
  Props:
    variant: primary | secondary | ghost | danger (default: ghost)
    size: xs | sm | md | lg (default: md)
    disabled: boolean (default: false)
    label: string - Accessible label for screen readers (required)
    tooltip: string - Tooltip text on hover
-->

<n:component>
  <n:props>
    variant: string = "ghost",
    size: string = "md",
    disabled: bool = false,
    label: string,
    tooltip: string = ""
  </n:props>

  <button
    type="button"
    disabled="{disabled}"
    aria-label="{label}"
    title="{tooltip || label}"
    class="
      nui-icon-btn
      nui-icon-btn--{variant}
      nui-icon-btn--{size}
      inline-flex items-center justify-center
      rounded-lg
      transition-all duration-200 ease-out
      focus-ring
      btn-press
      disabled:opacity-50 disabled:cursor-not-allowed
    "
  >
    <n:slot />
  </button>

  <style scoped>
    /* ═══════════════════════════════════════════════════════════════════════════
       SIZE VARIANTS
       ═══════════════════════════════════════════════════════════════════════════ */
    
    .nui-icon-btn--xs {
      @apply h-6 w-6;
    }
    
    .nui-icon-btn--sm {
      @apply h-8 w-8;
    }
    
    .nui-icon-btn--md {
      @apply h-10 w-10;
    }
    
    .nui-icon-btn--lg {
      @apply h-12 w-12;
    }

    /* ═══════════════════════════════════════════════════════════════════════════
       COLOR VARIANTS
       ═══════════════════════════════════════════════════════════════════════════ */
    
    .nui-icon-btn--primary {
      @apply bg-primary-600 text-white
             hover:bg-primary-500
             active:bg-primary-700;
    }
    
    .dark .nui-icon-btn--primary {
      @apply bg-primary-500 hover:bg-primary-400;
    }

    .nui-icon-btn--secondary {
      @apply bg-surface-100 text-surface-700
             hover:bg-surface-200
             active:bg-surface-300;
    }
    
    .dark .nui-icon-btn--secondary {
      @apply bg-surface-800 text-surface-300
             hover:bg-surface-700;
    }

    .nui-icon-btn--ghost {
      @apply bg-transparent text-surface-600
             hover:bg-surface-100 hover:text-surface-900
             active:bg-surface-200;
    }
    
    .dark .nui-icon-btn--ghost {
      @apply text-surface-400
             hover:bg-surface-800 hover:text-surface-100;
    }

    .nui-icon-btn--danger {
      @apply bg-transparent text-red-600
             hover:bg-red-50 hover:text-red-700
             active:bg-red-100;
    }
    
    .dark .nui-icon-btn--danger {
      @apply text-red-400
             hover:bg-red-950 hover:text-red-300;
    }
  </style>
</n:component>
