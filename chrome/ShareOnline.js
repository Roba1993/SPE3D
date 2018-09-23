document.body.innerHTML = `
    <div>
    <h1>reCAPTCHA demo: Explicit render after an onload callback</h1>
    </div>
    <div id="html_element"></div>
    `;

resetCss();

tab_id = null;

// Get the ShareOnline value from the store
chrome.storage.local.get(['ShareOnline'],
    function (result) {
        if (result.ShareOnline != null) {
            console.log("FILE RECEIVED:");
            console.log(result.ShareOnline);

            var so = result.ShareOnline;
            tab_id = so.tab_id;

            // execute the injection scripts
            addJs(`
                String.prototype.insertAt=function(index, string) { 
                    return this.substr(0, index) + string + this.substr(index);
                }

                function setDone() {
                    g = document.createElement('div');
                    g.setAttribute("id", "CaptchaSolved");
                    document.body.appendChild(g);
                }
            
                function captchaSolved(t) {
                    var body = "dl_free=1&captcha="+captcha+"&recaptcha_challenge_field=" + t + "&recaptcha_response_field=" + t;
            
                    console.log("body:");
                    console.log(body);
                    
                    var u = url.split("///").join("/free/captcha/");
                    u = u.insertAt(4, "s");
                    console.log("url");
                    console.log(u);
            
                    fetch(u, {
                        method: 'POST',
                        body: body,
                        cache: 'no-store',
                        timeout: 2e4,
                        headers: { 'Content-Type': 'application/x-www-form-urlencoded', },
                    })
                    .then(res => res.text())
                    .then(function(data) {
                        // Here you get the data to modify as you please
                        console.log("data")
                        console.log(data)
                        if (data == "0") {
                            grecaptcha.reset();
                            //location.reload();
                        }
            
                        console.log("file6")
                        my_file[6] = $.base64Decode($.trim(data))
                        console.log(my_file[6])
            
                        sendResult(my_file[6]);
                    })
                    .catch(function(error) {
                        // If there is any error you will catch them here
                        console.log(error);
                        setDone();
                    });   
                }

                function sendResult(url) {
                    var body = {
                        id: `+ so.id + `,
                        file_id: '`+ so.file_id + `',
                        hoster: '`+ so.hoster + `',
                        url: url
                    }

                    console.log(body);

                    fetch('http://localhost:8000/api/captcha-result', {
                        method: 'POST',
                        body: JSON.stringify(body),
                        cache: 'no-store',
                        headers: {
                            'Accept': 'application/json, text/plain, */*',
                            'Content-Type': 'application/json'
                        }
                    })
                    .then(res => res.text())
                    .then(function(data) {
                        console.log("Response successfully commited to server");
                        setDone();
                    })
                    .catch(function(error) {
                        // If there is any error you will catch them here
                        console.log(error);
                        setDone();;
                    });   
                }
            `);

            addJs(`
                console.log("nfo:");
                console.log(nfo);
                console.log("dl:");
                console.log(dl);
                my_file = info(nfo).split(div);
                my_file[5] = $.base64Decode(dl);
                console.log('my_file:');
                console.log(my_file);
            
                my_captcha = my_file[5].split("hk||")[1];
                console.log("captchar:");
                console.log(my_captcha);
            
                if (my_captcha == undefined) {
                    var url = "` + so.url + `/free/";
                    var obj, obj2; obj = document.createElement('form');
                    obj2 = document.createElement('input');
                    $(obj).attr("action", url).attr("method", "post");
                    $(obj2).attr("type", "hidden").attr("value", "1").attr("name", "dl_free");
                    $(obj).append(obj2); obj2 = document.createElement('input');
                    $(obj2).attr("type", "hidden").attr("value", "free").attr("name", "choice");
                    $(obj).append(obj2); $('body').append(obj); $('body form:last').submit()
                }
            
                grecaptcha.render('html_element', {
                    'sitekey' : '6LdnPkIUAAAAABqC_ITR9-LTJKSdyR_Etj1Sf-Xi',
                    'callback' : 'captchaSolved',
                });
            `);
        }
    }
);

function checkForClose() {
    if (document.getElementById('CaptchaSolved') != null) {
        console.log("Up to close close");
        // empty the store
        chrome.storage.local.set({ 'ShareOnline': null });
        // send close event
        chrome.runtime.sendMessage({ closeTab: tab_id });

        return;
    }

    setTimeout(checkForClose, 500);
}
checkForClose();

/* General Functions*/
function addJsSrc(src) {
    var s = document.createElement('script');
    s.type = 'text/javascript';
    s.src = src;
    try {
        s.appendChild();
        document.body.appendChild(s);
    } catch (e) {
        document.body.appendChild(s);
    }
}

function addJs(code) {
    var s = document.createElement('script');
    s.type = 'text/javascript';
    try {
        s.appendChild(document.createTextNode(code));
        document.body.appendChild(s);
    } catch (e) {
        s.text = code;
        document.body.appendChild(s);
    }
}

function resetCss() {
    document.body.innerHTML += `
        <style>
            /* http://meyerweb.com/eric/tools/css/reset/ 
            v2.0 | 20110126
            License: none (public domain)
            */

            html, body, div, span, applet, object, iframe,
            h1, h2, h3, h4, h5, h6, p, blockquote, pre,
            a, abbr, acronym, address, big, cite, code,
            del, dfn, em, img, ins, kbd, q, s, samp,
            small, strike, strong, sub, sup, tt, var,
            b, u, i, center,
            dl, dt, dd, ol, ul, li,
            fieldset, form, label, legend,
            table, caption, tbody, tfoot, thead, tr, th, td,
            article, aside, canvas, details, embed, 
            figure, figcaption, footer, header, hgroup, 
            menu, nav, output, ruby, section, summary,
            time, mark, audio, video {
                margin: 0;
                padding: 0;
                border: 0;
                font-size: 100%;
                font: inherit;
                vertical-align: baseline;
                color: black;
                text-shadow: none;
            }
            /* HTML5 display-role reset for older browsers */
            article, aside, details, figcaption, figure, 
            footer, header, hgroup, menu, nav, section {
                display: block;
            }
            body {
                line-height: 1;
                background: none;
            }
            ol, ul {
                list-style: none;
            }
            blockquote, q {
                quotes: none;
            }
            blockquote:before, blockquote:after,
            q:before, q:after {
                content: '';
                content: none;
            }
            table {
                border-collapse: collapse;
                border-spacing: 0;
            }
        </style>
`;
}

