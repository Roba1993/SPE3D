import { h, Component } from 'preact';
import style from './style.css';

export default class DropZone extends Component {
    state = { box_class: [style.box] };

    isAdvancedUpload() {
        var div = document.createElement('div');
        return (('draggable' in div) || ('ondragstart' in div && 'ondrop' in div)) && 'FormData' in window && 'FileReader' in window;
    }

    componentDidMount() {
        if(this.isAdvancedUpload()) {
            this.setState({box_class: [...this.state.box_class, style.has_advanced_upload]});
        }
    }

	render({dloads}) {
		return (
			<form class={this.state.box_class} method="post" action="" enctype="multipart/form-data">
                <div class={style.box_input}>
                    <input class={style.box_file} type="file" name="files[]" id="file" data-multiple-caption="{count} files selected" multiple />
                    <label for="file"><strong>Choose a file</strong><span class={style.box_dragndrop}> or drag it here</span>.</label>
                    <button class={style.box_button} type="submit">Upload</button>
                </div>
                <div class={style.box_uploading}>Uploading&hellip;</div>
                <div class={style.box_success}>Done!</div>
                <div class={style.box_error}>Error! <span></span>.</div>
            </form>
		);
	}
}
