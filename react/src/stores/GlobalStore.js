import { observable } from "mobx";
import NotifyStore from "./NotifyStore";
import UiStore from "./UiStore";

class GlobalStore {
    @observable notify = new NotifyStore();
    @observable ui = new UiStore();
}

const global = window.global = new GlobalStore();
export default global;