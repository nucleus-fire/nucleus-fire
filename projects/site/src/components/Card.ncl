<n:component name="Card">
<n:props>
    variant: String = "default"
    glass: String = "false"
</n:props>

<div class="card card-{{ variant }} {% if glass == 'true' %}glass{% endif %}">
    <n:slot />
</div>

<style scoped>
.card {
    background: var(--bg-card);
    border: 1px solid var(--border-subtle);
    border-radius: 16px;
    padding: 24px;
    transition: all 0.3s ease;
}

.card:hover {
    border-color: var(--border-hover);
}

.card.glass {
    background: rgba(255, 255, 255, 0.02);
    backdrop-filter: blur(10px);
}

.card-feature {
    text-align: center;
}

.card-highlight {
    border-color: rgba(0, 220, 130, 0.3);
    background: rgba(0, 220, 130, 0.05);
}
</style>
</n:component>
