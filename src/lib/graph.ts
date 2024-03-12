import type { User } from '$lib/types';
import '../app.css';
import { invoke } from '@tauri-apps/api/tauri';

async function login() {
    await invoke('authenticate');
}
async function check_login(): Promise<User> {
    const details: User = await invoke('get_latest_token');
    console.log(details.token.access_token, details.token.refresh_token);
    return details;
}

// Function to make a request to the Microsoft Graph API using 
async function makeGraphApiRequest(user: User, endpoint: string): Promise<any> { // eslint-disable-line @typescript-eslint/no-explicit-any
    const url = `https://graph.microsoft.com/v1.0/${endpoint}`;

    const requestOptions: RequestInit = {
        method: 'GET',
        headers: {
            Authorization: `Bearer ${user.token.access_token}`
        }
    };

    try {
        const response = await fetch(url, requestOptions);

        if (!response.ok) {
            throw new Error(`HTTP error! Status: ${response.status}, body: ${await response.text()}`);
        }

        return await response.json();
    } catch (error) {
        // Handle error
        console.error('Error making Microsoft Graph API request:', error);
        throw error;
    }
}
async function make_graph_request() {
    const details: User = await check_login();
    const resp = await makeGraphApiRequest(details, 'me');
    console.log(resp);
}
async function refresh() {
    const details: boolean = await invoke('refresh');
    if (details) {
        console.log('Refreshed');
    } else {
        console.log('Refresh failed');
    }
}

export { login, make_graph_request, refresh };