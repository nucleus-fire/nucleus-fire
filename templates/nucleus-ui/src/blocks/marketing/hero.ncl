<!--
  Nucleus UI - Hero Section Block
  
  Landing page hero with headline, description, and CTA.
  
  Props:
    variant: centered | split | with-image (default: centered)
    dark: boolean - use dark background
-->

<n:component>
  <n:props>
    variant: string = "centered",
    dark: bool = false
  </n:props>

  <section
    class="
      nui-hero
      nui-hero--{variant}
      relative
      overflow-hidden
      {if dark}bg-surface-900 text-white{else}bg-white dark:bg-surface-950{/if}
    "
  >
    <!-- Background gradient -->
    <div class="absolute inset-0 -z-10">
      <div class="absolute inset-0 bg-gradient-to-b from-primary-50/50 via-transparent to-transparent dark:from-primary-950/30"></div>
      <div class="absolute top-0 left-1/2 -translate-x-1/2 w-[1000px] h-[600px] bg-primary-500/10 rounded-full blur-3xl"></div>
    </div>

    <div class="mx-auto max-w-7xl px-6 py-24 sm:py-32 lg:px-8 lg:py-40">
      {if variant == "centered"}
        <div class="mx-auto max-w-3xl text-center">
          <n:slot name="badge" />
          
          <h1 class="mt-4 text-4xl font-bold tracking-tight text-surface-900 dark:text-white sm:text-5xl lg:text-6xl">
            <n:slot name="title" />
          </h1>
          
          <p class="mt-6 text-lg leading-8 text-surface-600 dark:text-surface-300 sm:text-xl">
            <n:slot name="description" />
          </p>
          
          <div class="mt-10 flex items-center justify-center gap-4 flex-wrap">
            <n:slot name="actions" />
          </div>
        </div>
      {else if variant == "split"}
        <div class="grid lg:grid-cols-2 gap-12 items-center">
          <div>
            <n:slot name="badge" />
            
            <h1 class="mt-4 text-4xl font-bold tracking-tight text-surface-900 dark:text-white sm:text-5xl lg:text-6xl">
              <n:slot name="title" />
            </h1>
            
            <p class="mt-6 text-lg leading-8 text-surface-600 dark:text-surface-300">
              <n:slot name="description" />
            </p>
            
            <div class="mt-10 flex items-center gap-4 flex-wrap">
              <n:slot name="actions" />
            </div>
          </div>
          
          <div class="relative lg:pl-8">
            <n:slot name="media" />
          </div>
        </div>
      {else}
        <div class="mx-auto max-w-3xl text-center">
          <n:slot />
        </div>
      {/if}
    </div>
  </section>

  <style scoped>
    .nui-hero--with-image {
      @apply min-h-[600px] flex items-center;
    }
  </style>
</n:component>
