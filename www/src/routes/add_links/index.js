import { h, Component } from 'preact';
import style from './style.css';

export default class AddLinks extends Component {
    state = { links: [] };

    componentDidMount() {
		this.setState({
            links: [""],
            name: "",
            file: ""
        });
    }
    
    add_link = (e) => {
        e.preventDefault();
        let {links} = this.state;
        links.push("");
        this.setState({links: links});
    }

    change_link = (id, e) => {
        e.preventDefault();
        let {links} = this.state;
        links[id] = e.target.value;
        this.setState({links: links});

        console.log(this.state.links);
    }

    send_links = (e) => {
        e.preventDefault();
        fetch("http://localhost:8000/api/add-links",
        {
            method: "POST",
            headers: {
                'Accept': 'application/json, text/plain, */*',
                'Content-Type': 'application/json'
              },
            body: JSON.stringify( this.state )
        })
        .then(function(res){ console.log(res) })
    }

    set_file = (e) => {
        this.setState({file:e.target.files[0]});
        console.log(this.state.file);
    }

    send_file = (e) => {
        e.preventDefault();

        var formData = new FormData();
        formData.append('file', this.state.file);

        fetch("http://localhost:8000/api/add-dlc",
        {
            method: "POST",
            headers: {
                'Accept': 'application/json, text/plain, */*',
                'content-type': 'text/plain'
              },
            body: formData
        })
        .then(function(res){ console.log(res) })
    }
    
	render({}, {links}) {
		return (
			<div class={style.main}>
            <div class="row">
                <div class="column">
                    <form class={style.form}>
                    <fieldset>
                        <label for="name">Name</label>
                        <input type="text" placeholder="Download Container" id="name" onInput={(e) => {this.setState({name: e.target.value})}}/>
                        <label for="link0">Links <button onclick={this.add_link} class="button button-outline">Add</button></label>
                        {links.map((item, index) => (
                            <input type="text" placeholder="http://www.share-online.biz/some-id" id={"link"} value={item} onInput={(e) => { this.change_link(index, e)}}/>
                        ))}
                        
                        <input class="button-primary" type="submit" value="Send" onclick={this.send_links}/>
                    </fieldset>
                    </form>
                </div>
                <div class="column">
                <form class={style.form}>
                    <fieldset>
                        <label for="file">Select a file</label>
                        <input type="file" id="file" onChange={this.set_file}/>

                        <input class="button-primary" type="submit" value="Send" onclick={this.send_file}/>
                    </fieldset>
                    </form>
                </div>
            </div>
            </div>
		);
	}
}
