import React, {Component} from 'react';
import DownloadList from '../comp/download-list';

export default class Home extends Component {
    render () {
        return <DownloadList data={this.props.dloads}/>
    }
}