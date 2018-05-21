import { observable } from "mobx";
import NotifyStore from "./NotifyStore";
import UiStore from "./UiStore";
import DloadStore from "./DloadStore";

class GlobalStore {
    @observable notify = new NotifyStore();
    @observable ui = new UiStore();
    @observable dload = new DloadStore(this);
}

const global = window.global = new GlobalStore();
export default global;