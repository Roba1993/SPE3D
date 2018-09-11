import { computed, observable } from "mobx";

export default class UiStore {
    gloabl;
    @observable selected = false;
    @observable path = '/';
    @observable modalAddAccount = false;
    @observable accountSelected = false;
    @observable configTab = 'server'

    constructor(global) {
        this.gloabl = global;
    }

    setSelected(value) {
        this.selected = value;
    }
}