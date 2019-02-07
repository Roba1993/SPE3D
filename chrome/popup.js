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


$.fn.redraw = function () {
    $(this).each(function () {
        var redraw = this.offsetHeight;
    });
};

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

// set download view initially
chrome.storage.local.get(['DownloadList'], function (result) {
    $('#console').text(JSON.stringify(result.DownloadList));
    updateDownloadList(result.DownloadList);
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
        else if (key == "DownloadList") {
            $('#console').text(JSON.stringify(changes[key]));
            updateDownloadList(changes[key]);
        }
    }
});

function updateDownloadList(data) {
    var html = '';

    for (container of data) {
        var finished = container.files.reduce((pre, curr) => (curr.status == "Downloaded") ? pre += 1 : pre, 0);

        html += `<div class="ui segment">
                    <div class="ui equal width grid ">
                        <div class="column">
                            <button class="circular ui icon button green medium">
                                <i class="arrow down icon"></i>
                            </button>
                        </div>
                    
                        
                    `;

        html += '<div class="column" style="text-align: center; font-weight: bold; font-size: 20px;">';
        html += finished + '/' + container.files.length;
        html += `</div>`

        html += `</div></div>`
    }

    $('#downloads').html(html);
}