<n:view title="Data Grid">
    <h1>Data Grid</h1>
    <p>A sophisticated data table powered by SQLite.</p>

    <div class="card" style="padding: 0; overflow: hidden;">
        <div style="padding: 1rem; border-bottom: 1px solid var(--border); display: flex; justify-content: space-between; align-items: center;">
            <input type="text" placeholder="Search users..." style="max-width: 300px">
            <div>
                <button class="btn" style="padding: 0.5rem 1rem; font-size: 0.9rem;">Export CSV</button>
            </div>
        </div>
        
        <table style="width: 100%; border-collapse: collapse;">
            <thead>
                <tr style="background: var(--bg); text-align: left;">
                    <th style="padding: 1rem;">ID</th>
                    <th style="padding: 1rem;">Name</th>
                    <th style="padding: 1rem;">Email</th>
                    <th style="padding: 1rem;">Role</th>
                    <th style="padding: 1rem;">Status</th>
                </tr>
            </thead>
            <tbody>
                <!-- Mock Data Row 1 -->
                <tr style="border-bottom: 1px solid var(--border);">
                    <td style="padding: 1rem; color: var(--text-muted);">#1024</td>
                    <td style="padding: 1rem;"><strong>Alice Freeman</strong></td>
                    <td style="padding: 1rem;">alice@nucleus.dev</td>
                    <td style="padding: 1rem;"><span style="background: rgba(59, 130, 246, 0.2); color: #60a5fa; padding: 2px 8px; border-radius: 4px; font-size: 0.8rem;">Admin</span></td>
                    <td style="padding: 1rem;">Active</td>
                </tr>
                <!-- Mock Data Row 2 -->
                <tr style="border-bottom: 1px solid var(--border);">
                    <td style="padding: 1rem; color: var(--text-muted);">#1025</td>
                    <td style="padding: 1rem;"><strong>Bob Smith</strong></td>
                    <td style="padding: 1rem;">bob@example.com</td>
                    <td style="padding: 1rem;"><span style="background: rgba(148, 163, 184, 0.2); color: #cbd5e1; padding: 2px 8px; border-radius: 4px; font-size: 0.8rem;">User</span></td>
                    <td style="padding: 1rem;">Offline</td>
                </tr>
            </tbody>
        </table>
        
        <div style="padding: 1rem; display: flex; justify-content: space-between; align-items: center; color: var(--text-muted); font-size: 0.9rem;">
            <span>Showing 1-2 of 50 results</span>
            <div style="display: flex; gap: 0.5rem;">
                <button class="btn" disabled style="background: var(--surface); color: var(--text-muted);">Prev</button>
                <button class="btn" style="background: var(--surface);">Next</button>
            </div>
        </div>
    </div>
</n:view>
