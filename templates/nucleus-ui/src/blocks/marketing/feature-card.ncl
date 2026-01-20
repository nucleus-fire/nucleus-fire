<!--
  Nucleus UI - Feature Card Component
  
  Individual feature for the Features section.
  
  Props:
    icon_color: primary | blue | green | purple | orange (default: primary)
-->

<n:component>
  <n:props>
    icon_color: string = "primary"
  </n:props>

  <div class="nui-feature-card group relative animate-fade-in">
    <!-- Icon -->
    <div class="
      nui-feature-card__icon
      h-12 w-12
      flex items-center justify-center
      rounded-xl
      bg-{icon_color}-100 dark:bg-{icon_color}-900/30
      text-{icon_color}-600 dark:text-{icon_color}-400
      mb-4
      transition-transform duration-300
      group-hover:scale-110
    ">
      <n:slot name="icon" />
    </div>

    <!-- Title -->
    <h3 class="text-lg font-semibold text-surface-900 dark:text-white mb-2">
      <n:slot name="title" />
    </h3>

    <!-- Description -->
    <p class="text-surface-600 dark:text-surface-400 leading-relaxed">
      <n:slot name="description" />
    </p>
  </div>

  <style scoped>
    /* Icon color overrides - cannot use dynamic classes in Tailwind */
    .nui-feature-card__icon {
      @apply bg-primary-100 dark:bg-primary-900/30 text-primary-600 dark:text-primary-400;
    }
  </style>
</n:component>
