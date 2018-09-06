import { observable } from "mobx";

export default class ConfigStore {
    global;
    @observable server;
    @observable share_online = [];


    constructor() {
        this.fetchConfig();
    }

    replaceConfig(rawObj) {
        this.server = new Server(rawObj.server);

        var so = [];
        rawObj.share_online.forEach(c => {
            so.push(new ShareOnline(c))
        });
        this.share_online.replace(so);
    }

    fetchConfig() {
        fetch(`http://` + window.location.hostname + `:8000/api/config`)
            .then(res => {
                if (res.status != 200) {
                    this.global.notify.createErrorMsg("Download list not avialable", "The server was not able to provide the config");
                }

                return res.json()
            })
            .then(config => {
                this.replaceConfig(config);
            })
            .catch(error => {
                this.global.notify.createErrorMsg("Connection to server failed", "Can't get the config list from server");
            });
    }
}

class Server {
    @observable ip;
    @observable webserver_port;
    @observable websocket_port;

    constructor(rawObj) {
        this.ip = rawObj.ip;
        this.webserver_port = rawObj.webserver_port;
        this.websocket_port = rawObj.websocket_port;    
    }
}

class ShareOnline {
    @observable username;
    @observable password;

    constructor(rawObj) {
        this.username = rawObj.username;
        this.password = rawObj.password;
    }
}