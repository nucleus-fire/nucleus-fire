<n:component name="SplitPane">
  <n:props>
    <n:prop name="id" type="string" default="split-pane" />
    <n:prop name="direction" type="string" default="horizontal" />
    <n:prop name="minSize" type="string" default="200px" />
    <n:prop name="defaultSize" type="string" default="50%" />
    <n:prop name="collapsible" type="boolean" default="true" />
  </n:props>

  <div id="{{ id }}" class="split-pane split-{{ direction }}">
    <div class="split-panel split-first" style="flex-basis: {{ defaultSize }}; min-width: {{ minSize }};">
      <n:slot name="first" />
    </div>
    
    <div class="split-divider" role="separator" aria-orientation="{{ direction }}">
      <div class="divider-handle">
        <div class="divider-grip"></div>
      </div>
    </div>
    
    <div class="split-panel split-second">
      <n:slot name="second" />
    </div>
  </div>

  <style scoped>
    .split-pane {
      display: flex;
      width: 100%;
      height: 100%;
      overflow: hidden;
    }
    
    .split-horizontal {
      flex-direction: row;
    }
    
    .split-vertical {
      flex-direction: column;
    }
    
    .split-panel {
      overflow: hidden;
      position: relative;
    }
    
    .split-first {
      flex-shrink: 0;
    }
    
    .split-second {
      flex: 1;
      min-width: 0;
      min-height: 0;
    }
    
    .split-divider {
      flex-shrink: 0;
      position: relative;
      z-index: 10;
      background: rgba(255, 255, 255, 0.06);
      transition: background-color 0.2s;
    }
    
    .split-horizontal .split-divider {
      width: 1px;
      cursor: col-resize;
    }
    
    .split-vertical .split-divider {
      height: 1px;
      cursor: row-resize;
    }
    
    .split-divider:hover,
    .split-divider:active {
      background: rgba(139, 92, 246, 0.5);
    }
    
    .divider-handle {
      position: absolute;
      display: flex;
      align-items: center;
      justify-content: center;
    }
    
    .split-horizontal .divider-handle {
      top: 50%;
      left: 50%;
      transform: translate(-50%, -50%);
      width: 12px;
      height: 40px;
      padding: 0 4px;
    }
    
    .split-vertical .divider-handle {
      top: 50%;
      left: 50%;
      transform: translate(-50%, -50%);
      width: 40px;
      height: 12px;
      padding: 4px 0;
    }
    
    .divider-grip {
      background: rgba(255, 255, 255, 0.15);
      border-radius: 2px;
      opacity: 0;
      transition: opacity 0.2s;
    }
    
    .split-horizontal .divider-grip {
      width: 4px;
      height: 24px;
    }
    
    .split-vertical .divider-grip {
      width: 24px;
      height: 4px;
    }
    
    .split-divider:hover .divider-grip,
    .split-divider:active .divider-grip {
      opacity: 1;
    }
    
    /* Responsive collapse */
    @media (max-width: 1024px) {
      .split-horizontal {
        flex-direction: column;
      }
      
      .split-horizontal .split-first {
        flex-basis: auto !important;
        min-width: 0 !important;
        max-height: 50%;
      }
      
      .split-horizontal .split-divider {
        width: 100%;
        height: 1px;
        cursor: row-resize;
      }
      
      .split-horizontal .divider-handle {
        width: 40px;
        height: 12px;
        padding: 4px 0;
      }
      
      .split-horizontal .divider-grip {
        width: 24px;
        height: 4px;
      }
    }
    
    @media (max-width: 768px) {
      .split-horizontal .split-first,
      .split-vertical .split-first {
        display: none;
      }
      
      .split-divider {
        display: none;
      }
    }
  </style>
</n:component>
