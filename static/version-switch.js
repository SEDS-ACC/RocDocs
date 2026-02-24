document.addEventListener('DOMContentLoaded', () => {
    const selector = document.getElementById('version-select');
    if (!selector) return;

    const currentPath = window.location.pathname;
    // Map your folder names here. Ensure these match your crates/ subdirs.
    const bookFolders = ["Team", "L1", "L2", "L3"]; 

    // 1. Highlight the current book in the dropdown
    const currentBook = bookFolders.find(folder => currentPath.includes(`/${folder}/`));
    if (currentBook) {
        selector.value = currentBook;
    }

    // 2. Handle Navigation
    selector.addEventListener('change', (e) => {
    const targetBook = e.target.value;
    const segments = currentPath.split('/');

    // Find the index where the current book name lives
    const bookIndex = segments.findIndex(s => bookFolders.includes(s));

    if (bookIndex !== -1) {
        // Replace the book name
        segments[bookIndex] = targetBook;
        
        // 1. Join segments back into a path
        // 2. slice(0, bookIndex + 1) drops everything after the book name
        // 3. .join('/') creates the path string
        let newPath = segments.slice(0, bookIndex + 1).join('/') + '/';

        // FORCE ABSOLUTE: If it doesn't start with /, add it.
        // This prevents the browser from "appending" to the current URL.
        if (!newPath.startsWith('/')) {
            newPath = '/' + newPath;
        }

        window.location.href = newPath;
    } else {
        window.location.href = window.location.origin + '/' + targetBook + '/';
    }
});
});
