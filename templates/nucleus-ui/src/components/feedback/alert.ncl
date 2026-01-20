<!--
  Nucleus UI - Alert Component
  
  Message banner for feedback and notifications.
  
  Props:
    variant: info | success | warning | error (default: info)
    title: string - optional title
    dismissible: boolean - show close button
    icon: boolean - show status icon (default: true)
-->

<n:component>
  <n:props>
    variant: string = "info",
    title: string = "",
    dismissible: bool = false,
    icon: bool = true
  </n:props>

  <div
    role="alert"
    class="
      nui-alert
      nui-alert--{variant}
      relative
      rounded-lg
      p-4
      animate-fade-in
    "
  >
    <div class="flex gap-3">
      {if icon}
        <div class="nui-alert__icon shrink-0">
          {if variant == "success"}
            <svg class="h-5 w-5" fill="none" viewBox="0 0 24 24" stroke="currentColor" stroke-width="2">
              <path stroke-linecap="round" stroke-linejoin="round" d="M9 12l2 2 4-4m6 2a9 9 0 11-18 0 9 9 0 0118 0z" />
            </svg>
          {else if variant == "warning"}
            <svg class="h-5 w-5" fill="none" viewBox="0 0 24 24" stroke="currentColor" stroke-width="2">
              <path stroke-linecap="round" stroke-linejoin="round" d="M12 9v2m0 4h.01m-6.938 4h13.856c1.54 0 2.502-1.667 1.732-3L13.732 4c-.77-1.333-2.694-1.333-3.464 0L3.34 16c-.77 1.333.192 3 1.732 3z" />
            </svg>
          {else if variant == "error"}
            <svg class="h-5 w-5" fill="none" viewBox="0 0 24 24" stroke="currentColor" stroke-width="2">
              <path stroke-linecap="round" stroke-linejoin="round" d="M10 14l2-2m0 0l2-2m-2 2l-2-2m2 2l2 2m7-2a9 9 0 11-18 0 9 9 0 0118 0z" />
            </svg>
          {else}
            <svg class="h-5 w-5" fill="none" viewBox="0 0 24 24" stroke="currentColor" stroke-width="2">
              <path stroke-linecap="round" stroke-linejoin="round" d="M13 16h-1v-4h-1m1-4h.01M21 12a9 9 0 11-18 0 9 9 0 0118 0z" />
            </svg>
          {/if}
        </div>
      {/if}

      <div class="flex-1 min-w-0">
        {if title}
          <h3 class="text-sm font-semibold nui-alert__title">{title}</h3>
        {/if}
        <div class="text-sm nui-alert__message {if title}mt-1{/if}">
          <n:slot />
        </div>
      </div>

      {if dismissible}
        <button
          type="button"
          class="nui-alert__dismiss shrink-0 -mr-1 p-1 rounded hover:bg-black/10 dark:hover:bg-white/10 transition-colors"
          aria-label="Dismiss"
        >
          <svg class="h-4 w-4" fill="none" viewBox="0 0 24 24" stroke="currentColor" stroke-width="2">
            <path stroke-linecap="round" stroke-linejoin="round" d="M6 18L18 6M6 6l12 12" />
          </svg>
        </button>
      {/if}
    </div>
  </div>

  <style scoped>
    /* Info */
    .nui-alert--info {
      @apply bg-blue-50 dark:bg-blue-950 border border-blue-200 dark:border-blue-800;
    }
    .nui-alert--info .nui-alert__icon {
      @apply text-blue-600 dark:text-blue-400;
    }
    .nui-alert--info .nui-alert__title {
      @apply text-blue-800 dark:text-blue-200;
    }
    .nui-alert--info .nui-alert__message {
      @apply text-blue-700 dark:text-blue-300;
    }
    .nui-alert--info .nui-alert__dismiss {
      @apply text-blue-600 dark:text-blue-400;
    }

    /* Success */
    .nui-alert--success {
      @apply bg-green-50 dark:bg-green-950 border border-green-200 dark:border-green-800;
    }
    .nui-alert--success .nui-alert__icon {
      @apply text-green-600 dark:text-green-400;
    }
    .nui-alert--success .nui-alert__title {
      @apply text-green-800 dark:text-green-200;
    }
    .nui-alert--success .nui-alert__message {
      @apply text-green-700 dark:text-green-300;
    }
    .nui-alert--success .nui-alert__dismiss {
      @apply text-green-600 dark:text-green-400;
    }

    /* Warning */
    .nui-alert--warning {
      @apply bg-yellow-50 dark:bg-yellow-950 border border-yellow-200 dark:border-yellow-800;
    }
    .nui-alert--warning .nui-alert__icon {
      @apply text-yellow-600 dark:text-yellow-400;
    }
    .nui-alert--warning .nui-alert__title {
      @apply text-yellow-800 dark:text-yellow-200;
    }
    .nui-alert--warning .nui-alert__message {
      @apply text-yellow-700 dark:text-yellow-300;
    }
    .nui-alert--warning .nui-alert__dismiss {
      @apply text-yellow-600 dark:text-yellow-400;
    }

    /* Error */
    .nui-alert--error {
      @apply bg-red-50 dark:bg-red-950 border border-red-200 dark:border-red-800;
    }
    .nui-alert--error .nui-alert__icon {
      @apply text-red-600 dark:text-red-400;
    }
    .nui-alert--error .nui-alert__title {
      @apply text-red-800 dark:text-red-200;
    }
    .nui-alert--error .nui-alert__message {
      @apply text-red-700 dark:text-red-300;
    }
    .nui-alert--error .nui-alert__dismiss {
      @apply text-red-600 dark:text-red-400;
    }
  </style>
</n:component>
