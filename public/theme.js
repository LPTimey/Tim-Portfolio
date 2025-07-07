// document.getElementById("ThemeSelect").addEventListener("change", function () {
//     const selected = this.value;
//     const radios = document.querySelectorAll('input[name="theme"]');
//     radios.forEach(radio => {
//         radio.checked = radio.value === selected;
//     });
// });
const select = document.getElementById("ThemeSelect");
const radios = document.querySelectorAll('input[name="theme"]');

// Funktion: Theme setzen
function setTheme(value) {
    radios.forEach(radio => {
        radio.checked = (radio.value === value);
    });
    select.value = value;
}

// 1. Wiederherstellen aus localStorage oder Default (Radio-Button checked)
const storedTheme = localStorage.getItem("preferredTheme");
if (storedTheme) {
    setTheme(storedTheme);
} else {
    // Falls kein gespeicherter Wert: current checked Radio übernehmen
    const checkedRadio = Array.from(radios).find(r => r.checked);
    if (checkedRadio) {
        select.value = checkedRadio.value;
    }
}

// 2. EventListener: Änderung im <select>
select.addEventListener("change", function () {
    const selected = this.value;
    setTheme(selected);
    localStorage.setItem("preferredTheme", selected);
});

// 3. Optional: Theme auch setzen, wenn Radio manuell geändert wird
radios.forEach(radio => {
    radio.addEventListener("change", function () {
        if (this.checked) {
            localStorage.setItem("preferredTheme", this.value);
            select.value = this.value;
        }
    });
});
