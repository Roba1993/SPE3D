import React, {Component} from 'react';
import { Table } from 'semantic-ui-react';

export default class DownloadList extends Component {

    formatBytes(bytes, decimals) {
        if(bytes == 0) return '0 Bytes';
        var k = 1024,
            dm = decimals || 2,
            sizes = ['Bytes', 'KB', 'MB', 'GB', 'TB', 'PB', 'EB', 'ZB', 'YB'],
            i = Math.floor(Math.log(bytes) / Math.log(k));
        return parseFloat((bytes / Math.pow(k, i)).toFixed(dm)) + ' ' + sizes[i];
     }

    start_download(e, id) {
        e.preventDefault();
        fetch("http://"+window.location.hostname+":8000/api/start-download/"+id,
        {
            method: "POST"
        })
        .then(function(res){ console.log(res) })
    }

    show_status(item) {
        if(item.status === "Downloading") {
            return this.formatBytes(item.downloaded, 2) + " downloaded";
        }
        else {
            return item.status;
        }
    }

    render () {
        var out = [];

        this.props.data.map((item, index) => (
                out.push({
                    key: item.id,
                    name: "> " + item.name,
                    size: this.formatBytes(item.files.reduce((pre, curr) => pre + curr.size, 0), 2),
                    status: ""
                }),

                item.files.map((item, index) => (
                    out.push({
                        key: item.id,
                        name: "- " + item.name,
                        size: this.formatBytes(item.size, 2),
                        status: this.show_status(item)
                    })
                ))
        ));


        return (<Table singleLine attached>
            <Table.Header>
                <Table.Row>
                    <Table.HeaderCell>Name</Table.HeaderCell>
                    <Table.HeaderCell>Size</Table.HeaderCell>
                    <Table.HeaderCell>Status</Table.HeaderCell>
                    <Table.HeaderCell>Actions</Table.HeaderCell>
                </Table.Row>
            </Table.Header>

            <Table.Body>
                {out.map((item, index) => (
                    <Table.Row key={item.key}>
                        <Table.Cell>{item.name}</Table.Cell>
                        <Table.Cell>{item.size}</Table.Cell>
                        <Table.Cell>{item.status}</Table.Cell>
                        <Table.Cell><a href="#" onClick={(e) => {this.start_download(e, item.key)}} >D</a></Table.Cell>
                    </Table.Row>
                ))}
            </Table.Body>
        </Table>);
    }
}