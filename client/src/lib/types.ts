export interface TrackPoints{
    Category: string;
    Event: string;
    Gender: string;
    Mark: number;
    Points: number;
};


export interface CpuInfo {
    brand: string;
    id: string;
    name: string;
}

export interface SystemInfo {
    cpu_info: CpuInfo;
    disks: string[];
    os: string;
    ram_total: number;
}

export interface Gender {
    Value: number;
    Name: string;
  }
  
export interface Category {
    Value: number;
    Name: string;
  }

export interface AccessToken {
    status: string,
    access_token: string
}

export interface LoggedInUserDetails{
    id: number,
    first_name: string,
    last_name: string,
    user_name: string,
    email: string,
    phone: string,
    password: string,
}

// Define the TypeScript type
export interface UpdateUserRequest {
    first_name: string;
    last_name: string;
    user_name: string;
    email: string;
    phone?: string | null; // phone is optional and can be null
}

export interface State{ 
    user_name: string, 
    id : number,
    first_name: string,
    last_name: string,
    email: string,
    phone: string,
    password: string,
}
