import React, { Component } from 'react';
import { observer } from "mobx-react";
import { Input, Icon, Button, Header, Modal, Segment, Grid, Form } from 'semantic-ui-react'
import Dropzone from 'react-dropzone'

@observer
export default class AddAccount extends Component {
    constructor() {
        super()
        this.state = { hoster: '', username: '', password: '' }
    }

    onDrop(acceptedFiles) {
        acceptedFiles.forEach(file => {
            const reader = new FileReader();
            reader.onload = () => {
                const fileAsBinaryString = reader.result;

                fetch("http://" + window.location.hostname + ":8000/api/add-ejs",
                    {
                        method: "POST",
                        headers: {
                            'Accept': 'application/json, text/plain, */*',
                            'content-type': 'text/plain'
                        },
                        body: fileAsBinaryString
                    })
                    .then(res => {
                        if (res.status != 200) {
                            this.props.global.notify.createErrorMsg("The .ejs file is not valid", "The server was not able to interpret password file");
                        }
                        else {
                            this.props.global.notify.createOkMsg("The .ejs file is valid", "The server successfully added the accounts");
                        }
                    })
            };
            reader.onabort = () => this.props.global.notify.createErrorMsg("The .ejs file reading interrupted", "The file reading was interrupted");
            reader.onerror = () => this.props.global.notify.createErrorMsg("The .ejs file reading failed", "The file reading failed");

            reader.readAsDataURL(file);
        });
    }

    handleChange = (e, { name, value }) => this.setState({ [name]: value })

    handleSubmit = () => {
        this.props.global.config.addAccount(this.state);
        this.handleClose();
    }

    handleClose = () => {
        this.props.global.ui.modalAddAccount = false;
    }

    render() {
        return <Modal open={this.props.global.ui.modalAddAccount} onClose={this.handleClose}>
            <div style={styleGlobal} >
                <Segment style={styleSegment} >
                    <Grid columns={2} divided>
                        <Grid.Row>
                            <Grid.Column>
                                <Form onSubmit={this.handleSubmit}>
                                    <Form.Select inline label='Hoster' options={options} placeholder='Hoster' name='hoster' onChange={this.handleChange} />
                                    <Form.Input inline label='Username' placeholder='Username' name='username' value={this.state.username} onChange={this.handleChange} />
                                    <Form.Input inline label='Password' placeholder='Password' name='password' value={this.state.password} onChange={this.handleChange} />
                                    <Button type='submit'>Submit</Button>
                                </Form>
                            </Grid.Column>
                            <Grid.Column>
                                <Dropzone onDrop={this.onDrop.bind(this)} >
                                    <p>Try dropping some files here, or click to select files to upload.</p>
                                    <p>Only JDownloader .ejs account files are supported.</p>
                                </Dropzone>
                            </Grid.Column>
                        </Grid.Row>
                    </Grid>
                </Segment>
            </div>
        </Modal>
    }
}

const options = [
    { key: 'so', text: 'Share-Online.biz', value: 'ShareOnline' },
    { key: 'filer', text: 'Filer.net', value: 'Filer' },
]

const styleGlobal = {
    position: 'absolute',
    heitght: '100%',
    width: '100%',
}

const styleSegment = {
    position: 'relative',
    heitght: '90%',
    width: '90%',
    margin: '10%'
}