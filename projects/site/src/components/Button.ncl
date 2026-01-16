<n:component name="Button">
<n:props>
    variant: String = "primary"
    size: String = "md"
    href: String = ""
</n:props>

{% if href %}
<a href="{{ href }}" class="btn btn-{{ variant }} btn-{{ size }}">
    <n:slot />
</a>
{% endif %}

{% if !href %}
<button class="btn btn-{{ variant }} btn-{{ size }}">
    <n:slot />
</button>
{% endif %}

<style scoped>
.btn {
    display: inline-flex;
    align-items: center;
    gap: 8px;
    padding: 12px 24px;
    font-weight: 600;
    font-size: 14px;
    border-radius: 8px;
    border: none;
    cursor: pointer;
    transition: all 0.2s ease;
    text-decoration: none;
}

.btn-primary {
    background: linear-gradient(135deg, var(--gradient-start), var(--gradient-end));
    color: #000;
}

.btn-primary:hover {
    transform: translateY(-2px);
    box-shadow: 0 8px 30px rgba(0, 220, 130, 0.4);
}

.btn-secondary {
    background: transparent;
    border: 1px solid var(--border-subtle);
    color: var(--text-secondary);
}

.btn-secondary:hover {
    border-color: var(--border-hover);
    color: var(--text-primary);
}

.btn-gradient {
    background: linear-gradient(135deg, var(--gradient-start), var(--gradient-end));
    color: #000;
}

.btn-sm { padding: 8px 16px; font-size: 12px; }
.btn-md { padding: 12px 24px; font-size: 14px; }
.btn-lg { padding: 16px 32px; font-size: 16px; }
</style>
</n:component>
