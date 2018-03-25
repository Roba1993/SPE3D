import 'semantic-ui-css/semantic.min.css';

import React, {Component} from 'react';
import ReactDOM from 'react-dom';
import {
  HashRouter as Router,
  Route,
  Link
} from 'react-router-dom';


import { Sidebar, Segment, Button, Menu, Image, Icon, Header } from 'semantic-ui-react';
import DownloadList from './comp/download-list';
import Home from './view/home';
import Links from './view/links';

class App extends Component {
	state = { dloads: [] };

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

    render () {
        return <Router>
                <Sidebar.Pushable as={Page}>
                    <Sidebar as={Menu} animation='push' direction='top' visible={true} inverted>
                        <Menu.Item name='home'>
                            <Link to="/"><Icon name='home' /> Home</Link>
                        </Menu.Item>
                        <Menu.Item name='links'>
                            <Link to="/links"><Icon name='write' />Links</Link>
                        </Menu.Item>
                    </Sidebar>
                    <Sidebar.Pusher>
                        <Segment basic>
                            <Route exact path="/" render={()=><Home dloads={this.state.dloads}/>}/>
                            <Route path="/links" render={()=><Links/>}/>
                        </Segment>
                    </Sidebar.Pusher>
                </Sidebar.Pushable>
        </Router>
    }
}

class Page extends Component {
    render () {
        return <div>
            {this.props.children}
        </div>
    }
}

ReactDOM.render(<App />, document.getElementById('app'))