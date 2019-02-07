
export default class WsRest {
    global;
    connection;

    constructor(global_store) {
        this.global = global_store;

        this.connect();
    }

    connect() {
        this.connection = new WebSocket('ws://' + this.global.config.server_remote + ':8000/updates');

        this.connection.onmessage = (evt) => {
            var msg = JSON.parse(evt.data);

            // process download list data
            if(msg.DownloadList != undefined) {
                this.global.dload.replaceDloads(msg.DownloadList);
            }
            // process download speed data
            else if(msg.DownloadSpeed != undefined) {
                // data is send as an array where position 0 is the file and 
                // position 1 is the speed per second
                var file = this.global.dloads.getFileById(msg.DownloadSpeed[0]);
                file.downloaded += msg.DownloadSpeed[1];
                file.speed = msg.DownloadSpeed[1];
            }
        }
    }
}