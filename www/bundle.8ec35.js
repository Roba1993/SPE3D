!function(t){function e(n){if(o[n])return o[n].exports;var r=o[n]={i:n,l:!1,exports:{}};return t[n].call(r.exports,r,r.exports,e),r.l=!0,r.exports}var n=window.webpackJsonp;window.webpackJsonp=function(e,o,i){for(var u,a,c=0,l=[];c<e.length;c++)a=e[c],r[a]&&l.push(r[a][0]),r[a]=0;for(u in o)Object.prototype.hasOwnProperty.call(o,u)&&(t[u]=o[u]);for(n&&n(e,o,i);l.length;)l.shift()()};var o={},r={3:0};e.e=function(t){function n(){a.onerror=a.onload=null,clearTimeout(c);var e=r[t];0!==e&&(e&&e[1](new Error("Loading chunk "+t+" failed.")),r[t]=void 0)}var o=r[t];if(0===o)return new Promise(function(t){t()});if(o)return o[2];var i=new Promise(function(e,n){o=r[t]=[e,n]});o[2]=i;var u=document.getElementsByTagName("head")[0],a=document.createElement("script");a.type="text/javascript",a.charset="utf-8",a.async=!0,a.timeout=12e4,e.nc&&a.setAttribute("nonce",e.nc),a.src=e.p+""+({0:"route-home",1:"route-profile",2:"route-add_links"}[t]||t)+".chunk."+{0:"181c7",1:"2ab21",2:"7c50f"}[t]+".js";var c=setTimeout(n,12e4);return a.onerror=a.onload=n,u.appendChild(a),i},e.m=t,e.c=o,e.d=function(t,n,o){e.o(t,n)||Object.defineProperty(t,n,{configurable:!1,enumerable:!0,get:o})},e.n=function(t){var n=t&&t.__esModule?function(){return t.default}:function(){return t};return e.d(n,"a",n),n},e.o=function(t,e){return Object.prototype.hasOwnProperty.call(t,e)},e.p="/",e.oe=function(t){throw console.error(t),t},e(e.s="pwNi")}({"/QC5":function(t,e,n){"use strict";function o(t,e){for(var n in e)t[n]=e[n];return t}function r(t,e,n){var o,r=/(?:\?([^#]*))?(#.*)?$/,i=t.match(r),u={};if(i&&i[1])for(var c=i[1].split("&"),l=0;l<c.length;l++){var p=c[l].split("=");u[decodeURIComponent(p[0])]=decodeURIComponent(p.slice(1).join("="))}t=a(t.replace(r,"")),e=a(e||"");for(var s=Math.max(t.length,e.length),f=0;f<s;f++)if(e[f]&&":"===e[f].charAt(0)){var d=e[f].replace(/(^\:|[+*?]+$)/g,""),h=(e[f].match(/[+*?]+$/)||O)[0]||"",_=~h.indexOf("+"),v=~h.indexOf("*"),m=t[f]||"";if(!m&&!v&&(h.indexOf("?")<0||_)){o=!1;break}if(u[d]=decodeURIComponent(m),_||v){u[d]=t.slice(f).map(decodeURIComponent).join("/");break}}else if(e[f]!==t[f]){o=!1;break}return(!0===n.default||!1!==o)&&u}function i(t,e){return t.rank<e.rank?1:t.rank>e.rank?-1:t.index-e.index}function u(t,e){return t.index=e,t.rank=p(t),t.attributes}function a(t){return t.replace(/(^\/+|\/+$)/g,"").split("/")}function c(t){return":"==t.charAt(0)?1+"*+?".indexOf(t.charAt(t.length-1))||4:5}function l(t){return a(t).map(c).join("")}function p(t){return t.attributes.default?0:l(t.attributes.path)}function s(t){return null!=t.__preactattr_||"undefined"!=typeof Symbol&&null!=t[Symbol.for("preactattr")]}function f(t,e){void 0===e&&(e="push"),k&&k[e]?k[e](t):"undefined"!=typeof history&&history[e+"State"]&&history[e+"State"](null,null,t)}function d(){var t;return t=k&&k.location?k.location:k&&k.getCurrentLocation?k.getCurrentLocation():"undefined"!=typeof location?location:N,""+(t.pathname||"")+(t.search||"")}function h(t,e){return void 0===e&&(e=!1),"string"!=typeof t&&t.url&&(e=t.replace,t=t.url),_(t)&&f(t,e?"replace":"push"),v(t)}function _(t){for(var e=x.length;e--;)if(x[e].canRoute(t))return!0;return!1}function v(t){for(var e=!1,n=0;n<x.length;n++)!0===x[n].routeTo(t)&&(e=!0);for(var o=j.length;o--;)j[o](t);return e}function m(t){if(t&&t.getAttribute){var e=t.getAttribute("href"),n=t.getAttribute("target");if(e&&e.match(/^\//g)&&(!n||n.match(/^_?self$/i)))return h(e)}}function b(t){if(0==t.button)return m(t.currentTarget||t.target||this),y(t)}function y(t){return t&&(t.stopImmediatePropagation&&t.stopImmediatePropagation(),t.stopPropagation&&t.stopPropagation(),t.preventDefault()),!1}function g(t){if(!(t.ctrlKey||t.metaKey||t.altKey||t.shiftKey||0!==t.button)){var e=t.target;do{if("A"===String(e.nodeName).toUpperCase()&&e.getAttribute("href")&&s(e)){if(e.hasAttribute("native"))return;if(m(e))return y(t)}}while(e=e.parentNode)}}function w(){U||("function"==typeof addEventListener&&(k||addEventListener("popstate",function(){v(d())}),addEventListener("click",g)),U=!0)}Object.defineProperty(e,"__esModule",{value:!0}),n.d(e,"subscribers",function(){return j}),n.d(e,"getCurrentUrl",function(){return d}),n.d(e,"route",function(){return h}),n.d(e,"Router",function(){return M}),n.d(e,"Route",function(){return P}),n.d(e,"Link",function(){return L});var C=n("KM04"),O=(n.n(C),{}),k=null,x=[],j=[],N={},U=!1,M=function(t){function e(e){t.call(this,e),e.history&&(k=e.history),this.state={url:e.url||d()},w()}return t&&(e.__proto__=t),e.prototype=Object.create(t&&t.prototype),e.prototype.constructor=e,e.prototype.shouldComponentUpdate=function(t){return!0!==t.static||(t.url!==this.props.url||t.onChange!==this.props.onChange)},e.prototype.canRoute=function(t){return this.getMatchingChildren(this.props.children,t,!1).length>0},e.prototype.routeTo=function(t){return this._didRoute=!1,this.setState({url:t}),this.updating?this.canRoute(t):(this.forceUpdate(),this._didRoute)},e.prototype.componentWillMount=function(){x.push(this),this.updating=!0},e.prototype.componentDidMount=function(){var t=this;k&&(this.unlisten=k.listen(function(e){t.routeTo(""+(e.pathname||"")+(e.search||""))})),this.updating=!1},e.prototype.componentWillUnmount=function(){"function"==typeof this.unlisten&&this.unlisten(),x.splice(x.indexOf(this),1)},e.prototype.componentWillUpdate=function(){this.updating=!0},e.prototype.componentDidUpdate=function(){this.updating=!1},e.prototype.getMatchingChildren=function(t,e,n){return t.filter(u).sort(i).map(function(t){var i=r(e,t.attributes.path,t.attributes);if(i){if(!1!==n){var u={url:e,matches:i};return o(u,i),delete u.ref,delete u.key,Object(C.cloneElement)(t,u)}return t}}).filter(Boolean)},e.prototype.render=function(t,e){var n=t.children,o=t.onChange,r=e.url,i=this.getMatchingChildren(n,r,!0),u=i[0]||null;this._didRoute=!!u;var a=this.previousUrl;return r!==a&&(this.previousUrl=r,"function"==typeof o&&o({router:this,url:r,previous:a,active:i,current:u})),u},e}(C.Component),L=function(t){return Object(C.h)("a",o({onClick:b},t))},P=function(t){return Object(C.h)(t.component,t)};M.subscribers=j,M.getCurrentUrl=d,M.route=h,M.Router=M,M.Route=P,M.Link=L,e.default=M},"7N8r":function(t,e,n){"use strict";e.__esModule=!0,e.default=function(t){function e(){var e=this;o.Component.call(this);var n=function(t){e.setState({child:t&&t.default||t})},r=t(n);r&&r.then&&r.then(n)}return e.prototype=new o.Component,e.prototype.constructor=e,e.prototype.render=function(t,e){return(0,o.h)(e.child,t)},e};var o=n("KM04")},JkW7:function(t,e,n){"use strict";function o(t,e){if(!t)throw new ReferenceError("this hasn't been initialised - super() hasn't been called");return!e||"object"!=typeof e&&"function"!=typeof e?t:e}function r(t,e){if("function"!=typeof e&&null!==e)throw new TypeError("Super expression must either be null or a function, not "+typeof e);t.prototype=Object.create(e&&e.prototype,{constructor:{value:t,enumerable:!1,writable:!0,configurable:!0}}),e&&(Object.setPrototypeOf?Object.setPrototypeOf(t,e):t.__proto__=e)}function i(t){n.e(0).then(function(){t(n("Ubzz"))}.bind(null,n)).catch(n.oe)}function u(t){n.e(1).then(function(){t(n("MRAa"))}.bind(null,n)).catch(n.oe)}function a(t){n.e(2).then(function(){t(n("s70X"))}.bind(null,n)).catch(n.oe)}function c(t){if(null==t)throw new TypeError("Cannot destructure undefined")}function l(t,e){if(!(t instanceof e))throw new TypeError("Cannot call a class as a function")}function p(t,e){if(!t)throw new ReferenceError("this hasn't been initialised - super() hasn't been called");return!e||"object"!=typeof e&&"function"!=typeof e?t:e}function s(t,e){if("function"!=typeof e&&null!==e)throw new TypeError("Super expression must either be null or a function, not "+typeof e);t.prototype=Object.create(e&&e.prototype,{constructor:{value:t,enumerable:!1,writable:!0,configurable:!0}}),e&&(Object.setPrototypeOf?Object.setPrototypeOf(t,e):t.__proto__=e)}Object.defineProperty(e,"__esModule",{value:!0});var f=(n("UjWa"),n("rq4c"),n("KM04")),d=n("/QC5"),h=n("sw5u"),_=n("u3et"),v=n.n(_),m=Object(f.h)("h1",null,"RDM"),b=function(t){function e(){return o(this,t.apply(this,arguments))}return r(e,t),e.prototype.render=function(){return Object(f.h)("header",{class:v.a.header},m,Object(f.h)("nav",null,Object(f.h)(h.Link,{activeClassName:v.a.active,href:"/"},"Home"),Object(f.h)(h.Link,{activeClassName:v.a.active,href:"/profile"},"Me"),Object(f.h)(h.Link,{activeClassName:v.a.active,href:"/profile/john"},"John")))},e}(f.Component),y=n("7N8r"),g=n.n(y),w=g()(i),C=g()(u),O=g()(a),k=Object(f.h)(b,null),x=Object(f.h)(O,{path:"/add-links"}),j=Object(f.h)(C,{path:"/profile/",user:"me"}),N=Object(f.h)(C,{path:"/profile/:user"}),U=function(t){function e(){var n,o,r;l(this,e);for(var i=arguments.length,u=Array(i),a=0;a<i;a++)u[a]=arguments[a];return n=o=p(this,t.call.apply(t,[this].concat(u))),o.state={dloads:[]},o.handleRoute=function(t){o.currentUrl=t.url},r=n,p(o,r)}return s(e,t),e.prototype.loadLinks=function(){var t=this;fetch("http://"+window.location.hostname+":8000/api/downloads").then(function(t){return t.json()}).then(function(e){return t.setState({dloads:e})})},e.prototype.ws=function(){var t=this;new WebSocket("ws://"+window.location.hostname+":8001").onmessage=function(e){var n=JSON.parse(e.data);console.log(n),t.setState({dloads:n})}},e.prototype.componentDidMount=function(){this.loadLinks(),this.ws()},e.prototype.render=function(t,e){var n=e.dloads;return c(t),Object(f.h)("div",{id:"app"},k,Object(f.h)(d.Router,{onChange:this.handleRoute},Object(f.h)(w,{path:"/",dloads:n}),x,j,N))},e}(f.Component);e.default=U},KM04:function(t){!function(){"use strict";function e(){}function n(t,n){var o,r,i,u,a=S;for(u=arguments.length;u-- >2;)R.push(arguments[u]);for(n&&null!=n.children&&(R.length||R.push(n.children),delete n.children);R.length;)if((r=R.pop())&&void 0!==r.pop)for(u=r.length;u--;)R.push(r[u]);else"boolean"==typeof r&&(r=null),(i="function"!=typeof t)&&(null==r?r="":"number"==typeof r?r+="":"string"!=typeof r&&(i=!1)),i&&o?a[a.length-1]+=r:a===S?a=[r]:a.push(r),o=i;var c=new e;return c.nodeName=t,c.children=a,c.attributes=null==n?void 0:n,c.key=null==n?void 0:n.key,void 0!==P.vnode&&P.vnode(c),c}function o(t,e){for(var n in e)t[n]=e[n];return t}function r(t,e){return n(t.nodeName,o(o({},t.attributes),e),arguments.length>2?[].slice.call(arguments,2):t.children)}function i(t){!t.__d&&(t.__d=!0)&&1==W.push(t)&&(P.debounceRendering||E)(u)}function u(){var t,e=W;for(W=[];t=e.pop();)t.__d&&j(t)}function a(t,e,n){return"string"==typeof e||"number"==typeof e?void 0!==t.splitText:"string"==typeof e.nodeName?!t._componentConstructor&&c(t,e.nodeName):n||t._componentConstructor===e.nodeName}function c(t,e){return t.__n===e||t.nodeName.toLowerCase()===e.toLowerCase()}function l(t){var e=o({},t.attributes);e.children=t.children;var n=t.nodeName.defaultProps;if(void 0!==n)for(var r in n)void 0===e[r]&&(e[r]=n[r]);return e}function p(t,e){var n=e?document.createElementNS("http://www.w3.org/2000/svg",t):document.createElement(t);return n.__n=t,n}function s(t){var e=t.parentNode;e&&e.removeChild(t)}function f(t,e,n,o,r){if("className"===e&&(e="class"),"key"===e);else if("ref"===e)n&&n(null),o&&o(t);else if("class"!==e||r)if("style"===e){if(o&&"string"!=typeof o&&"string"!=typeof n||(t.style.cssText=o||""),o&&"object"==typeof o){if("string"!=typeof n)for(var i in n)i in o||(t.style[i]="");for(var i in o)t.style[i]="number"==typeof o[i]&&!1===T.test(i)?o[i]+"px":o[i]}}else if("dangerouslySetInnerHTML"===e)o&&(t.innerHTML=o.__html||"");else if("o"==e[0]&&"n"==e[1]){var u=e!==(e=e.replace(/Capture$/,""));e=e.toLowerCase().substring(2),o?n||t.addEventListener(e,h,u):t.removeEventListener(e,h,u),(t.__l||(t.__l={}))[e]=o}else if("list"!==e&&"type"!==e&&!r&&e in t)d(t,e,null==o?"":o),null!=o&&!1!==o||t.removeAttribute(e);else{var a=r&&e!==(e=e.replace(/^xlink\:?/,""));null==o||!1===o?a?t.removeAttributeNS("http://www.w3.org/1999/xlink",e.toLowerCase()):t.removeAttribute(e):"function"!=typeof o&&(a?t.setAttributeNS("http://www.w3.org/1999/xlink",e.toLowerCase(),o):t.setAttribute(e,o))}else t.className=o||""}function d(t,e,n){try{t[e]=n}catch(t){}}function h(t){return this.__l[t.type](P.event&&P.event(t)||t)}function _(){for(var t;t=A.pop();)P.afterMount&&P.afterMount(t),t.componentDidMount&&t.componentDidMount()}function v(t,e,n,o,r,i){D++||(I=null!=r&&void 0!==r.ownerSVGElement,K=null!=t&&!("__preactattr_"in t));var u=m(t,e,n,o,i);return r&&u.parentNode!==r&&r.appendChild(u),--D||(K=!1,i||_()),u}function m(t,e,n,o,r){var i=t,u=I;if(null!=e&&"boolean"!=typeof e||(e=""),"string"==typeof e||"number"==typeof e)return t&&void 0!==t.splitText&&t.parentNode&&(!t._component||r)?t.nodeValue!=e&&(t.nodeValue=e):(i=document.createTextNode(e),t&&(t.parentNode&&t.parentNode.replaceChild(i,t),y(t,!0))),i.__preactattr_=!0,i;var a=e.nodeName;if("function"==typeof a)return N(t,e,n,o);if(I="svg"===a||"foreignObject"!==a&&I,a+="",(!t||!c(t,a))&&(i=p(a,I),t)){for(;t.firstChild;)i.appendChild(t.firstChild);t.parentNode&&t.parentNode.replaceChild(i,t),y(t,!0)}var l=i.firstChild,s=i.__preactattr_,f=e.children;if(null==s){s=i.__preactattr_={};for(var d=i.attributes,h=d.length;h--;)s[d[h].name]=d[h].value}return!K&&f&&1===f.length&&"string"==typeof f[0]&&null!=l&&void 0!==l.splitText&&null==l.nextSibling?l.nodeValue!=f[0]&&(l.nodeValue=f[0]):(f&&f.length||null!=l)&&b(i,f,n,o,K||null!=s.dangerouslySetInnerHTML),w(i,e.attributes,s),I=u,i}function b(t,e,n,o,r){var i,u,c,l,p,f=t.childNodes,d=[],h={},_=0,v=0,b=f.length,g=0,w=e?e.length:0;if(0!==b)for(var C=0;C<b;C++){var O=f[C],k=O.__preactattr_,x=w&&k?O._component?O._component.__k:k.key:null;null!=x?(_++,h[x]=O):(k||(void 0!==O.splitText?!r||O.nodeValue.trim():r))&&(d[g++]=O)}if(0!==w)for(var C=0;C<w;C++){l=e[C],p=null;var x=l.key;if(null!=x)_&&void 0!==h[x]&&(p=h[x],h[x]=void 0,_--);else if(!p&&v<g)for(i=v;i<g;i++)if(void 0!==d[i]&&a(u=d[i],l,r)){p=u,d[i]=void 0,i===g-1&&g--,i===v&&v++;break}p=m(p,l,n,o),c=f[C],p&&p!==t&&p!==c&&(null==c?t.appendChild(p):p===c.nextSibling?s(c):t.insertBefore(p,c))}if(_)for(var C in h)void 0!==h[C]&&y(h[C],!1);for(;v<=g;)void 0!==(p=d[g--])&&y(p,!1)}function y(t,e){var n=t._component;n?U(n):(null!=t.__preactattr_&&t.__preactattr_.ref&&t.__preactattr_.ref(null),!1!==e&&null!=t.__preactattr_||s(t),g(t))}function g(t){for(t=t.lastChild;t;){var e=t.previousSibling;y(t,!0),t=e}}function w(t,e,n){var o;for(o in n)e&&null!=e[o]||null==n[o]||f(t,o,n[o],n[o]=void 0,I);for(o in e)"children"===o||"innerHTML"===o||o in n&&e[o]===("value"===o||"checked"===o?t[o]:n[o])||f(t,o,n[o],n[o]=e[o],I)}function C(t){var e=t.constructor.name;($[e]||($[e]=[])).push(t)}function O(t,e,n){var o,r=$[t.name];if(t.prototype&&t.prototype.render?(o=new t(e,n),M.call(o,e,n)):(o=new M(e,n),o.constructor=t,o.render=k),r)for(var i=r.length;i--;)if(r[i].constructor===t){o.__b=r[i].__b,r.splice(i,1);break}return o}function k(t,e,n){return this.constructor(t,n)}function x(t,e,n,o,r){t.__x||(t.__x=!0,(t.__r=e.ref)&&delete e.ref,(t.__k=e.key)&&delete e.key,!t.base||r?t.componentWillMount&&t.componentWillMount():t.componentWillReceiveProps&&t.componentWillReceiveProps(e,o),o&&o!==t.context&&(t.__c||(t.__c=t.context),t.context=o),t.__p||(t.__p=t.props),t.props=e,t.__x=!1,0!==n&&(1!==n&&!1===P.syncComponentUpdates&&t.base?i(t):j(t,1,r)),t.__r&&t.__r(t))}function j(t,e,n,r){if(!t.__x){var i,u,a,c=t.props,p=t.state,s=t.context,f=t.__p||c,d=t.__s||p,h=t.__c||s,m=t.base,b=t.__b,g=m||b,w=t._component,C=!1;if(m&&(t.props=f,t.state=d,t.context=h,2!==e&&t.shouldComponentUpdate&&!1===t.shouldComponentUpdate(c,p,s)?C=!0:t.componentWillUpdate&&t.componentWillUpdate(c,p,s),t.props=c,t.state=p,t.context=s),t.__p=t.__s=t.__c=t.__b=null,t.__d=!1,!C){i=t.render(c,p,s),t.getChildContext&&(s=o(o({},s),t.getChildContext()));var k,N,M=i&&i.nodeName;if("function"==typeof M){var L=l(i);u=w,u&&u.constructor===M&&L.key==u.__k?x(u,L,1,s,!1):(k=u,t._component=u=O(M,L,s),u.__b=u.__b||b,u.__u=t,x(u,L,0,s,!1),j(u,1,n,!0)),N=u.base}else a=g,k=w,k&&(a=t._component=null),(g||1===e)&&(a&&(a._component=null),N=v(a,i,s,n||!m,g&&g.parentNode,!0));if(g&&N!==g&&u!==w){var R=g.parentNode;R&&N!==R&&(R.replaceChild(N,g),k||(g._component=null,y(g,!1)))}if(k&&U(k),t.base=N,N&&!r){for(var S=t,E=t;E=E.__u;)(S=E).base=N;N._component=S,N._componentConstructor=S.constructor}}if(!m||n?A.unshift(t):C||(t.componentDidUpdate&&t.componentDidUpdate(f,d,h),P.afterUpdate&&P.afterUpdate(t)),null!=t.__h)for(;t.__h.length;)t.__h.pop().call(t);D||r||_()}}function N(t,e,n,o){for(var r=t&&t._component,i=r,u=t,a=r&&t._componentConstructor===e.nodeName,c=a,p=l(e);r&&!c&&(r=r.__u);)c=r.constructor===e.nodeName;return r&&c&&(!o||r._component)?(x(r,p,3,n,o),t=r.base):(i&&!a&&(U(i),t=u=null),r=O(e.nodeName,p,n),t&&!r.__b&&(r.__b=t,u=null),x(r,p,1,n,o),t=r.base,u&&t!==u&&(u._component=null,y(u,!1))),t}function U(t){P.beforeUnmount&&P.beforeUnmount(t);var e=t.base;t.__x=!0,t.componentWillUnmount&&t.componentWillUnmount(),t.base=null;var n=t._component;n?U(n):e&&(e.__preactattr_&&e.__preactattr_.ref&&e.__preactattr_.ref(null),t.__b=e,s(e),C(t),g(e)),t.__r&&t.__r(null)}function M(t,e){this.__d=!0,this.context=e,this.props=t,this.state=this.state||{}}function L(t,e,n){return v(n,t,{},!1,e,!1)}var P={},R=[],S=[],E="function"==typeof Promise?Promise.resolve().then.bind(Promise.resolve()):setTimeout,T=/acit|ex(?:s|g|n|p|$)|rph|ows|mnc|ntw|ine[ch]|zoo|^ord/i,W=[],A=[],D=0,I=!1,K=!1,$={};o(M.prototype,{setState:function(t,e){var n=this.state;this.__s||(this.__s=o({},n)),o(n,"function"==typeof t?t(n,this.props):t),e&&(this.__h=this.__h||[]).push(e),i(this)},forceUpdate:function(t){t&&(this.__h=this.__h||[]).push(t),j(this,2)},render:function(){}});var J={h:n,createElement:n,cloneElement:r,Component:M,render:L,rerender:u,options:P};t.exports=J}()},UjWa:function(){},pwNi:function(t,e,n){"use strict";var o=n("KM04");"serviceWorker"in navigator&&"https:"===location.protocol&&navigator.serviceWorker.register(n.p+"sw.js");var r=function(t){return t&&t.default?t.default:t};if("function"==typeof r(n("JkW7"))){var i=document.body.firstElementChild,u=function(){var t=r(n("JkW7"));i=(0,o.render)((0,o.h)(t),document.body,i)};u()}},rq4c:function(){},sw5u:function(t,e,n){"use strict";function o(t,e){var n={};for(var o in t)e.indexOf(o)>=0||Object.prototype.hasOwnProperty.call(t,o)&&(n[o]=t[o]);return n}function r(t,e){if(!t)throw new ReferenceError("this hasn't been initialised - super() hasn't been called");return!e||"object"!=typeof e&&"function"!=typeof e?t:e}function i(t,e){if("function"!=typeof e&&null!==e)throw new TypeError("Super expression must either be null or a function, not "+typeof e);t.prototype=Object.create(e&&e.prototype,{constructor:{value:t,enumerable:!1,writable:!0,configurable:!0}}),e&&(Object.setPrototypeOf?Object.setPrototypeOf(t,e):t.__proto__=e)}Object.defineProperty(e,"__esModule",{value:!0}),e.Link=e.Match=void 0;var u=Object.assign||function(t){for(var e=1;e<arguments.length;e++){var n=arguments[e];for(var o in n)Object.prototype.hasOwnProperty.call(n,o)&&(t[o]=n[o])}return t},a=n("KM04"),c=n("/QC5"),l=e.Match=function(t){function e(){for(var e,n,o,i=arguments.length,u=Array(i),a=0;a<i;a++)u[a]=arguments[a];return e=n=r(this,t.call.apply(t,[this].concat(u))),n.update=function(t){n.nextUrl=t,n.setState({})},o=e,r(n,o)}return i(e,t),e.prototype.componentDidMount=function(){c.subscribers.push(this.update)},e.prototype.componentWillUnmount=function(){c.subscribers.splice(c.subscribers.indexOf(this.update)>>>0,1)},e.prototype.render=function(t){var e=this.nextUrl||(0,c.getCurrentUrl)(),n=e.replace(/\?.+$/,"");return this.nextUrl=null,t.children[0]&&t.children[0]({url:e,path:n,matches:n===t.path})},e}(a.Component),p=function(t){var e=t.activeClassName,n=t.path,r=o(t,["activeClassName","path"]);return(0,a.h)(l,{path:n||r.href},function(t){var n=t.matches;return(0,a.h)(c.Link,u({},r,{class:[r.class||r.className,n&&e].filter(Boolean).join(" ")}))})};e.Link=p,e.default=l,l.Link=p},u3et:function(t){t.exports={header:"header__3QGkI",active:"active__3gItZ"}}});
//# sourceMappingURL=bundle.8ec35.js.map