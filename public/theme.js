"use strict";

// document.getElementById("ThemeSelect").addEventListener("change", function () {
//     const selected = this.value;
//     const radios = document.querySelectorAll('input[name="theme"]');
//     radios.forEach(radio => {
//         radio.checked = radio.value === selected;
//     });
// });
/** @type {HTMLSelectElement} */
const select = /***/(document.getElementById("ThemeSelect"));
/** @type {NodeListOf<HTMLInputElement>} */
const radios = /***/(document.querySelectorAll('input[name="theme"]'));

/**
 * 
 * @param {string} value Themenname
 */
function setTheme(value) {
    radios.forEach(radio => {
        radio.checked = (radio.value === value);
    });
    select.value = value;
}

const storedTheme = localStorage.getItem("preferredTheme");
if (storedTheme) {
    setTheme(storedTheme);
} else {
    const checkedRadio = Array.from(radios).find(r => r.checked);
    if (checkedRadio) {
        select.value = checkedRadio.value;
    }
}

select.addEventListener("change", function () {
    const selected = this.value;
    setTheme(selected);
    localStorage.setItem("preferredTheme", selected);
});

radios.forEach(radio => {
    radio.addEventListener("change", function () {
        if (this.checked) {
            localStorage.setItem("preferredTheme", this.value);
            select.value = this.value;
        }
    });
});
