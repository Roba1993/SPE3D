export default class Config {
    store;

    constructor(store) {
        this.store = store;
    }

    getConfig() {
        // return the async function
        return fetch(`http://` + this.store.server + `/api/config`)
            .then(res => {
                // only the 200 status indicates that every went correct, every other message leads to an error
                if (res.status != 200) {
                    throw { error: "No 200 header returned", details: res };
                }

                return res.json()
            })
            .catch(error => {
                this.store.notify.createErrorMsg("Connection to server failed", "Can't get the config from server");
                throw error;
            });
    }

    updateConfigServer(server) {
        fetch("http://" + this.store.server + "/api/config/server",
            {
                method: "POST",
                headers: {
                    'Accept': 'application/json, text/plain, */*',
                    'Content-Type': 'application/json'
                },
                body: JSON.stringify(server)
            })
            .then(res => {
                // only the 200 status indicates that every went correct, every other message leads to an error
                if (res.status != 200) {
                    throw { error: "No 200 header returned", details: res };
                }

                this.store.notify.createOkMsg("Server settings updated", "The server successfully updated the server settings");
            })
            .catch(error => {
                console.error("The function 'removeDloadById' returned the following error:");
                console.error(error);
                this.store.notify.createErrorMsg("Deletion failed", "The server was not able to remove the link");
            });
    }

    addAccount(acc) {
        fetch("http://" + this.store.server + "/api/config/account",
            {
                method: "POST",
                headers: {
                    'Accept': 'application/json, text/plain, */*',
                    'Content-Type': 'application/json'
                },
                body: JSON.stringify(acc)
            })
            .then(res => {
                // only the 200 status indicates that every went correct, every other message leads to an error
                if (res.status != 200) {
                    throw { error: "No 200 header returned", details: res };
                }

                this.store.notify.createOkMsg("Account added", "The server successfully added a hoster account");
            })
            .catch(error => {
                console.error("The function 'removeDloadById' returned the following error:");
                console.error(error);
                this.store.notify.createErrorMsg("Add of Account failed", "The server was not able to interpret the account settings");
            });
    }

    removeAccount(id) {
        fetch("http://" + this.store.server + "/api/config/account/" + id,
            {
                method: "DELETE",
                headers: {
                    'Accept': 'application/json, text/plain, */*'
                },
            })
            .then(res => {
                // only the 200 status indicates that every went correct, every other message leads to an error
                if (res.status != 200) {
                    throw { error: "No 200 header returned", details: res };
                }

                this.store.notify.createOkMsg("Account removed", "The server successfully removed a hoster account");
            })
            .catch(error => {
                console.error("The function 'removeDloadById' returned the following error:");
                console.error(error);
                this.store.notify.createErrorMsg("Removing of Account failed", "The server was not able to remove the account");
            });
    }
}