<n:component name="DataTable">
  <n:props>
    <n:prop name="id" type="string" default="data-table" />
    <n:prop name="striped" type="boolean" default="true" />
    <n:prop name="hoverable" type="boolean" default="true" />
    <n:prop name="compact" type="boolean" default="false" />
    <n:prop name="emptyMessage" type="string" default="No data available" />
  </n:props>

  <div class="table-wrapper">
    <div class="table-container">
      <table id="{{ id }}" class="data-table {{ striped ? 'striped' : '' }} {{ hoverable ? 'hoverable' : '' }} {{ compact ? 'compact' : '' }}">
        <n:slot />
      </table>
    </div>
    
    <div class="table-empty">
      <div class="empty-icon">
        <svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="1.5">
          <path d="M20 7l-8-4-8 4m16 0l-8 4m8-4v10l-8 4m0-10L4 7m8 4v10M4 7v10l8 4"/>
        </svg>
      </div>
      <p class="empty-text">{{ emptyMessage }}</p>
    </div>
  </div>

  <style scoped>
    .table-wrapper {
      --table-bg: rgba(255, 255, 255, 0.02);
      --table-border: rgba(255, 255, 255, 0.06);
      --table-header-bg: rgba(0, 0, 0, 0.2);
      --table-hover-bg: rgba(255, 255, 255, 0.04);
      --table-stripe-bg: rgba(255, 255, 255, 0.02);
      
      background: var(--table-bg);
      border: 1px solid var(--table-border);
      border-radius: 12px;
      overflow: hidden;
    }
    
    .table-container {
      overflow-x: auto;
      scrollbar-width: thin;
      scrollbar-color: rgba(255,255,255,0.1) transparent;
    }
    
    .table-container::-webkit-scrollbar {
      height: 6px;
    }
    
    .table-container::-webkit-scrollbar-track {
      background: transparent;
    }
    
    .table-container::-webkit-scrollbar-thumb {
      background: rgba(255,255,255,0.1);
      border-radius: 3px;
    }
    
    .data-table {
      width: 100%;
      border-collapse: collapse;
      font-size: 0.9rem;
    }
    
    .data-table thead {
      background: var(--table-header-bg);
    }
    
    .data-table th {
      padding: 1rem 1.25rem;
      text-align: left;
      font-weight: 600;
      font-size: 0.75rem;
      text-transform: uppercase;
      letter-spacing: 0.05em;
      color: #a1a1aa;
      border-bottom: 1px solid var(--table-border);
      white-space: nowrap;
    }
    
    .data-table td {
      padding: 1rem 1.25rem;
      color: #e4e4e7;
      border-bottom: 1px solid var(--table-border);
      vertical-align: middle;
    }
    
    .data-table tbody tr:last-child td {
      border-bottom: none;
    }
    
    /* Striped rows */
    .data-table.striped tbody tr:nth-child(even) {
      background: var(--table-stripe-bg);
    }
    
    /* Hoverable rows */
    .data-table.hoverable tbody tr {
      transition: background-color 0.15s ease;
    }
    
    .data-table.hoverable tbody tr:hover {
      background: var(--table-hover-bg);
    }
    
    /* Compact variant */
    .data-table.compact th,
    .data-table.compact td {
      padding: 0.65rem 1rem;
    }
    
    /* Empty state */
    .table-empty {
      display: none;
      flex-direction: column;
      align-items: center;
      justify-content: center;
      padding: 3rem 2rem;
      text-align: center;
    }
    
    .table-wrapper:has(.data-table tbody:empty) .table-container {
      display: none;
    }
    
    .table-wrapper:has(.data-table tbody:empty) .table-empty {
      display: flex;
    }
    
    .empty-icon {
      width: 48px;
      height: 48px;
      margin-bottom: 1rem;
      color: #52525b;
    }
    
    .empty-icon svg {
      width: 100%;
      height: 100%;
    }
    
    .empty-text {
      margin: 0;
      color: #71717a;
      font-size: 0.9rem;
    }
    
    /* Responsive */
    @media (max-width: 768px) {
      .data-table th,
      .data-table td {
        padding: 0.75rem 1rem;
      }
      
      .data-table th:first-child,
      .data-table td:first-child {
        position: sticky;
        left: 0;
        background: inherit;
        z-index: 1;
      }
    }
  </style>
</n:component>
