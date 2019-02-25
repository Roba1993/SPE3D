import { observable, computed, action } from "mobx";
import HosterImage from '../asset/hoster/hoster';
import Config from '../con/Config';

export default class ConfigStore {
    global;
    hoster;
    con;

    @observable server;
    @observable server_online = false;
    @observable accounts = [];

    constructor(global) {
        this.global = global;
        this.hoster = [
            { key: 'so', text: 'Share-Online.biz', value: 'ShareOnline', img: HosterImage.shareonline() },
            { key: 'filer', text: 'Filer.net', value: 'Filer', img: HosterImage.filer() },
        ];

        this.updateConfig();
    }

    updateConfig() {
        this.con = new Config(this.global);

        var that = this;
        this.con.getConfig().then(res => {
            that.server = new Server(res.server);

            var acc = [];
            res.accounts.forEach(c => {
                acc.push(new Account(c))
            });
            that.accounts.replace(acc);
        })
    }

    getHosterImage(hoster) {
        for (var h of this.hoster) {
            if (h.value == hoster) {
                return h.img;
            }
        }

        return HosterImage.other();
    }

    updateServer() {
        fetch("http://" + this.global.server + "/api/config/server",
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
        fetch("http://" + this.global.server + "/api/config/account",
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
                    this.fetchConfig();
                }
            })
    }

    removeAccount(id) {
        fetch("http://" + this.global.server + "/api/config/account/" + id,
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
}

class Server {
    @observable ip;
    @observable port;

    constructor(rawObj) {
        this.ip = rawObj.ip;
        this.port = rawObj.port;
    }

    @action setPort(port) {
        this.port = parseInt(port);
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