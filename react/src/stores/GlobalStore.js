import { observable } from "mobx";
import NotifyStore from "./NotifyStore";
import UiStore from "./UiStore";
import DloadStore from "./DloadStore";
import ConfigStore from "./ConfigStore";

class GlobalStore {
    server;
    addon;

    @observable notify;
    @observable config;
    @observable ui;
    @observable dload;

    constructor() {
        // check if we are in addon mode or not
        this.addon = (window.chrome && chrome.runtime && chrome.runtime.id != undefined);

        // when not in addon mode, set server adress from url request
        if (this.addon == false) {
            this.server = window.location.host;
            this.initStores();
        }
        // when in addon mode, get the adress from the addon store
        else {
            var that = this;
            chrome.storage.local.get(["spe3d_server"], function (result) {
                that.server = result.spe3d_server;
                that.initStores();
            });
        }
    }

    // set up all the stores
    initStores() {
        this.notify = new NotifyStore();
        this.config = new ConfigStore(this);
        this.ui = new UiStore(this);
        this.dload = new DloadStore(this);
    }
}

const global = window.global = new GlobalStore();
export default global;