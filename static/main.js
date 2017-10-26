(function() {
    "use strict";

    document.querySelector('.sidebar-toggle').addEventListener('click', function toggleSidebar(e) {
        var body = document.querySelector('body');
        body.classList.add('sidebar-toggling');
        body.classList.toggle('sidebar-hidden');
        setTimeout(function() {
            body.classList.remove('sidebar-toggling');
        }, 500);
    });

    var lastWindowWidth = window.innerWidth;
    var throttled = false;
    window.addEventListener('resize', function resetSidebar(e) {
        if (!throttled) {
            throttled = true;
            if (lastWindowWidth !== window.innerWidth) {
                lastWindowWidth = window.innerWidth;
                var body = document.querySelector('body');

                body.classList.remove('sidebar-hidden');
                if (lastWindowWidth < 768) {
                    body.classList.add('sidebar-hidden');
                }
            }

            setTimeout(function() {
                throttled = false;
            }, 250);
        }
    });
})();
