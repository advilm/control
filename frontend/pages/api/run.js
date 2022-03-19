export default function handler(req, res) {
	if (['up', 'down', 'left', 'right'].includes(req.query.q)) {
		fetch('http://localhost:3000/run?command=' + req.query.q, {method: 'POST'});
	}
	res.status(200).json({});
}