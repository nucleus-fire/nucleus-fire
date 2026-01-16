<n:view title="Island Verification">
    <div style="padding: 2rem;">
        <h1>🏝️ Islands Architecture</h1>
        <p>Below is a counter rendered as an island component.</p>
        
        <n:island src="src/views/components/counter_widget.ncl" client:load />

        <div style="height: 150vh; background: #222; margin: 2rem 0; padding: 1rem; color: #666;">
            Scroll down for lazy island...
        </div>

        <n:island src="src/views/components/counter_widget.ncl" client:visible />
    </div>
</n:view>
