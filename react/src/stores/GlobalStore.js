import { observable } from "mobx";
import NotifyStore from "./NotifyStore";
import UiStore from "./UiStore";
import DloadStore from "./DloadStore";
import ConfigStore from "./ConfigStore";

class GlobalStore {
    @observable notify = new NotifyStore();
    @observable ui = new UiStore(this);
    @observable dload = new DloadStore(this);
    @observable config = new ConfigStore();
}

const global = window.global = new GlobalStore();
export default global;