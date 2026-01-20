<!--
  Nucleus UI - Card Component
  
  Flexible container for content grouping.
  
  Props:
    variant: default | elevated | bordered | ghost (default: default)
    padding: none | sm | md | lg (default: md)
    hover: boolean - enable hover effect
    clickable: boolean - appear clickable with cursor
-->

<n:component>
  <n:props>
    variant: string = "default",
    padding: string = "md",
    hover: bool = false,
    clickable: bool = false
  </n:props>

  <div
    class="
      nui-card
      nui-card--{variant}
      nui-card--padding-{padding}
      rounded-xl
      overflow-hidden
      {if hover}card-hover{/if}
      {if clickable}cursor-pointer{/if}
    "
  >
    <n:slot />
  </div>

  <style scoped>
    /* Variants */
    .nui-card--default {
      @apply bg-white dark:bg-surface-900 
             border border-surface-200 dark:border-surface-700
             shadow-sm;
    }

    .nui-card--elevated {
      @apply bg-white dark:bg-surface-900 
             shadow-lg;
    }

    .nui-card--bordered {
      @apply bg-transparent
             border-2 border-surface-200 dark:border-surface-700;
    }

    .nui-card--ghost {
      @apply bg-surface-50 dark:bg-surface-800/50;
    }

    /* Padding */
    .nui-card--padding-none {
      @apply p-0;
    }

    .nui-card--padding-sm {
      @apply p-4;
    }

    .nui-card--padding-md {
      @apply p-6;
    }

    .nui-card--padding-lg {
      @apply p-8;
    }
  </style>
</n:component>
