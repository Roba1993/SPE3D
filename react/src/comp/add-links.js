import React, { Component } from 'react';
import { observer } from "mobx-react";
import { Input, Icon, Button, Header } from 'semantic-ui-react';

@observer
export default class AddLinks extends Component {
    state = { links: [] };

    componentDidMount() {
        this.setState({
            links: [""],
            name: ""
        });
    }

    add_link(e) {
        e.preventDefault();
        let { links } = this.state;
        links.push("");
        this.setState({ links: links });
    }

    change_link(id, e) {
        e.preventDefault();
        let { links } = this.state;
        links[id] = e.target.value;
        this.setState({ links: links });
    }

    send_links(e) {
        e.preventDefault();
        fetch("http://" + window.location.hostname + ":8000/api/add-links",
            {
                method: "POST",
                headers: {
                    'Accept': 'application/json, text/plain, */*',
                    'Content-Type': 'application/json'
                },
                body: JSON.stringify(this.state)
            })
            .then(res => { 
                if (res.status != 200) {
                    this.props.global.notify.createErrorMsg("The links are not valid", "The server was not able to interpret the links");
                }
                else {
                    this.props.global.notify.createOkMsg("The links are valid", "The server successfully added the links");
                }
            })
    }

    render() {
        return <div>
            <Header as='h2'>Container</Header>
            <Input label='Name' placeholder='Container name' onChange={(e) => { this.setState({ name: e.target.value }) }} />
            <Header as='h2'>Links{' '}
                <Button animated secondary onClick={this.add_link.bind(this)}>
                    <Button.Content visible>Add</Button.Content>
                    <Button.Content hidden>
                        <Icon name='plus' />
                    </Button.Content>
                </Button>
            </Header>

            {this.state.links.map((item, index) => (
                <div key={index}>
                    <Input label='http://' placeholder="http://www.share-online.biz/some-id" onChange={(e) => { this.change_link(index, e) }} /><Icon name='remove' size="big" />
                </div>
            ))}

            <br /><Button animated primary onClick={this.send_links.bind(this)}>
                <Button.Content visible>Send</Button.Content>
                <Button.Content hidden>
                    <Icon name='right arrow' />
                </Button.Content>
            </Button>

            <br /><br /><p>Example link: http://www.share-online.biz/dl/Q1EZRULP0H</p>
        </div>
    }
}