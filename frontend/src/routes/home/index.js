import { h, Component } from 'preact';
import { Link } from 'preact-router/match';
import DTable from '../../components/dload_table';
import style from './style';

export default class Home extends Component {
	render({dloads}) {
		return (
			<div class={style.home}>
				<h2 class={style.h2}>Downloads <Link class={style.h2_link} href="/add-links">Add-Links</Link></h2>
				<div class="row">
					<div class="column"><DTable dloads={dloads}/></div>
				</div>
			</div>
		);
	}
}
