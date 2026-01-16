<n:component name="Toast">
  <n:props>
    <n:prop name="id" type="string" default="toast" />
    <n:prop name="type" type="string" default="success" />
    <n:prop name="message" type="string" />
    <n:prop name="dismissable" type="boolean" default="true" />
    <n:prop name="duration" type="number" default="5000" />
  </n:props>

  <div id="{{ id }}" class="toast toast-{{ type }}" role="alert" aria-live="polite">
    <div class="toast-icon">
      <n:if condition="{{ type == 'success' }}">
        <svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
          <path d="M22 11.08V12a10 10 0 1 1-5.93-9.14"/>
          <polyline points="22 4 12 14.01 9 11.01"/>
        </svg>
      </n:if>
      <n:if condition="{{ type == 'error' }}">
        <svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
          <circle cx="12" cy="12" r="10"/>
          <line x1="15" y1="9" x2="9" y2="15"/>
          <line x1="9" y1="9" x2="15" y2="15"/>
        </svg>
      </n:if>
      <n:if condition="{{ type == 'info' }}">
        <svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
          <circle cx="12" cy="12" r="10"/>
          <line x1="12" y1="16" x2="12" y2="12"/>
          <line x1="12" y1="8" x2="12.01" y2="8"/>
        </svg>
      </n:if>
      <n:if condition="{{ type == 'warning' }}">
        <svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
          <path d="M10.29 3.86L1.82 18a2 2 0 0 0 1.71 3h16.94a2 2 0 0 0 1.71-3L13.71 3.86a2 2 0 0 0-3.42 0z"/>
          <line x1="12" y1="9" x2="12" y2="13"/>
          <line x1="12" y1="17" x2="12.01" y2="17"/>
        </svg>
      </n:if>
    </div>
    
    <div class="toast-content">
      <span class="toast-message">{{ message }}</span>
      <n:slot />
    </div>
    
    <n:if condition="{{ dismissable }}">
      <button class="toast-close" onclick="this.parentElement.classList.add('toast-hiding')" aria-label="Dismiss">
        <svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
          <line x1="18" y1="6" x2="6" y2="18"/>
          <line x1="6" y1="6" x2="18" y2="18"/>
        </svg>
      </button>
    </n:if>
    
    <div class="toast-progress"></div>
  </div>

  <style scoped>
    .toast {
      --toast-bg: rgba(24, 24, 27, 0.95);
      --toast-border: rgba(255, 255, 255, 0.08);
      --toast-accent: #8b5cf6;
      
      display: flex;
      align-items: center;
      gap: 0.75rem;
      min-width: 320px;
      max-width: 450px;
      padding: 1rem 1.25rem;
      background: var(--toast-bg);
      backdrop-filter: blur(16px);
      border: 1px solid var(--toast-border);
      border-left: 4px solid var(--toast-accent);
      border-radius: 12px;
      box-shadow: 
        0 20px 25px -5px rgba(0, 0, 0, 0.3),
        0 10px 10px -5px rgba(0, 0, 0, 0.2);
      animation: toastSlideIn 0.4s cubic-bezier(0.16, 1, 0.3, 1);
      position: relative;
      overflow: hidden;
    }
    
    .toast-hiding {
      animation: toastSlideOut 0.3s ease forwards;
    }
    
    .toast-success { --toast-accent: #10b981; }
    .toast-error { --toast-accent: #ef4444; }
    .toast-warning { --toast-accent: #f59e0b; }
    .toast-info { --toast-accent: #06b6d4; }
    
    .toast-icon {
      flex-shrink: 0;
      width: 20px;
      height: 20px;
      color: var(--toast-accent);
    }
    
    .toast-icon svg {
      width: 100%;
      height: 100%;
    }
    
    .toast-content {
      flex: 1;
      min-width: 0;
    }
    
    .toast-message {
      color: #f4f4f5;
      font-size: 0.9rem;
      font-weight: 500;
      line-height: 1.4;
    }
    
    .toast-close {
      flex-shrink: 0;
      width: 28px;
      height: 28px;
      display: flex;
      align-items: center;
      justify-content: center;
      background: transparent;
      border: none;
      border-radius: 6px;
      color: #71717a;
      cursor: pointer;
      transition: all 0.2s;
    }
    
    .toast-close:hover {
      background: rgba(255, 255, 255, 0.1);
      color: #f4f4f5;
    }
    
    .toast-close svg {
      width: 16px;
      height: 16px;
    }
    
    .toast-progress {
      position: absolute;
      bottom: 0;
      left: 0;
      height: 3px;
      background: var(--toast-accent);
      opacity: 0.6;
      animation: toastProgress 5s linear forwards;
    }
    
    @keyframes toastSlideIn {
      from {
        opacity: 0;
        transform: translateX(100%) scale(0.95);
      }
      to {
        opacity: 1;
        transform: translateX(0) scale(1);
      }
    }
    
    @keyframes toastSlideOut {
      from {
        opacity: 1;
        transform: translateX(0) scale(1);
      }
      to {
        opacity: 0;
        transform: translateX(20px) scale(0.95);
      }
    }
    
    @keyframes toastProgress {
      from { width: 100%; }
      to { width: 0%; }
    }
  </style>
</n:component>
