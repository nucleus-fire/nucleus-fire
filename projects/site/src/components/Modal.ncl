<n:component name="Modal">
  <n:props>
    <n:prop name="id" type="string" required="true" />
    <n:prop name="title" type="string" />
    <n:prop name="size" type="string" default="md" />
    <n:prop name="closable" type="boolean" default="true" />
  </n:props>

  <div id="{{ id }}" class="modal-overlay" onclick="if(event.target === this) this.classList.remove('modal-open')">
    <div class="modal modal-{{ size }}" role="dialog" aria-modal="true" aria-labelledby="{{ id }}-title">
      <div class="modal-header">
        <n:if condition="{{ title }}">
          <h2 id="{{ id }}-title" class="modal-title">{{ title }}</h2>
        </n:if>
        <n:if condition="{{ closable }}">
          <button class="modal-close" onclick="this.closest('.modal-overlay').classList.remove('modal-open')" aria-label="Close modal">
            <svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
              <line x1="18" y1="6" x2="6" y2="18"/>
              <line x1="6" y1="6" x2="18" y2="18"/>
            </svg>
          </button>
        </n:if>
      </div>
      
      <div class="modal-body">
        <n:slot />
      </div>
      
      <div class="modal-footer">
        <n:slot name="footer" />
      </div>
    </div>
  </div>

  <style scoped>
    .modal-overlay {
      position: fixed;
      inset: 0;
      z-index: 999;
      display: none;
      align-items: center;
      justify-content: center;
      padding: 1rem;
      background: rgba(0, 0, 0, 0.7);
      backdrop-filter: blur(4px);
      opacity: 0;
      transition: opacity 0.2s ease;
    }
    
    .modal-overlay.modal-open {
      display: flex;
      opacity: 1;
    }
    
    .modal {
      --modal-bg: #18181b;
      --modal-border: rgba(255, 255, 255, 0.08);
      
      background: var(--modal-bg);
      border: 1px solid var(--modal-border);
      border-radius: 16px;
      box-shadow: 
        0 25px 50px -12px rgba(0, 0, 0, 0.5),
        0 0 0 1px rgba(255, 255, 255, 0.05);
      max-height: 90vh;
      display: flex;
      flex-direction: column;
      animation: modalSlideIn 0.3s cubic-bezier(0.16, 1, 0.3, 1);
      overflow: hidden;
    }
    
    .modal-sm { width: 100%; max-width: 400px; }
    .modal-md { width: 100%; max-width: 560px; }
    .modal-lg { width: 100%; max-width: 720px; }
    .modal-xl { width: 100%; max-width: 900px; }
    .modal-full { width: 95vw; max-width: 1200px; height: 90vh; }
    
    .modal-header {
      display: flex;
      align-items: center;
      justify-content: space-between;
      padding: 1.25rem 1.5rem;
      border-bottom: 1px solid var(--modal-border);
    }
    
    .modal-title {
      margin: 0;
      font-size: 1.25rem;
      font-weight: 600;
      color: #f4f4f5;
      letter-spacing: -0.02em;
    }
    
    .modal-close {
      width: 32px;
      height: 32px;
      display: flex;
      align-items: center;
      justify-content: center;
      background: transparent;
      border: none;
      border-radius: 8px;
      color: #71717a;
      cursor: pointer;
      transition: all 0.2s;
    }
    
    .modal-close:hover {
      background: rgba(255, 255, 255, 0.1);
      color: #f4f4f5;
    }
    
    .modal-close svg {
      width: 18px;
      height: 18px;
    }
    
    .modal-body {
      flex: 1;
      padding: 1.5rem;
      overflow-y: auto;
      color: #a1a1aa;
      line-height: 1.6;
    }
    
    .modal-footer {
      display: flex;
      align-items: center;
      justify-content: flex-end;
      gap: 0.75rem;
      padding: 1rem 1.5rem;
      border-top: 1px solid var(--modal-border);
      background: rgba(0, 0, 0, 0.2);
    }
    
    .modal-footer:empty {
      display: none;
    }
    
    @keyframes modalSlideIn {
      from {
        opacity: 0;
        transform: scale(0.95) translateY(-10px);
      }
      to {
        opacity: 1;
        transform: scale(1) translateY(0);
      }
    }
    
    @media (max-width: 640px) {
      .modal {
        max-height: 85vh;
        margin: auto 0.5rem;
      }
      
      .modal-sm,
      .modal-md,
      .modal-lg {
        max-width: 100%;
      }
    }
  </style>
</n:component>
