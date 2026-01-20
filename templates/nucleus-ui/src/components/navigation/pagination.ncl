<!--
  Nucleus UI - Pagination Component
  
  Page navigation for paginated content.
  
  Props:
    current_page: number
    total_pages: number
    show_first_last: boolean - show first/last page buttons (default: true)
    show_prev_next: boolean - show prev/next buttons (default: true)
    max_visible: number - max page buttons to show (default: 5)
-->

<n:component>
  <n:props>
    current_page: i32 = 1,
    total_pages: i32 = 1,
    show_first_last: bool = true,
    show_prev_next: bool = true,
    max_visible: i32 = 5
  </n:props>

  <nav aria-label="Pagination" class="nui-pagination flex items-center gap-1">
    {if show_first_last && current_page > 2}
      <button
        type="button"
        class="nui-pagination__btn"
        aria-label="First page"
      >
        <svg class="h-4 w-4" fill="none" viewBox="0 0 24 24" stroke="currentColor" stroke-width="2">
          <path stroke-linecap="round" stroke-linejoin="round" d="M11 19l-7-7 7-7m8 14l-7-7 7-7" />
        </svg>
      </button>
    {/if}

    {if show_prev_next}
      <button
        type="button"
        class="nui-pagination__btn"
        disabled="{current_page <= 1}"
        aria-label="Previous page"
      >
        <svg class="h-4 w-4" fill="none" viewBox="0 0 24 24" stroke="currentColor" stroke-width="2">
          <path stroke-linecap="round" stroke-linejoin="round" d="M15 19l-7-7 7-7" />
        </svg>
      </button>
    {/if}

    <!-- Page numbers injected via slot -->
    <n:slot />

    {if show_prev_next}
      <button
        type="button"
        class="nui-pagination__btn"
        disabled="{current_page >= total_pages}"
        aria-label="Next page"
      >
        <svg class="h-4 w-4" fill="none" viewBox="0 0 24 24" stroke="currentColor" stroke-width="2">
          <path stroke-linecap="round" stroke-linejoin="round" d="M9 5l7 7-7 7" />
        </svg>
      </button>
    {/if}

    {if show_first_last && current_page < total_pages - 1}
      <button
        type="button"
        class="nui-pagination__btn"
        aria-label="Last page"
      >
        <svg class="h-4 w-4" fill="none" viewBox="0 0 24 24" stroke="currentColor" stroke-width="2">
          <path stroke-linecap="round" stroke-linejoin="round" d="M13 5l7 7-7 7M5 5l7 7-7 7" />
        </svg>
      </button>
    {/if}
  </nav>

  <style scoped>
    .nui-pagination__btn {
      @apply h-9 min-w-9 px-2
             inline-flex items-center justify-center
             rounded-lg
             text-sm font-medium
             text-surface-600 dark:text-surface-400
             bg-white dark:bg-surface-800
             border border-surface-200 dark:border-surface-700
             hover:bg-surface-50 dark:hover:bg-surface-700
             hover:border-surface-300 dark:hover:border-surface-600
             transition-all duration-200
             disabled:opacity-50 disabled:cursor-not-allowed disabled:hover:bg-white dark:disabled:hover:bg-surface-800;
    }

    .nui-pagination :global(.nui-pagination__page) {
      @apply h-9 min-w-9 px-3
             inline-flex items-center justify-center
             rounded-lg
             text-sm font-medium
             text-surface-600 dark:text-surface-400
             hover:bg-surface-100 dark:hover:bg-surface-800
             transition-all duration-200;
    }

    .nui-pagination :global(.nui-pagination__page[aria-current="page"]) {
      @apply bg-primary-600 text-white dark:bg-primary-500
             hover:bg-primary-700 dark:hover:bg-primary-600;
    }

    .nui-pagination :global(.nui-pagination__ellipsis) {
      @apply h-9 min-w-9 
             inline-flex items-center justify-center
             text-surface-400 dark:text-surface-500;
    }
  </style>
</n:component>
