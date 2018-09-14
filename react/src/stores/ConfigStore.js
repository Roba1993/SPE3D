import { observable, computed } from "mobx";

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

    addAccount(acc) {
        fetch("http://" + window.location.hostname + ":8000/api/config/account",
            {
                method: "POST",
                headers: {
                    'Accept': 'application/json, text/plain, */*',
                    'Content-Type': 'application/json'
                },
                body: JSON.stringify(acc)
            })
            .then(res => {
                if (res.status != 200) {
                    this.global.notify.createErrorMsg("Add of Account failed", "The server was not able to interpret the account settings");
                }
                else {
                    this.global.notify.createOkMsg("Account added", "The server successfully added a hoster account");
                }
            })
    }

    removeAccount(id) {
        fetch("http://" + window.location.hostname + ":8000/api/config/account/"+id,
            {
                method: "DELETE",
                headers: {
                    'Accept': 'application/json, text/plain, */*'
                },
            })
            .then(res => {
                if (res.status != 200) {
                    this.global.notify.createErrorMsg("Removing of Account failed", "The server was not able to remove the account");
                }
                else {
                    this.global.notify.createOkMsg("Account removed", "The server successfully removed a hoster account");
                    this.fetchConfig();
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
    @observable id;
    @observable hoster;
    @observable username;
    @observable password;
    @observable status;
    @observable raw_checked;
    @computed get checked() { return formatTime((new Date() / 1000) - this.raw_checked.secs_since_epoch) }

    constructor(rawObj) {
        this.id = rawObj.id;
        this.hoster = rawObj.hoster;
        this.username = rawObj.username;
        this.password = rawObj.password;
        this.status = rawObj.status;
        this.raw_checked = rawObj.checked;
    }
}


function formatTime(seconds) {
    if (seconds < 60) return seconds.toFixed(0) + ' Seconds';

    var min = (seconds / 60).toFixed(0);
    if (min < 60) {
        return min + " Minutes";
    }

    var hours = (min / 60).toFixed(0);
    if (hours < 24) {
        return hours + " Hours";
    }

    return (hours / 24).toFixed(2) + " Days";
}