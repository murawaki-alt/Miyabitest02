/**
 * McKinsey Japan Clone - JavaScript
 */

document.addEventListener('DOMContentLoaded', () => {
    initSlider();
    initHeaderScroll();
    initMobileMenu();
});

/**
 * Hero Slider
 */
function initSlider() {
    const dots = document.querySelectorAll('.slider-dot');
    const leftArrow = document.querySelector('.slider-arrow-left');
    const rightArrow = document.querySelector('.slider-arrow-right');
    let currentSlide = 0;
    const totalSlides = dots.length;

    function updateDots() {
        dots.forEach((dot, index) => {
            dot.classList.toggle('active', index === currentSlide);
        });
    }

    function nextSlide() {
        currentSlide = (currentSlide + 1) % totalSlides;
        updateDots();
    }

    function prevSlide() {
        currentSlide = (currentSlide - 1 + totalSlides) % totalSlides;
        updateDots();
    }

    // Arrow click handlers
    rightArrow?.addEventListener('click', nextSlide);
    leftArrow?.addEventListener('click', prevSlide);

    // Dot click handlers
    dots.forEach((dot, index) => {
        dot.addEventListener('click', () => {
            currentSlide = index;
            updateDots();
        });
    });

    // Auto-advance slider
    setInterval(nextSlide, 6000);
}

/**
 * Header scroll behavior
 */
function initHeaderScroll() {
    const header = document.querySelector('.header');
    let lastScroll = 0;

    window.addEventListener('scroll', () => {
        const currentScroll = window.pageYOffset;
        
        if (currentScroll > 100) {
            header.style.boxShadow = '0 2px 10px rgba(0,0,0,0.1)';
        } else {
            header.style.boxShadow = 'none';
        }
        
        lastScroll = currentScroll;
    }, { passive: true });
}

/**
 * Mobile menu
 */
function initMobileMenu() {
    const hamburger = document.querySelector('.hamburger-btn');
    const nav = document.querySelector('.main-nav');
    
    hamburger?.addEventListener('click', () => {
        nav?.classList.toggle('is-open');
        hamburger.classList.toggle('is-active');
    });
}

/**
 * Smooth scroll
 */
document.querySelectorAll('a[href^="#"]').forEach(anchor => {
    anchor.addEventListener('click', function(e) {
        e.preventDefault();
        const target = document.querySelector(this.getAttribute('href'));
        if (target) {
            target.scrollIntoView({
                behavior: 'smooth',
                block: 'start'
            });
        }
    });
});

console.log('%c McKinsey Japan Clone ', 'background: #051c2c; color: white; padding: 10px 20px; font-size: 14px;');
