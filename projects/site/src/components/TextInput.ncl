<n:component name="TextInput">
  <n:props>
    <n:prop name="name" type="string" required="true" />
    <n:prop name="type" type="string" default="text" />
    <n:prop name="label" type="string" />
    <n:prop name="placeholder" type="string" />
    <n:prop name="required" type="boolean" default="false" />
    <n:prop name="disabled" type="boolean" default="false" />
    <n:prop name="error" type="string" />
    <n:prop name="help" type="string" />
    <n:prop name="icon" type="string" />
    <n:prop name="size" type="string" default="medium" />
    <n:prop name="variant" type="string" default="default" />
  </n:props>

  <div class="text-input-wrapper size-{{ size }} variant-{{ variant }}">
    <n:if condition="{{ label }}">
      <label for="{{ name }}" class="text-input-label">
        {{ label }}
        <n:if condition="{{ required }}">
          <span class="required-mark" aria-hidden="true">*</span>
        </n:if>
      </label>
    </n:if>
    
    <div class="text-input-container">
      <n:if condition="{{ icon }}">
        <span class="input-icon">{{ icon }}</span>
      </n:if>
      
      <input 
        type="{{ type }}"
        id="{{ name }}"
        name="{{ name }}"
        placeholder="{{ placeholder }}"
        class="text-input {{ error ? 'has-error' : '' }}"
        aria-describedby="{{ name }}-feedback"
        aria-invalid="{{ error ? 'true' : 'false' }}"
        {{ required ? 'required' : '' }}
        {{ disabled ? 'disabled' : '' }}
      />
    </div>
    
    <n:if condition="{{ help && !error }}">
      <small id="{{ name }}-feedback" class="input-help">{{ help }}</small>
    </n:if>
    
    <n:if condition="{{ error }}">
      <div id="{{ name }}-feedback" class="input-error" role="alert">{{ error }}</div>
    </n:if>
  </div>

  <style scoped>
    .text-input-wrapper {
      display: flex;
      flex-direction: column;
      gap: 0.5rem;
      width: 100%;
    }
    
    .text-input-label {
      font-weight: 500;
      font-size: 0.875rem;
      color: var(--text-primary, #1a1a2e);
    }
    
    .required-mark {
      color: var(--error, #ef4444);
      margin-left: 0.25rem;
    }
    
    .text-input-container {
      position: relative;
      display: flex;
      align-items: center;
    }
    
    .input-icon {
      position: absolute;
      left: 0.75rem;
      color: var(--text-muted, #64748b);
      pointer-events: none;
    }
    
    .text-input {
      width: 100%;
      padding: 0.75rem 1rem;
      font-size: 1rem;
      border: 1px solid var(--border, #e2e8f0);
      border-radius: 0.5rem;
      background: var(--bg-input, #fff);
      color: var(--text-primary, #1a1a2e);
      transition: border-color 0.2s, box-shadow 0.2s;
    }
    
    .text-input:focus {
      outline: none;
      border-color: var(--primary, #6366f1);
      box-shadow: 0 0 0 3px rgba(99, 102, 241, 0.1);
    }
    
    .text-input.has-error {
      border-color: var(--error, #ef4444);
    }
    
    .text-input.has-error:focus {
      box-shadow: 0 0 0 3px rgba(239, 68, 68, 0.1);
    }
    
    .text-input:disabled {
      background: var(--bg-disabled, #f1f5f9);
      cursor: not-allowed;
    }
    
    /* Icon variant */
    .text-input-container:has(.input-icon) .text-input {
      padding-left: 2.5rem;
    }
    
    /* Sizes */
    .size-small .text-input {
      padding: 0.5rem 0.75rem;
      font-size: 0.875rem;
    }
    
    .size-large .text-input {
      padding: 1rem 1.25rem;
      font-size: 1.125rem;
    }
    
    /* Variants */
    .variant-filled .text-input {
      background: var(--bg-secondary, #f1f5f9);
      border-color: transparent;
    }
    
    .variant-underline .text-input {
      border: none;
      border-bottom: 2px solid var(--border, #e2e8f0);
      border-radius: 0;
      padding-left: 0;
      padding-right: 0;
    }
    
    .input-help {
      font-size: 0.75rem;
      color: var(--text-muted, #64748b);
    }
    
    .input-error {
      font-size: 0.75rem;
      color: var(--error, #ef4444);
    }
  </style>
</n:component>
