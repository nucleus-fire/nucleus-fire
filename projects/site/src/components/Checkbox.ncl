<n:component name="Checkbox">
  <n:props>
    <n:prop name="name" type="string" required="true" />
    <n:prop name="label" type="string" required="true" />
    <n:prop name="checked" type="boolean" default="false" />
    <n:prop name="disabled" type="boolean" default="false" />
    <n:prop name="error" type="string" />
    <n:prop name="variant" type="string" default="default" />
  </n:props>

  <div class="checkbox-wrapper variant-{{ variant }} {{ error ? 'has-error' : '' }}">
    <label class="checkbox-label">
      <input 
        type="checkbox"
        id="{{ name }}"
        name="{{ name }}"
        class="checkbox-input"
        value="1"
        aria-describedby="{{ name }}-error"
        {{ checked ? 'checked' : '' }}
        {{ disabled ? 'disabled' : '' }}
      />
      <span class="checkbox-custom">
        <svg class="check-icon" viewBox="0 0 12 12" fill="none">
          <path d="M2.5 6L5 8.5L9.5 3.5" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"/>
        </svg>
      </span>
      <span class="checkbox-text">{{ label }}</span>
    </label>
    
    <n:if condition="{{ error }}">
      <div id="{{ name }}-error" class="checkbox-error" role="alert">{{ error }}</div>
    </n:if>
  </div>

  <style scoped>
    .checkbox-wrapper {
      display: flex;
      flex-direction: column;
      gap: 0.25rem;
    }
    
    .checkbox-label {
      display: flex;
      align-items: center;
      gap: 0.75rem;
      cursor: pointer;
      user-select: none;
    }
    
    .checkbox-input {
      position: absolute;
      opacity: 0;
      width: 0;
      height: 0;
    }
    
    .checkbox-custom {
      width: 1.25rem;
      height: 1.25rem;
      border: 2px solid var(--border, #e2e8f0);
      border-radius: 0.25rem;
      background: var(--bg-input, #fff);
      display: flex;
      align-items: center;
      justify-content: center;
      transition: all 0.2s;
      flex-shrink: 0;
    }
    
    .check-icon {
      width: 0.75rem;
      height: 0.75rem;
      color: white;
      opacity: 0;
      transform: scale(0.5);
      transition: all 0.2s;
    }
    
    .checkbox-input:checked + .checkbox-custom {
      background: var(--primary, #6366f1);
      border-color: var(--primary, #6366f1);
    }
    
    .checkbox-input:checked + .checkbox-custom .check-icon {
      opacity: 1;
      transform: scale(1);
    }
    
    .checkbox-input:focus + .checkbox-custom {
      box-shadow: 0 0 0 3px rgba(99, 102, 241, 0.2);
    }
    
    .checkbox-input:disabled + .checkbox-custom {
      background: var(--bg-disabled, #f1f5f9);
      cursor: not-allowed;
    }
    
    .checkbox-input:disabled ~ .checkbox-text {
      color: var(--text-muted, #64748b);
    }
    
    .checkbox-text {
      font-size: 0.9375rem;
      color: var(--text-primary, #1a1a2e);
    }
    
    /* Toggle variant */
    .variant-toggle .checkbox-custom {
      width: 2.5rem;
      height: 1.5rem;
      border-radius: 1rem;
      padding: 0.125rem;
      justify-content: flex-start;
    }
    
    .variant-toggle .check-icon {
      display: none;
    }
    
    .variant-toggle .checkbox-custom::after {
      content: '';
      width: 1.125rem;
      height: 1.125rem;
      background: var(--text-muted, #94a3b8);
      border-radius: 50%;
      transition: all 0.2s;
    }
    
    .variant-toggle .checkbox-input:checked + .checkbox-custom::after {
      transform: translateX(1rem);
      background: white;
    }
    
    .has-error .checkbox-custom {
      border-color: var(--error, #ef4444);
    }
    
    .checkbox-error {
      font-size: 0.75rem;
      color: var(--error, #ef4444);
      margin-left: 2rem;
    }
  </style>
</n:component>
