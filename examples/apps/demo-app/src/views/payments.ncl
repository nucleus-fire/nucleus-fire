<n:view title="Payments">
    <h1>Payments</h1>
    <p>Integrated billing with Stripe (Mocked).</p>

    <div class="card glass" style="max-width: 800px">
        <div style="display: grid; grid-template-columns: 1fr 1fr; gap: 2rem;">
            <!-- Plan 1 -->
            <div style="border: 1px solid var(--border); padding: 1.5rem; border-radius: 8px; text-align: center;">
                <h3 style="margin-top: 0;">Starter</h3>
                <div style="font-size: 2.5rem; font-weight: 800; margin: 1rem 0;">$10<span style="font-size: 1rem; color: var(--text-muted);">/mo</span></div>
                <ul style="list-style: none; padding: 0; color: var(--text-muted); line-height: 2; margin-bottom: 2rem;">
                    <li>1 User</li>
                    <li>Basic Support</li>
                    <li>1GB Storage</li>
                </ul>
                <button class="btn" style="width: 100%" onclick="checkout('starter')">Subscribe</button>
            </div>

            <!-- Plan 2 -->
            <div style="border: 1px solid var(--primary); background: rgba(59, 130, 246, 0.1); padding: 1.5rem; border-radius: 8px; text-align: center; position: relative;">
                <div style="position: absolute; top: -10px; left: 50%; transform: translateX(-50%); background: var(--primary); color: white; padding: 2px 10px; border-radius: 12px; font-size: 0.75rem; font-weight: 600;">RECOMMENDED</div>
                <h3 style="margin-top: 0;">Pro</h3>
                <div style="font-size: 2.5rem; font-weight: 800; margin: 1rem 0;">$29<span style="font-size: 1rem; color: var(--text-muted);">/mo</span></div>
                <ul style="list-style: none; padding: 0; color: var(--text-muted); line-height: 2; margin-bottom: 2rem;">
                    <li>5 Users</li>
                    <li>Priority Support</li>
                    <li>100GB Storage</li>
                </ul>
                <button class="btn" style="width: 100%" onclick="checkout('pro')">Subscribe</button>
            </div>
        </div>
    </div>

    <script>
        async function checkout(plan) {
            alert(`Mock: Redirecting to Stripe Checkout for ${plan} plan...`);
            // In real app:
            // const res = await fetch('/api/billing/create-session', { ... });
            // window.location = res.url;
        }
    </script>
</n:view>
