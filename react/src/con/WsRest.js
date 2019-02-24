
export default class WsRest {
    store;
    connection;

    constructor(store) {
        this.store = store;

        this.connect();
    }

    connect() {
        this.connection = new WebSocket('ws://' + this.store.server + '/updates');

        this.connection.onmessage = (evt) => {
            var msg = JSON.parse(evt.data);

            // process download list data
            if(msg.DownloadList != undefined) {
                this.store.dload.replaceDloads(msg.DownloadList);
            }
            // process download speed data
            else if(msg.DownloadSpeed != undefined) {
                // data is send as an array where position 0 is the file and 
                // position 1 is the speed per second
                var file = this.store.dload.getFileById(msg.DownloadSpeed[0]);
                file.downloaded += msg.DownloadSpeed[1];
                file.speed = msg.DownloadSpeed[1];
            }
        }
    }
}