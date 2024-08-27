import type { AccessToken, LoggedInUserDetails, TrackPoints } from "./types";
import { HttpStatusCode, baseAPIUrl } from "./const";

//is dependancy injection a thing in Svelte? 
export class APIClient {
    private baseAPIUrl: string

    constructor(url?: string) {
        this.baseAPIUrl = url ?? baseAPIUrl;
    }

    async getResults(category: string, gender: string, event: string, time: number): Promise<TrackPoints | undefined> {
        const url = `${this.baseAPIUrl}/api/world-aths/points/${category}/${gender}/${event}?mark=${time}`;
        console.log(url);
        const response = await fetch(url);
        const data = await response.json();
        return data.points;
    }

    async loadDataToDB() {
        const url = `${this.baseAPIUrl}/api/world-aths/read`;
        const response = await fetch(url);
        const data = await response.json();
        return data;
    }

    async userLogin(username: string, password: string): Promise<[HttpStatusCode, AccessToken]> {
        const url = `${this.baseAPIUrl}/api/auth/login`
        const rawResponse = await fetch(url, {
            method: 'POST',
            headers: {
                'Accept': 'application/json',
                'Content-Type': 'application/json'
            },
            body: JSON.stringify({
                user_name: username,
                password: password,
            }),
        });
        const content: AccessToken = await rawResponse.json();
        return [HttpStatusCode.Accepted, content]
    }

    async getMyDetails(token:string): Promise<[HttpStatusCode, LoggedInUserDetails]> {
        const url = `${this.baseAPIUrl}/api/user/me`
        const result = await fetch(url, {
            method: 'GET',
            headers: {
                'Content-Type': 'application/json',
                'Authorization': `Bearer ${token}`
            }
        });
        const content = await result.json();
        let x = content.data.user
        return [HttpStatusCode.Accepted, x]
    }

    async createNewUser(username: string, password: string): Promise<[HttpStatusCode, string]> {
        const url = `${this.baseAPIUrl}/api/user/login`
        const rawResponse = await fetch(url, {
            method: 'POST',
            headers: {
                'Accept': 'application/json',
                'Content-Type': 'application/json'
            },
            body: JSON.stringify({
                user_name: username,
                password: password,
            }),
        });
        const content = await rawResponse.json();
        return [HttpStatusCode.Accepted, content]
    }
}


