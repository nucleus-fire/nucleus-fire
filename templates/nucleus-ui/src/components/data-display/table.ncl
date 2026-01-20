<!--
  Nucleus UI - Table Component
  
  Data table with optional striping, hovering, and responsive behavior.
  
  Props:
    striped: boolean - alternate row colors
    hoverable: boolean - highlight row on hover
    bordered: boolean - add cell borders
    compact: boolean - reduce padding
    responsive: boolean - horizontal scroll on mobile (default: true)
-->

<n:component>
  <n:props>
    striped: bool = false,
    hoverable: bool = false,
    bordered: bool = false,
    compact: bool = false,
    responsive: bool = true
  </n:props>

  <div class="{if responsive}overflow-x-auto{/if}">
    <table
      class="
        nui-table
        {if striped}nui-table--striped{/if}
        {if hoverable}nui-table--hoverable{/if}
        {if bordered}nui-table--bordered{/if}
        {if compact}nui-table--compact{/if}
        w-full
        text-left
        text-sm
      "
    >
      <n:slot />
    </table>
  </div>

  <style scoped>
    .nui-table {
      @apply border-separate border-spacing-0;
    }

    .nui-table :global(thead) {
      @apply bg-surface-50 dark:bg-surface-800;
    }

    .nui-table :global(th) {
      @apply px-4 py-3 font-semibold text-surface-700 dark:text-surface-300
             border-b border-surface-200 dark:border-surface-700;
    }

    .nui-table :global(td) {
      @apply px-4 py-3 text-surface-900 dark:text-surface-100
             border-b border-surface-100 dark:border-surface-800;
    }

    /* Compact */
    .nui-table--compact :global(th),
    .nui-table--compact :global(td) {
      @apply px-3 py-2;
    }

    /* Striped */
    .nui-table--striped :global(tbody tr:nth-child(even)) {
      @apply bg-surface-50 dark:bg-surface-800/50;
    }

    /* Hoverable */
    .nui-table--hoverable :global(tbody tr:hover) {
      @apply bg-surface-100 dark:bg-surface-800;
    }

    /* Bordered */
    .nui-table--bordered :global(th),
    .nui-table--bordered :global(td) {
      @apply border border-surface-200 dark:border-surface-700;
    }
  </style>
</n:component>
