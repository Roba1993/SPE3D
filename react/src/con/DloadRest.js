export default class DloadRest {
    global;

    constructor(global_store) {
        this.global = global_store;
    }

    startDloadById(id) {
        if (!id) {
            console.error("No 'id' was given for function call 'startDloadById'");
            throw "No 'id' was given for function call 'startDloadById'";
        }

        fetch("http://" + window.location.hostname + ":8000/api/start-download/" + id,
            {
                method: "POST"
            })
            .then(res => {
                // only the 200 status indicates that every went correct, every other message leads to an error
                if (res.status != 200) {
                    throw { error: "No 200 header returned", details: res };
                }
            })
            .catch(error => {
                console.error("The function 'startDloadById' returned the following error:");
                console.error(error);
                this.global.notify.createErrorMsg("Download not started", "The server was not able to start the download");
            });
    }

    removeDloadById(id) {
        if (!id) {
            console.error("No 'id' was given for function call 'removeDloadById'");
            throw "No 'id' was given for function call 'removeDloadById'";
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
                // only the 200 status indicates that every went correct, every other message leads to an error
                if (res.status != 200) {
                    throw { error: "No 200 header returned", details: res };
                }
            })
            .catch(error => {
                console.error("The function 'removeDloadById' returned the following error:");
                console.error(error);
                this.global.notify.createErrorMsg("Deletion failed", "The server was not able to remove the link");
            });
    }

    getDloads() {
        // return the async function
        return fetch(`http://` + window.location.hostname + `:8000/api/downloads`)
            .then(res => {
                // only the 200 status indicates that every went correct, every other message leads to an error
                if (res.status != 200) {
                    throw { error: "No 200 header returned", details: res };
                }

                return res.json()
            })
            .catch(error => {
                this.global.notify.createErrorMsg("Connection to server failed", "Can't get the download list from server");
                throw error;
            });
    }
}