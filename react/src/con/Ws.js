import WsAddon from "./WsAddon";
import WsRest from "./WsRest";

const exportedAPI = (window.chrome && chrome.runtime && chrome.runtime.id != undefined) ? WsAddon : WsRest;
export default exportedAPI