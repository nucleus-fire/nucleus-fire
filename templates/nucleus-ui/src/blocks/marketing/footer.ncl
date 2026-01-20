<!--
  Nucleus UI - Footer Block
  
  Site footer with navigation and social links.
  
  Props:
    variant: simple | mega | centered (default: simple)
-->

<n:component>
  <n:props>
    variant: string = "simple"
  </n:props>

  <footer class="nui-footer bg-surface-50 dark:bg-surface-900 border-t border-surface-200 dark:border-surface-800">
    <div class="mx-auto max-w-7xl px-6 py-12 lg:px-8">
      {if variant == "simple"}
        <div class="flex flex-col md:flex-row items-center justify-between gap-6">
          <!-- Logo/Brand -->
          <div class="flex items-center gap-4">
            <n:slot name="logo" />
          </div>

          <!-- Links -->
          <nav class="flex flex-wrap items-center justify-center gap-6">
            <n:slot name="links" />
          </nav>

          <!-- Social -->
          <div class="flex items-center gap-4">
            <n:slot name="social" />
          </div>
        </div>

        <!-- Copyright -->
        <div class="mt-8 border-t border-surface-200 dark:border-surface-800 pt-8 text-center">
          <p class="text-sm text-surface-500 dark:text-surface-400">
            <n:slot name="copyright" />
          </p>
        </div>
      {else if variant == "mega"}
        <div class="grid gap-8 lg:grid-cols-5">
          <!-- Brand column -->
          <div class="lg:col-span-2">
            <n:slot name="logo" />
            <p class="mt-4 text-sm text-surface-600 dark:text-surface-400 max-w-xs">
              <n:slot name="description" />
            </p>
            <div class="mt-6 flex items-center gap-4">
              <n:slot name="social" />
            </div>
          </div>

          <!-- Link columns -->
          <n:slot name="columns" />
        </div>

        <!-- Bottom bar -->
        <div class="mt-12 border-t border-surface-200 dark:border-surface-800 pt-8 flex flex-col md:flex-row items-center justify-between gap-4">
          <p class="text-sm text-surface-500 dark:text-surface-400">
            <n:slot name="copyright" />
          </p>
          <div class="flex flex-wrap items-center gap-4 text-sm">
            <n:slot name="legal" />
          </div>
        </div>
      {else}
        <div class="text-center">
          <n:slot name="logo" />
          <nav class="mt-8 flex flex-wrap items-center justify-center gap-6">
            <n:slot name="links" />
          </nav>
          <div class="mt-8 flex items-center justify-center gap-4">
            <n:slot name="social" />
          </div>
          <p class="mt-8 text-sm text-surface-500 dark:text-surface-400">
            <n:slot name="copyright" />
          </p>
        </div>
      {/if}
    </div>
  </footer>

  <style scoped>
    .nui-footer :global(a) {
      @apply text-surface-600 dark:text-surface-400
             hover:text-primary-600 dark:hover:text-primary-400
             transition-colors duration-200;
    }
  </style>
</n:component>
