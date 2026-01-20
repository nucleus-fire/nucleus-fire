<!--
  Nucleus UI - Pricing Section Block
  
  Pricing cards/tiers showcase.
  
  Props:
    variant: cards | comparison (default: cards)
-->

<n:component>
  <n:props>
    variant: string = "cards"
  </n:props>

  <section class="nui-pricing py-24 sm:py-32 bg-white dark:bg-surface-950">
    <div class="mx-auto max-w-7xl px-6 lg:px-8">
      <!-- Header -->
      <div class="mx-auto max-w-2xl text-center mb-16">
        <n:slot name="header" />
      </div>

      <!-- Pricing Cards -->
      <div class="mx-auto max-w-5xl grid gap-8 lg:grid-cols-3">
        <n:slot />
      </div>
    </div>
  </section>
</n:component>
