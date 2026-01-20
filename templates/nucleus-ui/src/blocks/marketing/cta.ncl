<!--
  Nucleus UI - CTA Section Block
  
  Call-to-action section with headline and buttons.
  
  Props:
    variant: simple | split | gradient (default: simple)
-->

<n:component>
  <n:props>
    variant: string = "simple"
  </n:props>

  <section class="nui-cta nui-cta--{variant} relative overflow-hidden">
    {if variant == "simple"}
      <div class="bg-surface-50 dark:bg-surface-900 py-16 sm:py-24">
        <div class="mx-auto max-w-7xl px-6 lg:px-8">
          <div class="mx-auto max-w-2xl text-center">
            <h2 class="text-3xl font-bold tracking-tight text-surface-900 dark:text-white sm:text-4xl">
              <n:slot name="title" />
            </h2>
            <p class="mt-4 text-lg text-surface-600 dark:text-surface-300">
              <n:slot name="description" />
            </p>
            <div class="mt-8 flex items-center justify-center gap-4 flex-wrap">
              <n:slot name="actions" />
            </div>
          </div>
        </div>
      </div>
    {else if variant == "gradient"}
      <div class="relative isolate bg-gradient-to-r from-primary-600 to-primary-800 py-16 sm:py-24">
        <div class="absolute inset-0 overflow-hidden">
          <div class="absolute -top-1/2 -right-1/4 w-96 h-96 bg-white/10 rounded-full blur-3xl"></div>
          <div class="absolute -bottom-1/2 -left-1/4 w-96 h-96 bg-primary-400/20 rounded-full blur-3xl"></div>
        </div>
        <div class="relative mx-auto max-w-7xl px-6 lg:px-8">
          <div class="mx-auto max-w-2xl text-center">
            <h2 class="text-3xl font-bold tracking-tight text-white sm:text-4xl">
              <n:slot name="title" />
            </h2>
            <p class="mt-4 text-lg text-primary-100">
              <n:slot name="description" />
            </p>
            <div class="mt-8 flex items-center justify-center gap-4 flex-wrap">
              <n:slot name="actions" />
            </div>
          </div>
        </div>
      </div>
    {else}
      <div class="bg-white dark:bg-surface-950 py-16 sm:py-24">
        <div class="mx-auto max-w-7xl px-6 lg:px-8">
          <div class="grid lg:grid-cols-2 gap-12 items-center">
            <div>
              <h2 class="text-3xl font-bold tracking-tight text-surface-900 dark:text-white sm:text-4xl">
                <n:slot name="title" />
              </h2>
              <p class="mt-4 text-lg text-surface-600 dark:text-surface-300">
                <n:slot name="description" />
              </p>
            </div>
            <div class="flex items-center justify-center lg:justify-end gap-4 flex-wrap">
              <n:slot name="actions" />
            </div>
          </div>
        </div>
      </div>
    {/if}
  </section>
</n:component>
