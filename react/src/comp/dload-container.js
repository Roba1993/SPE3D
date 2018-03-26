import React, { Component } from 'react';
import { Segment, Button, Icon, Progress, Grid, Header } from 'semantic-ui-react'

export default class DloadContainer extends Component {
    state = { open: false };

    formatBytes(bytes, decimals) {
        if (bytes == 0) return '0 Bytes';
        var k = 1024,
            dm = decimals || 2,
            sizes = ['Bytes', 'Kb', 'Mb', 'Gb', 'Tb', 'Pb', 'Eb', 'Zb', 'Yb'],
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

    formatTime(seconds) {
        if (seconds < 60) return seconds + ' Seconds';

        var min = (seconds / 60).toFixed(0);
        if (min < 60) {
            return min + " Minutes";
        }

        var hours = (min / 60).toFixed(0);
        if (hours < 24) {
            return hours + " Hours";
        }

        return (hours / 24).toFixed(2) + " Days";
    }

    change_open(e) {
        e.preventDefault();
        this.setState(prevState => ({
            open: !prevState.open
        }));
        
        if (!this.state.open) {
            this.set_selection(e, this.props.container.id);
        }
        else {
            this.set_selection(e, false);
        }
    }

    set_selection(e, id) {
        e.preventDefault();

        if (id == this.props.selected) {
            this.props.changeSelection(false);
        }
        else {
            this.props.changeSelection(id);
        }
    }

    get_selected(id) {
        if (id != this.props.selected) {
            return;
        }

        return {
            color: 'olive',
            inverted: true
        }
    }

    render() {
        var c = this.props.container;

        if (c === undefined) {
            return <div></div>;
        }

        console.log(this.props.selected);

        var size_raw = c.files.reduce((pre, curr) => pre + curr.size, 0);
        var size = this.formatBytes(size_raw, 2);
        var loaded_raw = c.files.reduce((pre, curr) => pre + curr.downloaded, 0);
        var complete = (loaded_raw / size_raw * 100).toFixed(0);
        var downloaded = c.files.reduce((pre, curr) => (curr.status == "Downloaded") ? pre += 1 : pre, 0);
        var indi = c.files.some(f => f.status == "Downloading");
        var success = c.files.every(f => f.status == "Downloaded");
        var warning = c.files.some(f => f.status == "WrongHash");
        var speed_raw = c.files.reduce((pre, curr) => pre + curr.speed, 0);
        var speed = this.formatBytes(speed_raw, 2);
        var mins = (speed_raw != 0) ? this.formatTime((size_raw / speed_raw)) : (success) ? 'Done' : 'Not Started';

        return <Segment.Group>
            <Segment {...this.get_selected(c.id)} onClick={(e) => { this.change_open(e) }}>
                <Grid columns='equal'>
                    <Grid.Column computer={1} mobile={4} style={styleCenter}>
                        <Button circular color='green' size='huge' style={styleButton} onClick={(e) => { this.start_download(e) }}
                            icon={(success) ? 'check' :
                                (warning) ? 'warning sign' :
                                    (indi) ? 'spinner' : 'arrow down'}
                            loading={indi}
                        />
                    </Grid.Column>
                    <Grid.Column computer={5} mobile={12} centered='true'>
                        <div style={styleText}>
                            <div style={styleName}>{c.name}</div>
                            <div style={stylePercent}>{complete}%</div>
                        </div>
                        <Progress percent={complete} indicating={indi} success={success} warning={warning} style={{ marginBottom: '0' }} />
                    </Grid.Column>
                    <Grid.Column computer={3} style={styleCenter}>
                        <Header as='h3'>{speed}/s</Header>
                    </Grid.Column>
                    <Grid.Column computer={2} style={styleCenter}>
                        <Header as='h3'>{size}</Header>
                    </Grid.Column>
                    <Grid.Column computer={2} style={styleCenter}>
                        <Header as='h3'>{mins}</Header>
                    </Grid.Column>
                    <Grid.Column computer={2} style={styleCenter}>
                        <Header as='h3'>{downloaded}/{c.files.length}</Header>
                    </Grid.Column>
                </Grid>
            </Segment>
            {this.state.open == true &&
                c.files.map((item, index) => (
                    <Segment key={index} {...this.get_selected(item.id)} onClick={(e) => { this.set_selection(e, item.id) }}>
                        <Grid columns='equal' >
                            <Grid.Column computer={1} style={styleCenter}>
                                <Header as='h3' onClick={(e) => { this.change_open(e) }}><Icon name="angle double right" /></Header>
                            </Grid.Column>
                            <Grid.Column computer={1} mobile={4} style={styleCenter}>
                                <Button circular color='green' size='huge' style={styleButton} onClick={(e) => { this.start_download(e, item.id) }}
                                    icon={(item.status == "Downloaded") ? 'check' :
                                        (item.status == "Online") ? 'arrow down' :
                                            (item.status == "WrongHash") ? 'close' :
                                                (item.status == "Downloading") ? 'spinner' : 'arrow down'}
                                    loading={item.status == "Downloading"}
                                />
                            </Grid.Column>
                            <Grid.Column computer={4} mobile={12} centered='true'>
                                <div style={styleText}>
                                    <div style={styleName}>{item.name}</div>
                                    <div style={stylePercent}>{(item.downloaded / item.size * 100).toFixed(0)}%</div>
                                </div>
                                <Progress percent={item.downloaded / item.size * 100} indicating={item.status == "Downloading"} success={item.status == "Downloaded"} error={item.status == "WrongHash"} style={{ marginBottom: '0' }} />
                            </Grid.Column>
                            <Grid.Column computer={3} style={styleCenter}>
                                <Header as='h3'>{this.formatBytes(item.speed, 2)}/s</Header>
                            </Grid.Column>
                            <Grid.Column computer={2} style={styleCenter}>
                                <Header as='h3'>{this.formatBytes(item.size, 2)}</Header>
                            </Grid.Column>
                            <Grid.Column computer={2} style={styleCenter}>
                                <Header as='h3'>{(item.speed != 0) ? this.formatTime((item.size / item.speed)) : (success) ? 'Done' : 'Not Started'}</Header>
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

const styleButton = {
    boxShadow: '0 4px 8px 0 rgba(0, 0, 0, 0.2), 0 6px 20px 0 rgba(0, 0, 0, 0.19)'
}