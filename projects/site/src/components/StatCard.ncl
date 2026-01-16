<n:component name="StatCard">
<n:props>
    value: String
    label: String
    highlight: String = "false"
    id: String = ""
</n:props>

<div class="race-stat-card {% if highlight == 'true' %}highlight{% endif %}">
    {% if id %}
    <div class="stat-value" id="{{ id }}">{{ value }}</div>
    {% endif %}
    {% if !id %}
    <div class="stat-value">{{ value }}</div>
    {% endif %}
    <div class="stat-label">{{ label }}</div>
</div>
</n:component>
