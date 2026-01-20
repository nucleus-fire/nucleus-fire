<!--
  Nucleus UI - Radio Component
  
  Custom styled radio button with label and description.
  
  Props:
    label: string
    description: string
    name: string - group name
    value: string
    checked: boolean
    disabled: boolean
    size: sm | md | lg (default: md)
-->

<n:component>
  <n:props>
    label: string = "",
    description: string = "",
    name: string = "",
    value: string = "",
    checked: bool = false,
    disabled: bool = false,
    size: string = "md"
  </n:props>

  <label class="nui-radio nui-radio--{size} inline-flex items-start gap-3 cursor-pointer group {if disabled}opacity-60 cursor-not-allowed{/if}">
    <div class="relative flex items-center justify-center shrink-0">
      <input
        type="radio"
        name="{name}"
        value="{value}"
        checked="{checked}"
        disabled="{disabled}"
        class="
          nui-radio__input
          peer
          appearance-none
          bg-white dark:bg-surface-900
          border-2 border-surface-300 dark:border-surface-600
          rounded-full
          transition-all duration-200 ease-out
          checked:border-primary-600 dark:checked:border-primary-500
          hover:border-primary-400 dark:hover:border-primary-500
          focus:outline-none focus:ring-2 focus:ring-primary-500 focus:ring-offset-2 dark:focus:ring-offset-surface-900
          disabled:hover:border-surface-300 disabled:dark:hover:border-surface-600
        "
      />
      
      <!-- Radio dot -->
      <div class="
        absolute pointer-events-none
        rounded-full
        bg-primary-600 dark:bg-primary-500
        scale-0 peer-checked:scale-100
        transition-transform duration-200 ease-out
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
    .nui-radio--sm .nui-radio__input {
      @apply h-4 w-4;
    }
    .nui-radio--sm .nui-radio__input + div {
      @apply h-2 w-2;
    }
    .nui-radio--sm span:first-child {
      @apply text-sm;
    }
    
    .nui-radio--md .nui-radio__input {
      @apply h-5 w-5;
    }
    .nui-radio--md .nui-radio__input + div {
      @apply h-2.5 w-2.5;
    }
    
    .nui-radio--lg .nui-radio__input {
      @apply h-6 w-6;
    }
    .nui-radio--lg .nui-radio__input + div {
      @apply h-3 w-3;
    }
    .nui-radio--lg span:first-child {
      @apply text-base;
    }
  </style>
</n:component>
