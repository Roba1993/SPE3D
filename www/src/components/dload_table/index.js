import { h, Component } from 'preact';
import style from './style.css';

export default class TableDownload extends Component {

    start_download = (e, id) => {
        e.preventDefault();
        fetch("http://localhost:8000/api/start-download/"+id,
        {
            method: "POST"
        })
        .then(function(res){ console.log(res) })
    }

    show_status = (s) => {
        if(typeof s === "string") {
            return s;
        }
        else if('Downloading' in s) {
            return this.formatBytes(s["Downloading"], 2) + " downloaded";
        }
        else {
            return "";
        }
    }

    formatBytes = (bytes, decimals) => {
        if(bytes == 0) return '0 Bytes';
        var k = 1024,
            dm = decimals || 2,
            sizes = ['Bytes', 'KB', 'MB', 'GB', 'TB', 'PB', 'EB', 'ZB', 'YB'],
            i = Math.floor(Math.log(bytes) / Math.log(k));
        return parseFloat((bytes / Math.pow(k, i)).toFixed(dm)) + ' ' + sizes[i];
     }

	render({dloads}) {
		return (
			<table class={style.table}>
                <thead>
                <tr>
                    <th>Name</th>
                    <th>Size</th>
                    <th>Status</th>
                    <th>Actions</th>
                </tr>
                </thead>
                    {dloads.map((item, index) => (
                        <tbody>
                            <tr class={style.bold}>
                                <td>> {item.name}</td>
                                <td>
                                    {this.formatBytes(
                                        item.files.reduce((pre, curr) => 
                                            pre + curr.size, 0
                                        ), 
                                        2
                                    )}
                                </td>
                                <td></td>
                                <td><a href="#" onclick={(e) => {this.start_download(e, item.id)}} >D</a></td>
                            </tr>

                            {item.files.map((item, index) => (
                                <tr>
                                    <td>&nbsp;&nbsp;&nbsp;&nbsp;-&nbsp;{item.name}</td>
                                    <td>{this.formatBytes(item.size, 2)}</td>
                                    <td>{this.show_status(item.status)}</td>
                                    <td><a href="#" onclick={(e) => {this.start_download(e, item.id)}} >D</a></td>
                                </tr>
                            ))}
                        </tbody>
                    ))}
            </table>
		);
	}
}
