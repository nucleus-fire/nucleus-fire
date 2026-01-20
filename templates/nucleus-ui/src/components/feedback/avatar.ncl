<!--
  Nucleus UI - Avatar Component
  
  User profile pictures with fallback to initials.
  
  Props:
    src: string - image URL
    alt: string - alt text for image
    name: string - name for initials fallback
    size: xs | sm | md | lg | xl | 2xl (default: md)
    status: online | offline | busy | away | none (default: none)
    shape: circle | square (default: circle)
-->

<n:component>
  <n:props>
    src: string = "",
    alt: string = "",
    name: string = "",
    size: string = "md",
    status: string = "none",
    shape: string = "circle"
  </n:props>

  <div class="nui-avatar nui-avatar--{size} {if shape == 'square'}nui-avatar--square{/if} relative inline-flex">
    {if src}
      <img
        src="{src}"
        alt="{alt || name}"
        class="nui-avatar__image h-full w-full object-cover"
        loading="lazy"
      />
    {else}
      <span class="nui-avatar__initials flex h-full w-full items-center justify-center bg-primary-100 text-primary-700 dark:bg-primary-900 dark:text-primary-300 font-medium uppercase">
        {name[0..2]}
      </span>
    {/if}

    {if status != "none"}
      <span class="nui-avatar__status nui-avatar__status--{status} absolute animate-pulse-subtle"></span>
    {/if}
  </div>

  <style scoped>
    /* ═══════════════════════════════════════════════════════════════════════════
       BASE STYLES
       ═══════════════════════════════════════════════════════════════════════════ */
    
    .nui-avatar {
      @apply rounded-full overflow-hidden bg-surface-200 dark:bg-surface-700;
    }

    .nui-avatar--square {
      @apply rounded-lg;
    }

    .nui-avatar__image {
      @apply rounded-[inherit];
    }

    /* ═══════════════════════════════════════════════════════════════════════════
       SIZE VARIANTS
       ═══════════════════════════════════════════════════════════════════════════ */
    
    .nui-avatar--xs {
      @apply h-6 w-6 text-2xs;
    }
    
    .nui-avatar--sm {
      @apply h-8 w-8 text-xs;
    }
    
    .nui-avatar--md {
      @apply h-10 w-10 text-sm;
    }
    
    .nui-avatar--lg {
      @apply h-12 w-12 text-base;
    }
    
    .nui-avatar--xl {
      @apply h-16 w-16 text-lg;
    }
    
    .nui-avatar--2xl {
      @apply h-20 w-20 text-xl;
    }

    /* ═══════════════════════════════════════════════════════════════════════════
       STATUS INDICATOR
       ═══════════════════════════════════════════════════════════════════════════ */
    
    .nui-avatar__status {
      @apply h-3 w-3 rounded-full border-2 border-white dark:border-surface-900
             bottom-0 right-0;
    }

    .nui-avatar--xs .nui-avatar__status,
    .nui-avatar--sm .nui-avatar__status {
      @apply h-2.5 w-2.5;
    }

    .nui-avatar--xl .nui-avatar__status,
    .nui-avatar--2xl .nui-avatar__status {
      @apply h-4 w-4;
    }

    .nui-avatar__status--online {
      @apply bg-green-500;
    }

    .nui-avatar__status--offline {
      @apply bg-surface-400;
      animation: none;
    }

    .nui-avatar__status--busy {
      @apply bg-red-500;
    }

    .nui-avatar__status--away {
      @apply bg-yellow-500;
    }
  </style>
</n:component>
