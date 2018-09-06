import React, { Component } from 'react';
import { observer } from "mobx-react";
import { Segment, Button, Icon, Progress, Grid, Header } from 'semantic-ui-react'

@observer
export default class DloadContainer extends Component {
    state = { open: false };

    start_download(e, id) {
        e.preventDefault();
        this.props.global.dload.startDload(id);
    }

    change_open(e) {
        e.preventDefault();
        this.setState(prevState => ({
            open: !prevState.open
        }));
        
        this.set_selection(e, this.props.container.id);
    }

    set_selection(e, id) {
        e.preventDefault();

        this.props.global.ui.setSelected(id);
    }

    get_selected(id) {
        if (id != this.props.global.ui.selected) {
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

        return <Segment.Group>
            <Segment {...this.get_selected(c.id)} onClick={(e) => { this.change_open(e) }}>
                <Grid columns='equal'>
                    <Grid.Column computer={1} mobile={4} style={styleCenter}>
                        <Button circular color='green' size='huge' style={styleButton} onClick={(e) => { this.start_download(e, c.id) }}
                            icon={c.icon}
                            loading={c.isDownloading}
                        />
                    </Grid.Column>
                    <Grid.Column computer={5} mobile={12} centered='true'>
                        <div style={styleText}>
                            <div style={styleName}>{c.name}</div>
                            <div style={stylePercent}>{c.downloadedPercent}%</div>
                        </div>
                        <Progress percent={c.downloadedPercent} indicating={c.isDownloading} success={c.isDownloaded} warning={c.isWarning} style={{ marginBottom: '0' }} />
                    </Grid.Column>
                    <Grid.Column computer={3} style={styleCenter}>
                        <Header as='h3'>{c.speedFmt}/s</Header>
                    </Grid.Column>
                    <Grid.Column computer={2} style={styleCenter}>
                        <Header as='h3'>{c.sizeFmt}</Header>
                    </Grid.Column>
                    <Grid.Column computer={2} style={styleCenter}>
                        <Header as='h3'>{c.downloadTime}</Header>
                    </Grid.Column>
                    <Grid.Column computer={2} style={styleCenter}>
                        <Header as='h3'>{c.finishedDownloads}/{c.files.length}</Header>
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
                                    icon={item.icon}
                                    loading={item.isDownloading}
                                />
                            </Grid.Column>
                            <Grid.Column computer={4} mobile={12} centered='true'>
                                <div style={styleText}>
                                    <div style={styleName}>{item.name}</div>
                                    <div style={stylePercent}>{item.downloadedPercent}%</div>
                                </div>
                                <Progress percent={item.downloadedPercent} indicating={item.isDownloading} success={item.isDownloaded} error={item.isWarning} style={{ marginBottom: '0' }} />
                            </Grid.Column>
                            <Grid.Column computer={3} style={styleCenter}>
                                <Header as='h3'>{item.speedFmt}/s</Header>
                            </Grid.Column>
                            <Grid.Column computer={2} style={styleCenter}>
                                <Header as='h3'>{item.sizeFmt}</Header>
                            </Grid.Column>
                            <Grid.Column computer={2} style={styleCenter}>
                                <Header as='h3'>{item.downloadTime}</Header>
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