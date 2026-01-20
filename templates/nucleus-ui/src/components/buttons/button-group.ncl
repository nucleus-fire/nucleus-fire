<!--
  Nucleus UI - Button Group Component
  
  Groups multiple buttons together with connected styling.
  
  Props:
    attached: boolean - whether buttons are visually connected (default: true)
    orientation: horizontal | vertical (default: horizontal)
-->

<n:component>
  <n:props>
    attached: bool = true,
    orientation: string = "horizontal"
  </n:props>

  <div
    role="group"
    class="
      nui-btn-group
      {if attached}nui-btn-group--attached{/if}
      {if orientation == 'vertical'}nui-btn-group--vertical{else}nui-btn-group--horizontal{/if}
    "
  >
    <n:slot />
  </div>

  <style scoped>
    .nui-btn-group {
      @apply inline-flex;
    }

    /* Horizontal */
    .nui-btn-group--horizontal {
      @apply flex-row;
    }

    .nui-btn-group--horizontal.nui-btn-group--attached > :global(.nui-btn:not(:first-child):not(:last-child)) {
      @apply rounded-none border-l-0;
    }

    .nui-btn-group--horizontal.nui-btn-group--attached > :global(.nui-btn:first-child) {
      @apply rounded-r-none;
    }

    .nui-btn-group--horizontal.nui-btn-group--attached > :global(.nui-btn:last-child) {
      @apply rounded-l-none border-l-0;
    }

    /* Vertical */
    .nui-btn-group--vertical {
      @apply flex-col;
    }

    .nui-btn-group--vertical.nui-btn-group--attached > :global(.nui-btn:not(:first-child):not(:last-child)) {
      @apply rounded-none border-t-0;
    }

    .nui-btn-group--vertical.nui-btn-group--attached > :global(.nui-btn:first-child) {
      @apply rounded-b-none;
    }

    .nui-btn-group--vertical.nui-btn-group--attached > :global(.nui-btn:last-child) {
      @apply rounded-t-none border-t-0;
    }

    /* Gap for non-attached */
    .nui-btn-group:not(.nui-btn-group--attached).nui-btn-group--horizontal {
      @apply gap-2;
    }

    .nui-btn-group:not(.nui-btn-group--attached).nui-btn-group--vertical {
      @apply gap-2;
    }
  </style>
</n:component>
