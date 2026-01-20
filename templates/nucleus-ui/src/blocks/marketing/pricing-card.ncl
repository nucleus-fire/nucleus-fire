<!--
  Nucleus UI - Pricing Card Component
  
  Individual pricing tier card.
  
  Props:
    name: string - tier name
    price: string - price display
    period: string - billing period (default: /month)
    featured: boolean - highlight this tier
    badge: string - optional badge text (e.g., "Most Popular")
-->

<n:component>
  <n:props>
    name: string = "",
    price: string = "",
    period: string = "/month",
    featured: bool = false,
    badge: string = ""
  </n:props>

  <div class="
    nui-pricing-card
    relative
    rounded-2xl
    p-8
    animate-fade-in
    transition-all duration-300
    {if featured}
      bg-primary-600 text-white ring-2 ring-primary-600 scale-105 shadow-xl
    {else}
      bg-white dark:bg-surface-900 ring-1 ring-surface-200 dark:ring-surface-700 hover:ring-primary-200 dark:hover:ring-primary-800 hover:shadow-lg
    {/if}
  ">
    {if badge}
      <div class="absolute -top-4 left-1/2 -translate-x-1/2">
        <span class="
          inline-flex items-center
          rounded-full
          px-4 py-1
          text-xs font-semibold
          {if featured}bg-white text-primary-600{else}bg-primary-600 text-white{/if}
          shadow-lg
        ">
          {badge}
        </span>
      </div>
    {/if}

    <!-- Tier name -->
    <h3 class="
      text-lg font-semibold
      {if featured}text-white{else}text-surface-900 dark:text-white{/if}
    ">
      {name}
    </h3>

    <!-- Price -->
    <div class="mt-4 flex items-baseline gap-1">
      <span class="
        text-4xl font-bold tracking-tight
        {if featured}text-white{else}text-surface-900 dark:text-white{/if}
      ">
        {price}
      </span>
      <span class="
        text-sm
        {if featured}text-primary-100{else}text-surface-500 dark:text-surface-400{/if}
      ">
        {period}
      </span>
    </div>

    <!-- Description -->
    <p class="
      mt-4 text-sm leading-6
      {if featured}text-primary-100{else}text-surface-600 dark:text-surface-400{/if}
    ">
      <n:slot name="description" />
    </p>

    <!-- Features -->
    <ul class="mt-8 space-y-3">
      <n:slot name="features" />
    </ul>

    <!-- CTA -->
    <div class="mt-8">
      <n:slot name="cta" />
    </div>
  </div>
</n:component>
