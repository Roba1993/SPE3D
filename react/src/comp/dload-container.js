import React, { Component } from 'react';
import { Segment, Button, Icon, Progress, Grid, Header } from 'semantic-ui-react'

export default class DloadContainer extends Component {
    state = { open: false };

    formatBytes(bytes, decimals) {
        if (bytes == 0) return '0 Bytes';
        var k = 1024,
            dm = decimals || 2,
            sizes = ['Bytes', 'KB', 'MB', 'GB', 'TB', 'PB', 'EB', 'ZB', 'YB'],
            i = Math.floor(Math.log(bytes) / Math.log(k));
        return parseFloat((bytes / Math.pow(k, i)).toFixed(dm)) + ' ' + sizes[i];
    }

    start_download(e, id) {
        e.preventDefault();
        fetch("http://" + window.location.hostname + ":8000/api/start-download/" + id,
            {
                method: "POST"
            })
            .then(function (res) { console.log(res) })
    }

    show_status(item) {
        if (item.status === "Downloading") {
            return this.formatBytes(item.downloaded, 2) + " downloaded";
        }
        else {
            return item.status;
        }
    }

    change_open(e) {
        e.preventDefault();
        this.setState(prevState => ({
            open: !prevState.open
        }));
    }

    render() {
        var c = this.props.container;

        if (c === undefined) {
            return <div></div>;
        }

        var size_raw = c.files.reduce((pre, curr) => pre + curr.size, 0);
        var size = this.formatBytes(size_raw, 2);
        var loaded_raw = c.files.reduce((pre, curr) => pre + curr.downloaded, 0);
        var complete = loaded_raw / size_raw * 100;
        var downloaded = c.files.reduce((pre, curr) => (curr.status == "Downloaded")? pre+=1: pre , 0);

        return <Segment.Group>
            <Segment onClick={(e) => { this.change_open(e) }}>
                <Grid columns='equal'>
                    <Grid.Column computer={1} mobile={4} style={styleCenter}>
                        <Button circular color='green' size='huge' icon='cloud download' />
                    </Grid.Column>
                    <Grid.Column computer={5} mobile={12} centered='true'>
                        <div style={styleText}>
                            <div style={styleName}>{c.name}</div>
                            <div style={stylePercent}>{complete}%</div>
                        </div>
                        <Progress percent={complete} indicating style={{ marginBottom: '0' }} />
                    </Grid.Column>
                    <Grid.Column computer={3} style={styleCenter}>
                        <Header as='h3'>0.0Mb/s</Header>
                    </Grid.Column>
                    <Grid.Column computer={2} style={styleCenter}>
                        <Header as='h3'>{size}</Header>
                    </Grid.Column>
                    <Grid.Column computer={2} style={styleCenter}>
                        <Header as='h3'>0Mins</Header>
                    </Grid.Column>
                    <Grid.Column computer={2} style={styleCenter}>
                        <Header as='h3'>{downloaded}/{c.files.length}</Header>
                    </Grid.Column>
                </Grid>
            </Segment>
            {this.state.open == true &&
                c.files.map((item, index) => (
                <Segment key={index}>
                    <Grid columns='equal' >
                        <Grid.Column computer={1} style={styleCenter}>
                            <Header as='h3' onClick={(e) => { this.change_open(e) }}>{">"}</Header>
                        </Grid.Column>
                        <Grid.Column computer={1} mobile={4} style={styleCenter}>
                            <Button circular color='green' size='huge' icon='cloud download' />
                        </Grid.Column>
                        <Grid.Column computer={4} mobile={12} centered='true'>
                            <div style={styleText}>
                                <div style={styleName}>Harry Potter und der Stein der Weisen</div>
                                <div style={stylePercent}>{item.downloaded / item.size * 100}%</div>
                            </div>
                            <Progress percent={item.downloaded / item.size * 100} indicating style={{ marginBottom: '0' }} />
                        </Grid.Column>
                        <Grid.Column computer={3} style={styleCenter}>
                            <Header as='h3'>0.0Mb/s</Header>
                        </Grid.Column>
                        <Grid.Column computer={2} style={styleCenter}>
                            <Header as='h3'>{this.formatBytes(item.size, 2)}</Header>
                        </Grid.Column>
                        <Grid.Column computer={2} style={styleCenter}>
                            <Header as='h3'>0Mins</Header>
                        </Grid.Column>
                        <Grid.Column computer={2} style={styleCenter}>
                            <Header as='h3'>{item.status}</Header>
                        </Grid.Column>
                    </Grid>
                </Segment>
                ))
            }
        </Segment.Group>
    }
}

const styleCenter = {
    display: 'flex',
    alignItems: 'center',
    justifyContent: 'center',
}

const styleText = {
    display: 'flex',
    justifyContent: 'space-between',
}

const styleName = {
    width: '80%',
    overflow: 'hidden',
    whiteSpace: 'nowrap',
    textOverflow: 'ellipsis',
    float: 'left'
}

const stylePercent = {
    float: 'right'
}