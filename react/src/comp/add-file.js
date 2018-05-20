import React, { Component } from 'react';
import { observer } from "mobx-react";
import { Input, Icon, Button, Header } from 'semantic-ui-react'
import Dropzone from 'react-dropzone'

@observer
export default class AddFile extends Component {
    constructor() {
        super()
        this.state = { files: [] }
    }

    onDrop(acceptedFiles) {
        acceptedFiles.forEach(file => {
            const reader = new FileReader();
            reader.onload = () => {
                const fileAsBinaryString = reader.result;

                fetch("http://" + window.location.hostname + ":8000/api/add-dlc",
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
                            this.props.global.notify.createErrorMsg("The .dlc file is not valid", "The server was not able to interpret the .dlc file");
                        }
                        else {
                            this.props.global.notify.createOkMsg("The .dlc file is valid", "The server successfully added the .dlc file");
                        }
                    })
            };
            reader.onabort = () => this.props.global.notify.createErrorMsg("The .dlc file reading interrupted", "The file reading was interrupted");
            reader.onerror = () => this.props.global.notify.createErrorMsg("The .dlc file reading failed", "The file reading failed");

            reader.readAsBinaryString(file);
        });
    }

    render() {
        return <div>
            <Dropzone onDrop={this.onDrop.bind(this)}>
                <p>Try dropping some files here, or click to select files to upload.</p>
            </Dropzone>
        </div>
    }
}