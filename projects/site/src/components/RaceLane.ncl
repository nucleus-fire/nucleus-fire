<n:component name="RaceLane">
<n:props>
    name: String
    icon: String
    logo: String
    color: String = "#fff"
    progressId: String
    counterId: String
</n:props>

<div class="race-lane {{ name }}-lane">
    <div class="lane-info">
        <span class="lane-logo">{{ logo }}</span>
        <span class="lane-name">{{ name }}</span>
    </div>
    <div class="lane-track">
        <div class="lane-progress" id="{{ progressId }}">
            <div class="lane-car">{{ icon }}</div>
        </div>
        <div class="lane-finish">🏁</div>
    </div>
    <div class="lane-stats">
        <span class="lane-counter" id="{{ counterId }}">0</span>
        <span class="lane-unit">req/s</span>
    </div>
</div>
</n:component>
