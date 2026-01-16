<n:component name="FormGroup">
  <n:props>
    <n:prop name="legend" type="string" />
    <n:prop name="columns" type="number" default="1" />
    <n:prop name="gap" type="string" default="1rem" />
  </n:props>

  <fieldset class="form-group columns-{{ columns }}">
    <n:if condition="{{ legend }}">
      <legend class="form-group-legend">{{ legend }}</legend>
    </n:if>
    
    <div class="form-group-fields" style="gap: {{ gap }}">
      <n:slot />
    </div>
  </fieldset>

  <style scoped>
    .form-group {
      border: none;
      padding: 0;
      margin: 0 0 1.5rem 0;
    }
    
    .form-group-legend {
      font-size: 1.125rem;
      font-weight: 600;
      color: var(--text-primary, #1a1a2e);
      margin-bottom: 1rem;
      padding: 0;
    }
    
    .form-group-fields {
      display: grid;
      gap: 1rem;
    }
    
    .columns-1 .form-group-fields {
      grid-template-columns: 1fr;
    }
    
    .columns-2 .form-group-fields {
      grid-template-columns: repeat(2, 1fr);
    }
    
    .columns-3 .form-group-fields {
      grid-template-columns: repeat(3, 1fr);
    }
    
    .columns-4 .form-group-fields {
      grid-template-columns: repeat(4, 1fr);
    }
    
    @media (max-width: 640px) {
      .columns-2 .form-group-fields,
      .columns-3 .form-group-fields,
      .columns-4 .form-group-fields {
        grid-template-columns: 1fr;
      }
    }
  </style>
</n:component>
