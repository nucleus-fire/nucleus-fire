<!--
  Nucleus UI - Textarea Component
  
  Multi-line text input with auto-grow and character count.
  
  Props:
    label: string
    placeholder: string
    value: string
    name: string
    rows: number (default: 4)
    disabled: boolean
    required: boolean
    error: string
    hint: string
    max_length: number - show character counter
    auto_grow: boolean - auto-expand height
-->

<n:component>
  <n:props>
    label: string = "",
    placeholder: string = "",
    value: string = "",
    name: string = "",
    rows: i32 = 4,
    disabled: bool = false,
    required: bool = false,
    error: string = "",
    hint: string = "",
    max_length: i32 = 0,
    auto_grow: bool = false
  </n:props>

  <div class="nui-textarea-group w-full">
    {if label}
      <label 
        for="{name}"
        class="block text-sm font-medium text-surface-700 dark:text-surface-300 mb-1.5"
      >
        {label}
        {if required}<span class="text-red-500 ml-0.5">*</span>{/if}
      </label>
    {/if}

    <textarea
      id="{name}"
      name="{name}"
      rows="{rows}"
      placeholder="{placeholder}"
      disabled="{disabled}"
      required="{required}"
      maxlength="{if max_length > 0}{max_length}{/if}"
      aria-invalid="{if error}true{else}false{/if}"
      aria-describedby="{if error}{name}-error{else if hint}{name}-hint{/if}"
      class="
        nui-textarea
        {if error}nui-textarea--error{/if}
        {if auto_grow}nui-textarea--auto-grow{/if}
        block w-full
        px-4 py-3
        bg-white dark:bg-surface-900
        border border-surface-300 dark:border-surface-600
        text-surface-900 dark:text-surface-100
        placeholder-surface-400 dark:placeholder-surface-500
        rounded-lg
        resize-y
        transition-all duration-200 ease-out
        focus:outline-none focus:ring-2 focus:ring-primary-500 focus:border-primary-500
        disabled:bg-surface-100 disabled:dark:bg-surface-800 disabled:cursor-not-allowed disabled:opacity-60
      "
    >{value}</textarea>

    <div class="mt-1.5 flex justify-between items-center">
      {if error}
        <p id="{name}-error" class="text-sm text-red-600 dark:text-red-400 flex items-center gap-1">
          <svg class="h-4 w-4" fill="none" viewBox="0 0 24 24" stroke="currentColor">
            <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M12 8v4m0 4h.01M21 12a9 9 0 11-18 0 9 9 0 0118 0z" />
          </svg>
          {error}
        </p>
      {else if hint}
        <p id="{name}-hint" class="text-sm text-surface-500 dark:text-surface-400">
          {hint}
        </p>
      {else}
        <span></span>
      {/if}

      {if max_length > 0}
        <span class="text-sm text-surface-500 dark:text-surface-400 tabular-nums">
          <span data-char-count>{value.length}</span> / {max_length}
        </span>
      {/if}
    </div>
  </div>

  <style scoped>
    .nui-textarea {
      min-height: 80px;
    }

    .nui-textarea--error {
      @apply border-red-500 dark:border-red-500
             focus:ring-red-500 focus:border-red-500;
    }

    .nui-textarea:not(:disabled):not(:focus):hover {
      @apply border-surface-400 dark:border-surface-500;
    }

    .nui-textarea--auto-grow {
      overflow-y: hidden;
      resize: none;
    }
  </style>
</n:component>
