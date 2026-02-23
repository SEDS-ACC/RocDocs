document.addEventListener('DOMContentLoaded', () => {
    const selector = document.getElementById('version-select');
    if (!selector) return; // Failsafe

    const path = window.location.pathname;

    // Auto-select the correct dropdown option based on current URL
    if (path.includes('/v1/')) selector.value = '/v1/';
    if (path.includes('/v2/')) selector.value = '/v2/';
    if (path.includes('/v3/')) selector.value = '/v3/';
    if (path.includes('/team/')) selector.value = '/team/';

    // Navigate when a new option is clicked
    selector.addEventListener('change', (e) => {
        window.location.href = e.target.value;
    });
});
