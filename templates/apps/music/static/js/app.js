// --- State ---
let featuredVideo = null;
let selectedVideo = null;
let previewTimeout = null;

function startPreview(card, title, path) {
    // Hover logic only on non-touch (handled by CSS, but good to check capability if needed)
    const video = card.querySelector('.card-preview');
    // If already playing, ignore
    if(video.src) return;

    const filename = path.split('/').pop();
    const url = '/static/music/' + filename;

    // Delay start to avoid flashing on quick scroll
    previewTimeout = setTimeout(() => {
        video.src = url;
        // Muted autoplay is allowed by browsers
        video.play().catch(e => console.log("Autoplay blocked", e));
    }, 500);
}

function stopPreview(card) {
    clearTimeout(previewTimeout);
    const video = card.querySelector('.card-preview');
    video.pause();
    video.src = ""; // Unload
}

// --- Init ---
window.addEventListener('scroll', () => {
    const nav = document.getElementById('navbar');
    if(window.scrollY > 50) nav.classList.add('scrolled');
    else nav.classList.remove('scrolled');
});

// Load Data
(async () => {
    await Promise.all([loadFeatured(), loadGrouped()]);
    // Fade in hero
    document.getElementById('hero').style.opacity = 1;
    // Stagger fade in rows
    const rows = document.querySelectorAll('.row');
    rows.forEach((r, i) => {
        setTimeout(() => r.classList.add('visible'), 200 + (i * 100));
    });
})();

async function loadFeatured() {
    try {
        const res = await fetch('/api/featured');
        if(!res.ok) return;
        const vid = await res.json();
        if(vid) {
            featuredVideo = vid;
            document.getElementById('hero-title').innerText = vid.title;
            document.getElementById('hero-desc').innerText = vid.plot || "No description available.";
            document.getElementById('hero-year').innerText = vid.year || new Date().getFullYear();
            if(vid.cover_path) {
                document.getElementById('hero').style.backgroundImage = `url('${vid.cover_path}')`;
            } else {
                // Fallback gradient for hero
                document.getElementById('hero').classList.add('gradient-placeholder-0');
            }
        }
    } catch(e) { console.error(e); }
}

async function loadGrouped() {
    try {
        const res = await fetch('/api/grouped');
        const data = await res.json(); // { "Trending Now": [...], ... }
        
        const container = document.getElementById('rows-container');
        
        // Keep order roughly
        const keys = ["Trending Now", "New Releases", "Continue Watching"];
        
        keys.forEach((key, idx) => {
            const videos = data[key];
            if(!videos || videos.length === 0) return;

            const row = document.createElement('div');
            row.className = 'row';
            
            let html = `<div class="row-title">${key}</div><div class="slider">`;
            html += videos.map(v => {
                const gradClass = `gradient-placeholder-${(v.id || 0) % 4}`;
                // Use a fallback image if cover_path is empty
                const imgSrc = v.cover_path ? `src="${v.cover_path}"` : '';
                const imgClass = v.cover_path ? 'card-img' : `card-img ${gradClass}`;
                
                return `
                <div class="card" onmouseenter="startPreview(this, '${v.title.replace(/'/g, "\\'")}', '${v.path.replace(/'/g, "\\'")}')" onmouseleave="stopPreview(this)" onclick="openModalFromData('${v.id}')">
                    <img ${imgSrc} class="${imgClass}" onerror="this.src='', this.className='card-img ${gradClass}'">
                    <video class="card-preview" muted loop playsinline></video>
                    <div class="card-info">
                        <div style="font-weight:bold;">${v.title}</div>
                        <div>${v.year || ''}</div>
                    </div>
                </div>
            `}).join('');
            html += `</div>`;
            row.innerHTML = html;
            
            // Store data ref locally (simplified for demo)
            videos.forEach(v => window['vid_'+v.id] = v);

            container.appendChild(row);
        });

    } catch(e) { console.error(e); }
}

// --- Actions ---
function playHero() {
    if(featuredVideo) launchVideo(featuredVideo.path);
}

function openModalHero() {
    if(featuredVideo) showModal(featuredVideo);
}

function openModalFromData(id) {
    const v = window['vid_'+id];
    if(v) showModal(v);
}

function showModal(v) {
    selectedVideo = v;
    document.getElementById('modal-title').innerText = v.title;
    document.getElementById('modal-plot').innerText = v.plot || "No description.";
    document.getElementById('modal-year').innerText = v.year || "";
    document.getElementById('modal-duration').innerText = formatTime(v.duration || 0);
    
    const modalHero = document.getElementById('modal-hero-img');
    modalHero.className = 'modal-hero'; // reset
    if(v.cover_path) {
        modalHero.style.backgroundImage = `url('${v.cover_path}')`;
    } else {
            modalHero.style.backgroundImage = '';
            modalHero.classList.add(`gradient-placeholder-${(v.id||0) % 4}`);
    }

    const overlay = document.getElementById('detail-modal');
    overlay.style.display = 'flex';
    // Slight delay for animation
    setTimeout(() => {
        overlay.style.opacity = 1;
        document.getElementById('modal-box').classList.add('active');
    }, 10);
}

function closeModal(e) {
    if(e.target.id === 'detail-modal') closeModalForce();
}

function closeModalForce() {
    const overlay = document.getElementById('detail-modal');
    overlay.style.opacity = 0;
    document.getElementById('modal-box').classList.remove('active');
    setTimeout(() => {
        overlay.style.display = 'none';
        selectedVideo = null;
    }, 300);
}

function playModal() {
    if(selectedVideo) {
        closeModalForce();
        launchVideo(selectedVideo.path);
    }
}

function launchVideo(path) {
    const fs = document.getElementById('video-fs');
    const video = document.getElementById('fs-player');
    const filename = path.split('/').pop();
    video.src = '/static/music/' + filename;
    
    fs.style.display = 'block';
    video.play();
}

function closeVideo() {
    const fs = document.getElementById('video-fs');
    const video = document.getElementById('fs-player');
    video.pause();
    fs.style.display = 'none';
}

// --- Helpers ---
function formatTime(sec) {
    if(!sec) return "0m";
    const h = Math.floor(sec / 3600);
    const m = Math.floor((sec % 3600) / 60);
    return h > 0 ? `${h}h ${m}m` : `${m}m`;
}
