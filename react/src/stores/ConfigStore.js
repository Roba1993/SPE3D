import { observable, toJS } from "mobx";

export default class ConfigStore {
    global;
    @observable server;
    @observable share_online = [];


    constructor(global) {
        this.global = global;
        this.fetchConfig();
    }

    updateServer() {
        fetch("http://" + window.location.hostname + ":8000/api/config/server",
            {
                method: "POST",
                headers: {
                    'Accept': 'application/json, text/plain, */*',
                    'Content-Type': 'application/json'
                },
                body: JSON.stringify(this.server)
            })
            .then(res => {
                if (res.status != 200) {
                    this.global.notify.createErrorMsg("Update of server settings failed", "The server was not able to interpret the server settings");
                }
                else {
                    this.global.notify.createOkMsg("Server settings updated", "The server successfully updated the server settings");
                }
            })
    }

    replaceConfig(rawObj) {
        this.server = new Server(rawObj.server);

        var so = [];
        rawObj.accounts.forEach(c => {
            so.push(new Account(c))
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

class Account {
    @observable hoster;
    @observable username;
    @observable password;

    constructor(rawObj) {
        this.hoster = rawObj.hoster;
        this.username = rawObj.username;
        this.password = rawObj.password;
    }
}