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

// set the server color
chrome.storage.local.get(['ServerStatus'], function (result) {
    $('#server_status').hide();
    if (result.ServerStatus) {
        $('#server_status').removeClass("red").addClass("green").redraw();
    }
    else {
        $('#server_status').removeClass("green").addClass("red").redraw();
    }
    $('#server_status').show();
});

// when the server status changes, update icon
chrome.storage.onChanged.addListener(function (changes) {
    for (key in changes) {
        if (key === "ServerStatus") {
            $('#server_status').hide();
            if (changes[key].newValue) {
                $('#server_status').removeClass("red").addClass("green").redraw();
            }
            else {
                $('#server_status').removeClass("green").addClass("red").redraw();
            }
            $('#server_status').show();
        }
    }
});