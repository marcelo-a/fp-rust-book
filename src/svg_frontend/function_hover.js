function displayFn(evt, classname) {
    // get svg elements
    var vis_num = document.getElementsByClassName(classname);
    var code_obj = vis_num[0];
    var tl_obj = vis_num[1];
    var c_svg = code_obj.contentDocument.firstChild;
    var tl_svg = tl_obj.contentDocument.firstChild
    // get elements that will trigger function
    var triggers = tl_svg.getElementsByClassName('fn-trigger');
    var functions = c_svg.getElementsByClassName('fn');
    
    for (var i = 0; i < triggers.length; i++) {
        triggers[i].addEventListener('mouseover', showFn);
        triggers[i].addEventListener('mouseout', hideFn);
    }
    
    function showFn(evt) {
        // get target attributes
        var evt_hash = evt.target.dataset.hash;
        var color = evt.target.style.fill;

        for (var i = 0; i < functions.length; i++) {
            // if hashes match, temporarily change color
            if (functions[i].getAttribute('hash') == evt_hash) {
                functions[i].dataset.hash = evt_hash;
            }
        }
    }

    function hideFn(evt) {
        // reset to hash 0, styling to black on mouseout
        for (var i = 0; i < functions.length; i++) {
            functions[i].dataset.hash = 0;
        }
    }
}