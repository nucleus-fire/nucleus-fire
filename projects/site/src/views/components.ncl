<n:view title="Components Showcase | Nucleus Framework">
<n:layout name="layout">

<!-- Navigation -->
<nav class="showcase-nav">
    <n:link href="/" class="showcase-logo">🧩 Components Showcase</n:link>
    <div class="showcase-links">
        <a href="#buttons">Buttons</a>
        <a href="#cards">Cards</a>
        <a href="#badges">Badges</a>
        <n:link href="/docs/components">Docs</n:link>
    </div>
</nav>

<!-- Hero -->
<section class="showcase-hero">
    <h1>Nucleus Components</h1>
    <p>Reusable, type-safe, scoped-styled UI components</p>
</section>

<!-- Buttons -->
<section id="buttons" class="showcase-section">
    <h2>Button Component</h2>
    <p class="section-desc">Buttons with variants, sizes, and optional href for links</p>
    
    <div class="component-row">
        <Button variant="primary">Primary</Button>
        <Button variant="secondary">Secondary</Button>
        <Button variant="gradient">Gradient</Button>
    </div>
    
    <div class="component-row">
        <Button size="sm" variant="primary">Small</Button>
        <Button size="md" variant="primary">Medium</Button>
        <Button size="lg" variant="primary">Large</Button>
    </div>
    
    <div class="component-row">
        <Button href="/docs" variant="primary">Link Button →</Button>
    </div>
    
    <div class="code-preview">
&lt;Button variant="primary" size="md"&gt;Click Me&lt;/Button&gt;
&lt;Button href="/docs" variant="gradient"&gt;Learn More →&lt;/Button&gt;
    </div>
</section>

<!-- Cards -->
<section id="cards" class="showcase-section">
    <h2>Card Component</h2>
    <p class="section-desc">Container cards with variants and glass effect</p>
    
    <div class="component-row">
        <Card variant="default">
            <h3>Default Card</h3>
            <p>Basic container card</p>
        </Card>
        
        <Card variant="feature">
            <h3>Feature Card</h3>
            <p>Highlighted for features</p>
        </Card>
        
        <Card variant="highlight" glass="true">
            <h3>Glass Card</h3>
            <p>Glassmorphism effect</p>
        </Card>
    </div>
    
    <div class="code-preview">
&lt;Card variant="highlight" glass="true"&gt;
  &lt;h3&gt;Title&lt;/h3&gt;
  &lt;p&gt;Content&lt;/p&gt;
&lt;/Card&gt;
    </div>
</section>

<!-- Badges -->
<section id="badges" class="showcase-section">
    <h2>Badge Component</h2>
    <p class="section-desc">Labels and tags with variants and icons</p>
    
    <div class="component-row">
        <Badge>Default</Badge>
        <Badge variant="primary">Primary</Badge>
        <Badge variant="primary" icon="⚡">With Icon</Badge>
        <Badge icon="🚀">Rocket</Badge>
    </div>
    
    <div class="code-preview">
&lt;Badge variant="primary" icon="⚡"&gt;New Feature&lt;/Badge&gt;
    </div>
</section>

<!-- FeatureCard -->
<section id="feature-cards" class="showcase-section">
    <h2>FeatureCard Component</h2>
    <p class="section-desc">Feature showcase cards with icon, title, and description</p>
    
    <div class="component-row">
        <FeatureCard 
            icon="⚡" 
            title="Hyper Speed" 
            description="20,000+ RPS with sub-ms latency" 
        />
        <FeatureCard 
            icon="🛡️" 
            title="Type Safe" 
            description="End-to-end type safety from DB to UI" 
        />
        <FeatureCard 
            icon="🔥" 
            title="Hot Reload" 
            description="State-preserving instant updates" 
        />
    </div>
    
    <div class="code-preview">
&lt;FeatureCard 
  icon="⚡" 
  title="Hyper Speed" 
  description="20,000+ RPS with sub-ms latency" 
/&gt;
    </div>
</section>

<!-- StatCard -->
<section id="stat-cards" class="showcase-section">
    <h2>StatCard Component</h2>
    <p class="section-desc">Metrics and statistics display</p>
    
    <div class="component-row">
        <StatCard value="20,000+" label="Requests/sec" />
        <StatCard value="5x" label="Faster" highlight="true" />
        <StatCard value="&lt;1ms" label="p99 Latency" />
        <StatCard value="15MB" label="Memory" />
    </div>
    
    <div class="code-preview">
&lt;StatCard value="5x" label="Faster" highlight="true" /&gt;
    </div>
</section>

<!-- Footer -->
<footer class="showcase-footer">
    <p>Nucleus Component System — Props, Slots, Scoped CSS</p>
</footer>

</n:layout>
</n:view>
