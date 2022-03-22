import bulmaCollapsible from '@creativebulma/bulma-collapsible';

import { default as vip, Events } from './vip';
const databaseAddress = "/orbitdb/zdpuAoCUkoLBRLp3HNFzCRrULZVgBVkqiRYmfLAuZXEmfD1Tg/ZZhzpRwVEP";

function randomAddress(length) {
    let result = '';
    const characters = 'ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789';
    const charactersLength = characters.length;
    for ( let i = 0; i < length; i++ ) {
        result += characters.charAt(Math.floor(Math.random() *
            charactersLength));
    }
    return result;
}

const check = document.getElementById('vip-check');
const dbPeers = document.getElementById('vip-peers-db');
const networkPeers = document.getElementById('vip-peers-network');
const progress = document.getElementById('vip-progress');
const signup = document.getElementById('vip-signup');
const signupStatus = document.getElementById('vip-signup-status');
const status = document.getElementById('vip-status');
const total = document.getElementById('vip-total');
const userAddress = randomAddress(20);

async function main () {

    vip.debug();

    // Add event listeners
    vip.addEventListener(Events.NetworkPeersChanged, () => {
        networkPeers.innerText = vip.peers.network.toString();
    });

    vip.addEventListener(Events.Connected, () => {
        dbPeers.innerText = vip.peers.database.toString();
        status.classList.remove('blink');
        setStatus("> Connected - Signup now available");
        signup.disabled = false;
        check.disabled = false;
    });
    vip.addEventListener(Events.Disconnected, () => {
        status.classList.add('blink');
        setStatus("> No peers currently available - waiting until one becomes available...");
        signup.disabled = true;
        check.disabled = true;
    });
    vip.addEventListener(Events.ReplicationProgress, () => {
        setStatus("> Synchronising VIP list...");
    });
    vip.addEventListener(Events.Replicated, () => {
        console.log("Remote entries synchronised.");
    });
    vip.addEventListener(Events.TotalChanged, () => {
        total.innerText = vip.total;
    });
    signup.addEventListener('click', async () => {
        await vip.register(userAddress);
        total.innerText = vip.total;

        setStatus("> Connected to MetaFashion HQ");

        signupStatus.classList.remove('has-text-success', 'has-text-danger');
        if (await vip.check(userAddress)) {
            signupStatus.classList.add('has-text-success');
            signupStatus.innerText = '> You have successfully signed up - welcome to the MetaFashion VIP list!';
        }
        else {
            signupStatus.classList.add('has-text-danger');
            signupStatus.innerText = '> Your address was not found on the VIP list';
        }

        signup.disabled = true;
    });
    check.addEventListener('click', async () => {
        signupStatus.classList.remove('has-text-success', 'has-text-danger');
        if (await vip.check(userAddress)) {
            signupStatus.innerText = '> You have successfully signed up - welcome to the MetaFashion VIP list!';
            signupStatus.classList.add('has-text-success');
        }
        else {
            signupStatus.classList.add('has-text-danger');
            signupStatus.innerText = '> Your address was not found on the VIP list';
        }

        check.disabled = true;
    });



    const setStatus = (message) => {
        console.info(message);
        status.innerText = message;
    };

    // Detect Safari
    let chromeAgent = navigator.userAgent.indexOf("Chrome") > -1;
    let safariAgent = navigator.userAgent.indexOf("Safari") > -1;
    if ((chromeAgent) && (safariAgent)) safariAgent = false;

    try {
        status.classList.add('blink');
        setStatus("> Connecting to network");
        await vip.init();
    }
    catch (err)
    {
        status.classList.remove('blink');
        console.error(err);
        status.classList.add('has-text-danger');
        setStatus(`> Could not connect to network - ${err.message}`);
        return;
    }

    try {
        setStatus("> Contacting MetaFashion HQ");
        await vip.connect(databaseAddress);
    }
    catch (err)
    {
        status.classList.remove('blink');
        console.error(err);
        status.classList.add('has-text-danger');
        setStatus(`> Could not contact MetaFashion HQ - ${err.message}`);
    }
}

main()



const font = new FontFace("Druk Wide Medium",
    "url(\"/assets/fonts/7e389c5e310dc537b083e0e25ea6eab5.woff2\") format(\"woff2\")");
const images = [
    '/assets/logo.svg',
    '/assets/header.jpg',
    '/assets/star.svg',
    '/assets/arrow.svg',
    '/assets/cb.png',
    '/assets/je.png',
    '/assets/h.png',
];

document.addEventListener('DOMContentLoaded', () => {
    // Check if there are any navbar burgers
    const nav = document.getElementsByTagName("nav")[0];
    const $navbarBurgers = Array.prototype.slice.call(document.querySelectorAll('.navbar-burger'), 0);
    if ($navbarBurgers.length > 0) {
        // Add a click event on each of them
        $navbarBurgers.forEach(burger => {
            // Get the target from the "data-target" attribute
            const target = burger.dataset.target;
            const $target = document.getElementById(target);

            burger.addEventListener('click', () => {
                // Toggle the "is-active" class on both the "navbar-burger" and the "navbar-menu"
                nav.classList.toggle('is-active');
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

        // Add class to nav once user scrolls
        if (window.scrollY > 10)
            nav.classList.add('scroll');
        else
            nav.classList.remove('scroll');

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

async function init() {
    // Cache assets
    const total = images.length + 1;
    const status = document.getElementById('status');
    for (const asset of images) {
        new Image().src = asset;
        setStatus(status, ((images.indexOf(asset) + 1) / total) * 100);
    }
    await font.load();
    setStatus(status,100);
    await delay(350);

    function setStatus(status, percent) {
        let indicator = document.querySelectorAll('.progress-bar .value')[0];
        indicator.style.width = percent + '%';
        if (percent >= 100)
            indicator.classList.add('is-complete');
        status.innerText = percent + '%';
    }

    // Hide loading page and show site
    document.getElementById('landing').style.display = 'none';
    document.getElementsByTagName('html')[0].classList.add('has-navbar-fixed-top');
    document.getElementById('hero-image').classList.add('glitch'); // Glitch effect
    document.getElementById('site').style.display = 'initial';
    bulmaCollapsible.attach('.is-collapsible');
    document.getElementById('phase1').style.removeProperty('height');

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