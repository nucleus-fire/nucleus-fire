<!--
  Nucleus UI - Modal Component
  
  Dialog overlay for focused interactions.
  
  Props:
    open: boolean - visibility state
    size: sm | md | lg | xl | full (default: md)
    title: string - modal title
    closable: boolean - show close button (default: true)
    close_on_outside: boolean - close when clicking backdrop (default: true)
    close_on_escape: boolean - close on ESC key (default: true)
-->

<n:component>
  <n:props>
    open: bool = false,
    size: string = "md",
    title: string = "",
    closable: bool = true,
    close_on_outside: bool = true,
    close_on_escape: bool = true
  </n:props>

  {if open}
    <div
      class="nui-modal-backdrop fixed inset-0 z-[var(--z-modal-backdrop)] bg-black/50 backdrop-blur-sm animate-fade-in"
      aria-hidden="true"
    ></div>

    <div
      role="dialog"
      aria-modal="true"
      aria-labelledby="{if title}modal-title{/if}"
      class="nui-modal fixed inset-0 z-[var(--z-modal)] overflow-y-auto"
    >
      <div class="flex min-h-full items-center justify-center p-4">
        <div
          class="
            nui-modal__panel
            nui-modal__panel--{size}
            relative
            w-full
            bg-white dark:bg-surface-900
            rounded-xl
            shadow-2xl
            animate-scale-in
            overflow-hidden
          "
        >
          <!-- Header -->
          {if title || closable}
            <div class="flex items-center justify-between px-6 py-4 border-b border-surface-200 dark:border-surface-700">
              {if title}
                <h2 id="modal-title" class="text-lg font-semibold text-surface-900 dark:text-surface-100">
                  {title}
                </h2>
              {else}
                <span></span>
              {/if}
              
              {if closable}
                <button
                  type="button"
                  class="p-2 -mr-2 rounded-lg text-surface-500 hover:text-surface-700 hover:bg-surface-100 dark:text-surface-400 dark:hover:text-surface-200 dark:hover:bg-surface-800 transition-colors"
                  aria-label="Close"
                >
                  <svg class="h-5 w-5" fill="none" viewBox="0 0 24 24" stroke="currentColor" stroke-width="2">
                    <path stroke-linecap="round" stroke-linejoin="round" d="M6 18L18 6M6 6l12 12" />
                  </svg>
                </button>
              {/if}
            </div>
          {/if}

          <!-- Body -->
          <div class="nui-modal__body px-6 py-4">
            <n:slot />
          </div>

          <!-- Footer (optional slot) -->
          <n:slot name="footer" />
        </div>
      </div>
    </div>
  {/if}

  <style scoped>
    /* Size variants */
    .nui-modal__panel--sm {
      @apply max-w-sm;
    }
    
    .nui-modal__panel--md {
      @apply max-w-lg;
    }
    
    .nui-modal__panel--lg {
      @apply max-w-2xl;
    }
    
    .nui-modal__panel--xl {
      @apply max-w-4xl;
    }
    
    .nui-modal__panel--full {
      @apply max-w-none m-4;
      height: calc(100vh - 2rem);
    }

    /* Prevent body scroll when modal is open */
    :global(body:has(.nui-modal)) {
      overflow: hidden;
    }
  </style>
</n:component>
