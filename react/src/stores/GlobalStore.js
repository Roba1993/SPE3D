import { observable } from "mobx";
import NotifyStore from "./NotifyStore";
import UiStore from "./UiStore";
import DloadStore from "./DloadStore";
import ConfigStore from "./ConfigStore";

class GlobalStore {
    @observable notify = new NotifyStore();
    @observable config = new ConfigStore(this);
    @observable ui = new UiStore(this);
    @observable dload = new DloadStore(this);
}

const global = window.global = new GlobalStore();
export default global;