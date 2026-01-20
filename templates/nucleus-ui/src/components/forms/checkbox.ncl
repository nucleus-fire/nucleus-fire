<!--
  Nucleus UI - Checkbox Component
  
  Custom styled checkbox with label and description.
  
  Props:
    label: string
    description: string - secondary text
    name: string
    value: string
    checked: boolean
    disabled: boolean
    indeterminate: boolean - partial selection state
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
    indeterminate: bool = false,
    size: string = "md"
  </n:props>

  <label class="nui-checkbox nui-checkbox--{size} inline-flex items-start gap-3 cursor-pointer group {if disabled}opacity-60 cursor-not-allowed{/if}">
    <div class="relative flex items-center justify-center shrink-0">
      <input
        type="checkbox"
        name="{name}"
        value="{value}"
        checked="{checked}"
        disabled="{disabled}"
        class="
          nui-checkbox__input
          peer
          appearance-none
          bg-white dark:bg-surface-900
          border-2 border-surface-300 dark:border-surface-600
          rounded
          transition-all duration-200 ease-out
          checked:bg-primary-600 checked:border-primary-600
          checked:dark:bg-primary-500 checked:dark:border-primary-500
          hover:border-primary-400 dark:hover:border-primary-500
          focus:outline-none focus:ring-2 focus:ring-primary-500 focus:ring-offset-2 dark:focus:ring-offset-surface-900
          disabled:hover:border-surface-300 disabled:dark:hover:border-surface-600
        "
      />
      
      <!-- Checkmark -->
      <svg 
        class="absolute pointer-events-none text-white opacity-0 peer-checked:opacity-100 transition-opacity duration-200"
        fill="none" 
        viewBox="0 0 24 24" 
        stroke="currentColor" 
        stroke-width="3"
      >
        {if indeterminate}
          <path stroke-linecap="round" stroke-linejoin="round" d="M5 12h14" />
        {else}
          <path stroke-linecap="round" stroke-linejoin="round" d="M5 13l4 4L19 7" />
        {/if}
      </svg>
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
    .nui-checkbox--sm .nui-checkbox__input {
      @apply h-4 w-4;
    }
    .nui-checkbox--sm svg {
      @apply h-3 w-3;
    }
    .nui-checkbox--sm span:first-child {
      @apply text-sm;
    }
    
    .nui-checkbox--md .nui-checkbox__input {
      @apply h-5 w-5;
    }
    .nui-checkbox--md svg {
      @apply h-3.5 w-3.5;
    }
    
    .nui-checkbox--lg .nui-checkbox__input {
      @apply h-6 w-6;
    }
    .nui-checkbox--lg svg {
      @apply h-4 w-4;
    }
    .nui-checkbox--lg span:first-child {
      @apply text-base;
    }
  </style>
</n:component>
