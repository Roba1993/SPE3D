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
                                    {item.files.reduce((pre, curr) => 
                                        pre + curr.size, 0
                                    )} Byte
                                </td>
                                <td></td>
                                <td><a href="#" onclick={(e) => {this.start_download(e, item.id)}} >D</a></td>
                            </tr>

                            {item.files.map((item, index) => (
                                <tr>
                                    <td>&nbsp;&nbsp;&nbsp;&nbsp;-&nbsp;{item.name}</td>
                                    <td>{item.size} Byte</td>
                                    <td>{item.status}</td>
                                    <td><a href="#" onclick={(e) => {this.start_download(e, item.id)}} >D</a></td>
                                </tr>
                            ))}
                        </tbody>
                    ))}
            </table>
		);
	}
}
