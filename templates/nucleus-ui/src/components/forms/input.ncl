<!--
  Nucleus UI - Input Component
  
  Text input with label, error states, and icon support.
  
  Props:
    type: text | email | password | number | search | tel | url (default: text)
    label: string - field label
    placeholder: string
    value: string
    name: string - form field name
    disabled: boolean
    required: boolean
    error: string - error message
    hint: string - helper text
    icon: boolean - has left icon (pass via slot)
    icon_right: boolean - has right icon
    size: sm | md | lg (default: md)
-->

<n:component>
  <n:props>
    input_type: string = "text",
    label: string = "",
    placeholder: string = "",
    value: string = "",
    name: string = "",
    disabled: bool = false,
    required: bool = false,
    error: string = "",
    hint: string = "",
    icon: bool = false,
    icon_right: bool = false,
    size: string = "md"
  </n:props>

  <div class="nui-input-group w-full">
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
      {if icon}
        <div class="absolute inset-y-0 left-0 pl-3 flex items-center pointer-events-none text-surface-400 dark:text-surface-500">
          <n:slot name="icon" />
        </div>
      {/if}

      <input
        type="{input_type}"
        id="{name}"
        name="{name}"
        value="{value}"
        placeholder="{placeholder}"
        disabled="{disabled}"
        required="{required}"
        aria-invalid="{if error}true{else}false{/if}"
        aria-describedby="{if error}{name}-error{else if hint}{name}-hint{/if}"
        class="
          nui-input
          nui-input--{size}
          {if error}nui-input--error{/if}
          {if icon}pl-10{/if}
          {if icon_right}pr-10{/if}
          block w-full
          bg-white dark:bg-surface-900
          border border-surface-300 dark:border-surface-600
          text-surface-900 dark:text-surface-100
          placeholder-surface-400 dark:placeholder-surface-500
          rounded-lg
          transition-all duration-200 ease-out
          focus:outline-none focus:ring-2 focus:ring-primary-500 focus:border-primary-500
          disabled:bg-surface-100 disabled:dark:bg-surface-800 disabled:cursor-not-allowed disabled:opacity-60
        "
      />

      {if icon_right}
        <div class="absolute inset-y-0 right-0 pr-3 flex items-center pointer-events-none text-surface-400 dark:text-surface-500">
          <n:slot name="icon-right" />
        </div>
      {/if}
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
    .nui-input--sm {
      @apply h-8 px-3 text-sm;
    }
    
    .nui-input--md {
      @apply h-10 px-4 text-sm;
    }
    
    .nui-input--lg {
      @apply h-12 px-4 text-base;
    }

    /* Error state */
    .nui-input--error {
      @apply border-red-500 dark:border-red-500
             focus:ring-red-500 focus:border-red-500;
    }

    /* Hover state */
    .nui-input:not(:disabled):not(:focus):hover {
      @apply border-surface-400 dark:border-surface-500;
    }
  </style>
</n:component>
