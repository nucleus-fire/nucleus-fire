<!--
  Nucleus UI - Divider Component
  
  Visual separator for content sections.
  
  Props:
    orientation: horizontal | vertical (default: horizontal)
    label: string - optional text label in divider
    variant: solid | dashed | dotted (default: solid)
-->

<n:component>
  <n:props>
    orientation: string = "horizontal",
    label: string = "",
    variant: string = "solid"
  </n:props>

  {if orientation == "horizontal"}
    <div
      role="separator"
      aria-orientation="horizontal"
      class="nui-divider nui-divider--horizontal nui-divider--{variant} flex items-center w-full"
    >
      {if label}
        <div class="nui-divider__line flex-1"></div>
        <span class="nui-divider__label px-4 text-sm text-surface-500 dark:text-surface-400">{label}</span>
        <div class="nui-divider__line flex-1"></div>
      {else}
        <div class="nui-divider__line flex-1"></div>
      {/if}
    </div>
  {else}
    <div
      role="separator"
      aria-orientation="vertical"
      class="nui-divider nui-divider--vertical nui-divider--{variant} inline-block h-full"
    >
      <div class="nui-divider__line h-full"></div>
    </div>
  {/if}

  <style scoped>
    /* ═══════════════════════════════════════════════════════════════════════════
       BASE STYLES
       ═══════════════════════════════════════════════════════════════════════════ */
    
    .nui-divider__line {
      @apply bg-surface-200 dark:bg-surface-700;
    }

    /* Horizontal */
    .nui-divider--horizontal .nui-divider__line {
      @apply h-px;
    }

    /* Vertical */
    .nui-divider--vertical .nui-divider__line {
      @apply w-px;
    }

    /* ═══════════════════════════════════════════════════════════════════════════
       VARIANT STYLES
       ═══════════════════════════════════════════════════════════════════════════ */
    
    .nui-divider--solid .nui-divider__line {
      @apply bg-surface-200 dark:bg-surface-700;
    }

    .nui-divider--dashed .nui-divider__line {
      @apply bg-transparent;
      background-image: linear-gradient(to right, rgb(var(--color-surface-300)) 50%, transparent 50%);
      background-size: 8px 1px;
    }
    .dark .nui-divider--dashed .nui-divider__line {
      background-image: linear-gradient(to right, rgb(var(--color-surface-600)) 50%, transparent 50%);
    }

    .nui-divider--dotted .nui-divider__line {
      @apply bg-transparent;
      background-image: linear-gradient(to right, rgb(var(--color-surface-300)) 25%, transparent 25%);
      background-size: 4px 1px;
    }
    .dark .nui-divider--dotted .nui-divider__line {
      background-image: linear-gradient(to right, rgb(var(--color-surface-600)) 25%, transparent 25%);
    }
  </style>
</n:component>
