<!--
  Nucleus UI - Breadcrumbs Component
  
  Navigation trail showing current location in hierarchy.
  
  Props:
    separator: string - custom separator character (default: /)
-->

<n:component>
  <n:props>
    separator: string = "/"
  </n:props>

  <nav aria-label="Breadcrumb">
    <ol class="nui-breadcrumbs flex items-center gap-2 text-sm flex-wrap">
      <n:slot />
    </ol>
  </nav>

  <style scoped>
    .nui-breadcrumbs :global(.nui-breadcrumb-item) {
      @apply flex items-center gap-2;
    }

    .nui-breadcrumbs :global(.nui-breadcrumb-item:not(:last-child)::after) {
      content: '/';
      @apply text-surface-400 dark:text-surface-500;
    }

    .nui-breadcrumbs :global(a) {
      @apply text-surface-600 dark:text-surface-400
             hover:text-primary-600 dark:hover:text-primary-400
             transition-colors duration-200;
    }

    .nui-breadcrumbs :global([aria-current="page"]) {
      @apply text-surface-900 dark:text-surface-100 font-medium;
    }
  </style>
</n:component>
