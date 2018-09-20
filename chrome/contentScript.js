//alert("TESTA")


// actions for share-online site
if (window.location.href.includes("share-online.biz")) {
    document.body.innerHTML = `
    <div>
    <h1>reCAPTCHA demo: Explicit render after an onload callback</h1>
    </div>
    <div id="html_element"></div>
    `;

    resetCss();
    addJs(`
    function test(t) {
        console.log("rep");
        console.log(grecaptcha.getResponse());
        console.log(grecaptcha.getResponse());


        var body = "dl_free=1&captcha="+captcha+"&recaptcha_challenge_field=" + t + "&recaptcha_response_field=" + t;

        console.log("body");
        console.log(body);

        var u = url.split("///").join("/free/captcha/");
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

            setTimeout(function(){ window.location = my_file[6] }, 30000);
            
        })
        .catch(function(error) {
            // If there is any error you will catch them here
            console.log(error)
        });   
    }

    function a () {
        console.log("Timeout");
    }
    `);
    //addJsSrc('https://www.google.com/recaptcha/api.js?onload=onloadCallback&render=explicit');
    addJs(`
    
        grecaptcha.render('html_element', {
          'sitekey' : '6LdnPkIUAAAAABqC_ITR9-LTJKSdyR_Etj1Sf-Xi',
          'callback' : 'test',
        });
      


      console.log("nfo");
      console.log(nfo);
      console.log("dl");
      console.log(dl);
      my_file = info(nfo).split(div);
      my_file[5] = $.base64Decode(dl);
      console.log(my_file);

      my_captcha = my_file[5].split("hk||")[1];
      console.log("captchar");
      console.log(my_captcha);

      if (my_captcha == undefined) {
        var url = "http://www.share-online.biz/dl/NPE1KXEPFQM/free/";
        var obj, obj2; obj = document.createElement('form');
        obj2 = document.createElement('input');
        $(obj).attr("action", url).attr("method", "post");
        $(obj2).attr("type", "hidden").attr("value", "1").attr("name", "dl_free");
        $(obj).append(obj2); obj2 = document.createElement('input');
        $(obj2).attr("type", "hidden").attr("value", "free").attr("name", "choice");
        $(obj).append(obj2); $('body').append(obj); $('body form:last').submit()
      }

      //$('body > :not(#dl_captcha)').hide(); //hide all nodes directly under the body
        //$('#dl_captcha').appendTo('body');
        //$('div:not(#dl_captcha)').hide();
    `)
}

function go_free() {
    var url = "http://www.share-online.biz/dl/NPE1KXEPFQM/free/";
    var obj, obj2; obj = document.createElement('form');
    obj2 = document.createElement('input');
    $(obj).attr("action", url).attr("method", "post");
    $(obj2).attr("type", "hidden").attr("value", "1").attr("name", "dl_free");
    $(obj).append(obj2); obj2 = document.createElement('input');
    $(obj2).attr("type", "hidden").attr("value", "free").attr("name", "choice");
    $(obj).append(obj2); $('body').append(obj); $('body form:last').submit()
}

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

