<!--
  Nucleus UI - Tab Component
  
  Individual tab button within Tabs container.
  
  Props:
    active: boolean - currently selected
    disabled: boolean
    icon: boolean - has icon (pass via slot)
-->

<n:component>
  <n:props>
    active: bool = false,
    disabled: bool = false,
    icon: bool = false
  </n:props>

  <button
    type="button"
    role="tab"
    aria-selected="{active}"
    disabled="{disabled}"
    class="
      nui-tab
      inline-flex items-center justify-center gap-2
      font-medium
      whitespace-nowrap
      transition-all duration-200 ease-out
      focus:outline-none focus-visible:ring-2 focus-visible:ring-primary-500 focus-visible:ring-offset-2 dark:focus-visible:ring-offset-surface-900
      disabled:opacity-50 disabled:cursor-not-allowed
    "
  >
    <n:slot />
  </button>
</n:component>
