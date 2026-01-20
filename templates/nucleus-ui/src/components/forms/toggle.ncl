<!--
  Nucleus UI - Toggle/Switch Component
  
  On/off toggle switch with label.
  
  Props:
    label: string
    description: string
    name: string
    checked: boolean
    disabled: boolean
    size: sm | md | lg (default: md)
-->

<n:component>
  <n:props>
    label: string = "",
    description: string = "",
    name: string = "",
    checked: bool = false,
    disabled: bool = false,
    size: string = "md"
  </n:props>

  <label class="nui-toggle nui-toggle--{size} inline-flex items-center gap-3 cursor-pointer {if disabled}opacity-60 cursor-not-allowed{/if}">
    <div class="relative">
      <input
        type="checkbox"
        name="{name}"
        checked="{checked}"
        disabled="{disabled}"
        role="switch"
        aria-checked="{checked}"
        class="sr-only peer"
      />
      
      <!-- Track -->
      <div class="
        nui-toggle__track
        bg-surface-200 dark:bg-surface-700
        peer-checked:bg-primary-600 dark:peer-checked:bg-primary-500
        rounded-full
        transition-colors duration-200 ease-out
        peer-focus:ring-2 peer-focus:ring-primary-500 peer-focus:ring-offset-2 dark:peer-focus:ring-offset-surface-900
      "></div>
      
      <!-- Thumb -->
      <div class="
        nui-toggle__thumb
        absolute top-0.5 left-0.5
        bg-white
        rounded-full
        shadow-sm
        transform transition-transform duration-200 ease-out
        peer-checked:translate-x-full
      "></div>
    </div>

    {if label || description}
      <div class="flex flex-col">
        {if label}
          <span class="text-surface-900 dark:text-surface-100 font-medium select-none">
            {label}
          </span>
        {/if}
        {if description}
          <span class="text-surface-500 dark:text-surface-400 text-sm">
            {description}
          </span>
        {/if}
      </div>
    {/if}
  </label>

  <style scoped>
    /* Size variants */
    .nui-toggle--sm .nui-toggle__track {
      @apply h-5 w-9;
    }
    .nui-toggle--sm .nui-toggle__thumb {
      @apply h-4 w-4;
    }
    .nui-toggle--sm span:first-child {
      @apply text-sm;
    }
    
    .nui-toggle--md .nui-toggle__track {
      @apply h-6 w-11;
    }
    .nui-toggle--md .nui-toggle__thumb {
      @apply h-5 w-5;
    }
    
    .nui-toggle--lg .nui-toggle__track {
      @apply h-7 w-14;
    }
    .nui-toggle--lg .nui-toggle__thumb {
      @apply h-6 w-6;
    }
    .nui-toggle--lg span:first-child {
      @apply text-base;
    }

    /* Thumb position when checked */
    .nui-toggle--sm .peer:checked ~ .nui-toggle__thumb {
      transform: translateX(100%);
    }
    .nui-toggle--md .peer:checked ~ .nui-toggle__thumb {
      transform: translateX(100%);
    }
    .nui-toggle--lg .peer:checked ~ .nui-toggle__thumb {
      transform: translateX(100%);
    }
  </style>
</n:component>
