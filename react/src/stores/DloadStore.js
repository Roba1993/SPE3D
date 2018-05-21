import { computed, observable } from "mobx";

export default class DloadStore {
    global;
    @observable dloads = [];

    constructor(global) {
        this.global = global;

        this.fetchDloads();
        this.websocket();
    }

    getContainer(id) {
        return this.dloads.find(c => c.id == id);
    }

    startDload(id) {
        if (!id) {
            return;
        }

        fetch("http://" + window.location.hostname + ":8000/api/start-download/" + id,
            {
                method: "POST"
            })
            .then(res => { 
                if (res.status != 200) {
                    this.global.notify.createErrorMsg("Download not started", "The server was not able to start the download");
                }
            })
    }

    removeDload(id) {
        if (!id) {
            return;
        }

        fetch("http://" + window.location.hostname + ":8000/api/delete-link/" + id,
            {
                method: "POST",
                headers: {
                    'Accept': 'application/json, text/plain, */*',
                    'Content-Type': 'application/json'
                },
            })
            .then(res => {
                if (res.status != 200) {
                    this.global.notify.createErrorMsg("Deletion failed", "The server was not able to remove the link");
                }
            });
    }

    replaceDloads(rawObj) {
        var dloads = [];

        rawObj.forEach(c => {
            dloads.push(new Container(c))
        });

        this.dloads.replace(dloads);
    }

    fetchDloads() {
        fetch(`http://` + window.location.hostname + `:8000/api/downloads`)
            .then(res => {
                if (res.status != 200) {
                    this.global.notify.createErrorMsg("Download list not avialable", "The server was not able to provide the download list");
                }

                return res.json()
            })
            .then(dloads => {
                this.replaceDloads(dloads);
                //console.log(dloads);
                //console.log(this.dloads);
            })
            .catch(error => {
                this.global.notify.createErrorMsg("Connection to server failed", "Can't get the download list from server");
            });
    }

    websocket() {
        var websocket = new WebSocket('ws://' + window.location.hostname + ':8001');
        websocket.onmessage = (evt) => {
            this.replaceDloads(JSON.parse(evt.data))
        }
    }
}

class Container {
    @observable id;
    @observable name;
    @observable files;
    @computed get size() { return this.files.reduce((pre, curr) => pre + curr.size, 0); }
    @computed get sizeFmt() { return formatBytes(this.size, 2); }
    @computed get speed() { return this.files.reduce((pre, curr) => pre + curr.speed, 0); }
    @computed get speedFmt() { return formatBytes(this.speed, 2); }
    @computed get downloaded() { return this.files.reduce((pre, curr) => pre + curr.downloaded, 0); }
    @computed get downloadedFmt() { return formatBytes(this.downloaded, 2); }
    @computed get downloadedPercent() { return (this.size != 0) ? (this.downloaded / this.size * 100).toFixed(0): 0; }
    @computed get finishedDownloads() { return this.files.reduce((pre, curr) => (curr.status == "Downloaded") ? pre += 1 : pre, 0); }
    @computed get isDownloading() { return this.files.some(f => f.status == "Downloading"); }
    @computed get isDownloaded() { return this.files.every(f => f.status == "Downloaded"); }
    @computed get isWarning() { return this.files.some(f => f.status == "WrongHash"); }
    @computed get downloadTime() { return (this.speed != 0) ? formatTime((this.size / this.speed)) : (this.isDownloaded) ? 'Done' : 'Not Started'; }
    @computed get icon() { return (this.isDownloaded) ? 'check' :(this.isWarning) ? 'warning sign' :(this.isDownloading) ? 'spinner' : 'arrow down'; }

    constructor(rawObj) {
        this.id = rawObj.id;
        this.name = rawObj.name;
        this.files = [];

        rawObj.files.forEach(f => {
            this.files.push(new File(f));
        });
    }
}

class File {
    @observable id;
    @observable name;
    @observable downloaded;
    @observable hash;
    @observable host;
    @observable infos;
    @observable size;
    @observable speed;
    @observable status;
    @observable url;
    @computed get sizeFmt() { return formatBytes(this.size, 2); }
    @computed get speedFmt() { return formatBytes(this.speed, 2); }
    @computed get isDownloading() { return this.status == "Downloading"; }
    @computed get isDownloaded() { return this.status == "Downloaded"; }
    @computed get isWarning() { return this.status == "WrongHash"; }
    @computed get downloadedPercent() { return (this.size != 0) ? (this.downloaded / this.size * 100).toFixed(0): 0; }
    @computed get downloadTime() { return (this.speed != 0) ? formatTime((this.size / this.speed)) : (this.isDownloaded) ? 'Done' : 'Not Started'; }
    @computed get icon() { return (this.isDownloaded) ? 'check' :(this.isWarning) ? 'warning sign' :(this.isDownloading) ? 'spinner' : 'arrow down'; }

    constructor(rawObj) {
        this.id = rawObj.id
        this.name = rawObj.name
        this.downloaded = rawObj.downloaded
        this.hash = rawObj.hash
        this.host = rawObj.host
        this.infos = rawObj.infos
        this.size = rawObj.size
        this.speed = rawObj.speed
        this.status = rawObj.status
        this.url = rawObj.url
    }
}

function formatBytes(bytes, decimals) {
    if (bytes == 0) return '0 Bytes';
    var k = 1024,
        dm = decimals || 2,
        sizes = ['Bytes', 'Kb', 'Mb', 'Gb', 'Tb', 'Pb', 'Eb', 'Zb', 'Yb'],
        i = Math.floor(Math.log(bytes) / Math.log(k));
    return parseFloat((bytes / Math.pow(k, i)).toFixed(dm)) + ' ' + sizes[i];
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