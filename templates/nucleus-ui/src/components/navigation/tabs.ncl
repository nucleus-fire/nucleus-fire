<!--
  Nucleus UI - Tabs Component
  
  Tab navigation for switching between views.
  
  Props:
    variant: underline | pills | boxed (default: underline)
    size: sm | md | lg (default: md)
    full_width: boolean - stretch tabs to fill container
-->

<n:component>
  <n:props>
    variant: string = "underline",
    size: string = "md",
    full_width: bool = false
  </n:props>

  <div
    role="tablist"
    class="
      nui-tabs
      nui-tabs--{variant}
      nui-tabs--{size}
      {if full_width}w-full{/if}
      flex
    "
  >
    <n:slot />
  </div>

  <style scoped>
    /* Base */
    .nui-tabs {
      @apply gap-1;
    }

    /* Underline variant */
    .nui-tabs--underline {
      @apply border-b border-surface-200 dark:border-surface-700 gap-0;
    }

    .nui-tabs--underline :global(.nui-tab) {
      @apply border-b-2 border-transparent -mb-px
             text-surface-600 dark:text-surface-400
             hover:text-surface-900 dark:hover:text-surface-100
             hover:border-surface-300 dark:hover:border-surface-600;
    }

    .nui-tabs--underline :global(.nui-tab[aria-selected="true"]) {
      @apply border-primary-600 text-primary-600 
             dark:border-primary-400 dark:text-primary-400;
    }

    /* Pills variant */
    .nui-tabs--pills :global(.nui-tab) {
      @apply rounded-lg
             text-surface-600 dark:text-surface-400
             hover:bg-surface-100 dark:hover:bg-surface-800
             hover:text-surface-900 dark:hover:text-surface-100;
    }

    .nui-tabs--pills :global(.nui-tab[aria-selected="true"]) {
      @apply bg-primary-600 text-white dark:bg-primary-500;
    }

    /* Boxed variant */
    .nui-tabs--boxed {
      @apply bg-surface-100 dark:bg-surface-800 p-1 rounded-lg;
    }

    .nui-tabs--boxed :global(.nui-tab) {
      @apply rounded-md
             text-surface-600 dark:text-surface-400
             hover:text-surface-900 dark:hover:text-surface-100;
    }

    .nui-tabs--boxed :global(.nui-tab[aria-selected="true"]) {
      @apply bg-white dark:bg-surface-900 
             text-surface-900 dark:text-surface-100
             shadow-sm;
    }

    /* Size variants */
    .nui-tabs--sm :global(.nui-tab) {
      @apply px-3 py-1.5 text-sm;
    }

    .nui-tabs--md :global(.nui-tab) {
      @apply px-4 py-2 text-sm;
    }

    .nui-tabs--lg :global(.nui-tab) {
      @apply px-5 py-2.5 text-base;
    }
  </style>
</n:component>
