import { computed, observable } from "mobx";

export default class UiStore {
    gloabl;
    @observable selected = false;
    @observable path = '/';

    constructor(global) {
        this.gloabl = global;
    }

    setSelected(value) {
        this.selected = value;
    }
}