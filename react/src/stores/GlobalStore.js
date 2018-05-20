import { observable } from "mobx";
import NotifyStore from "./NotifyStore";

class GlobalStore {
    @observable notify = new NotifyStore();
    @observable test = "Hallo";
}

const global = window.global = new GlobalStore();
export default global;