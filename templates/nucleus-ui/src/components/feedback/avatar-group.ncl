<!--
  Nucleus UI - Avatar Group Component
  
  Stacked avatars for showing multiple users.
  
  Props:
    max: number - maximum avatars to show before +N indicator (default: 4)
    size: xs | sm | md | lg (default: md)
-->

<n:component>
  <n:props>
    max: i32 = 4,
    size: string = "md"
  </n:props>

  <div class="nui-avatar-group nui-avatar-group--{size} flex -space-x-2">
    <n:slot />
    <!-- Overflow indicator handled by parent logic -->
  </div>

  <style scoped>
    .nui-avatar-group > :global(.nui-avatar) {
      @apply ring-2 ring-white dark:ring-surface-900;
    }

    .nui-avatar-group--xs > :global(.nui-avatar) {
      @apply -ml-1.5 first:ml-0;
    }

    .nui-avatar-group--sm > :global(.nui-avatar) {
      @apply -ml-2 first:ml-0;
    }

    .nui-avatar-group--md > :global(.nui-avatar) {
      @apply -ml-2.5 first:ml-0;
    }

    .nui-avatar-group--lg > :global(.nui-avatar) {
      @apply -ml-3 first:ml-0;
    }
  </style>
</n:component>
