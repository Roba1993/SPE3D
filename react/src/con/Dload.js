import DloadAddon from "./DloadAddon";
import DloadRest from "./DloadRest";

const exportedAPI = (window.chrome && chrome.runtime && chrome.runtime.id != undefined) ? DloadAddon : DloadRest;
export default exportedAPI