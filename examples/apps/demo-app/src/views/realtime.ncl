<n:view title="Real-time">
    <h1>Real-time Streaming</h1>
    <p>Live data updates over WebSockets.</p>

    <div class="card glass">
        <div style="display: flex; justify-content: space-between; align-items: end; margin-bottom: 2rem;">
            <div>
                <div style="color: var(--text-muted); font-size: 0.9rem; text-transform: uppercase; letter-spacing: 1px;">Nucleus Stock (NCL)</div>
                <div style="font-size: 4rem; font-weight: 800; line-height: 1;" id="price">1,245.00</div>
            </div>
            <div style="color: #4ade80; font-size: 1.25rem; font-weight: 600;" id="delta">+2.4%</div>
        </div>
        
        <div style="height: 200px; background: rgba(0,0,0,0.2); border-radius: 8px; position: relative; overflow: hidden;">
            <!-- Mock Chart -->
            <svg viewBox="0 0 100 20" preserveAspectRatio="none" style="width: 100%; height: 100%;">
                <polyline id="chart-line" fill="none" stroke="#3b82f6" stroke-width="0.5" points="0,10 10,12 20,8 30,11 40,9 50,14 60,10 70,12 80,8 90,13 100,10" vector-effect="non-scaling-stroke"/>
                <defs>
                    <linearGradient id="fade" x1="0" x2="0" y1="0" y2="1">
                        <stop offset="0%" stop-color="#3b82f6" stop-opacity="0.2"/>
                        <stop offset="100%" stop-color="#3b82f6" stop-opacity="0"/>
                    </linearGradient>
                </defs>
            </svg>
        </div>
    </div>

    <script>
        // Simulate WebSocket Ticker
        setInterval(() => {
            const priceEl = document.getElementById('price');
            const current = parseFloat(priceEl.innerText.replace(',', ''));
            const change = (Math.random() - 0.5) * 10;
            const next = current + change;
            
            priceEl.innerText = next.toFixed(2).replace(/\B(?=(\d{3})+(?!\d))/g, ",");
            
            const deltaEl = document.getElementById('delta');
            deltaEl.innerText = (change > 0 ? "+" : "") + ((change / current) * 100).toFixed(2) + "%";
            deltaEl.style.color = change > 0 ? "#4ade80" : "#ef4444";
        }, 1000);
    </script>
</n:view>
