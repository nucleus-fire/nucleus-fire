<!--
  Nucleus UI - Progress Bar Component
  
  Visual progress indicator.
  
  Props:
    value: number - current value (0-100)
    max: number - maximum value (default: 100)
    size: xs | sm | md | lg (default: md)
    color: primary | success | warning | error (default: primary)
    label: boolean - show percentage label
    animated: boolean - animate progress bar
    indeterminate: boolean - show indeterminate/loading state
-->

<n:component>
  <n:props>
    value: i32 = 0,
    max: i32 = 100,
    size: string = "md",
    color: string = "primary",
    label: bool = false,
    animated: bool = false,
    indeterminate: bool = false
  </n:props>

  <div class="nui-progress-group w-full">
    {if label}
      <div class="flex justify-between items-center mb-1.5">
        <span class="text-sm font-medium text-surface-700 dark:text-surface-300">
          <n:slot name="label" />
        </span>
        <span class="text-sm text-surface-600 dark:text-surface-400 tabular-nums">
          {if indeterminate}Loading...{else}{value}%{/if}
        </span>
      </div>
    {/if}

    <div
      role="progressbar"
      aria-valuenow="{value}"
      aria-valuemin="0"
      aria-valuemax="{max}"
      class="
        nui-progress
        nui-progress--{size}
        w-full
        bg-surface-200 dark:bg-surface-700
        rounded-full
        overflow-hidden
      "
    >
      <div
        class="
          nui-progress__bar
          nui-progress--{color}
          h-full
          rounded-full
          transition-all duration-500 ease-out
          {if animated}animate-pulse-subtle{/if}
          {if indeterminate}nui-progress--indeterminate{/if}
        "
        style="width: {if indeterminate}100%{else}{value}%{/if}"
      ></div>
    </div>
  </div>

  <style scoped>
    /* Size variants */
    .nui-progress--xs {
      @apply h-1;
    }
    
    .nui-progress--sm {
      @apply h-1.5;
    }
    
    .nui-progress--md {
      @apply h-2;
    }
    
    .nui-progress--lg {
      @apply h-3;
    }

    /* Color variants */
    .nui-progress--primary {
      @apply bg-primary-600 dark:bg-primary-500;
    }

    .nui-progress--success {
      @apply bg-green-600 dark:bg-green-500;
    }

    .nui-progress--warning {
      @apply bg-yellow-500;
    }

    .nui-progress--error {
      @apply bg-red-600 dark:bg-red-500;
    }

    /* Indeterminate animation */
    .nui-progress--indeterminate {
      animation: indeterminate 1.5s ease-in-out infinite;
      width: 30% !important;
    }

    @keyframes indeterminate {
      0% {
        transform: translateX(-100%);
      }
      100% {
        transform: translateX(400%);
      }
    }
  </style>
</n:component>
