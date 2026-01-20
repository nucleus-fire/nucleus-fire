<!--
  Nucleus UI - Button Component
  
  A versatile button component with multiple variants, sizes, and states.
  
  Props:
    variant: primary | secondary | outline | ghost | danger | success (default: primary)
    size: xs | sm | md | lg | xl (default: md)
    disabled: boolean (default: false)
    loading: boolean (default: false)
    icon: string - icon name for left icon
    icon-right: string - icon name for right icon
    full-width: boolean - whether button takes full width
    type: button | submit | reset (default: button)
-->

<n:component>
  <n:props>
    variant: string = "primary",
    size: string = "md",
    disabled: bool = false,
    loading: bool = false,
    icon: string = "",
    icon_right: string = "",
    full_width: bool = false,
    btn_type: string = "button"
  </n:props>

  <button
    type="{btn_type}"
    disabled="{disabled || loading}"
    class="
      nui-btn
      nui-btn--{variant}
      nui-btn--{size}
      {if full_width}w-full{/if}
      inline-flex items-center justify-center gap-2
      font-medium
      transition-all duration-200 ease-out
      focus-ring
      btn-press
      disabled:opacity-50 disabled:cursor-not-allowed disabled:pointer-events-none
    "
  >
    <!-- Loading Spinner -->
    {if loading}
      <svg class="animate-spin -ml-1 h-4 w-4" xmlns="http://www.w3.org/2000/svg" fill="none" viewBox="0 0 24 24">
        <circle class="opacity-25" cx="12" cy="12" r="10" stroke="currentColor" stroke-width="4"></circle>
        <path class="opacity-75" fill="currentColor" d="M4 12a8 8 0 018-8V0C5.373 0 0 5.373 0 12h4zm2 5.291A7.962 7.962 0 014 12H0c0 3.042 1.135 5.824 3 7.938l3-2.647z"></path>
      </svg>
    {else if icon}
      <n:icon name="{icon}" size="sm" />
    {/if}

    <!-- Button Content -->
    <n:slot />

    <!-- Right Icon -->
    {if icon_right && !loading}
      <n:icon name="{icon_right}" size="sm" />
    {/if}
  </button>

  <style scoped>
    /* ═══════════════════════════════════════════════════════════════════════════
       SIZE VARIANTS
       ═══════════════════════════════════════════════════════════════════════════ */
    
    .nui-btn--xs {
      @apply h-7 px-2.5 text-xs rounded-md;
    }
    
    .nui-btn--sm {
      @apply h-8 px-3 text-sm rounded-md;
    }
    
    .nui-btn--md {
      @apply h-10 px-4 text-sm rounded-lg;
    }
    
    .nui-btn--lg {
      @apply h-12 px-6 text-base rounded-lg;
    }
    
    .nui-btn--xl {
      @apply h-14 px-8 text-lg rounded-xl;
    }

    /* ═══════════════════════════════════════════════════════════════════════════
       COLOR VARIANTS
       ═══════════════════════════════════════════════════════════════════════════ */
    
    /* Primary - Filled */
    .nui-btn--primary {
      @apply bg-primary-600 text-white
             hover:bg-primary-500
             active:bg-primary-700
             shadow-sm hover:shadow-md;
    }
    
    .dark .nui-btn--primary {
      @apply bg-primary-500 hover:bg-primary-400 active:bg-primary-600;
    }

    /* Secondary - Subtle */
    .nui-btn--secondary {
      @apply bg-surface-100 text-surface-900
             hover:bg-surface-200
             active:bg-surface-300;
    }
    
    .dark .nui-btn--secondary {
      @apply bg-surface-800 text-surface-100
             hover:bg-surface-700
             active:bg-surface-600;
    }

    /* Outline */
    .nui-btn--outline {
      @apply bg-transparent text-primary-600
             border-2 border-primary-600
             hover:bg-primary-50 hover:border-primary-500
             active:bg-primary-100;
    }
    
    .dark .nui-btn--outline {
      @apply text-primary-400 border-primary-400
             hover:bg-primary-950 hover:border-primary-300
             active:bg-primary-900;
    }

    /* Ghost - No background */
    .nui-btn--ghost {
      @apply bg-transparent text-surface-700
             hover:bg-surface-100 hover:text-surface-900
             active:bg-surface-200;
    }
    
    .dark .nui-btn--ghost {
      @apply text-surface-300
             hover:bg-surface-800 hover:text-surface-100
             active:bg-surface-700;
    }

    /* Danger */
    .nui-btn--danger {
      @apply bg-red-600 text-white
             hover:bg-red-500
             active:bg-red-700
             shadow-sm hover:shadow-md;
    }
    
    .dark .nui-btn--danger {
      @apply bg-red-500 hover:bg-red-400 active:bg-red-600;
    }

    /* Success */
    .nui-btn--success {
      @apply bg-green-600 text-white
             hover:bg-green-500
             active:bg-green-700
             shadow-sm hover:shadow-md;
    }
    
    .dark .nui-btn--success {
      @apply bg-green-500 hover:bg-green-400 active:bg-green-600;
    }
  </style>
</n:component>
