// Nucleus Home Page JavaScript
// Page transitions, enhanced race simulation, copy command, etc.

document.addEventListener('DOMContentLoaded', function() {
    // ═══ Page Transitions ═══
    document.body.classList.add('page-wrapper');
    
    // Add scroll progress indicator
    const scrollIndicator = document.createElement('div');
    scrollIndicator.className = 'scroll-indicator';
    document.body.appendChild(scrollIndicator);
    
    window.addEventListener('scroll', function() {
        const scrollTop = window.scrollY;
        const docHeight = document.documentElement.scrollHeight - window.innerHeight;
        const scrollPercent = (scrollTop / docHeight) * 100;
        scrollIndicator.style.width = scrollPercent + '%';
    });
    
    // Smooth page transitions for internal links
    document.querySelectorAll('a[href^="/"]').forEach(function(link) {
        link.addEventListener('click', function(e) {
            const href = link.getAttribute('href');
            if (href && !href.includes('#')) {
                e.preventDefault();
                document.body.classList.add('page-exit');
                setTimeout(function() {
                    window.location.href = href;
                }, 300);
            }
        });
    });

    // ═══ Copy Install Command ═══
    const copyBtn = document.getElementById('copy-install');
    if (copyBtn) {
        copyBtn.addEventListener('click', function() {
            const code = document.getElementById('install-code').textContent;
            navigator.clipboard.writeText(code);
            copyBtn.querySelector('span').textContent = 'Copied!';
            setTimeout(function() {
                copyBtn.querySelector('span').textContent = 'Copy';
            }, 2000);
        });
    }
    
    // ═══ Tab Switching ═══
    const tabBtns = document.querySelectorAll('.tab-btn');
    tabBtns.forEach(function(btn) {
        btn.addEventListener('click', function() {
            const tab = btn.getAttribute('data-tab');
            
            document.querySelectorAll('.tab-btn').forEach(function(b) {
                b.classList.remove('active');
            });
            btn.classList.add('active');
            
            document.querySelectorAll('.tab-panel').forEach(function(p) {
                p.classList.remove('active');
            });
            document.getElementById('tab-' + tab).classList.add('active');
        });
    });
    
    // ═══ Newsletter Modal ═══
    const modal = document.getElementById('newsletter-modal');
    const openBtn = document.getElementById('open-newsletter');
    const closeBtn = document.querySelector('.modal-close');
    
    if (openBtn && modal) {
        openBtn.addEventListener('click', () => {
             modal.classList.add('active');
             setTimeout(() => document.getElementById('modal-email').focus(), 100);
        });
        
        const close = () => modal.classList.remove('active');
        
        if (closeBtn) closeBtn.addEventListener('click', close);
        
        // Close on click outside
        modal.addEventListener('click', (e) => {
            if (e.target === modal) close();
        });
        
        // Close on Escape
        document.addEventListener('keydown', (e) => {
            if (e.key === 'Escape' && modal.classList.contains('active')) close();
        });
    }

    // ═══ Newsletter Form ═══
    const newsletterForm = document.querySelector('.newsletter-form');
    if (newsletterForm) {
        newsletterForm.addEventListener('submit', async function(e) {
            e.preventDefault();
            const input = newsletterForm.querySelector('input');
            const btn = newsletterForm.querySelector('button');
            const originalContent = btn.innerHTML; // Save SVG/Text
            
            btn.disabled = true;
            input.disabled = true;
            btn.textContent = 'Subscribing...';

            try {
                const res = await fetch('/api/newsletter', {
                    method: 'POST',
                    headers: { 'Content-Type': 'application/json' },
                    body: JSON.stringify({ email: input.value })
                });

                if (res.ok) {
                    btn.textContent = '✓ Subscribed!';
                    btn.style.background = '#10b981';
                    input.value = '';
                    setTimeout(() => {
                        modal.classList.remove('active');
                        // Reset button style
                        setTimeout(() => {
                            btn.innerHTML = originalContent;
                            btn.style.background = '';
                            btn.disabled = false;
                            input.disabled = false;
                        }, 500);
                    }, 1500);
                } else {
                    const err = await res.text();
                    
                    if (res.status === 409) {
                        btn.textContent = 'Joined';
                        alert('You are already subscribed!');
                    } else {
                        btn.textContent = 'Error';
                        alert(err || 'Subscription failed');
                    }
                    
                    // Reset after delay
                    setTimeout(() => {
                        btn.disabled = false;
                        input.disabled = false;
                        btn.innerHTML = originalContent;
                    }, 3000);
                }
            } catch {
                btn.textContent = 'Net Error';
            }
        });
    }
    
    // ═══════════════════════════════════════════════════════════════════════════
    // ENHANCED RACE SIMULATION
    // Particle effects, smooth animations, better visuals
    // ═══════════════════════════════════════════════════════════════════════════
    
    // ═══════════════════════════════════════════════════════════════════════════
    // RENDERING CORE SIMULATION
    // Spinning 'Reactors' with FPS metrics and particle effects
    // ═══════════════════════════════════════════════════════════════════════════
    
    const startRenderBtn = document.getElementById('start-render');
    
    // Performance Tiers (Target FPS and Operations)
    const tiers = {
        nucleus: { 
            targetFps: 60, 
            opsBase: 24000, 
            fpsEl: 'nucleus-fps', 
            opsEl: 'nucleus-ops',
            emitter: 'nucleus-emitter'
        },
        nextjs: { targetFps: 38, opsBase: 8000, fpsEl: 'nextjs-fps', opsEl: 'nextjs-ops' },
        remix: { targetFps: 42, opsBase: 9500, fpsEl: 'remix-fps', opsEl: 'remix-ops' },
        fastapi: { targetFps: 40, opsBase: 9000, fpsEl: 'fastapi-fps', opsEl: 'fastapi-ops' }
    };
    
    let isRendering = false;
    let frameCount = 0;
    
    // Create DOM particles for visual flair
    function emitParticle(emitterId) {
        const emitter = document.getElementById(emitterId);
        if (!emitter) return;
        
        const p = document.createElement('div');
        p.className = 'core-particle';
        // Random trajectory
        const angle = Math.random() * Math.PI * 2;
        const velocity = 50 + Math.random() * 50;
        const tx = Math.cos(angle) * velocity;
        const ty = Math.sin(angle) * velocity;
        
        p.style.setProperty('--tx', `${tx}px`);
        p.style.setProperty('--ty', `${ty}px`);
        p.style.left = '50%';
        p.style.top = '50%';
        
        emitter.appendChild(p);
        setTimeout(() => p.remove(), 800);
    }

    function formatOps(num) {
        return new Intl.NumberFormat('en-US', { notation: "compact", maximumFractionDigits: 1 }).format(num);
    }
    
    function startRendering() {
        if (isRendering) return;
        isRendering = true;
        
        if (startRenderBtn) {
            startRenderBtn.innerHTML = '<span class="loader-spinner"></span> Benchmarking...';
            startRenderBtn.disabled = true;
            startRenderBtn.classList.add('active');
        }
        
        // Add active classes to visual cores
        document.querySelectorAll('.framework-core').forEach(core => {
            core.classList.add('active');
            core.querySelector('.core-status').textContent = 'Running';
            // Start spinning animations by adding CSS class
            core.querySelector('.core-visual').classList.add('spinning');
        });

        // Loop for updating stats
        const startTime = performance.now();
        
        function renderLoop() {
            if (!isRendering) return;
            frameCount++;
            
            const now = performance.now();
            const elapsed = (now - startTime) / 1000;
            
            Object.keys(tiers).forEach(key => {
                const tier = tiers[key];
                
                // Simulate frame drops / fps variance
                const variance = Math.random() * 5;
                const currentFps = Math.min(60, Math.max(0, tier.targetFps - variance + (Math.sin(now / 500) * 2)));
                
                // Ops fluctuate with FPS
                const currentOps = tier.opsBase * (currentFps / 60) * (0.9 + Math.random() * 0.2);
                
                // Update DOM
                const fpsEl = document.getElementById(tier.fpsEl);
                const opsEl = document.getElementById(tier.opsEl);
                
                if (fpsEl) fpsEl.textContent = Math.round(currentFps);
                if (opsEl) opsEl.textContent = formatOps(currentOps);

                // Nucleus Particles
                if (key === 'nucleus' && frameCount % 5 === 0) {
                    emitParticle(tier.emitter);
                }
            });
            
            // Stop after 8 seconds
            if (elapsed > 8) {
                stopRendering();
            } else {
                requestAnimationFrame(renderLoop);
            }
        }
        
        requestAnimationFrame(renderLoop);
    }
    
    function stopRendering() {
        isRendering = false;
        
        // Reset UI
        if (startRenderBtn) {
            startRenderBtn.innerHTML = '<span class="btn-icon">▶</span> Run Again';
            startRenderBtn.disabled = false;
            startRenderBtn.classList.remove('active');
        }
        
        document.querySelectorAll('.framework-core').forEach(core => {
            core.classList.remove('active');
            core.querySelector('.core-status').textContent = 'Completed';
            core.querySelector('.core-visual').classList.remove('spinning');
        });
    }

    if (startRenderBtn) {
        startRenderBtn.addEventListener('click', startRendering);
    }
    
    // Add confetti animation style for rendering core particles
    const style = document.createElement('style');
    style.textContent = `
        @keyframes confettiFall {
            0% { transform: translateY(0) rotate(0deg); opacity: 1; }
            100% { transform: translateY(-100px) rotate(720deg); opacity: 0; }
        }
    `;
    document.head.appendChild(style);
});
