import React, { Component } from 'react';
import { Input, Icon, Button, Header } from 'semantic-ui-react'
import Dropzone from 'react-dropzone'

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
                    .then(function (res) { console.log(res) })
            };
            reader.onabort = () => console.log('file reading was aborted');
            reader.onerror = () => console.log('file reading has failed');

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