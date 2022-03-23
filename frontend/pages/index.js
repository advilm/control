import React from 'react';
import { MantineProvider } from '@mantine/core';
import { Button, Grid, Container } from '@mantine/core';
export default function App() {
	const run = cmd => fetch(`/run?command=${cmd}`, { method: 'POST' });

	return (
		<MantineProvider theme={{ colorScheme: 'dark' }}>
			<Container size={300}>
				<Grid columns={3}>
					<Grid.Col span={1}></Grid.Col>
					<Grid.Col span={1}>
						<Button fullWidth='50' onClick={() => run('up')}>Up</Button>
					</Grid.Col>
					<Grid.Col span={1}></Grid.Col>
					<Grid.Col span={1}>
						<Button fullWidth='50' onClick={() => run('left')}>Left</Button>
					</Grid.Col>
					<Grid.Col span={1}>
						<Button fullWidth='50' onClick={() => run('click')}>Click</Button>
					</Grid.Col>
					<Grid.Col span={1}>
						<Button fullWidth='50' onClick={() => run('right')}>Right</Button>
					</Grid.Col>
					<Grid.Col span={1}></Grid.Col>
					<Grid.Col span={1}>
						<Button fullWidth='50' onClick={() => run('down')}>Down</Button>
					</Grid.Col>
				</Grid>
			</Container>
		</MantineProvider>
	);
}