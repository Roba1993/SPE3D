// activate checkboxes
$('.ui.checkbox').checkbox();

// on settings click
$('#settings_button').click(function () {
    $("#settings").slideToggle("slow");
});

// hide settings on default
$('#settings').toggle();

// set the correct status
chrome.storage.local.get(["spe3d_server"], function (result) {
    $('#settings input[name="spe3d_server"]').val(result.spe3d_server);
});

// set the correct status
chrome.storage.local.get(["spe3d_captcha"], function (result) {
    if (result.spe3d_captcha) {
        $('#settings .ui.checkbox').checkbox('check');
        console.log('check')
    }
    else {
        $('#settings .ui.checkbox').checkbox('uncheck');
        console.log('uncheck')
    }
});

// set the url on change
$('#settings input[name="spe3d_server"]').on('input', function () {
    chrome.storage.local.set({ 'spe3d_server': $(this).val() });
});

// set the captcha active
$('#settings .ui.checkbox').checkbox({
    onChecked: function () {
        chrome.storage.local.set({ 'spe3d_captcha': true });
    },
    onUnchecked: function () {
        chrome.storage.local.set({ 'spe3d_captcha': false });
    }
});