import React from 'react';
import { MantineProvider } from '@mantine/core';
import { Button, Grid, Container } from '@mantine/core';
export default function App() {
	return (
		<MantineProvider theme={{ colorScheme: 'dark' }}>
			<Container size={300}>
				<Grid columns={3}>
					<Grid.Col span={1}></Grid.Col>
					<Grid.Col span={1}>
						<Button fullWidth='50' onClick={() => fetch('/api/run?q=up')}>Up</Button>
					</Grid.Col>
					<Grid.Col span={1}></Grid.Col>
					<Grid.Col span={1}>
						<Button fullWidth='50' onClick={() => fetch('/api/run?q=left')}>Left</Button>
					</Grid.Col>
					<Grid.Col span={1}>
						<Button fullWidth='50' onClick={() => fetch('/api/run?q=click')}>Click</Button>
					</Grid.Col>
					<Grid.Col span={1}>
						<Button fullWidth='50' onClick={() => fetch('/api/run?q=right')}>Right</Button>
					</Grid.Col>
					<Grid.Col span={1}></Grid.Col>
					<Grid.Col span={1}>
						<Button fullWidth='50' onClick={() => fetch('/api/run?q=down')}>Down</Button>
					</Grid.Col>
				</Grid>
			</Container>
		</MantineProvider>
	);
}