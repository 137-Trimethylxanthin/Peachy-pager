export const prerender = true
export const ssr = false

// toggle to switch classes between .light and .dark
// if no class is present (initial state), then assume current state based on system color scheme
// if system color scheme is not supported, then assume current state is light

export function _toggleDarkMode() {
    if (document.documentElement.classList.contains("light")) {
        document.documentElement.classList.remove("light")
        document.documentElement.classList.add("dark")
    } else if (document.documentElement.classList.contains("dark")) {
        document.documentElement.classList.remove("dark")
        document.documentElement.classList.add("light")
    } else {
        if (window.matchMedia && window.matchMedia('(prefers-color-scheme: dark)').matches) {
            document.documentElement.classList.add("light")
        } else {
            document.documentElement.classList.add("dark")
        }
    }
}

