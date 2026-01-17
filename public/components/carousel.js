"use strict";

/**
 * @typedef {HTMLDivElement & {
 *   dataset: {
 *     current: string,
 *     scroll: string
 *   }
 * }} CarouselElement
 */
/**
 * @typedef {HTMLLIElement & {
 *   dataset: {
 *     for: string
 *   }
 * }} CarouselDot
 */
/**
 * @typedef {HTMLLIElement & {
 *   dataset: {
 *     index: string
 *   }
 * }} CarouselImg
 */

/**
 * @param {CarouselElement} carousel
 */
function initCarousel(carousel) {
    const content = carousel.querySelector(".carousel-content");
    if (!content) {
        console.error(`${carousel} doesn't have content`);
        return
    }
    const slides = /** @type {CarouselImg[]} */(
        Array.from(content.children)
    );
    const dots = /** @type {CarouselDot[]} */(
        Array.from(carousel.querySelectorAll(".carousel-dot"))
    );
    const btnLeft = /** @type {HTMLButtonElement | null} */(
        carousel.querySelector(".carousel-button-left")
    );
    const btnRight = /** @type {HTMLButtonElement | null} */(
        carousel.querySelector(".carousel-button-right")
    );

    const slideCount = slides.length;
    let current = Number(carousel.dataset.current) || 0;
    const autoScrollMs = Number(carousel.dataset.scroll) || 0;

    /**
     * @param {number} index
     */
    function goTo(index) {
        current = (index + slideCount) % slideCount;
        carousel.dataset.current = String(current);

        slides[current].scrollIntoView({
            behavior: "smooth",
            inline: "start",
            block: "nearest",
        });

        dots.forEach((dot, i) => {
            dot.classList.toggle("active", i === current);
        });
    }

    function next() {
        goTo(current + 1);
    }

    function prev() {
        goTo(current - 1);
    }

    // Initial
    goTo(current);

    // Buttons
    btnLeft?.addEventListener("click", prev);
    btnRight?.addEventListener("click", next);

    // Dots
    dots.forEach(dot => {
        dot.addEventListener("click", () => {
            const index = Number(dot.dataset.for);
            if (!Number.isNaN(index)) {
                goTo(index);
            }
        });
    });

    // Auto-Scroll
    if (autoScrollMs > 0) {
        let timer = setInterval(next, autoScrollMs);

        carousel.addEventListener("mouseenter", () => clearInterval(timer));
        carousel.addEventListener("mouseleave", () => {
            timer = setInterval(next, autoScrollMs);
        });
    }

    // Sync bei manuellem Scroll (Touch / Trackpad)
    /**
     * @type {number | undefined}
     */
    let scrollTimeout;
    content.addEventListener("scroll", () => {
        clearTimeout(scrollTimeout);
        scrollTimeout = setTimeout(() => {
            const index = Math.round(
                content.scrollLeft / content.clientWidth
            );
            if (index !== current) {
                current = index;
                carousel.dataset.current = String(current);
                dots.forEach((dot, i) =>
                    dot.classList.toggle("active", i === current)
                );
            }
        }, 80);
    });
}

/**
 * Init aller Carousels
 */
function initCarousels() {
    document
        .querySelectorAll(".carousel")
        // @ts-ignore
        .forEach(initCarousel);
}

if (document.readyState === "loading") {
    document.addEventListener("DOMContentLoaded", initCarousels);
} else {
    initCarousels();
}
