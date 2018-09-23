function connect() {
    var websocket = new WebSocket('ws://127.0.0.1:8000/updates');

    websocket.onclose = function (e) {
        console.log('Socket is closed. Reconnect will be attempted in 1 second.', e.reason);
        setTimeout(function () {
            connect();
        }, 1000);
    };

    websocket.onerror = function (err) {
        console.error('Socket encountered error: ', err.message, 'Closing socket');
        websocket.close();
    };

    websocket.onmessage = (evt) => {
        var msg = JSON.parse(evt.data);

        if (msg.CaptchaRequest != undefined) {
            console.log(msg);

            switch (msg.CaptchaRequest.hoster) {
                case 'ShareOnline':
                    handleShareOnline(msg.CaptchaRequest)
                    break;
            }
        }
    };
}
connect();


function emptyStore() {
    chrome.storage.local.set({ 'ShareOnline': null }, function () {
    });
}
emptyStore();


chrome.runtime.onMessage.addListener(
    function (request) {
        if (request.closeTab) {
            chrome.tabs.remove(request.closeTab);
        }
    }
);


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