import React, { Component } from 'react'
import { observer } from "mobx-react"
import { Form, Segment } from 'semantic-ui-react'

@observer
export default class ConfigAddon extends Component {
    handleChange = (e, { name, value }) => {
        switch (name) {
            case 'server':
                chrome.storage.local.set({ 'spe3d_server': value });
                this.props.global.server = value;
                break;
        }
    }

    render() {
        if (this.props.global.addon != true) {
            return (<Segment>No Addon Config available if you don't use the Addon</Segment>);
        }

        return (
            <Segment>
                <Form onSubmit={this.handleSubmit}>
                    <Form.Input label='Server' name='server' value={this.props.global.server} onChange={this.handleChange} />
                    <Form.Button content='Save' />
                </Form>
            </Segment>
        )
    }
}