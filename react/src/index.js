import 'semantic-ui-css/semantic.min.css';

import React, { Component } from 'react';
import ReactDOM from 'react-dom';
import { HashRouter as Router, Route, Link } from 'react-router-dom';
import { Sidebar, Segment, Button, Menu, Image, Icon, Header } from 'semantic-ui-react';
import DownloadList from './comp/download-list';
import Home from './view/home';
import Links from './view/links';

class App extends Component {
    state = {
        dloads: [],
        selected: false,
    };

    componentDidMount() {
        this.loadLinks();
        this.ws();
    }

    loadLinks() {
        fetch(`http://` + window.location.hostname + `:8000/api/downloads`).then(r => r.json())
            //.then(links => console.log(links))
            .then(dloads => this.setState({ dloads: dloads }));
    }

    ws() {
        var websocket = new WebSocket('ws://' + window.location.hostname + ':8001');
        var that = this;
        websocket.onmessage = function (evt) {
            var obj = JSON.parse(evt.data);
            //console.log(obj);
            that.setState({ dloads: obj });
        };
    }

    deleteLink(e) {
        e.preventDefault();

        if (!this.state.selected) {
            return;
        }

        var id = this.state.selected;
        fetch("http://" + window.location.hostname + ":8000/api/delete-link/"+id,
            {
                method: "POST",
                headers: {
                    'Accept': 'application/json, text/plain, */*',
                    'Content-Type': 'application/json'
                },
                //body: JSON.stringify(this.state)
            })
            .then(function (res) {
                console.log(res);
            })
    }

    render() {
        return <Router>
            <Sidebar.Pushable as={Page}>
                <Sidebar as={Menu} animation='push' direction='top' visible={true} size='large' borderless>
                    <Menu.Item name='name'>
                        <Link to="/"><Header as='h2' style={styleMenu}>SPE3D</Header></Link>
                    </Menu.Item>
                    <Menu.Item name='links' style={styleButtons}>
                        <Link to="/links" style={styleButton}><Icon name='plus' size='large' /></Link>
                        <Icon name='trash' size='large' color={this.state.selected?'green':'grey'} onClick={(e) => { this.deleteLink(e) }}/>
                    </Menu.Item>
                </Sidebar>
                <Sidebar.Pusher>
                    <Segment basic style={styleSegment}>
                        <Route exact path="/" render={() => <Home dloads={this.state.dloads} selected={this.state.selected} changeSelection={(d) => { this.setState({ selected: d }) }} />} />
                        <Route path="/links" render={() => <Links />} />
                    </Segment>
                </Sidebar.Pusher>
            </Sidebar.Pushable>
        </Router>
    }
}

class Page extends Component {
    render() {
        return <div>
            {this.props.children}
        </div>
    }
}

const styleMenu = {
    color: '#00ca34',
    marginLeft: '10px'
}

const styleButtons = {
    color: '#00ca34',
    marginLeft: '200px'
}

const styleSegment = {
    color: '#00ca34',
    marginTop: '20px'
}

const styleButton = {
    color: '#00ca34',
}


ReactDOM.render(<App />, document.getElementById('app'))