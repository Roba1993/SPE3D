var spe3d_server;
var websocket;

// function to connect to the webserver
function connect() {
    websocket = new WebSocket('ws://' + spe3d_server + '/updates');

    websocket.onopen = function () {
        chrome.storage.local.set({ 'ServerStatus': true });
    };

    websocket.onclose = function (e) {
        chrome.storage.local.set({ 'ServerStatus': false });
        //console.log('Socket is closed. Reconnect will be attempted in 1 second.', e.reason);
        setTimeout(function () {
            connect();
        }, 1000);
    };

    websocket.onerror = function (err) {
        //console.error('Socket encountered error: ', err.message, 'Closing socket');
        websocket.close();
    };

    websocket.onmessage = (evt) => {
        var msg = JSON.parse(evt.data);

        // only process captcha requests
        if (msg.CaptchaRequest != undefined) {
            // only when captcha is active check the captchars
            chrome.storage.local.get(["spe3d_captcha"], function (result) {
                if (result.spe3d_captcha) {
                    switch (msg.CaptchaRequest.hoster) {
                        case 'ShareOnline':
                            handleShareOnline(msg.CaptchaRequest)
                            break;
                    }
                }
            });
        }
        // Save download list into store
        else if (msg.DownloadList != undefined) {
            console.log(msg);
            console.log("set download list");
            chrome.storage.local.set({ 'DownloadList': msg.DownloadList });
        }
    };
}

// load dload list
function loadDownloadList() {
    fetch(`http://` + spe3d_server + `/api/downloads`)
        .then(res => {
            if (res.status != 200) {
                // handle error
            }

            return res.json()
        })
        .then(dloads => {
            chrome.storage.local.set({ 'DownloadList': dloads });
        })
        .catch(error => {
            // handle error
        });
}

// get the spe3d server value and start the websocket and load the download list
chrome.storage.local.get(["spe3d_server"], function (result) {
    spe3d_server = result.spe3d_server;
    connect();
    loadDownloadList();
});

// empty the store on every start
function emptyStore() {
    chrome.storage.local.set({ 'ShareOnline': null }, function () {
    });
}
emptyStore();



// only run when the plugin get's installed
chrome.runtime.onInstalled.addListener(function () {
    chrome.storage.local.set({ 'spe3d_captcha': true });
    chrome.storage.local.set({ 'spe3d_server': 'localhost:8000' });
});


// listener to remove tabs
chrome.runtime.onMessage.addListener(
    function (request) {
        if (request.closeTab) {
            chrome.tabs.remove(request.closeTab);
        }
    }
);

// when the spe3d server changed, update the value
chrome.storage.onChanged.addListener(function (changes, namespace) {
    for (key in changes) {
        if (key === "spe3d_server") {
            spe3d_server = changes[key].newValue;
            websocket.close();
            connect();
        }
    }
});

// handle the share online captcha request
function handleShareOnline(file) {
    chrome.storage.local.get(['ShareOnline'], function (result) {
        // Only continue if no other ShareOnline request is set
        if (result.ShareOnline === null) {
            // set the ShareOnline as file value
            chrome.storage.local.set({ 'ShareOnline': file }, function () {

                // create a new tab
                chrome.tabs.create({
                    url: file.url
                }, function (tab) {
                    // set tab id to store
                    file.tab_id = tab.id;
                    chrome.storage.local.set({ 'ShareOnline': file });

                    // execute ShareOnline srcipt first time
                    chrome.tabs.executeScript(tab.id, {
                        file: 'ShareOnline.js'
                    }, function () {
                        // wait for reconect and load the script again
                        setTimeout(function () {
                            // execute ShareOnline a second time, because of reloading
                            chrome.tabs.executeScript(tab.id, {
                                file: 'ShareOnline.js'
                            });
                        }, 750);
                    })
                });

            });
        }
    });

}

console.log("Location: " + window.location.hostname);
console.log("addon:" + (window.chrome && chrome.runtime && chrome.runtime.id));