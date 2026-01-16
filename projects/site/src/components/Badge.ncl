<n:component name="Badge">
<n:props>
    variant: String = "default"
    icon: String = ""
</n:props>

<span class="badge badge-{{ variant }}">
    {% if icon %}<span class="badge-icon">{{ icon }}</span>{% endif %}
    <n:slot />
</span>

<style scoped>
.badge {
    display: inline-flex;
    align-items: center;
    gap: 8px;
    padding: 8px 16px;
    font-size: 12px;
    font-weight: 600;
    border-radius: 100px;
    background: rgba(255, 255, 255, 0.05);
    border: 1px solid rgba(255, 255, 255, 0.1);
}

.badge-primary {
    background: rgba(0, 220, 130, 0.1);
    border-color: rgba(0, 220, 130, 0.3);
    color: var(--accent);
}

.badge-icon {
    font-size: 14px;
}
</style>
</n:component>
