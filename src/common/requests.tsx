import axios from "axios";

const generate_path = (path: string, query: string | null): string => {
    const base = import.meta.env.VITE_API_URL;
    if (query) {
        return `${base}${path}?${query}`;
    }

    return `${base}${path}`;
}

export const get = async (path: string, query: string | null): Promise<any> => {
    try {
        const url = generate_path(path, query);
        const headers = {
            'Content-Type': 'application/json',
        };
        console.log('to get', url);
        const response = await axios.get(url, { headers });
        return response.data;
    } catch (error) {
        return error;
    }
}


export const post = async (path: string, query: string | null, data: any): Promise<any> => {
    try {
        const url = generate_path(path, query);
        const headers = {
            'Content-Type': 'application/json',
        };
        console.log('to post', url, data);
        const response = await axios.post(url, data, { headers });
        return response.data;
    } catch (error) {
        return error;
    }
}
