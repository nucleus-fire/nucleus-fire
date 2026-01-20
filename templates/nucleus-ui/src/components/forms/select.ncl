<!--
  Nucleus UI - Select Component
  
  Native select dropdown with custom styling.
  
  Props:
    label: string
    name: string
    value: string
    disabled: boolean
    required: boolean
    error: string
    hint: string
    placeholder: string - first option as placeholder
    size: sm | md | lg (default: md)
-->

<n:component>
  <n:props>
    label: string = "",
    name: string = "",
    value: string = "",
    disabled: bool = false,
    required: bool = false,
    error: string = "",
    hint: string = "",
    placeholder: string = "",
    size: string = "md"
  </n:props>

  <div class="nui-select-group w-full">
    {if label}
      <label 
        for="{name}"
        class="block text-sm font-medium text-surface-700 dark:text-surface-300 mb-1.5"
      >
        {label}
        {if required}<span class="text-red-500 ml-0.5">*</span>{/if}
      </label>
    {/if}

    <div class="relative">
      <select
        id="{name}"
        name="{name}"
        disabled="{disabled}"
        required="{required}"
        aria-invalid="{if error}true{else}false{/if}"
        aria-describedby="{if error}{name}-error{else if hint}{name}-hint{/if}"
        class="
          nui-select
          nui-select--{size}
          {if error}nui-select--error{/if}
          block w-full
          appearance-none
          bg-white dark:bg-surface-900
          border border-surface-300 dark:border-surface-600
          text-surface-900 dark:text-surface-100
          rounded-lg
          pr-10
          transition-all duration-200 ease-out
          focus:outline-none focus:ring-2 focus:ring-primary-500 focus:border-primary-500
          disabled:bg-surface-100 disabled:dark:bg-surface-800 disabled:cursor-not-allowed disabled:opacity-60
        "
      >
        {if placeholder}
          <option value="" disabled selected>{placeholder}</option>
        {/if}
        <n:slot />
      </select>

      <!-- Dropdown arrow -->
      <div class="absolute inset-y-0 right-0 pr-3 flex items-center pointer-events-none text-surface-400 dark:text-surface-500">
        <svg class="h-5 w-5" fill="none" viewBox="0 0 24 24" stroke="currentColor">
          <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M19 9l-7 7-7-7" />
        </svg>
      </div>
    </div>

    {if error}
      <p id="{name}-error" class="mt-1.5 text-sm text-red-600 dark:text-red-400 flex items-center gap-1">
        <svg class="h-4 w-4" fill="none" viewBox="0 0 24 24" stroke="currentColor">
          <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M12 8v4m0 4h.01M21 12a9 9 0 11-18 0 9 9 0 0118 0z" />
        </svg>
        {error}
      </p>
    {else if hint}
      <p id="{name}-hint" class="mt-1.5 text-sm text-surface-500 dark:text-surface-400">
        {hint}
      </p>
    {/if}
  </div>

  <style scoped>
    /* Size variants */
    .nui-select--sm {
      @apply h-8 pl-3 text-sm;
    }
    
    .nui-select--md {
      @apply h-10 pl-4 text-sm;
    }
    
    .nui-select--lg {
      @apply h-12 pl-4 text-base;
    }

    .nui-select--error {
      @apply border-red-500 dark:border-red-500
             focus:ring-red-500 focus:border-red-500;
    }

    .nui-select:not(:disabled):not(:focus):hover {
      @apply border-surface-400 dark:border-surface-500;
    }
  </style>
</n:component>
