export default function handler(req, res) {
	if (['up', 'down', 'left', 'right', 'click'].includes(req.query.q)) {
		fetch('http://localhost:9000/run?command=' + req.query.q, {method: 'POST'});
	}
	res.status(200).json({});
}