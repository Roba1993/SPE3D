import { computed, observable } from "mobx";

export default class UiStore {
    @observable selected = false;

    setSelected(value) {
        this.selected = value;
    }
}