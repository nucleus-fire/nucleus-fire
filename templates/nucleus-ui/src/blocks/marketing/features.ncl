<!--
  Nucleus UI - Feature Section Block
  
  Showcase features with icons, titles, and descriptions.
  
  Props:
    variant: grid | alternating | centered (default: grid)
    columns: 2 | 3 | 4 (default: 3)
-->

<n:component>
  <n:props>
    variant: string = "grid",
    columns: i32 = 3
  </n:props>

  <section class="nui-features py-24 sm:py-32 bg-white dark:bg-surface-950">
    <div class="mx-auto max-w-7xl px-6 lg:px-8">
      <!-- Header -->
      <div class="mx-auto max-w-2xl text-center mb-16">
        <n:slot name="header" />
      </div>

      <!-- Features Grid -->
      {if variant == "grid"}
        <div class="
          mx-auto max-w-5xl
          grid gap-8
          {if columns == 2}sm:grid-cols-2{else if columns == 4}sm:grid-cols-2 lg:grid-cols-4{else}sm:grid-cols-2 lg:grid-cols-3{/if}
        ">
          <n:slot />
        </div>
      {else if variant == "alternating"}
        <div class="space-y-24">
          <n:slot />
        </div>
      {else}
        <div class="mx-auto max-w-3xl space-y-12">
          <n:slot />
        </div>
      {/if}
    </div>
  </section>
</n:component>
