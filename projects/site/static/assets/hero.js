// Hero "Nucleus" Canvas Effect & Interactions
// ====================================================
// Enhanced with Dribbble-inspired premium design:
// - Multi-ring orbits with different speeds
// - Internal energy core glow
// - Floating ambient shapes
// - Dynamic particle constellation

document.addEventListener('DOMContentLoaded', function() {
    // Copy Install Command
    window.copyInstallCmd = function() {
        const cmd = document.getElementById('install-cmd-text');
        if (!cmd) return;
        navigator.clipboard.writeText(cmd.textContent);
        
        const btn = document.querySelector('button[onclick="copyInstallCmd()"]');
        if (btn) {
            const icon = btn.innerHTML;
            btn.innerHTML = '<span class="text-green-400 font-bold">✓</span>';
            setTimeout(() => btn.innerHTML = icon, 2000);
        }
    };

    // Premium Nucleus Canvas Effect
    const canvas = document.getElementById('hero-canvas');
    if (canvas) {
        const ctx = canvas.getContext('2d');
        let width, height, centerX, centerY;
        let particles = [];
        let electrons = [];
        let ambientShapes = [];
        let time = 0;
        
        // Enhanced Configuration
        const PARTICLE_COUNT = 100;
        const ELECTRON_COUNT = 16;
        const NUCLEUS_RADIUS = 70;
        const ORBIT_RADII = [110, 160, 220, 290];
        const CONNECT_DIST = 100;
        const AMBIENT_SHAPE_COUNT = 8;
        
        // Premium Color Palette (Dribbble-inspired)
        const COLORS = {
            primary: { h: 230, s: 80, l: 60 },      // Electric Blue
            secondary: { h: 270, s: 70, l: 55 },    // Proton Purple
            accent: { h: 180, s: 90, l: 60 },       // Electric Cyan
            success: { h: 145, s: 70, l: 55 },      // Neon Green
        };

        let mouse = { x: -1000, y: -1000, active: false };

        canvas.addEventListener('mousemove', (e) => {
            const rect = canvas.getBoundingClientRect();
            mouse.x = e.clientX - rect.left;
            mouse.y = e.clientY - rect.top;
            mouse.active = true;
        });

        canvas.addEventListener('mouseleave', () => {
            mouse.active = false;
        });

        // Ambient Floating Shape Class (Dribbble trend: background depth)
        class AmbientShape {
            constructor() {
                this.reset();
            }
            
            reset() {
                this.x = Math.random() * width;
                this.y = Math.random() * height;
                this.size = 20 + Math.random() * 60;
                this.rotation = Math.random() * Math.PI * 2;
                this.rotationSpeed = (Math.random() - 0.5) * 0.002;
                this.type = Math.floor(Math.random() * 3); // 0: circle, 1: ring, 2: hexagon
                this.opacity = 0.02 + Math.random() * 0.04;
                this.hue = COLORS.primary.h + Math.random() * 60;
                this.driftX = (Math.random() - 0.5) * 0.2;
                this.driftY = (Math.random() - 0.5) * 0.15;
                this.phase = Math.random() * Math.PI * 2;
            }

            update() {
                this.x += this.driftX;
                this.y += this.driftY;
                this.rotation += this.rotationSpeed;
                
                // Wrap around
                if (this.x < -this.size) this.x = width + this.size;
                if (this.x > width + this.size) this.x = -this.size;
                if (this.y < -this.size) this.y = height + this.size;
                if (this.y > height + this.size) this.y = -this.size;
            }

            draw(time) {
                ctx.save();
                ctx.translate(this.x, this.y);
                ctx.rotate(this.rotation);
                
                const pulse = Math.sin(time * 0.001 + this.phase) * 0.3 + 1;
                const size = this.size * pulse;
                
                ctx.strokeStyle = `hsla(${this.hue}, 50%, 50%, ${this.opacity})`;
                ctx.lineWidth = 1;
                
                if (this.type === 0) {
                    // Blurred circle
                    ctx.beginPath();
                    ctx.arc(0, 0, size, 0, Math.PI * 2);
                    ctx.stroke();
                } else if (this.type === 1) {
                    // Double ring
                    ctx.beginPath();
                    ctx.arc(0, 0, size, 0, Math.PI * 2);
                    ctx.stroke();
                    ctx.beginPath();
                    ctx.arc(0, 0, size * 0.6, 0, Math.PI * 2);
                    ctx.stroke();
                } else {
                    // Hexagon
                    ctx.beginPath();
                    for (let i = 0; i < 6; i++) {
                        const angle = (Math.PI / 3) * i;
                        const x = Math.cos(angle) * size;
                        const y = Math.sin(angle) * size;
                        if (i === 0) ctx.moveTo(x, y);
                        else ctx.lineTo(x, y);
                    }
                    ctx.closePath();
                    ctx.stroke();
                }
                
                ctx.restore();
            }
        }

        // Enhanced Particle Class
        class Particle {
            constructor() {
                this.reset();
            }
            
            reset() {
                const edge = Math.random() > 0.3;
                if (edge) {
                    const side = Math.floor(Math.random() * 4);
                    if (side === 0) { this.x = 0; this.y = Math.random() * height; }
                    else if (side === 1) { this.x = width; this.y = Math.random() * height; }
                    else if (side === 2) { this.x = Math.random() * width; this.y = 0; }
                    else { this.x = Math.random() * width; this.y = height; }
                } else {
                    this.x = Math.random() * width;
                    this.y = Math.random() * height;
                }
                
                this.vx = (Math.random() - 0.5) * 0.5;
                this.vy = (Math.random() - 0.5) * 0.5;
                this.size = Math.random() * 2.5 + 0.5;
                this.brightness = 0.4 + Math.random() * 0.4;
                this.hue = COLORS.primary.h + Math.random() * 50;
                this.phase = Math.random() * Math.PI * 2;
            }

            update(time) {
                this.x += this.vx;
                this.y += this.vy;
                
                // Gentle pull towards nucleus center
                const dx = centerX - this.x;
                const dy = centerY - this.y;
                const dist = Math.sqrt(dx * dx + dy * dy);
                if (dist > 80) {
                    this.vx += dx / dist * 0.004;
                    this.vy += dy / dist * 0.004;
                }
                
                // Mouse interaction - stronger effect
                if (mouse.active) {
                    const mdx = this.x - mouse.x;
                    const mdy = this.y - mouse.y;
                    const mDist = Math.sqrt(mdx * mdx + mdy * mdy);
                    if (mDist < 180 && mDist > 0) {
                        const force = (180 - mDist) / 180;
                        this.vx += (mdx / mDist) * force * 0.4;
                        this.vy += (mdy / mDist) * force * 0.4;
                    }
                }
                
                this.vx *= 0.985;
                this.vy *= 0.985;
                
                if (this.x < -50 || this.x > width + 50 || this.y < -50 || this.y > height + 50) {
                    this.reset();
                }
            }

            draw(time) {
                const pulse = Math.sin(time * 0.004 + this.phase) * 0.15 + 1;
                const size = this.size * pulse;
                
                // Add subtle glow to particles
                const gradient = ctx.createRadialGradient(this.x, this.y, 0, this.x, this.y, size * 3);
                gradient.addColorStop(0, `hsla(${this.hue}, 70%, 80%, ${this.brightness})`);
                gradient.addColorStop(0.5, `hsla(${this.hue}, 60%, 60%, ${this.brightness * 0.4})`);
                gradient.addColorStop(1, `hsla(${this.hue}, 50%, 50%, 0)`);
                
                ctx.fillStyle = gradient;
                ctx.beginPath();
                ctx.arc(this.x, this.y, size * 3, 0, Math.PI * 2);
                ctx.fill();
            }
        }

        // Enhanced Electron Class with energy trails
        class Electron {
            constructor(orbitIndex) {
                this.orbitRadius = ORBIT_RADII[orbitIndex];
                this.orbitIndex = orbitIndex;
                this.angle = Math.random() * Math.PI * 2;
                this.speed = (0.006 + Math.random() * 0.008) * (orbitIndex % 2 === 0 ? 1 : -1);
                this.size = 5 + Math.random() * 3;
                this.tilt = [0.15, -0.25, 0.35, -0.2][orbitIndex];
                this.phase = Math.random() * Math.PI * 2;
                this.trail = [];
            }

            update() {
                // Store trail positions
                const x = centerX + Math.cos(this.angle) * this.orbitRadius;
                const y = centerY + Math.sin(this.angle) * this.orbitRadius * Math.cos(this.tilt);
                this.trail.unshift({ x, y });
                if (this.trail.length > 15) this.trail.pop();
                
                this.angle += this.speed;
            }

            draw(time) {
                const x = centerX + Math.cos(this.angle) * this.orbitRadius;
                const y = centerY + Math.sin(this.angle) * this.orbitRadius * Math.cos(this.tilt);
                
                // Draw energy trail
                if (this.trail.length > 1) {
                    ctx.beginPath();
                    ctx.moveTo(this.trail[0].x, this.trail[0].y);
                    for (let i = 1; i < this.trail.length; i++) {
                        ctx.lineTo(this.trail[i].x, this.trail[i].y);
                    }
                    const trailGrad = ctx.createLinearGradient(
                        this.trail[0].x, this.trail[0].y,
                        this.trail[this.trail.length - 1].x, this.trail[this.trail.length - 1].y
                    );
                    trailGrad.addColorStop(0, `hsla(${COLORS.accent.h}, 80%, 60%, 0.6)`);
                    trailGrad.addColorStop(1, `hsla(${COLORS.accent.h}, 80%, 60%, 0)`);
                    ctx.strokeStyle = trailGrad;
                    ctx.lineWidth = this.size * 0.5;
                    ctx.lineCap = 'round';
                    ctx.stroke();
                }
                
                // Electron glow
                const pulse = Math.sin(time * 0.004 + this.phase) * 0.25 + 1;
                const size = this.size * pulse;
                
                ctx.shadowBlur = 25;
                ctx.shadowColor = `hsla(${COLORS.accent.h}, 90%, 65%, 0.9)`;
                
                // Outer glow
                const glowGrad = ctx.createRadialGradient(x, y, 0, x, y, size * 4);
                glowGrad.addColorStop(0, `hsla(${COLORS.accent.h}, 70%, 95%, 1)`);
                glowGrad.addColorStop(0.3, `hsla(${COLORS.accent.h}, 80%, 70%, 0.8)`);
                glowGrad.addColorStop(0.6, `hsla(${COLORS.accent.h}, 90%, 55%, 0.3)`);
                glowGrad.addColorStop(1, `hsla(${COLORS.accent.h}, 90%, 50%, 0)`);
                
                ctx.fillStyle = glowGrad;
                ctx.beginPath();
                ctx.arc(x, y, size * 4, 0, Math.PI * 2);
                ctx.fill();
                
                // Core
                ctx.fillStyle = `hsla(${COLORS.accent.h}, 60%, 95%, 1)`;
                ctx.beginPath();
                ctx.arc(x, y, size * 0.6, 0, Math.PI * 2);
                ctx.fill();
                
                ctx.shadowBlur = 0;
            }
        }

        // Premium Nucleus Core with internal energy
        function drawNucleus(time) {
            const breathe = Math.sin(time * 0.0015) * 0.08 + 1;
            const pulse = Math.sin(time * 0.003) * 0.05 + 1;
            const radius = NUCLEUS_RADIUS * breathe;
            
            // Multiple glow layers for depth
            for (let i = 5; i >= 0; i--) {
                const glowRadius = radius + i * 25 + Math.sin(time * 0.002 + i) * 5;
                const opacity = 0.12 - i * 0.018;
                const gradient = ctx.createRadialGradient(centerX, centerY, 0, centerX, centerY, glowRadius);
                gradient.addColorStop(0, `hsla(${COLORS.primary.h}, 80%, 65%, ${opacity})`);
                gradient.addColorStop(0.4, `hsla(${COLORS.secondary.h}, 70%, 55%, ${opacity * 0.6})`);
                gradient.addColorStop(0.7, `hsla(${COLORS.secondary.h}, 60%, 45%, ${opacity * 0.3})`);
                gradient.addColorStop(1, `hsla(${COLORS.secondary.h}, 50%, 30%, 0)`);
                
                ctx.fillStyle = gradient;
                ctx.beginPath();
                ctx.arc(centerX, centerY, glowRadius, 0, Math.PI * 2);
                ctx.fill();
            }
            
            // Core sphere with 3D-like lighting
            const coreGrad = ctx.createRadialGradient(
                centerX - radius * 0.35, centerY - radius * 0.35, radius * 0.1,
                centerX + radius * 0.1, centerY + radius * 0.1, radius
            );
            coreGrad.addColorStop(0, `hsla(${COLORS.accent.h}, 50%, 95%, 0.95)`);
            coreGrad.addColorStop(0.2, `hsla(${COLORS.primary.h}, 70%, 75%, 0.9)`);
            coreGrad.addColorStop(0.5, `hsla(${COLORS.primary.h}, 80%, 55%, 0.85)`);
            coreGrad.addColorStop(0.75, `hsla(${COLORS.secondary.h}, 70%, 45%, 0.8)`);
            coreGrad.addColorStop(1, `hsla(${COLORS.secondary.h}, 80%, 30%, 0.6)`);
            
            ctx.fillStyle = coreGrad;
            ctx.beginPath();
            ctx.arc(centerX, centerY, radius, 0, Math.PI * 2);
            ctx.fill();
            
            // Internal energy swirl
            const swirlCount = 3;
            for (let s = 0; s < swirlCount; s++) {
                const swirlAngle = time * 0.001 + (Math.PI * 2 / swirlCount) * s;
                const swirlX = centerX + Math.cos(swirlAngle) * radius * 0.4;
                const swirlY = centerY + Math.sin(swirlAngle) * radius * 0.4;
                
                const swirlGrad = ctx.createRadialGradient(swirlX, swirlY, 0, swirlX, swirlY, radius * 0.5);
                swirlGrad.addColorStop(0, `hsla(${COLORS.accent.h}, 70%, 85%, 0.4)`);
                swirlGrad.addColorStop(1, `hsla(${COLORS.accent.h}, 70%, 85%, 0)`);
                
                ctx.fillStyle = swirlGrad;
                ctx.beginPath();
                ctx.arc(swirlX, swirlY, radius * 0.5, 0, Math.PI * 2);
                ctx.fill();
            }
            
            // Specular highlight
            const specGrad = ctx.createRadialGradient(
                centerX - radius * 0.4, centerY - radius * 0.4, 0,
                centerX - radius * 0.2, centerY - radius * 0.2, radius * 0.5
            );
            specGrad.addColorStop(0, `hsla(0, 0%, 100%, 0.5)`);
            specGrad.addColorStop(0.5, `hsla(0, 0%, 100%, 0.1)`);
            specGrad.addColorStop(1, `hsla(0, 0%, 100%, 0)`);
            
            ctx.fillStyle = specGrad;
            ctx.beginPath();
            ctx.arc(centerX, centerY, radius, 0, Math.PI * 2);
            ctx.fill();
        }

        // Draw orbital rings with varying opacity
        function drawOrbits(time) {
            ORBIT_RADII.forEach((radius, i) => {
                const tilt = [0.15, -0.25, 0.35, -0.2][i];
                const rotation = time * 0.0002 * (i % 2 === 0 ? 1 : -1);
                const pulse = Math.sin(time * 0.002 + i * 0.5) * 0.15 + 0.2;
                
                ctx.strokeStyle = `hsla(${COLORS.primary.h + i * 15}, 60%, 55%, ${pulse})`;
                ctx.lineWidth = 1.5;
                ctx.setLineDash([5, 15]);
                ctx.beginPath();
                
                for (let a = 0; a <= Math.PI * 2; a += 0.03) {
                    const x = centerX + Math.cos(a + rotation) * radius;
                    const y = centerY + Math.sin(a + rotation) * radius * Math.cos(tilt);
                    if (a === 0) ctx.moveTo(x, y);
                    else ctx.lineTo(x, y);
                }
                ctx.closePath();
                ctx.stroke();
                ctx.setLineDash([]);
            });
        }

        // Draw connections between nearby particles
        function drawConnections() {
            for (let i = 0; i < particles.length; i++) {
                const a = particles[i];
                for (let j = i + 1; j < particles.length; j++) {
                    const b = particles[j];
                    const dx = a.x - b.x;
                    const dy = a.y - b.y;
                    const dist = Math.sqrt(dx * dx + dy * dy);
                    
                    if (dist < CONNECT_DIST) {
                        const opacity = (1 - dist / CONNECT_DIST) * 0.15;
                        ctx.strokeStyle = `hsla(${COLORS.primary.h}, 50%, 60%, ${opacity})`;
                        ctx.lineWidth = 0.5;
                        ctx.beginPath();
                        ctx.moveTo(a.x, a.y);
                        ctx.lineTo(b.x, b.y);
                        ctx.stroke();
                    }
                }
            }
        }

        function resize() {
            width = canvas.width = canvas.offsetWidth;
            height = canvas.height = canvas.offsetHeight;
            centerX = width * 0.62;
            centerY = height * 0.48;
            
            particles = [];
            electrons = [];
            ambientShapes = [];
            
            for (let i = 0; i < PARTICLE_COUNT; i++) {
                particles.push(new Particle());
            }
            
            for (let i = 0; i < ELECTRON_COUNT; i++) {
                electrons.push(new Electron(i % ORBIT_RADII.length));
            }
            
            for (let i = 0; i < AMBIENT_SHAPE_COUNT; i++) {
                ambientShapes.push(new AmbientShape());
            }
        }

        function animate() {
            // Slight fade for motion blur effect
            ctx.fillStyle = 'rgba(0, 0, 0, 0.08)';
            ctx.fillRect(0, 0, width, height);
            
            ctx.clearRect(0, 0, width, height);
            time++;
            
            // Update all elements
            ambientShapes.forEach(s => s.update());
            particles.forEach(p => p.update(time));
            electrons.forEach(e => e.update());
            
            // Draw in order: ambient, connections, orbits, nucleus, electrons, particles
            ambientShapes.forEach(s => s.draw(time));
            drawConnections();
            drawOrbits(time);
            drawNucleus(time);
            electrons.forEach(e => e.draw(time));
            particles.forEach(p => p.draw(time));
            
            requestAnimationFrame(animate);
        }

        window.addEventListener('resize', resize);
        resize();
        animate();
    }
});
