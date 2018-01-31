import { h, Component } from 'preact';
import { Router } from 'preact-router';

import Header from './header';
import Home from '../routes/home';
import Profile from '../routes/profile';
import AddLinks from '../routes/add_links';
// import Home from 'async!../routes/home';
// import Profile from 'async!../routes/profile';

export default class App extends Component {
	state = { dloads: [] };

	/** Gets fired when the route changes.
	 *	@param {Object} event		"change" event from [preact-router](http://git.io/preact-router)
	 *	@param {string} event.url	The newly routed URL
	 */
	handleRoute = e => {
		this.currentUrl = e.url;
	};

	loadLinks() {
		fetch(`http://`+window.location.hostname+`:8000/api/downloads`).then(r => r.json())
			//.then(links => console.log(links))
			.then(dloads => this.setState({dloads: dloads}));
	}

	ws() {
		var websocket = new WebSocket('ws://'+window.location.hostname+':8001');
		var that = this;
    	websocket.onmessage = function(evt) { 
			var obj = JSON.parse(evt.data);
			console.log(obj);
			that.setState({dloads: obj});
		};
	}

	componentDidMount() {
		this.loadLinks();
		this.ws();
	}

	render({}, {dloads}) {
		return (
			<div id="app">
				<Header />

				<Router onChange={this.handleRoute}>
					<Home path="/" dloads={dloads} />
					<AddLinks path="/add-links" />
					<Profile path="/profile/" user="me" />
					<Profile path="/profile/:user" />
				</Router>
			</div>
		);
	}
}
