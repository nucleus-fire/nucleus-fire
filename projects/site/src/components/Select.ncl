<n:component name="Select">
  <n:props>
    <n:prop name="name" type="string" required="true" />
    <n:prop name="label" type="string" />
    <n:prop name="placeholder" type="string" default="Select an option..." />
    <n:prop name="options" type="array" required="true" />
    <n:prop name="required" type="boolean" default="false" />
    <n:prop name="disabled" type="boolean" default="false" />
    <n:prop name="error" type="string" />
    <n:prop name="searchable" type="boolean" default="false" />
    <n:prop name="multiple" type="boolean" default="false" />
  </n:props>

  <div class="select-wrapper {{ error ? 'has-error' : '' }}">
    <n:if condition="{{ label }}">
      <label for="{{ name }}" class="select-label">
        {{ label }}
        <n:if condition="{{ required }}">
          <span class="required-mark" aria-hidden="true">*</span>
        </n:if>
      </label>
    </n:if>
    
    <div class="select-container">
      <select 
        id="{{ name }}"
        name="{{ name }}"
        class="select-input"
        aria-describedby="{{ name }}-error"
        aria-invalid="{{ error ? 'true' : 'false' }}"
        {{ required ? 'required' : '' }}
        {{ disabled ? 'disabled' : '' }}
        {{ multiple ? 'multiple' : '' }}
      >
        <n:if condition="{{ placeholder }}">
          <option value="" disabled selected>{{ placeholder }}</option>
        </n:if>
        <n:slot />
      </select>
      
      <span class="select-arrow">
        <svg width="12" height="12" viewBox="0 0 12 12" fill="none">
          <path d="M3 4.5L6 7.5L9 4.5" stroke="currentColor" stroke-width="1.5" stroke-linecap="round" stroke-linejoin="round"/>
        </svg>
      </span>
    </div>
    
    <n:if condition="{{ error }}">
      <div id="{{ name }}-error" class="select-error" role="alert">{{ error }}</div>
    </n:if>
  </div>

  <style scoped>
    .select-wrapper {
      display: flex;
      flex-direction: column;
      gap: 0.5rem;
      width: 100%;
    }
    
    .select-label {
      font-weight: 500;
      font-size: 0.875rem;
      color: var(--text-primary, #1a1a2e);
    }
    
    .required-mark {
      color: var(--error, #ef4444);
      margin-left: 0.25rem;
    }
    
    .select-container {
      position: relative;
      display: flex;
      align-items: center;
    }
    
    .select-input {
      width: 100%;
      padding: 0.75rem 2.5rem 0.75rem 1rem;
      font-size: 1rem;
      border: 1px solid var(--border, #e2e8f0);
      border-radius: 0.5rem;
      background: var(--bg-input, #fff);
      color: var(--text-primary, #1a1a2e);
      cursor: pointer;
      appearance: none;
      transition: border-color 0.2s, box-shadow 0.2s;
    }
    
    .select-input:focus {
      outline: none;
      border-color: var(--primary, #6366f1);
      box-shadow: 0 0 0 3px rgba(99, 102, 241, 0.1);
    }
    
    .select-input:disabled {
      background: var(--bg-disabled, #f1f5f9);
      cursor: not-allowed;
    }
    
    .select-arrow {
      position: absolute;
      right: 0.75rem;
      color: var(--text-muted, #64748b);
      pointer-events: none;
      transition: transform 0.2s;
    }
    
    .select-input:focus + .select-arrow {
      transform: rotate(180deg);
    }
    
    .has-error .select-input {
      border-color: var(--error, #ef4444);
    }
    
    .select-error {
      font-size: 0.75rem;
      color: var(--error, #ef4444);
    }
  </style>
</n:component>
