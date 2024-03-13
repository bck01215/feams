import type { User } from '$lib/types';
import '../app.css';
import { invoke } from '@tauri-apps/api/tauri';
import { writable } from 'svelte/store';

const currentUser = writable<User | null>(null);
async function login() {
    await invoke('authenticate');
}
async function check_login(): Promise<User> {
    const user: User = await invoke('get_latest_token');
    currentUser.set(user);
    console.log(user.token.access_token, user.token.refresh_token);
    return user;
}

// Function to make a request to the Microsoft Graph API using 
async function make_graph_api_request(user: User, endpoint: string): Promise<Blob> {
    if (user.login_date + user.token.expires_in < Math.floor(Date.now() / 1000)) {
        const refreshResp = await refresh();
        if (refreshResp === null) {
            throw new Error('Refresh failed after evaluating token');
        }
        user = refreshResp;
    }
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

        return await response.blob();
    } catch (error) {
        // Handle error
        console.error('Error making Microsoft Graph API request:', error);
        throw error;
    }
}
async function make_graph_request() {
    const details: User = await check_login();
    const resp = await make_graph_api_request(details, 'me');
    console.log(resp);
}
async function refresh(): Promise<User | null> {
    const details: User | null = await invoke('refresh');
    if (details !== null) {
        console.log('Refreshed');
    } else {
        console.log('Refresh failed');
    }
    return details;
}


async function get_my_photo(user : User): Promise<Blob> {
    const resp = await make_graph_api_request(user, 'me/photo/$value');
    console.log(resp);
    return resp;
}
    


export { login, check_login, make_graph_request, refresh, currentUser, get_my_photo };