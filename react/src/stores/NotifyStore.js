import { computed, observable } from "mobx";

class Message {
    @observable id;
    @observable name;
    @observable description;
    @observable type;

    @computed get icon() {
        if(this.type == "error") {
            return "warning sign"
        }
        else if(this.type == "ok") {
            return "check circle outline"
        }
    }

    constructor(name, description, type) {
        this.id = Date.now();
        this.name = name;
        this.description = description;
        this.type = type;
    }
}

export default class NotifyStore {
    @observable messages = [];

    createErrorMsg(name, description) {
        const msg = new Message(name, description, "error");
        this.messages.push(msg);

        setTimeout(() => {
            this.deleteMsg(msg.id)
        }, 9000)
    }

    createOkMsg(name, description) {
        const msg = new Message(name, description, "ok");
        this.messages.push(msg);

        setTimeout(() => {
            this.deleteMsg(msg.id)
        }, 3000)
    }

    deleteMsg(id) {
        const newMsg = this.messages.filter(msg => msg.id != id);
        this.messages.replace(newMsg);
    }
}