import { h, Component } from 'preact';
import style from './style.css';

export default class AddLinks extends Component {
    state = { links: [] };

    componentDidMount() {
		this.setState({
            links: [""],
            name: ""
        });
    }
    
    add_link = (e) => {
        e.preventDefault();
        let {links} = this.state;
        links.push("");
        this.setState({links: links});
    }

    send_links = (e) => {
        
    }
    
	render({}, {links}) {
		return (
			<div class={style.main}>
            <div class="row">
                <div class="column">
                    <form class={style.form}>
                    <fieldset>
                        <label for="name">Name</label>
                        <input type="text" placeholder="Download Container" id="name" />
                        <label for="link0">Links <button onclick={this.add_link} class="button button-outline">Add</button></label>
                        {links.map((item, index) => (
                            <input type="text" placeholder="http://www.share-online.biz/some-id" id={"link"} value={item} />
                        ))}
                        
                        <input class="button-primary" type="submit" value="Send" />
                    </fieldset>
                    </form>
                </div>
                <div class="column">
                <form class={style.form}>
                    <fieldset>
                        <label for="file">Select a file</label>
                        <input type="file" id="file" />

                        <input class="button-primary" type="submit" value="Send" />
                    </fieldset>
                    </form>
                </div>
            </div>
            </div>
		);
	}
}
