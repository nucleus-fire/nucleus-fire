<n:component name="FeatureCard">
<n:props>
    icon: String
    title: String
    description: String
</n:props>

<div class="feature-card">
    <div class="feature-icon">{{ icon }}</div>
    <h3 class="feature-title">{{ title }}</h3>
    <p class="feature-desc">{{ description }}</p>
</div>

<style scoped>
.feature-card {
    background: var(--bg-card);
    border: 1px solid var(--border-subtle);
    border-radius: 16px;
    padding: 32px;
    text-align: center;
    transition: all 0.3s ease;
}

.feature-card:hover {
    border-color: var(--border-hover);
    transform: translateY(-4px);
}

.feature-icon {
    font-size: 48px;
    margin-bottom: 16px;
}

.feature-title {
    font-size: 18px;
    font-weight: 700;
    margin-bottom: 8px;
    color: var(--text-primary);
}

.feature-desc {
    font-size: 14px;
    color: var(--text-secondary);
    line-height: 1.6;
}
</style>
</n:component>
