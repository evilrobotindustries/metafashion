import bulmaCollapsible from '@creativebulma/bulma-collapsible';

document.addEventListener('DOMContentLoaded', () => {
    // Check if there are any navbar burgers
    const $navbarBurgers = Array.prototype.slice.call(document.querySelectorAll('.navbar-burger'), 0);
    if ($navbarBurgers.length > 0) {
        // Add a click event on each of them
        $navbarBurgers.forEach(burger => {
            // Get the target from the "data-target" attribute
            const target = burger.dataset.target;
            const $target = document.getElementById(target);

            burger.addEventListener('click', () => {
                // Toggle the "is-active" class on both the "navbar-burger" and the "navbar-menu"
                burger.classList.toggle('is-active');
                $target.classList.toggle('is-active');
            });

            // Close menu when item clicked
            const navbarItems = $target.querySelectorAll('.navbar-item');
            navbarItems.forEach(ni => {
                ni.addEventListener('click', () => {
                    navbarItems.forEach(x => x.classList.remove('is-active'));
                    ni.classList.toggle('is-active');
                    burger.classList.toggle('is-active');
                    $target.classList.toggle('is-active');
                });
            });
        });
    }

    // Set menu items as active as user scrolls through sections
    const sections = document.querySelectorAll("section");
    const navbarItems = document.querySelectorAll(".navbar-item");
    const offset = 70;
    window.onscroll = () => {
        let current = "";

        sections.forEach((section) => {
            if (section.offsetTop === 0) return;
            if (window.scrollY >= section.offsetTop - offset ) {
                current = "#" + section.getAttribute("id"); }
        });

        navbarItems.forEach((ni) => {
            if (ni.hash === current)
                ni.classList.add("is-active");
            else
                ni.classList.remove("is-active");
        });
    };
});

function delay(ms){
    return new Promise(resolve => {
        setTimeout(resolve,ms);
    })
}

function random(min, max) {
    return Math.floor(Math.random() * (max - min) ) + min;
}

async function init() {
    // Show random progress updates
    let status = 1;
    while (status < 100){
        await delay(random(500,1000));
        status = random(status, 101)
        if (status > 100) status = 100;
        setStatus(status);
    }
    await delay(500);

    function setStatus(percent) {
        let indicator = document.querySelectorAll('.progress-bar .value')[0];
        indicator.style.width = percent + '%';
        if (percent >= 100)
            indicator.classList.add('is-complete');
        document.getElementById('status').innerText = percent + '%';
    }

    // Hide loading page and show site
    document.getElementById('landing').style.display = 'none';
    document.getElementsByTagName('html')[0].classList.add('has-navbar-fixed-top');
    document.getElementById('site').style.display = 'initial';
    document.body.classList.add('imgloaded'); // Glitch effect
    bulmaCollapsible.attach('.is-collapsible');

    // Functions to open and close a modal
    function openModal($el) {
        $el.classList.add('is-active');
    }
    function closeModal($el) {
        $el.classList.remove('is-active');
    }

    function closeAllModals() {
        (document.querySelectorAll('.modal') || []).forEach(($modal) => {
            closeModal($modal);
        });
    }

    // Add a click event on buttons to open a specific modal
    (document.querySelectorAll('.modal-button') || []).forEach(($trigger) => {
        const modal = $trigger.dataset.target;
        const $target = document.getElementById(modal);
        console.log($target);

        $trigger.addEventListener('click', () => {
            openModal($target);
        });
    });

    // Add a click event on various child elements to close the parent modal
    (document.querySelectorAll('.modal-background, .modal-close, .modal-card-head .delete, .modal-card-foot .button') || []).forEach(($close) => {
        const $target = $close.closest('.modal');

        $close.addEventListener('click', () => {
            closeModal($target);
        });
    });

    // Add a keyboard event to close all modals
    document.addEventListener('keydown', (event) => {
        const e = event || window.event;

        if (e.keyCode === 27) { // Escape key
            closeAllModals();
        }
    });
}

init();