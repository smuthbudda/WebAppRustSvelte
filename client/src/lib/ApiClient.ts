import type { AccessToken, LoggedInUserDetails, TrackPoints, UpdateUserRequest } from "./types";
import { HttpStatusCode, baseAPIUrl } from "./const";

export class APIClient {
    private baseAPIUrl: string;

    constructor(url?: string) {
        this.baseAPIUrl = url ?? baseAPIUrl;
    }

    async getResults(category: string, gender: string, event: string, time: number): Promise<TrackPoints | undefined> {
        try {
            const url = `${this.baseAPIUrl}/api/world-aths/points/${category}/${gender}/${event}?mark=${time}`;
            console.log(url);
            const response = await fetch(url);
            if (response.ok) {
                const data = await response.json();
                return data.points;
            }
            console.error("Failed to fetch results:", response.statusText);
        } catch (error) {
            console.error("Error fetching results:", error);
        }
        return undefined;
    }

    async loadDataToDB(): Promise<any> {
        try {
            const url = `${this.baseAPIUrl}/api/world-aths/read`;
            const response = await fetch(url);
            if (response.ok) {
                return await response.json();
            }
            console.error("Failed to load data:", response.statusText);
        } catch (error) {
            console.error("Error loading data:", error);
        }
        return undefined;
    }

    async userLogin(username: string, password: string): Promise<[HttpStatusCode, AccessToken | undefined]> {
        try {
            const url = `${this.baseAPIUrl}/api/auth/login`;
            const rawResponse = await fetch(url, {
                method: 'POST',
                headers: {
                    'Accept': 'application/json',
                    'Content-Type': 'application/json',
                },
                body: JSON.stringify({ user_name: username, password: password }),
            });

            if (rawResponse.ok) {
                const content: AccessToken = await rawResponse.json();
                return [HttpStatusCode.Ok, content];
            }
            console.error("Login failed:", rawResponse.statusText);
        } catch (error) {
            console.error("Error during login:", error);
        }
        return [HttpStatusCode.BadRequest, undefined];
    }    
    
    async userLogout(token: string): Promise<HttpStatusCode> {
        try {
            const url = `${this.baseAPIUrl}/api/auth/logout`;
            const rawResponse = await fetch(url, {
                method: 'GET',
                headers: {
                    'Accept': 'application/json',
                    'Content-Type': 'application/json',
                    'Authorization': `Bearer ${token}`,
                }
            });

            if (rawResponse.ok) {
                return HttpStatusCode.Ok;
            }
            console.error("Logout Failed:", rawResponse.statusText);
        } catch (error) {
            console.error("Error during login:", error);
        }
        return HttpStatusCode.BadRequest;
    }

    async getMyDetails(token: string): Promise<[HttpStatusCode, LoggedInUserDetails?]> {
        try {
            const url = `${this.baseAPIUrl}/api/user/me`;
            const result = await fetch(url, {
                method: 'GET',
                headers: {
                    'Content-Type': 'application/json',
                    'Authorization': `Bearer ${token}`,
                },
            });

            if (result.ok) {
                const content = await result.json();
                const user = content.data.user;

                if (!user?.user_name) {
                    console.log('User is undefined');
                    return [HttpStatusCode.BadRequest, undefined];
                }

                // console.log(content);
                return [HttpStatusCode.Ok, user];
            }
            console.error("Failed to get user details:", result.statusText);
        } catch (error) {
            console.error("Error fetching user details:", error);
        }
        return [HttpStatusCode.BadRequest, undefined];
    }

    async updateMyDetails(token: string, myDetails: UpdateUserRequest, id: number): Promise<[HttpStatusCode, LoggedInUserDetails?]> {
        try {
            const url = `${this.baseAPIUrl}/api/user/update/${id}`;
            const result = await fetch(url, {
                method: 'PUT',
                headers: {
                    'Content-Type': 'application/json',
                    'Authorization': `Bearer ${token}`,
                },
                body: JSON.stringify({
                    user_name: myDetails.user_name,
                    first_name: myDetails.first_name,
                    last_name: myDetails.last_name,
                    email: myDetails.email,
                    phone: myDetails.phone,
                }),
            });

            if (result.ok) {
                const content = await result.json();
                return [HttpStatusCode.Ok, content.data.user];
            }
            console.error("Failed to update user details:", result.statusText);
        } catch (error) {
            console.error("Error updating user details:", error);
        }
        return [HttpStatusCode.BadRequest, undefined];
    }

    async createNewUser(user_name: string, first_name: string, last_name: string, email: string, phone: string, password: string): Promise<[HttpStatusCode, string | undefined]> {
        try {
            const url = `${this.baseAPIUrl}/api/user/register`;
            const rawResponse = await fetch(url, {
                method: 'POST',
                headers: {
                    'Accept': 'application/json',
                    'Content-Type': 'application/json',
                },
                body: JSON.stringify({
                    user_name,
                    first_name,
                    last_name,
                    email,
                    phone,
                    password,
                }),
            });

            if (rawResponse.ok) {
                const content = await rawResponse.json();
                return [HttpStatusCode.Created, content];
            }
            console.error("Failed to create new user:", rawResponse.statusText);
        } catch (error) {
            console.error("Error creating new user:", error);
        }
        return [HttpStatusCode.BadRequest, undefined];
    }

    async addUserPoints(token: string) {
        try{
            const url = `${this.baseAPIUrl}/api/user/register`;
            const rawResponse = await fetch(url, {
                method: 'POST',
                headers: {
                    'Accept': 'application/json',
                    'Content-Type': 'application/json',
                }
            });

            if (rawResponse.ok) {
                const content = await rawResponse.json();
                return [HttpStatusCode.Created, content];
            }
            console.error("Failed to create new user:", rawResponse.statusText);
        }catch(error){

        }
    }
}
